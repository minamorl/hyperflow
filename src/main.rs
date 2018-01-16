extern crate serde_json;

use std::fs::File;
use serde_json::{Value, Error, Map};
use std::io::{self, Read, BufRead, Write};
use std::fmt;


#[derive(Debug)]
enum InternalError {
    DictionaryNotFound,
    ParseFailed(Error)
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InternalError::DictionaryNotFound => "Dictionary not found".fmt(f),
            InternalError::ParseFailed(ref e) => e.fmt(f)
        }
    }
}

fn parse_json(path: &str) -> Result<Map<String, Value>, InternalError> {
    let mut file = File::open(path)
        .map_err(|_| InternalError::DictionaryNotFound)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| InternalError::DictionaryNotFound)?;
    let result = serde_json::from_str(contents.as_str())
        .map_err(|e| InternalError::ParseFailed(e));
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
