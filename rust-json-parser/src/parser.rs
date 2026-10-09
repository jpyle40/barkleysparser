use crate::tokenizer::Token;
use crate::tokenizer::Tokenizer;
use crate::value::JsonValue;
use crate::{JsonError, Result};
use std::collections::HashMap;

pub struct JsonParser {
    tokens: Vec<Token>,
    position: usize,
}

impl JsonParser {
    pub fn new(input: &str) -> Result<Self> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize()?;
        Ok(Self {
            tokens,
            position: 0,
        })
    }

    pub fn parse(&mut self) -> Result<JsonValue> {
        self.parse_value()
    }

    fn parse_value(&mut self) -> Result<JsonValue> {
        let token = self.advance();
        match token {
            Some(Token::Number(n)) => Ok(JsonValue::Number(n)),
            Some(Token::String(s)) => Ok(JsonValue::String(s)),
            Some(Token::Boolean(b)) => Ok(JsonValue::Boolean(b)),
            Some(Token::Null) => Ok(JsonValue::Null),
            Some(Token::LeftBracket) => self.parse_array(),
            Some(Token::LeftBrace) => self.parse_object(),

            None => Err(JsonError::UnexpectedEndOfInput {
                expected: "JSON value".to_string(),
                position: self.position.saturating_sub(1),
            }),
            Some(token) => Err(JsonError::UnexpectedToken {
                expected: "JSON value".to_string(),
                found: format!("{:?}", token),
                position: self.position - 1,
            }),
        }
    }
    fn parse_array(&mut self) -> Result<JsonValue> {
        if self.position >= self.tokens.len() {
            return Err(JsonError::UnexpectedEndOfInput {
                expected: "]".to_string(),
                position: self.position,
            });
        }
        if matches!(self.peek(), Some(Token::RightBracket)) {
            self.advance();
            return Ok(JsonValue::Array(vec![]));
        }
        let mut elements = Vec::new();

        loop {
            let value = self.parse_value()?;
            elements.push(value);

            if matches!(self.peek(), Some(Token::RightBracket)) {
                self.advance();
                return Ok(JsonValue::Array(elements));
            }
            if matches!(self.peek(), Some(Token::Comma)) {
                self.advance();
                continue;
            }
            return Err(JsonError::UnexpectedToken {
                expected: "comma or ]".to_string(),
                found: format!("{:?}", self.peek()),
                position: self.position,
            });
        }
    }
    fn parse_object(&mut self) -> Result<JsonValue> {
        if matches!(self.peek(), Some(Token::RightBrace)) {
            self.advance();
            return Ok(JsonValue::Object(HashMap::new()));
        }

        let mut members = HashMap::new();

        loop {
            let key = match self.advance() {
                Some(Token::String(key)) => key,
                other => {
                    return Err(JsonError::UnexpectedToken {
                        expected: "string key".to_string(),
                        found: format!("{:?}", other),
                        position: self.position,
                    });
                }
            };
            if !matches!(self.advance(), Some(Token::Colon)) {
                return Err(JsonError::UnexpectedToken {
                    expected: "colon".to_string(),
                    found: "missing colon".to_string(),
                    position: self.position,
                });
            }
            let value = self.parse_value()?;
            members.insert(key, value);

            if matches!(self.peek(), Some(Token::RightBrace)) {
                self.advance();
                return Ok(JsonValue::Object(members));
            }
            if matches!(self.peek(), Some(Token::Comma)) {
                self.advance();
                continue;
            }

            return Err(JsonError::UnexpectedToken {
                expected: "comma or }".to_string(),
                found: format!("{:?}", self.peek()),
                position: self.position,
            });
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        self.position += 1;
        token
    }
}

