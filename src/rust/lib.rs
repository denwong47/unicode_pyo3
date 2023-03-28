use pyo3::prelude::*;

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

/// Returns a `polars.Series[List[str]]` over substrings of each `str` in `texts` in
/// sequence, separated on [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
#[cfg(feature = "polars")]
#[pyfunction]
pub fn segmentate_to_polars(text: &str, bounds: Option<bool>) -> PyResult<PySeries> {
    let series: Series = segmentation::segmentate(text, bounds.unwrap_or(DEFAULT_BOUNDS));

    Ok(PySeries(series))
}

#[cfg(feature = "polars")]
#[pyfunction]
pub fn segmentate_all_to_polars(
    texts: PySeries,
    bounds: Option<bool>,
    max_workers: Option<usize>,
) -> PyResult<PySeries> {
    let series: Series = texts.into();

    segmentation::segmentate_multiple_parallel_to_list(
        series,
        bounds.unwrap_or(DEFAULT_BOUNDS),
        max_workers.unwrap_or(4),
    )
    .map(|series| PySeries(series))
    .map_err(|err| PyPolarsErr::Polars(err).into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn lib_unicode_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(segmentate, m)?)?;
    m.add_function(wrap_pyfunction!(segmentate_all, m)?)?;

    #[cfg(feature = "polars")]
    m.add_function(wrap_pyfunction!(segmentate_to_polars, m)?)?;

    #[cfg(feature = "polars")]
    m.add_function(wrap_pyfunction!(segmentate_all_to_polars, m)?)?;

    Ok(())
}
