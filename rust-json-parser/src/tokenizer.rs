#[derive(Debug, Clone, PartialEq)]
enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    String,
    Number,
    Boolean,
    Null,
}


//Example stub:
pub fn tokenize(input: &str) -> Vec<Token> {
    Vec::new() //Empty vector - flesh out step by step
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
}