pub fn parse_json(input: &str) -> Result<JsonValue> {
    JsonParser::new(input)?.parse()
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
    #[test]
    fn test_parser_creation() {
        let parser = JsonParser::new("42");
        assert!(parser.is_ok());
    }
    #[test]
    fn test_parser_creation_tokenize_error() {
        let parser = JsonParser::new(r#""\q""#);
        assert!(parser.is_err());
    }
    #[test]
    fn test_json_parse_number() -> Result<()> {
        let mut parser = JsonParser::new("42")?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::Number(42.0));
        Ok(())
    }
    #[test]
    fn test_json_parse_string() -> Result<()> {
        let mut parser = JsonParser::new(r#""hello""#)?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::String("hello".to_string()));
        Ok(())
    }
    #[test]
    fn test_json_parse_boolean() -> Result<()> {
        let mut parser = JsonParser::new("true")?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::Boolean(true));
        Ok(())
    }
    #[test]
    fn test_json_parse_null() -> Result<()> {
        let mut parser = JsonParser::new("null")?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::Null);
        Ok(())
    }
    #[test]
    fn test_parse_negative_number() -> Result<()> {
        let mut parser = JsonParser::new("-3.15")?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::Number(-3.15));
        Ok(())
    }
    #[test]
    fn test_parse_boolean_false() -> Result<()> {
        let mut parser = JsonParser::new("false")?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::Boolean(false));
        Ok(())
    }
    #[test]
    fn test_parse_empty_input() {
        // Could fail at tokenization (no tokens) or parsing (empty token list)
        // Either is acceptable - just verify it's an error
        let result = match JsonParser::new("") {
            Ok(mut parser) => parser.parse(),
            Err(e) => Err(e),
        };
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_whitespace_only() {
        let result = match JsonParser::new("   ") {
            Ok(mut parser) => parser.parse(),
            Err(e) => Err(e),
        };
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_string_with_newline() -> Result<()> {
        let mut parser = JsonParser::new(r#""hello\nworld""#)?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::String("hello\nworld".to_string()));
        Ok(())
    }
    #[test]
    fn test_parse_string_with_unicode() -> Result<()> {
        let mut parser = JsonParser::new(r#""\u0048\u0065\u006c\u006c\u006f""#)?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::String("Hello".to_string()));
        Ok(())
    }
    #[test]
    fn test_parse_complex_escapes() -> Result<()> {
        let mut parser = JsonParser::new(r#""line1\nline2\t\"quoted\"\u0021""#)?;
        let value = parser.parse()?;
        assert_eq!(
            value,
            JsonValue::String("line1\nline2\t\"quoted\"!".to_string())
        );
        Ok(())
    }
    #[test]
    fn test_parse_string_with_tab() -> Result<()> {
        let mut parser = JsonParser::new(r#""col1\tcol2""#)?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::String("col1\tcol2".to_string()));
        Ok(())
    }

    #[test]
    fn test_parse_string_with_quotes() -> Result<()> {
        let mut parser = JsonParser::new(r#""say \"hi\"""#)?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::String("say \"hi\"".to_string()));
        Ok(())
    }
    #[test]
    fn test_parse_empty_array() -> Result<()> {
        let mut parser = JsonParser::new("[]")?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::Array(vec![]));
        Ok(())
    }

    #[test]
    fn test_parse_array_single() -> Result<()> {
        let mut parser = JsonParser::new("[1]")?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::Array(vec![JsonValue::Number(1.0)]));
        Ok(())
    }

    #[test]
    fn test_parse_array_multiple() -> Result<()> {
        let mut parser = JsonParser::new("[1, 2, 3]")?;
        let value = parser.parse()?;
        let expected = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]);
        assert_eq!(value, expected);
        Ok(())
    }

    #[test]
    fn test_parse_array_mixed_types() -> Result<()> {
        let mut parser = JsonParser::new(r#"[1, "two", true, null]"#)?;
        let value = parser.parse()?;
        let expected = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::String("two".to_string()),
            JsonValue::Boolean(true),
            JsonValue::Null,
        ]);
        assert_eq!(value, expected);
        Ok(())
    }
    #[test]
    fn test_array_accessor() -> Result<()> {
        let mut parser = JsonParser::new("[1, 2, 3]")?;
        let value = parser.parse()?;
        assert_eq!(value.as_array().map(Vec::len), Some(3));
        Ok(())
    }

    #[test]
    fn test_array_get_index() -> Result<()> {
        let mut parser = JsonParser::new("[10, 20, 30]")?;
        let value = parser.parse()?;
        assert_eq!(value.get_index(1), Some(&JsonValue::Number(20.0)));
        assert_eq!(value.get_index(5), None);
        Ok(())
    }
    #[test]
    fn test_parse_empty_object() -> Result<()> {
        let mut parser = JsonParser::new("{}")?;
        let value = parser.parse()?;
        assert_eq!(value, JsonValue::Object(HashMap::new()));
        Ok(())
    }

    #[test]
    fn test_parse_object_single_key() -> Result<()> {
        let mut parser = JsonParser::new(r#"{"key": "value"}"#)?;
        let value = parser.parse()?;
        let mut expected = HashMap::new();
        expected.insert("key".to_string(), JsonValue::String("value".to_string()));
        assert_eq!(value, JsonValue::Object(expected));
        Ok(())
    }

    #[test]
    fn test_parse_object_multiple_keys() -> Result<()> {
        let mut parser = JsonParser::new(r#"{"name": "Alice", "age": 30}"#)?;
        let value = parser.parse()?;
        if let JsonValue::Object(obj) = value {
            assert_eq!(
                obj.get("name"),
                Some(&JsonValue::String("Alice".to_string()))
            );
            assert_eq!(obj.get("age"), Some(&JsonValue::Number(30.0)));
        } else {
            panic!("Expected object");
        }
        Ok(())
    }
    #[test]
    fn test_object_accessor() -> Result<()> {
        let mut parser = JsonParser::new(r#"{"name": "test"}"#)?;
        let value = parser.parse()?;
        assert_eq!(value.as_object().map(HashMap::len), Some(1));
        Ok(())
    }

    #[test]
    fn test_object_get() -> Result<()> {
        let mut parser = JsonParser::new(r#"{"name": "Alice", "age": 30}"#)?;
        let value = parser.parse()?;
        assert_eq!(
            value.get("name"),
            Some(&JsonValue::String("Alice".to_string()))
        );
        assert_eq!(value.get("missing"), None);
        Ok(())
    }
    #[test]
    fn test_parse_nested_arrays() -> Result<()> {
        let mut parser = JsonParser::new("[[1, 2], [3, 4]]")?;
        let value = parser.parse()?;
        let expected = JsonValue::Array(vec![
            JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]),
            JsonValue::Array(vec![JsonValue::Number(3.0), JsonValue::Number(4.0)]),
        ]);
        assert_eq!(value, expected);
        Ok(())
    }

    #[test]
    fn test_parse_deeply_nested() -> Result<()> {
        let mut parser = JsonParser::new("[[[1]]]")?;
        let value = parser.parse()?;
        let expected = JsonValue::Array(vec![JsonValue::Array(vec![JsonValue::Array(vec![
            JsonValue::Number(1.0),
        ])])]);
        assert_eq!(value, expected);
        Ok(())
    }

    #[test]
    fn test_parse_nested_object() -> Result<()> {
        let mut parser = JsonParser::new(r#"{"outer": {"inner": 1}}"#)?;
        let value = parser.parse()?;
        if let JsonValue::Object(outer) = value {
            if let Some(JsonValue::Object(inner)) = outer.get("outer") {
                assert_eq!(inner.get("inner"), Some(&JsonValue::Number(1.0)));
            } else {
                panic!("Expected nested object");
            }
        } else {
            panic!("Expected object");
        }
        Ok(())
    }

    #[test]
    fn test_parse_array_in_object() -> Result<()> {
        let mut parser = JsonParser::new(r#"{"items": [1, 2, 3]}"#)?;
        let value = parser.parse()?;
        if let JsonValue::Object(obj) = value {
            if let Some(JsonValue::Array(arr)) = obj.get("items") {
                assert_eq!(arr.len(), 3);
            } else {
                panic!("Expected array");
            }
        } else {
            panic!("Expected object");
        }
        Ok(())
    }

    #[test]
    fn test_parse_object_in_array() -> Result<()> {
        let mut parser = JsonParser::new(r#"[{"a": 1}, {"b": 2}]"#)?;
        let value = parser.parse()?;
        if let JsonValue::Array(arr) = value {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("Expected array");
        }
        Ok(())
    }
    #[test]
    fn test_error_unclosed_array() -> Result<()> {
        let mut parser = JsonParser::new("[1, 2")?;
        let result = parser.parse();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_error_unclosed_object() -> Result<()> {
        let mut parser = JsonParser::new(r#"{"key": 1"#)?;
        let result = parser.parse();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_error_trailing_comma_array() -> Result<()> {
        let mut parser = JsonParser::new("[1, 2,]")?;
        let result = parser.parse();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_error_trailing_comma_object() -> Result<()> {
        let mut parser = JsonParser::new(r#"{"a": 1,}"#)?;
        let result = parser.parse();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_error_missing_colon() -> Result<()> {
        let mut parser = JsonParser::new(r#"{"key" 1}"#)?;
        let result = parser.parse();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_error_invalid_key() -> Result<()> {
        let mut parser = JsonParser::new(r#"{123: "value"}"#)?;
        let result = parser.parse();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_error_missing_comma_array() -> Result<()> {
        let mut parser = JsonParser::new("[1 2 3]")?;
        let result = parser.parse();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_error_missing_comma_object() -> Result<()> {
        let mut parser = JsonParser::new(r#"{"a": 1 "b": 2}"#)?;
        let result = parser.parse();
        assert!(result.is_err());
        Ok(())
    }
}
