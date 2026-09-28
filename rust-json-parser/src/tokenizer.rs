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

        while !self.is_at_end() {
            let Some(character) = self.advance() else {
                break;
            };

            match character {
                '0'..='9' | '-' => {
                    let mut number = String::new();
                    number.push(character);

                    while let Some(next_character) = self.peek() {
                        if next_character.is_ascii_digit() || next_character == '.' {
                            number.push(next_character);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let parsed_number: f64 = match number.parse() {
                        Ok(n) => n,
                        Err(_) => {
                            return Err(JsonError::InvalidNumber {
                                value: number,
                                position: self.position,
                            });
                        }
                    };

                    tokens.push(Token::Number(parsed_number));
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut value = String::new();
                    value.push(character);

                    while let Some(next_character) = self.peek() {
                        if next_character.is_alphabetic() {
                            value.push(next_character);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    match value.as_str() {
                        "true" => tokens.push(Token::Boolean(true)),
                        "false" => tokens.push(Token::Boolean(false)),
                        "null" => tokens.push(Token::Null),
                        _ => {
                            return Err(JsonError::UnexpectedToken {
                                expected: "true, false, or null".to_string(),
                                found: value,
                                position: self.position,
                            });
                        }
                    }
                }

                _ => todo!(),
            }
        }

        Ok(tokens)
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
    fn advance(&mut self) -> Option<char> {
        let current = self.peek();
        self.position += 1;
        current
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
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

pub fn tokenize(input: &str) -> Result<Vec<Token>, JsonError> {
    let mut tokens = Vec::new();
    let mut characters = input.chars().peekable();

    while let Some(&character) = characters.peek() {
        characters.next();
        match character {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            '"' => {
                let mut value = String::new();
                let mut closed = false;
                while let Some(next_character) = characters.next() {
                    match next_character {
                        '"' => {
                            closed = true;
                            break;
                        }
                        _ => value.push(next_character),
                    }
                }
                if !closed {
                    return Err(JsonError::UnexpectedEndOfInput {
                        expected: "closing quote".to_string(),
                        position: 0,
                    });
                }
                tokens.push(Token::String(value));
            }

            '0'..='9' | '-' => {
                let mut number = String::new();
                number.push(character);

                while let Some(&next_character) = characters.peek() {
                    if next_character.is_ascii_digit() || next_character == '.' {
                        number.push(next_character);
                        characters.next();
                    } else {
                        break;
                    }
                }

                let parsed_number: f64 = match number.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        return Err(JsonError::InvalidNumber {
                            value: number,
                            position: 0,
                        });
                    }
                };
                tokens.push(Token::Number(parsed_number));
            }

            'a'..='z' | 'A'..='Z' => {
                let mut value = String::new();
                value.push(character);

                while let Some(&next_character) = characters.peek() {
                    if next_character.is_alphabetic() {
                        value.push(next_character);
                        characters.next();
                    } else {
                        break;
                    }
                }

                match value.as_str() {
                    "true" => tokens.push(Token::Boolean(true)),
                    "false" => tokens.push(Token::Boolean(false)),
                    "null" => tokens.push(Token::Null),
                    _ => {
                        return Err(JsonError::UnexpectedToken {
                            expected: "true, false, or null".to_string(),
                            found: value,
                            position: 0,
                        });
                    }
                }
            }
            ' ' | '\n' | '\t' | '\r' => {}
            _ => {
                return Err(JsonError::UnexpectedToken {
                    expected: "valid JSON token".to_string(),
                    found: character.to_string(),
                    position: 0,
                });
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_tokenizer_struct_creation() {
        let tokenizer = Tokenizer::new(r#""hello""#);
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

    // Tests will be added here, one step at a time.
    //     #[test]
    //     fn test_empty_braces() {
    //         let tokens = tokenize("{}");
    //         assert_eq!(tokens.len(), 2);
    //         assert_eq!(tokens[0], Token::LeftBrace);
    //         assert_eq!(tokens[1], Token::RightBrace);
    //     }
    //
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
        let tokens = tokenize(r#""""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("".to_string()));
        Ok(())
    }
    #[test]
    fn test_string_containing_json_special_chars() -> Result<()> {
        //Inner handing: JSON delimiters inside strings don't break tokenization
        let tokens = tokenize(r#""{key: value}""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("{key: value}".to_string()));
        Ok(())
    }
    #[test]
    fn test_string_with_keyword_like_content() -> Result<()> {
        //Inner handling: "true", "false", "null" inside strings stay as string content
        let tokens = tokenize(r#""not true or false""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("not true or false".to_string()));
        Ok(())
    }
    #[test]
    fn test_string_with_number_like_content() -> Result<()> {
        //Inner handling: numeric content inside doesn't become number tokens
        let tokens = tokenize(r#""phone: 555-1234""#)?;
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
        let tokens = tokenize("-42")?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-42.0));
        Ok(())
    }
    #[test]
    fn test_decimal_number() -> Result<()> {
        let tokens = tokenize("0.5")?;
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
        let err = tokenize(r#""missing end quote"#).unwrap_err();
        match err {
            JsonError::UnexpectedEndOfInput { position, .. } => {
                assert_eq!(position, 0);
            }
            other => panic!("expected UnexpectedEndOfInput, got {:?}", other),
        }
    }
}
