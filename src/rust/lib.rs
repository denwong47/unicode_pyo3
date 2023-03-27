use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn func(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn lib_unicode_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(func, m)?)?;
    Ok(())
}
