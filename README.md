# jql – A Tiny jq Clone in Rust

`jql` is a minimal command-line JSON query tool written in Rust.  
It allows you to extract values from JSON files or stdin using simple dot notation and single-level array indices.

## Features
- Query JSON from **files** or **stdin**.
- Supports **dot paths**: `.user.name`
- Supports **array indices**: `.items[0]`
- Lightweight and easy to extend.

## Installation
Clone and build with Cargo:

```bash
git clone https://github.com/Cl9s4bl3/jql.git
cd jql
cargo build --release
```
Optionally you can move it to the /usr/bin to access it everywhere using "jql"
```
sudo cp target/release/jql /usr/bin
```

## Usage

```
jql main.json .user.name

cat main.json | jql .items[0]
```
