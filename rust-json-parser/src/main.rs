use std::fs;

use rust_json_parser::tokenizer::tokenize;

fn main() {
    let input = fs::read_to_string("./test_data/sample_json1").expect("could not find file");

    let tokens = tokenize(&input);

    println!("{:?}", tokens);
}
