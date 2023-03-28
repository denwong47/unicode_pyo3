#[cfg(feature = "numpy")]
use pyo3::prelude::*;

#[cfg(feature = "numpy")]
use numpy::ndarray::{Array1, Axis};

#[cfg(feature = "numpy")]
use numpy::ToPyArray;

use rayon::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "polars")]
use polars::prelude::*;

use crate::config::CPU_COUNT;

/// Returns a generic type over substrings of self separated on
/// [UAX#29 sentence boundaries]. This is typically a [`Vec<&str>`] or a
/// [`Series`] (with feature `polars` enabled).
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
pub fn segmentate<'a, T>(text: &'a str, bounds: bool) -> T
where
    T: FromIterator<&'a str>,
{
    if bounds {
        text.split_sentence_bounds().collect::<T>()
    } else {
        text.unicode_sentences().collect::<T>()
    }
}

/// Returns a ndarray of type [`PyObject`] over substrings of self separated on
/// [UAX#29 sentence boundaries]. This is typically a [`Vec<&str>`] or a
/// [`Series`] (with feature `polars` enabled).
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
#[cfg(feature = "numpy")]
pub fn segmentate_to_py<'a>(text: &'a str, bounds: bool, py: Python<'_>) -> Array1<PyObject> {
    if bounds {
        text.split_sentence_bounds()
            .map(|s| s.to_object(py))
            .collect()
    } else {
        text.unicode_sentences().map(|s| s.to_object(py)).collect()
    }
}

/// Internal function running multiple texts through [`segmentate`] in serial.
///
/// The generic type `T` is typically a [`Vec<&str>`] or a
/// [`Series`] (with feature `polars` enabled).
fn segmentate_multiple_serial<'a, T>(texts: &[&'a str], bounds: bool) -> Vec<T>
where
    T: FromIterator<&'a str>,
{
    texts
        .into_iter()
        .map(|text| segmentate(text, bounds))
        .collect()
}

/// Internal function running multiple texts through [`segmentate`] in parallel into a
/// [`Vec<Vec<&'str>>`]. Using [`Vec`] of [`Vec`] comes with performance penalties, and
/// requires cloning when moving data back to Python.
fn segmentate_multiple_parallel_to_vec<'a>(
    texts: &[&'a str],
    bounds: bool,
    max_workers: usize,
) -> Vec<Vec<&'a str>> {
    let chunk_size = (texts.len() as f32 / max_workers as f32).ceil() as usize;

    texts
        .into_par_iter()
        .fold_chunks(
            chunk_size,
            || Vec::with_capacity(chunk_size),
            |mut v, text| {
                v.push(segmentate::<Vec<&'a str>>(text, bounds));
                v
            },
        )
        .flatten()
        .collect()
}

/// Internal function running multiple texts through [`segmentate`] in parallel into an
/// [`Array1<Array1<PyObject>>`].
#[cfg(feature = "numpy")]
pub fn segmentate_multiple_parallel_to_numpy<'a>(
    texts: Array1<String>,
    bounds: bool,
    max_workers: usize,
    py: Python<'_>,
) -> Array1<PyObject> {
    let chunk_size = (texts.len() as f32 / max_workers as f32).ceil() as usize;

    let array = texts
        .axis_chunks_iter(Axis(0), chunk_size)
        .into_par_iter()
        .map(|arr| {
            Array1::from_iter(
                arr.into_iter()
                    .map(|text| segmentate::<Array1<&str>>(text, bounds)),
            )
        })
        .reduce(
            || Array1::from_vec(vec![]),
            |mut lhs, rhs| {
                lhs.append(Axis(0), rhs.view()).unwrap();
                lhs
            },
        );

    array
        .into_iter()
        .map(|arr| {
            {
                arr.into_iter()
                    .map(|sentence| sentence.to_object(py))
                    .collect::<Array1<PyObject>>()
            }
            .to_pyarray(py)
            .into_py(py)
        })
        .collect()
}

/// Internal function running multiple texts through [`segmentate`] in parallel into a
/// [`Series`] of [`DataType::List`].
#[cfg(feature = "polars")]
pub fn segmentate_multiple_parallel_to_polars<'a>(
    texts: Series,
    bounds: bool,
    max_workers: usize,
) -> Result<Series, PolarsError> {
    let chunk_size = (texts.len() as f32 / max_workers as f32).ceil() as usize;

    let iter: Vec<Series> = texts
        .utf8()?
        .par_iter_indexed()
        .fold_chunks(
            chunk_size,
            || Vec::with_capacity(chunk_size),
            |mut v, text_opt| {
                if let Some(text) = text_opt {
                    // Bad unwrapping, deal with later.
                    let mut series = segmentate::<Series>(text, bounds);
                    series.rename(text);
                    v.push(series);
                } else {
                }
                v
            },
        )
        .flatten()
        .collect();

    Ok(Series::new("sentences", &iter))
}

/// Returns a [`Vec<Vec<&str>>`] over substrings of each &[`str`] in `texts` in sequence,
/// separated on [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
pub fn segmentate_all<'a>(
    texts: &[&'a str],
    bounds: bool,
    max_workers: Option<usize>,
) -> Vec<Vec<&'a str>> {
    let max_workers = max_workers.unwrap_or(CPU_COUNT.clone());

    if texts.len() >= max_workers {
        segmentate_multiple_parallel_to_vec(texts, bounds, max_workers)
    } else {
        segmentate_multiple_serial(texts, bounds)
    }
}

/// Returns a [`Array1<Array1<PyObject>>`] over substrings of each &[`str`] in
/// `texts` in sequence, separated on [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
#[cfg(feature = "numpy")]
pub fn segmentate_all_to_numpy(
    texts: Array1<String>,
    bounds: bool,
    max_workers: Option<usize>,
    py: Python<'_>,
) -> Array1<PyObject> {
    let max_workers = max_workers.unwrap_or(CPU_COUNT.clone());

    segmentate_multiple_parallel_to_numpy(texts, bounds, max_workers, py)
}

/// Returns a [`Series`] of [`DataType::List`] over substrings of each &[`str`] in
/// `texts` in sequence, separated on [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
#[cfg(feature = "polars")]
pub fn segmentate_all_to_polars(
    texts: Series,
    bounds: bool,
    max_workers: Option<usize>,
) -> Result<Series, PolarsError> {
    let max_workers = max_workers.unwrap_or(CPU_COUNT.clone());

    segmentate_multiple_parallel_to_polars(texts, bounds, max_workers)
}
