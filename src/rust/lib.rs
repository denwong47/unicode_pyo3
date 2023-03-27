use pyo3::prelude::*;

pub mod config;
pub mod segmentation;

/// Returns a `List[str]` over substrings of `text` separated on
/// [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
#[pyfunction]
fn segmentate(text: &str, bounds: Option<bool>) -> PyResult<Vec<&str>> {
    Ok(segmentation::segmentate(text, bounds.unwrap_or(false)))
}

/// Returns a `List[List[str]]` over substrings of each `str` in `texts` in sequence,
/// separated on [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
#[pyfunction]
fn segmentate_all(
    texts: Vec<&str>,
    bounds: Option<bool>,
    max_workers: Option<usize>,
) -> PyResult<Vec<Vec<&str>>> {
    Ok(segmentation::segmentate_all(
        &texts,
        bounds.unwrap_or(false),
        max_workers,
    ))
}

/// A Python module implemented in Rust.
#[pymodule]
fn lib_unicode_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(segmentate, m)?)?;
    m.add_function(wrap_pyfunction!(segmentate_all, m)?)?;
    Ok(())
}
