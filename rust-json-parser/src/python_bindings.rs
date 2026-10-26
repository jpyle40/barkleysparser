use crate::JsonError;
use crate::JsonValue;
use crate::parser::JsonParser;
use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::time::Instant;

impl<'py> IntoPyObject<'py> for JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            JsonValue::Null => Ok(py.None().into_bound_py_any(py)?),

            JsonValue::Boolean(value) => Ok(value.into_pyobject(py)?.to_owned().into_any()),

            JsonValue::Number(value) => Ok(value.into_pyobject(py)?.to_owned().into_any()),

            JsonValue::String(value) => Ok(value.into_pyobject(py)?.into_any()),

            JsonValue::Array(values) => {
                let list = PyList::empty(py);
                for value in values {
                    list.append(value.into_pyobject(py)?)?;
                }
                Ok(list.into_any())
            }

            JsonValue::Object(values) => {
                let dict = PyDict::new(py);
                for (key, value) in values {
                    dict.set_item(key, value.into_pyobject(py)?)?;
                }
                Ok(dict.into_any())
            }
        }
    }
}

impl From<JsonError> for PyErr {
    fn from(err: JsonError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

/// Parses a JSON string and returns the corresponding Python object.
///
/// Converts JSON objects into Python dictionaries, arrays into lists,
/// and other JSON values into their corresponding Python types.
///
/// # Examples
///
/// ```python
/// from rust_json_parser import parse_json
///
/// data = parse_json('{"name": "server01"}')
/// print(data["name"])
/// ```
///
/// # Errors
///
/// Raises a Python `ValueError` if the input contains invalid JSON,
/// including unexpected tokens, invalid numbers, or malformed
/// escape sequences.

#[pyfunction]
fn parse_json<'py>(py: Python<'py>, input: &str) -> PyResult<Bound<'py, PyAny>> {
    let mut parser = JsonParser::new(input)?;
    let value = parser.parse()?;
    value.into_pyobject(py)
}

/// Parses a JSON file and returns the result as a Python object.
///
/// Reads the file at the specified path and converts its JSON
/// contents into Python dictionaries, lists, strings, numbers,
/// booleans, or None.
///
/// # Examples
///
/// ```python
/// from rust_json_parser import parse_json_file
///
/// data = parse_json_file("example.json")
/// print(data)
/// ```
///
/// # Errors
///
/// Raises an exception if:
///
/// - The file does not exist or cannot be read.
/// - The file contains invalid JSON syntax.
/// - The JSON contains an invalid number or escape sequence.

#[pyfunction]
fn parse_json_file<'py>(py: Python<'py>, path: &str) -> PyResult<Bound<'py, PyAny>> {
    let contents = std::fs::read_to_string(path)?;
    let mut parser = JsonParser::new(&contents)?;
    let value = parser.parse()?;
    value.into_pyobject(py)
}

fn py_to_json_value(obj: &Bound<'_, PyAny>) -> PyResult<JsonValue> {
    if obj.is_none() {
        return Ok(JsonValue::Null);
    }

    if let Ok(value) = obj.extract::<bool>() {
        return Ok(JsonValue::Boolean(value));
    }

    if let Ok(value) = obj.extract::<f64>() {
        return Ok(JsonValue::Number(value));
    }

    if let Ok(value) = obj.extract::<String>() {
        return Ok(JsonValue::String(value));
    }

    if let Ok(list) = obj.cast::<PyList>() {
        let mut values = Vec::new();
        for item in list.iter() {
            values.push(py_to_json_value(&item)?);
        }
        return Ok(JsonValue::Array(values));
    }

    if let Ok(dict) = obj.cast::<PyDict>() {
        let mut values = std::collections::HashMap::new();
        for (key, value) in dict.iter() {
            let key: String = key.extract()?;
            values.insert(key, py_to_json_value(&value)?);
        }
        return Ok(JsonValue::Object(values));
    }

    Err(PyValueError::new_err("Unsupported Python type"))
}

#[pyfunction]
#[pyo3(signature = (obj, indent=None))]
fn dumps(obj: &Bound<'_, PyAny>, indent: Option<usize>) -> PyResult<String> {
    let value = py_to_json_value(obj)?;

    match indent {
        None => Ok(value.to_string()),
        Some(n) => Ok(value.pretty_print(n)),
    }
}

/// Benchmarks Rust JSON parsing against Python json and simplejson.
///
/// Returns elapsed times in seconds for all three parsers.
#[pyfunction]
#[pyo3(signature = (json_str, iterations=1000))]
fn benchmark_performance(
    py: Python<'_>,
    json_str: &str,
    iterations: usize,
) -> PyResult<(f64, f64, f64)> {
    if iterations == 0 {
        return Err(PyValueError::new_err(
            "iterations must be greater than zero",
        ));
    }

    // Benchmark the Rust parser.
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = JsonParser::new(json_str)?.parse()?;
    }
    let rust_time = start.elapsed().as_secs_f64();

    // Benchmark Python's built-in json parser.
    let json_module = py.import("json")?;
    let json_loads = json_module.getattr("loads")?;

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = json_loads.call1((json_str,))?;
    }
    let python_json_time = start.elapsed().as_secs_f64();

    // Benchmark Python's simplejson parser.
    let simplejson_module = py.import("simplejson")?;
    let simplejson_loads = simplejson_module.getattr("loads")?;

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = simplejson_loads.call1((json_str,))?;
    }
    let simplejson_time = start.elapsed().as_secs_f64();

    Ok((rust_time, python_json_time, simplejson_time))
}

#[pymodule]
fn _rust_json_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json_file, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_performance, m)?)?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
