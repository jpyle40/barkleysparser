// Week 2: Simple parser for primitive JSON values
use crate::error::JsonError;
use crate::tokenizer::{Token, tokenize};
use crate::value::JsonValue;

// Result type alias for convenience
type Result<T> = std::result::Result<T, JsonError>;

// TODO: Implement your parse_json function
pub fn parse_json(input: &str) -> Result<JsonValue> {
    // Your code goes here
    // Hint:
    // 1. Call tokenize(input)?  (? propagates errors)
    // 2. Check if tokens is empty
    // 3. Match on tokens[0] and convert to JsonValue
    let tokens = tokenize(input)?;

    match tokens.is_empty() {
        true => match input.trim().is_empty() {
            true => Err(JsonError::UnexpectedEndOfInput {
                expected: "JSON value".to_string(),
                position: 0,
            }),
            false => Err(JsonError::UnexpectedToken {
                expected: "JSON value".to_string(),
                found: input.to_string(),
                position: 0,
            }),
        },

        false => match &tokens[0] {
            Token::String(s) => Ok(JsonValue::String(s.clone())),
            Token::Number(n) => Ok(JsonValue::Number(*n)),
            Token::Boolean(b) => Ok(JsonValue::Boolean(*b)),
            Token::Null => Ok(JsonValue::Null),
            _ => todo!(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Result type alias for cleaner test signatures
    type Result<T> = std::result::Result<T, JsonError>;

    // Tests will be added at each step below.
    #[test]
    fn test_parse_string() -> Result<()> {
        let result = parse_json(r#""hello world""#)?;
        assert_eq!(result, JsonValue::String("hello world".to_string()));
        Ok(())
    }
    #[test]
    fn test_parse_number() -> Result<()> {
        let result = parse_json("42.5")?;
        assert_eq!(result, JsonValue::Number(42.5));

        let result = parse_json("0")?;
        assert_eq!(result, JsonValue::Number(0.0));

        let result = parse_json("-10")?;
        assert_eq!(result, JsonValue::Number(-10.0));
        Ok(())
    }
    #[test]
    fn test_parse_boolean() -> Result<()> {
        let result = parse_json("true")?;
        assert_eq!(result, JsonValue::Boolean(true));

        let result = parse_json("false")?;
        assert_eq!(result, JsonValue::Boolean(false));
        Ok(())
    }
    #[test]
    fn test_parse_null() -> Result<()> {
        let result = parse_json("null")?;
        assert_eq!(result, JsonValue::Null);
        Ok(())
    }
    #[test]
    fn test_parse_error_empty() {
        let result = parse_json("");
        assert!(result.is_err());

        match result {
            Err(JsonError::UnexpectedEndOfInput { expected, position }) => {
                assert_eq!(expected, "JSON value");
                assert_eq!(position, 0);
            }
            _ => panic!("Expected UnexpectedEndOfInput error"),
        }
    }
    #[test]
    fn test_parse_error_invalid_token() {
        let result = parse_json("@");
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_with_whitespace() -> Result<()> {
        let result = parse_json(" 42 ")?;
        assert_eq!(result, JsonValue::Number(42.0));

        let result = parse_json("\n\ttrue\n")?;
        assert_eq!(result, JsonValue::Boolean(true));
        Ok(())
    }
    #[test]
    fn test_result_patter_matching() {
        let result = parse_json("42");

        match result {
            Ok(JsonValue::Number(n)) => assert_eq!(n, 42.0),
            _ => panic!("Expected successful number parse"),
        }

        let result = parse_json("@invalid@");

        match result {
            Err(JsonError::UnexpectedToken { .. }) => {} // Expected
            _ => panic!("Expected UnexpectedToken error"),
        }
    }
}
