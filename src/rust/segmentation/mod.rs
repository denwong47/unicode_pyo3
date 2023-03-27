use rayon::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

use crate::config::CPU_COUNT;

/// Returns a [`Vec<&str>`] over substrings of self separated on
/// [UAX#29 sentence boundaries].
///
/// [UAX#29 sentence boundaries]: http://www.unicode.org/reports/tr29/#Sentence_Boundaries
pub fn segmentate(text: &str, bounds: bool) -> Vec<&str> {
    if bounds {
        text.split_sentence_bounds().collect()
    } else {
        text.unicode_sentences().collect()
    }
}

/// Internal function running multiple texts through [`segmentate`] in serial.
fn segmentate_multiple_serial<'a>(texts: &[&'a str], bounds: bool) -> Vec<Vec<&'a str>> {
    texts
        .into_iter()
        .map(|text| segmentate(text, bounds))
        .collect()
}

/// Internal function running multiple texts through [`segmentate`] in parallel.
fn segmentate_multiple_parallel<'a>(
    texts: &[&'a str],
    bounds: bool,
    max_workers: usize,
) -> Vec<Vec<&'a str>> {
    let chunk_size = (texts.len() as f32 / max_workers as f32).ceil() as usize;

    texts
        .into_par_iter()
        .fold_chunks(
            chunk_size,
            || Vec::new(),
            |mut v, text| {
                v.append(&mut segmentate(text, bounds));
                v
            },
        )
        .collect()
}

///
pub fn segmentate_all<'a>(
    texts: &[&'a str],
    bounds: bool,
    max_workers: Option<usize>,
) -> Vec<Vec<&'a str>> {
    let max_workers = max_workers.unwrap_or(CPU_COUNT.clone());

    if texts.len() >= max_workers {
        segmentate_multiple_parallel(texts, bounds, max_workers)
    } else {
        segmentate_multiple_serial(texts, bounds)
    }
}
