//use std::fs;

use rust_json_parser::parse_json;

fn main() {
    //let input = fs::read_to_string("./test_data/sample_json1").expect("could not find file");
    let input = r#""missing end quote"#;

    let result = parse_json(&input);

    match result {
        Ok(value) => println!("Parsed successfully: {:?}", value),
        Err(error) => println!("Parsed error: {}", error),

    }
}
