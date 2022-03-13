use pyo3::prelude::*;
use serde_json::Value;

#[pyfunction]
fn loads(s: &str) -> PyResult<String> {
    let a = json5::from_str::<Value>(&s).unwrap().to_string();
    Ok(a)
}

#[pymodule]
fn json5_se(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    Ok(())
}
