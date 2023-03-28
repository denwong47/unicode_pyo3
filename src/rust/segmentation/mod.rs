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

/// Internal function running multiple texts through [`segmentate`] in parallel into a
/// [`ChunkedArray`] of [`DataType::List`].
#[cfg(feature = "polars")]
pub fn segmentate_multiple_parallel_to_list<'a>(
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

/// Returns a `List[List[str]]` over substrings of each `str` in `texts` in sequence,
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
