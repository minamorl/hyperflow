extern crate serde_json;

use std::fs::File;
use serde_json::{Value, Error, Map};
use std::io::{self, Read, BufRead, Write};

fn parse_json(path: &str) -> Result<Map<String, Value>, Error> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let result = serde_json::from_str(contents.as_str());
    result
}


fn main() {
    let contents = parse_json("misc/dictionary.json").unwrap();
    print!("> ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut buffer).unwrap();

    println!("{}", contents[buffer.as_str().trim()]);
}
