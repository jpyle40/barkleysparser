// Week 2: Simple parser for primitive JSON values
use crate::error::JsonError;
use crate::tokenizer::{Token, tokenize};
use crate::value::JsonValue;

// Result type alias for convenience
type Result<T> = std::result::Result<T, JsonError>;

// TODO: Implement your parse_json function
// pub fn parse_json(input: &str) -> Result<JsonValue> {
//     // Your code goes here
//     // Hint:
//     // 1. Call tokenize(input)?  (? propagates errors)
//     // 2. Check if tokens is empty
//     // 3. Match on tokens[0] and convert to JsonValue
// }

#[cfg(test)]
mod tests {
    use super::*;

    // Result type alias for cleaner test signatures
    type Result<T> = std::result::Result<T, JsonError>;

    // Tests will be added at each step below.
}
