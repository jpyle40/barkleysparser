use crate::parse_json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_json_wasm(input: &str) -> Result<String, JsValue> {
    parse_json(input)
        .map(|value| value.to_string())
        .map_err(|error| JsValue::from_str(&error.to_string()))
}
