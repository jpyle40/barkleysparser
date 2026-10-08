use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

// TODO: Implement helper methods
impl JsonValue {
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(num_value) => Some(*num_value),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Boolean(bool_value) => Some(*bool_value),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(vec_value) => Some(vec_value),
            _ => None,
        }
    }
    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(hashmap_value) => Some(hashmap_value),
            _ => None,
        }
    }
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(values) => values.get(key),
            _ => None,
        }
    }
    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
            JsonValue::Array(values) => values.get(index),
            _ => None,
        }
    }
}
impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Boolean(value) => write!(f, "{}", value),
            JsonValue::Number(value) => {
                if value.fract() == 0.0 {
                    write!(f, "{:.0}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            JsonValue::String(value) => { 
                write!(f, "\"")?;

                for ch in value.chars() {
                    match ch {
                        '\n' => write!(f, "\\n")?,
                        '"' => write!(f, "\\\"")?,
                        _ => write!(f, "{}", ch)?,
                    }
                }
                write!(f, "\"")
            }
            JsonValue::Array(values) => {
                write!(f, "[")?;

                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(map) => {
                write!(f, "{{")?;

                for (i, (key, value)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }

                    write!(f, "\"{}\":{}", key, value)?;
                }

                write!(f, "}}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::JsonParser;
    use crate::error::JsonError;

    // Tests will be added at each step below.
    #[test]
    fn test_json_value_equality() {
        assert_eq!(JsonValue::Null, JsonValue::Null);
        assert_eq!(JsonValue::Boolean(true), JsonValue::Boolean(true));
        assert_eq!(JsonValue::Number(42.0), JsonValue::Number(42.0));
        assert_eq!(
            JsonValue::String("test".to_string()),
            JsonValue::String("test".to_string())
        );
        assert_ne!(JsonValue::Null, JsonValue::Boolean(false));
        assert_ne!(JsonValue::Number(1.0), JsonValue::Number(2.0));
    }
    #[test]
    fn test_json_value_creation() {
        let null_val = JsonValue::Null;
        let bool_val = JsonValue::Boolean(true);
        let num_val = JsonValue::Number(42.5);
        let str_val = JsonValue::String("hello".to_string());

        assert!(null_val.is_null());
        assert_eq!(bool_val.as_bool(), Some(true));
        assert_eq!(num_val.as_f64(), Some(42.5));
        assert_eq!(str_val.as_str(), Some("hello"));
    }
    #[test]
    fn test_json_value_assessors() {
        let value = JsonValue::String("test".to_string());
        assert_eq!(value.as_str(), Some("test"));
        assert_eq!(value.as_f64(), None);
        assert_eq!(value.as_bool(), None);
        assert!(!value.is_null());

        let value = JsonValue::Number(42.0);
        assert_eq!(value.as_f64(), Some(42.0));
        assert_eq!(value.as_str(), None);

        let value = JsonValue::Boolean(true);
        assert_eq!(value.as_bool(), Some(true));

        let value = JsonValue::Null;
        assert!(value.is_null());
    }
    #[test]
    fn test_display_primitives() {
        assert_eq!(JsonValue::Null.to_string(), "null");
        assert_eq!(JsonValue::Boolean(true).to_string(), "true");
        assert_eq!(JsonValue::Boolean(false).to_string(), "false");
        assert_eq!(JsonValue::Number(42.0).to_string(), "42");
        assert_eq!(JsonValue::Number(3.14).to_string(), "3.14");
        assert_eq!(
            JsonValue::String("hello".to_string()).to_string(),
            "\"hello\""
        );
    }

    #[test]
    fn test_display_array() {
        let value = JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]);
        assert_eq!(value.to_string(), "[1,2]");
    }

    #[test]
    fn test_display_empty_containers() {
        assert_eq!(JsonValue::Array(vec![]).to_string(), "[]");
        assert_eq!(JsonValue::Object(HashMap::new()).to_string(), "{}");
    }
    
        #[test]
        fn test_display_escape_string() {
            let value = JsonValue::String("hello\nworld".to_string());
            assert_eq!(value.to_string(), "\"hello\\nworld\"");
        }
    
        #[test]
        fn test_display_escape_quotes() {
            let value = JsonValue::String("say \"hi\"".to_string());
            assert_eq!(value.to_string(), "\"say \\\"hi\\\"\"");
        }
    
        #[test]
        fn test_display_nested() -> Result<(), JsonError> {
            let mut parser = JsonParser::new(r#"{"arr": [1, 2]}"#)?;
            let value = parser.parse()?;
            let output = value.to_string();
            // Object key order may vary, so check components
            assert!(output.contains("\"arr\""));
            assert!(output.contains("[1,2]"));
            Ok(())
        }
    
        #[test]
        fn test_display_nested_array() {
            let value = JsonValue::Array(vec![JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
            ])]);
            assert_eq!(value.to_string(), "[[1,2]]");
        }
}
