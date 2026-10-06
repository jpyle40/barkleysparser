use crate::error::JsonError;

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}
impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, JsonError> {
        let mut tokens = Vec::new();

        while let Some(character) = self.advance() {
            let token = match character {
                '"' => Token::String(self.read_string()?),
                '0'..='9' | '-' => Token::Number(self.read_number(character)?),
                'a'..='z' | 'A'..='Z' => self.read_literal(character)?,

                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                '[' => Token::LeftBracket,
                ']' => Token::RightBracket,
                ':' => Token::Colon,
                ',' => Token::Comma,

                ' ' | '\n' | '\t' | '\r' => continue,

                _ => {
                    return Err(JsonError::UnexpectedToken {
                        expected: "valid JSON token".to_string(),
                        found: character.to_string(),
                        position: self.position - 1,
                    });
                }
            };

            tokens.push(token);
        }

        Ok(tokens)
    }
    fn read_literal(&mut self, first_character: char) -> Result<Token, JsonError> {
        let start_position = self.position - 1;
        let mut value = String::new();
        value.push(first_character);

        while let Some(next_character) = self.peek() {
            if next_character.is_alphabetic() {
                value.push(next_character);
                self.advance();
            } else {
                break;
            }
        }

        match value.as_str() {
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            "null" => Ok(Token::Null),
            _ => Err(JsonError::UnexpectedToken {
                expected: "true, false, or null".to_string(),
                found: value,
                position: start_position,
            }),
        }
    }
    fn read_number(&mut self, first_character: char) -> Result<f64, JsonError> {
        let mut number = String::new();
        number.push(first_character);

        while let Some(next_character) = self.peek() {
            if next_character.is_ascii_digit() || next_character == '.' {
                number.push(next_character);
                self.advance();
            } else {
                break;
            }
        }

        match number.parse() {
            Ok(n) => Ok(n),
            Err(_) => Err(JsonError::InvalidNumber {
                value: number,
                position: self.position,
            }),
        }
    }
    fn read_string(&mut self) -> Result<String, JsonError> {
        let mut value = String::new();

        while let Some(next_character) = self.advance() {
            match next_character {
                '"' => return Ok(value),

                '\\' => match self.advance() {
                    Some('n') => value.push('\n'),
                    Some('t') => value.push('\t'),
                    Some('"') => value.push('\"'),
                    Some('\\') => value.push('\\'),
                    Some('/') => value.push('/'),
                    Some('r') => value.push('\r'),
                    Some('b') => value.push('\u{0008}'),
                    Some('f') => value.push('\u{000C}'),

                    Some('u') => {
                        let character = self.read_unicode_escape()?;
                        value.push(character);
                    }

                    Some(other) => {
                        return Err(JsonError::InvalidEscape {
                            char: other,
                            position: self.position - 1,
                        });
                    }

                    None => {
                        return Err(JsonError::UnexpectedEndOfInput {
                            expected: "Closing quote".to_string(),
                            position: self.position,
                        });
                    }
                },

                _ => value.push(next_character),
            }
        }

        Err(JsonError::UnexpectedEndOfInput {
            expected: "Closing quote".to_string(),
            position: self.position,
        })
    }
    fn read_unicode_escape(&mut self) -> Result<char, JsonError> {
        let mut hex = String::new();

        for _ in 0..4 {
            match self.advance() {
                Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                Some(c) => {
                    return Err(JsonError::InvalidUnicode {
                        sequence: c.to_string(),
                        position: self.position - 1,
                    });
                }
                None => {
                    return Err(JsonError::InvalidUnicode {
                        sequence: hex,
                        position: self.position,
                    });
                }
            }
        }

        let code = u32::from_str_radix(&hex, 16).map_err(|_| JsonError::InvalidUnicode {
            sequence: hex.clone(),
            position: self.position,
        })?;

        char::from_u32(code).ok_or(JsonError::InvalidUnicode {
            sequence: hex,
            position: self.position,
        })
    }
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
    fn advance(&mut self) -> Option<char> {
        let current = self.peek();
        self.position += 1;
        current
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_tokenizer_struct_creation() {
        let _tokenizer = Tokenizer::new(r#""hello""#);
    }

    #[test]
    fn test_tokenize_literals() -> Result<()> {
        let mut t1 = Tokenizer::new("true");
        assert_eq!(t1.tokenize()?, vec![Token::Boolean(true)]);

        let mut t2 = Tokenizer::new("false");
        assert_eq!(t2.tokenize()?, vec![Token::Boolean(false)]);

        let mut t3 = Tokenizer::new("null");
        assert_eq!(t3.tokenize()?, vec![Token::Null]);

        Ok(())
    }
    #[test]
    fn test_tokenize_simple_string() -> Result<()> {
        let mut tokenizer = Tokenizer::new(r#""hello""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("hello".to_string())]);
        Ok(())
    }

    // Tests will be added here, one step at a time.
    #[test]
    fn test_empty_braces() -> Result<()> {
        let mut tokenizer = Tokenizer::new("{}");
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::RightBrace);
        Ok(())
    }

    //     #[test]
    //     fn test_simple_string() {
    //         let tokens = tokenize(r#""hello""#);
    //         assert_eq!(tokens.len(), 1);
    //         assert_eq!(tokens[0], Token::String("hello".to_string()));
    //    }
    //
    //     #[test]
    //     fn test_tokenize_string() {
    //         let tokens = tokenize(r#""hello world""#);
    //
    //         assert_eq!(tokens.len(), 1);
    //         assert_eq!(tokens[0], Token::String("hello world".to_string()));
    //     }
    #[test]
    fn test_empty_string() -> Result<()> {
        //Outer boundary: adjacent quotes with no inner content
        let mut tokenizer = Tokenizer::new(r#""""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("".to_string()));
        Ok(())
    }
    #[test]
    fn test_string_containing_json_special_chars() -> Result<()> {
        //Inner handing: JSON delimiters inside strings don't break tokenization
        let mut tokenizer = Tokenizer::new(r#""{key: value}""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("{key: value}".to_string()));
        Ok(())
    }
    #[test]
    fn test_string_with_keyword_like_content() -> Result<()> {
        //Inner handling: "true", "false", "null" inside strings stay as string content
        let mut tokenizer = Tokenizer::new(r#""not true or false""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("not true or false".to_string()));
        Ok(())
    }
    #[test]
    fn test_string_with_number_like_content() -> Result<()> {
        //Inner handling: numeric content inside doesn't become number tokens
        let mut tokenizer = Tokenizer::new(r#""phone: 555-1234""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("phone: 555-1234".to_string()));
        Ok(())
    }
    #[test]
    fn test_tokenize_number() -> Result<()> {
        let mut tokenizer = Tokenizer::new("42");
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(42.0));
        Ok(())
    }
    #[test]
    fn test_negative_number() -> Result<()> {
        let mut tokenizer = Tokenizer::new("-42");
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-42.0));
        Ok(())
    }
    #[test]
    fn test_decimal_number() -> Result<()> {
        let mut tokenizer = Tokenizer::new("0.5");
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(0.5));
        Ok(())
    }

    //     #[test]
    //     fn test_simple_object() {
    //         let tokens = tokenize(r#"{"name": "Alice"}"#);
    //         assert_eq!(tokens.len(), 5);
    //         assert_eq!(tokens[0], Token::LeftBrace);
    //         assert_eq!(tokens[1], Token::String("name".to_string()));
    //         assert_eq!(tokens[2], Token::Colon);
    //         assert_eq!(tokens[3], Token::String("Alice".to_string()));
    //         assert_eq!(tokens[4], Token::RightBrace);
    //     }
    //     #[test]
    //     fn test_multiple_values() {
    //         let tokens = tokenize(r#"{"age": 30, "active": true}"#);
    //         //Verify we have the right tokens
    //         assert!(tokens.contains(&Token::String("age".to_string())));
    //         assert!(tokens.contains(&Token::Number(30.0)));
    //         assert!(tokens.contains(&Token::Comma));
    //         assert!(tokens.contains(&Token::String("active".to_string())));
    //         assert!(tokens.contains(&Token::Boolean(true)));
    //     }
    //     #[test]
    //     fn test_empty_brackets() {
    //         let tokens = tokenize("[]");
    //         assert_eq!(tokens.len(), 2);
    //         assert_eq!(tokens[0], Token::LeftBracket);
    //         assert_eq!(tokens[1], Token::RightBracket);
    //     }
    //     #[test]
    //    fn test_unterminated_string_not_be_valid() {
    //        //input is '"hello' (open quote, never closed). It should not be
    //        //accepted as a valid string. Fails today; passes once tokenize errors.
    //        let tokens = tokenize("\"hello");
    //        assert_ne!(tokens, vec![Token::String("hello".to_string())]);
    //   }
    #[test]
    fn test_unterminated_string() {
        let mut tokenizer = Tokenizer::new(r#""missing end quote"#);
        let err = tokenizer.tokenize().unwrap_err();
        match err {
            JsonError::UnexpectedEndOfInput { position, .. } => {
                assert_eq!(position, 19);
            }
            other => panic!("expected UnexpectedEndOfInput, got {:?}", other),
        }
    }
    #[test]
    fn test_tokenizer_multiple_tokens() -> Result<()> {
        let mut tokenizer = Tokenizer::new("123 456");
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 2);
        Ok(())
    }
    #[test]
    fn test_tokenize_negative_number() -> Result<()> {
        let mut tokenizer = Tokenizer::new("-3.15");
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::Number(-3.15)]);
        Ok(())
    }
    #[test]
    fn test_invalid_keyword_error_position_points_to_start() -> Result<()> {
        let input = "   xyz";
        let mut tokenizer = Tokenizer::new(input);
        let err = tokenizer.tokenize().unwrap_err();

        match err {
            JsonError::UnexpectedToken { position, .. } => {
                assert_eq!(
                    position, 3,
                    "error position should point to the start of 'xyz'c (index 3), not past it"
                );
            }
            other => panic!("expected UnexpectedToken, got {:?}", other),
        }
        Ok(())
    }
    #[test]
    fn test_escape_newline() -> Result<()> {
        let mut tokenizer = Tokenizer::new(r#""hello\nworld""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("hello\nworld".to_string())]);
        Ok(())
    }
    #[test]
    fn test_escape_tab() -> Result<()> {
        let mut tokenizer = Tokenizer::new(r#""col1\tcol2""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("col1\tcol2".to_string())]);
        Ok(())
    }

    #[test]
    fn test_escape_quote() -> Result<()> {
        let mut tokenizer = Tokenizer::new(r#""say \"hello\"""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("say \"hello\"".to_string())]);
        Ok(())
    }

    #[test]
    fn test_escape_backslash() -> Result<()> {
        let mut tokenizer = Tokenizer::new(r#""path\\to\\file""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("path\\to\\file".to_string())]);
        Ok(())
    }

    #[test]
    fn test_multiple_escapes() -> Result<()> {
        let mut tokenizer = Tokenizer::new(r#""a\nb\tc\"""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("a\nb\tc\"".to_string())]);
        Ok(())
    }
    #[test]
    fn test_escape_forward_slash() -> Result<()> {
        let mut tokenizer = Tokenizer::new(r#""a\/b""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("a/b".to_string())]);
        Ok(())
    }

    #[test]
    fn test_escape_carriage_return() -> Result<()> {
        let mut tokenizer = Tokenizer::new(r#""line\r\n""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("line\r\n".to_string())]);
        Ok(())
    }

    #[test]
    fn test_escape_backspace_formfeed() -> Result<()> {
        let mut tokenizer = Tokenizer::new(r#""\b\f""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("\u{0008}\u{000C}".to_string())]);
        Ok(())
    }
    #[test]
    fn test_unicode_escape_basic() -> Result<()> {
        // \u0041 is 'A'
        let mut tokenizer = Tokenizer::new(r#""\u0041""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("A".to_string())]);
        Ok(())
    }

    #[test]
    fn test_unicode_escape_multiple() -> Result<()> {
        // \u0048\u0069 is "Hi"
        let mut tokenizer = Tokenizer::new(r#""\u0048\u0069""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("Hi".to_string())]);
        Ok(())
    }

    #[test]
    fn test_unicode_escape_mixed() -> Result<()> {
        // Mix of regular chars and unicode escapes
        let mut tokenizer = Tokenizer::new(r#""Hello \u0057orld""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("Hello World".to_string())]);
        Ok(())
    }

    #[test]
    fn test_unicode_escape_lowercase() -> Result<()> {
        // Lowercase hex digits should work too
        let mut tokenizer = Tokenizer::new(r#""\u004a""#);
        let tokens = tokenizer.tokenize()?;
        assert_eq!(tokens, vec![Token::String("J".to_string())]);
        Ok(())
    }
    #[test]
    fn test_invalid_escape_sequence() {
        let mut tokenizer = Tokenizer::new(r#""\q""#);
        let result = tokenizer.tokenize();
        assert!(matches!(result, Err(JsonError::InvalidEscape { .. })));
    }

    #[test]
    fn test_invalid_unicode_too_short() {
        let mut tokenizer = Tokenizer::new(r#""\u004""#);
        let result = tokenizer.tokenize();
        assert!(matches!(result, Err(JsonError::InvalidUnicode { .. })));
    }

    #[test]
    fn test_invalid_unicode_bad_hex() {
        let mut tokenizer = Tokenizer::new(r#""\u00GG""#);
        let result = tokenizer.tokenize();
        assert!(matches!(result, Err(JsonError::InvalidUnicode { .. })));
    }

    #[test]
    fn test_unterminated_string_with_escape() {
        let mut tokenizer = Tokenizer::new(r#""hello\n"#);
        let result = tokenizer.tokenize();
        assert!(result.is_err());
    }
}
