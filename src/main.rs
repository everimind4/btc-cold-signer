mod param;
mod tx;

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let param_json = fs::read_to_string(path)
        .expect("cannot read the file");

    let parsed: param::Params = serde_json::from_str(&param_json)
        .expect("failed parse json");

    let transaction = tx::build_tx(&parsed);
    println!("{:#?}", transaction);
}
