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

//Example stub:
pub fn tokenize(input: &str) -> Vec<Token> {
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
                        '{' | '}' => continue,
                        _ => value.push(next_character),
                    }
                }
               match closed { 
                 true =>  tokens.push(Token::String(value)),
                 false => {}

               }
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

                let parsed_number: f64 = number.parse().unwrap();
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
                    _ => {}
                }
            }
            _ => {}
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests will be added here, one step at a time.
    #[test]
    fn test_empty_braces() {
        let tokens = tokenize("{}");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::RightBrace);
    }

    #[test]
    fn test_simple_string() {
        let tokens = tokenize(r#""hello""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello".to_string()));
    }

    #[test]
    fn test_tokenize_string() {
        let tokens = tokenize(r#""hello world""#);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello world".to_string()));
    }
    #[test]
    fn test_empty_string() {
        //Outer boundary: adjacent quotes with no inner content
        let tokens = tokenize(r#""""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("".to_string()));
    }
    #[test]
    fn test_string_containing_json_special_chars() {
        //Inner handing: JSON delimiters inside strings don't break tokenization
        let tokens = tokenize(r#""{key: value}""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("key: value".to_string()));
    }
    #[test]
    fn test_string_with_keyword_like_content() {
        //Inner handling: "true", "false", "null" inside strings stay as string content
        let tokens = tokenize(r#""not true or false""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("not true or false".to_string()));
    }
    #[test]
    fn test_string_with_number_like_content() {
        //Inner handling: numeric content inside doesn't become number tokens
        let tokens = tokenize(r#""phone: 555-1234""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("phone: 555-1234".to_string()));
    }
    #[test]
    fn test_number() {
        let tokens = tokenize("42");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(42.0));
    }
    #[test]
    fn test_negative_number() {
        let tokens = tokenize("-42");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-42.0));
    }
    #[test]
    fn test_decimal_number() {
        let tokens = tokenize("0.5");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(0.5));
    }
    #[test]
    fn test_leading_decimal_not_a_number() {
        //.5 is invalid JSON - number must have leading digit (0.5 is valid)
        let tokens = tokenize(".5");
        // should NOT be interpreted as 0.5
        assert!(!tokens.contains(&Token::Number(0.5)));
    }
    #[test]
    fn test_boolean_and_null() {
        let tokens = tokenize("true false null");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Boolean(true));
        assert_eq!(tokens[1], Token::Boolean(false));
        assert_eq!(tokens[2], Token::Null);
    }
    #[test]
    fn test_simple_object() {
        let tokens = tokenize(r#"{"name": "Alice"}"#);
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::String("name".to_string()));
        assert_eq!(tokens[2], Token::Colon);
        assert_eq!(tokens[3], Token::String("Alice".to_string()));
        assert_eq!(tokens[4], Token::RightBrace);
    }
    #[test]
    fn test_multiple_values() {
        let tokens = tokenize(r#"{"age": 30, "active": true}"#);
        //Verify we have the right tokens
        assert!(tokens.contains(&Token::String("age".to_string())));
        assert!(tokens.contains(&Token::Number(30.0)));
        assert!(tokens.contains(&Token::Comma));
        assert!(tokens.contains(&Token::String("active".to_string())));
        assert!(tokens.contains(&Token::Boolean(true)));
    }
    #[test]
    fn test_empty_brackets() {
        let tokens = tokenize("[]");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::LeftBracket);
        assert_eq!(tokens[1], Token::RightBracket);
    }
    #[test]
    fn test_unterminated_string_not_be_valid() {
        //input is '"hello' (open quote, never closed). It should not be
        //accepted as a valid string. Fails today; passes once tokenize errors.
        let tokens = tokenize("\"hello");
        assert_ne!(tokens, vec![Token::String("hello".to_string())]);
    }
}
