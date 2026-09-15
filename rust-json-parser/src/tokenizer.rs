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
    let mut characters = input.chars();

    while let Some(character) = characters.next() {
         
        if character == '{' {
            tokens.push(Token::LeftBrace);
        }
        if character == '}' {
            tokens.push(Token::RightBrace);
        }
        
        if character == '"' {
            let mut value = String::new();

            while let Some(next_character) = characters.next() {
                if next_character == '"' {
                    break;
                }

                value.push(next_character);
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
}
