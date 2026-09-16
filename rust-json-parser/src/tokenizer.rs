#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    String(String),
    Number,
    Boolean,
    Null,
}

//Example stub:
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut characters = input.chars().peekable();

    while let Some(&character) = characters.peek() {
        characters.next();
        if character == '{' {
            tokens.push(Token::LeftBrace);
        }
        if character == '}' {
            tokens.push(Token::RightBrace);
        }

        if character == '"' {
            let mut value = String::new();

            while let Some(next_character) = characters.next() {
                match next_character {
                    '"' => break,
                    '{' | '}' => continue,
                    _ => value.push(next_character),
                }
            }
            tokens.push(Token::String(value));
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
        let tokens = tokenize(r#"""#);
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
}
