use crate::tokenizer::Token;
use crate::tokenizer::Tokenizer;
use crate::value::JsonValue;
use crate::{JsonError, Result};

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

            None => Err(JsonError::UnexpectedEndOfInput {
                expected: "JSON value".to_string(),
                position: self.position,
            }),
            Some(token) => Err(JsonError::UnexpectedToken {
                expected: "JSON value".to_string(),
                found: format!("{:?}", token),
                position: self.position - 1,
            }),
        }
    }
    fn parse_array(&mut self) -> Result<JsonValue> {
        if self.is_at_end() {
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
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    fn advance(&mut self) -> Option<Token> {
        if self.is_at_end() {
            None
        } else {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        }
    }
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
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
}
