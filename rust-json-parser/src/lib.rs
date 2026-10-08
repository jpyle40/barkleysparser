mod error;
mod parser;
mod tokenizer;
mod value;

pub use error::JsonError;
pub use parser::parse_json;
pub use tokenizer::{Token, Tokenizer};
pub use value::JsonValue;

pub type Result<T> = std::result::Result<T, JsonError>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_integration() -> Result<()> {
        //Test the full parsing pipline
        assert_eq!(parse_json("42")?, JsonValue::Number(42.0));
        assert_eq!(parse_json("true")?, JsonValue::Boolean(true));
        assert_eq!(parse_json("null")?, JsonValue::Null);
        assert_eq!(
            parse_json(r#""hello""#)?,
            JsonValue::String("hello".to_string())
        );
        Ok(())
    }
    #[test]
    fn test_error_propagation() {
        let result = parse_json("@invalid@");
        assert!(result.is_err());

        match result {
            Err(JsonError::UnexpectedToken {
                expected,
                found,
                position,
            }) => {
                assert_eq!(expected, "valid JSON token");
                assert_eq!(found, "@");
                assert_eq!(position, 0);
            }

            _ => panic!("Expected UnexpectedToken error"),
        }
    }
}
