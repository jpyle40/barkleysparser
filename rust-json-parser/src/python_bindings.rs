use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;
use pyo3::types::{PyDict, PyList};
use crate::JsonValue;
use pyo3::exceptions::PyValueError;
use crate::JsonError;
use crate::parser::JsonParser;

impl<'py> IntoPyObject<'py> for JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            JsonValue::Null => Ok(py.None().into_bound_py_any(py)?),

            JsonValue::Boolean(value) => {
                Ok(value.into_pyobject(py)?.to_owned().into_any())
            }

            JsonValue::Number(value) => {
                Ok(value.into_pyobject(py)?.to_owned().into_any())
            }

            JsonValue::String(value) => {
                Ok(value.into_pyobject(py)?.into_any())
            }

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
#[pyfunction]
fn parse_json<'py>(py: Python<'py>, input: &str) -> PyResult<Bound<'py, PyAny>> {
    let mut parser = JsonParser::new(input)?;
    let value = parser.parse()?;
    Ok(value.into_pyobject(py)?)
}

#[pyfunction]
fn parse_json_file<'py>(py: Python<'py>, path: &str) -> PyResult<Bound<'py, PyAny>> {
    let contents = std::fs::read_to_string(path)?;
    let mut parser = JsonParser::new(&contents)?;
    let value = parser.parse()?;
    Ok(value.into_pyobject(py)?)
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

#[pymodule]
fn _rust_json_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json_file, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
