// Week 2: JSON Parser with Error Handling

// Declare modules - tell Rust which files contain your code
// `mod error;` looks for src/error.rs
mod error;
mod parser;
mod tokenizer;
mod value;

// Re-export types - make them accessible from the top level
// Without this: users write `use my_lib::parser::parse_json`
// With this: users write `use my_lib::parse_json` (cleaner!)
pub use error::JsonError;
pub use parser::parse_json;
pub use tokenizer::{Token, tokenize};
pub use value::JsonValue;

// Type alias for convenience
// Users can write Result<JsonValue> instead of std::result::Result<JsonValue, JsonError>
pub type Result<T> = std::result::Result<T, JsonError>;

#[cfg(test)]
mod tests {
    use super::*;
    // Tests will be added at each step below.
    //
    #[test]
    fn test_integration() -> Result<()> {
        //Test the full parsing pipline
        assert_eq!(parse_json("42")?, JsonValue::Number(42.0));
        assert_eq!(parse_json("true")?, JsonValue::Boolean(true));
        assert_eq!(parse_json("null")?, JsonValue::Null);
        assert_eq!(parse_json(r#""hello""#)?, JsonValue::String("hello".to_string()));
        Ok(())
    }
    #[test]
    fn test_error_propagation() {
        //Test that errors propagate properly with correct details
        let result = parse_json("@invalid@");
        assert!(result.is_err());

        //Validate error details through patter matching
        match result {
            Err(JsonError::UnexpectedToken { expected, found, position }) => {
                assert_eq!(expected, "valid JSON token");
                assert_eq!(found, "@");
                assert_eq!(position, 0);
            }

            _=> panic!("Expected UnexpectedToken error"),
        }
    }
}
