use std::io::{self, Read};
use std::env;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let (filename_opt, query) = match args.len() {
        2 => (None, &args[1]),
        3 => (Some(&args[1]), &args[2]),
        _ => {
            eprintln!("Usage: jql [filename] <query>");
            std::process::exit(1);
        }
    };

    let mut buffer = String::new();

    if let Some(filename) = filename_opt {
        buffer = std::fs::read_to_string(filename)?;
    } else {
        io::stdin().read_to_string(&mut buffer)?;
        if buffer.trim().is_empty() {
            eprintln!("No input provided");
            std::process::exit(1);
        }
    }

    let mut v: &Value = &serde_json::from_str(&buffer)?;

    let parts = query.trim_start_matches('.').split('.');

    for part in parts {
        if let Some((field, idx)) = part.split_once('[') {
            v = &v[field];
            let idx = idx.trim_end_matches(']').parse::<usize>()?;
            v = &v[idx];
        } else {
            v = &v[part];
        }
    }

    println!("{}", v);
    Ok(())
}
