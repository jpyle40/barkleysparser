use std::fmt;

/// Represents errors encountered while tokenizing or parsing JSON.
///
/// Each variant identifies a specific parsing failure and
/// includes information about where the error occurred.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonError {
    /// Occurs when the parser encounters a token that was not expected.
    ///
    /// For example, an invalid character such as `@` in JSON input.
    UnexpectedToken {
        expected: String,
        found: String,
        position: usize,
    },

    /// Occurs when JSON input ends before a required token is found.
    ///
    /// For example, an array missing its closing bracket.
    UnexpectedEndOfInput {
        expected: String,
        position: usize,
    },

    /// Occurs when a JSON number has an invalid format.
    ///
    /// For example, a number with an invalid decimal or exponent.
    InvalidNumber {
        value: String,
        position: usize,
    },

    /// Occurs when a JSON string contains an unsupported escape sequence.
    ///
    /// For example, `\q` is not a valid JSON escape sequence.
    InvalidEscape {
        char: char,
        position: usize,
    },

    /// Occurs when a JSON string contains an invalid Unicode escape.
    ///
    /// For example, `\uZZZZ` is not a valid Unicode escape.
    InvalidUnicode {
        sequence: String,
        position: usize,
    },
}
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::UnexpectedToken {
                expected,
                found,
                position,
            } => {
                write!(
                    f,
                    "Unexpected token at position {}: expected {}, found {}",
                    position, expected, found
                )
            }
            JsonError::UnexpectedEndOfInput { expected, position } => {
                write!(
                    f,
                    "Unexpected end of Input at position {}: expected {}",
                    position, expected
                )
            }
            JsonError::InvalidNumber { value, position } => {
                write!(
                    f,
                    "Invalid number at position {}: value {}",
                    position, value
                )
            }
            JsonError::InvalidEscape { char, position } => {
                write!(
                    f,
                    "Invalid escape character '{}' at position {}",
                    char, position
                )
            }
            JsonError::InvalidUnicode { sequence, position } => {
                write!(
                    f,
                    "Invalid Unicode sequence '{}' at position {}",
                    sequence, position
                )
            }
        }
    }
}

impl std::error::Error for JsonError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = JsonError::UnexpectedToken {
            expected: "number".to_string(),
            found: "@".to_string(),
            position: 5,
        };

        assert!(format!("{:?}", error).contains("UnexpectedToken"));
    }
    #[test]
    fn test_error_variants() {
        let token_error = JsonError::UnexpectedToken {
            expected: "number".to_string(),
            found: "x".to_string(),
            position: 3,
        };

        let eof_error = JsonError::UnexpectedEndOfInput {
            expected: "closing quote".to_string(),
            position: 10,
        };

        let num_error = JsonError::InvalidNumber {
            value: "12.34.56".to_string(),
            position: 0,
        };

        assert!(!format!("{:?}", token_error).is_empty());
        assert!(!format!("{:?}", eof_error).is_empty());
        assert!(!format!("{:?}", num_error).is_empty());
    }
    #[test]
    fn test_error_display() {
        let error = JsonError::UnexpectedToken {
            expected: "valid JSON".to_string(),
            found: "@".to_string(),
            position: 0,
        };

        let message = format!("{}", error);
        assert!(message.contains("position 0"));
        assert!(message.contains("valid JSON"));
        assert!(message.contains("@"));
    }
    #[test]
    fn test_invalid_escape_display() {
        let err = JsonError::InvalidEscape {
            char: 'q',
            position: 5,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("escape"));
        assert!(msg.contains("q"));
    }
    #[test]
    fn test_invalid_unicode_display() {
        let err = JsonError::InvalidUnicode {
            sequence: "00GG".to_string(),
            position: 3,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("unicode") || msg.contains("Unicode"));
    }
    #[test]
    fn test_error_is_std_error() {
        let err = JsonError::InvalidEscape {
            char: 'x',
            position: 0,
        };
        let _: &dyn std::error::Error = &err;
    }
}
