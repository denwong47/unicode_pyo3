//! Python package using official Unicode libraries by The Rust Project Developers.
//!
//! Rust have excellent support for Unicode characters, down to the

use pyo3::prelude::*;

#[cfg(feature = "numpy")]
use numpy::ndarray::{Array1, Ix1};
#[cfg(feature = "numpy")]
use numpy::{PyArray, ToPyArray};

#[cfg(feature = "polars")]
use polars::prelude::*;
#[cfg(feature = "polars")]
use pyo3_polars::error::PyPolarsErr;
#[cfg(feature = "polars")]
use pyo3_polars::PySeries;

pub mod config;
pub mod segmentation;

const DEFAULT_BOUNDS: bool = false;

/// Returns a `List[str]` over substrings of `text` separated on
/// [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
#[pyfunction]
fn segmentate(text: &str, bounds: Option<bool>) -> PyResult<Vec<&str>> {
    Ok(segmentation::segmentate(
        text,
        bounds.unwrap_or(DEFAULT_BOUNDS),
    ))
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
        bounds.unwrap_or(DEFAULT_BOUNDS),
        max_workers,
    ))
}

/// Returns a `numpy.ndarray` of `str` type over substrings of each `str` in `texts`
/// in sequence, separated on [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
#[cfg(feature = "numpy")]
#[pyfunction]
pub fn segmentate_to_numpy(text: &str, bounds: Option<bool>, py: Python<'_>) -> PyResult<PyObject> {
    let array_py: Array1<PyObject> =
        segmentation::segmentate_to_py(text, bounds.unwrap_or(DEFAULT_BOUNDS), py);

    Ok(array_py.to_pyarray(py).into_py(py))
}

/// Returns a nested `numpy.ndarray` of `str` type over substrings of each `str` in
/// `texts` in sequence, separated on [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
#[cfg(feature = "numpy")]
#[pyfunction]
pub fn segmentate_all_to_numpy(
    texts: &PyArray<PyObject, Ix1>,
    bounds: Option<bool>,
    max_workers: Option<usize>,
    py: Python<'_>,
) -> PyResult<PyObject> {
    let texts: Array1<String> = texts.to_owned_array().map(|obj| obj.to_string());

    Ok(segmentation::segmentate_all_to_numpy(
        texts,
        bounds.unwrap_or(DEFAULT_BOUNDS),
        max_workers,
        py,
    )
    .to_pyarray(py)
    .into_py(py))
}

/// Returns a `polars.Series`  of `list[str]` type over substrings of each `str` in
/// `texts` in sequence, separated on [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries

/// Returns a `polars.Series` of `str` type over substrings of each `str` in `texts`
/// in sequence, separated on [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
#[cfg(feature = "polars")]
#[pyfunction]
pub fn segmentate_to_polars(text: &str, bounds: Option<bool>) -> PyResult<PySeries> {
    let series: Series = segmentation::segmentate(text, bounds.unwrap_or(DEFAULT_BOUNDS));

    Ok(PySeries(series))
}

/// Returns a `polars.Series`  of `list[str]` type over substrings of each `str` in
/// `texts` in sequence, separated on [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
#[cfg(feature = "polars")]
#[pyfunction]
pub fn segmentate_all_to_polars(
    texts: PySeries,
    bounds: Option<bool>,
    max_workers: Option<usize>,
) -> PyResult<PySeries> {
    let series: Series = texts.into();

    segmentation::segmentate_all_to_polars(series, bounds.unwrap_or(DEFAULT_BOUNDS), max_workers)
        .map(|series| PySeries(series))
        .map_err(|err| PyPolarsErr::Polars(err).into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn lib_unicode_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(segmentate, m)?)?;
    m.add_function(wrap_pyfunction!(segmentate_all, m)?)?;

    #[cfg(feature = "numpy")]
    m.add_function(wrap_pyfunction!(segmentate_to_numpy, m)?)?;

    #[cfg(feature = "numpy")]
    m.add_function(wrap_pyfunction!(segmentate_all_to_numpy, m)?)?;

    #[cfg(feature = "polars")]
    m.add_function(wrap_pyfunction!(segmentate_to_polars, m)?)?;

    #[cfg(feature = "polars")]
    m.add_function(wrap_pyfunction!(segmentate_all_to_polars, m)?)?;

    Ok(())
}
