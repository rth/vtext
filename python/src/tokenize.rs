// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use pyo3::prelude::*;
use pyo3::types::PyList;

use vtext::tokenize::Tokenizer;

/// __init__(self, word_bounds=True)
///
/// Unicode Segmentation tokenizer
///
/// This implementation is a thin wrapper around the
/// `unicode-segmentation <https://github.com/unicode-rs/unicode-segmentation>`_
/// crate.
///
/// References
/// ----------
/// - `Unicode® Standard Annex #29 <http://www.unicode.org/reports/tr29/>`_
#[pyclass]
pub struct UnicodeSegmentTokenizer {
    pub word_bounds: bool,
    inner: vtext::tokenize::UnicodeSegmentTokenizer,
}

#[pymethods]
impl UnicodeSegmentTokenizer {
    #[new]
    #[args(word_bounds = true)]
    fn new(obj: &PyRawObject, word_bounds: bool) {
        let tokenizer = vtext::tokenize::UnicodeSegmentTokenizer::new(word_bounds);

        obj.init(UnicodeSegmentTokenizer {
            word_bounds: word_bounds,
            inner: tokenizer,
        });
    }

    /// tokenize(self, x)
    ///
    /// Tokenize a string
    ///
    /// Parameters
    /// ----------
    /// x : bool
    ///   the string to tokenize
    ///
    /// Returns
    /// -------
    /// tokens : List[str]
    ///    computed tokens
    fn tokenize<'py>(&self, py: Python<'py>, x: &str) -> PyResult<(&'py PyList)> {
        let res: Vec<&str> = self.inner.tokenize(x).collect();
        let list = PyList::new(py, res);
        Ok(list)
    }
}

/// __init__(self, lang)
///
/// VText tokenizer
///
/// This tokenizer a few additional rules on top of word boundaries computed
/// by unicode segmentation.
///
/// Additional language specific rules are implemented for English (en),
/// and French (en). Providing `lang` parameter with any other value, will silently
/// fall back to ``lang='any'``.
///
///
/// References
/// ----------
///
/// - `Unicode® Standard Annex #29 <http://www.unicode.org/reports/tr29/>`_
#[pyclass]
pub struct VTextTokenizer {
    pub lang: String,
    inner: vtext::tokenize::VTextTokenizer,
}

#[pymethods]
impl VTextTokenizer {
    #[new]
    fn new(obj: &PyRawObject, lang: String) {
        let tokenizer = vtext::tokenize::VTextTokenizer::new(&lang);
        obj.init(VTextTokenizer {
            lang: lang,
            inner: tokenizer,
        });
    }

    /// tokenize(self, x)
    ///
    /// Tokenize a string
    ///
    /// Parameters
    /// ----------
    /// x : bool
    ///    the string to tokenize
    ///
    /// Returns
    /// -------
    /// tokens : List[str]
    ///    computed tokens
    fn tokenize<'py>(&self, py: Python<'py>, x: &str) -> PyResult<(&'py PyList)> {
        let res: Vec<&str> = self.inner.tokenize(x).collect();
        let list = PyList::new(py, res);
        Ok(list)
    }
}

/// __init__(self, pattern=r'\\b\\w\\w+\\b')
///
/// Tokenize a document using regular expressions
#[pyclass]
pub struct RegexpTokenizer {
    pub pattern: String,
    inner: vtext::tokenize::RegexpTokenizer,
}

#[pymethods]
impl RegexpTokenizer {
    #[new]
    #[args(pattern = "\"\\\\b\\\\w\\\\w+\\\\b\"")]
    fn new(obj: &PyRawObject, pattern: &str) {
        let inner = vtext::tokenize::RegexpTokenizer::new(pattern.to_owned());

        obj.init(RegexpTokenizer {
            pattern: pattern.to_string(),
            inner: inner,
        });
    }

    /// tokenize(self, x)
    ///
    /// Tokenize a string
    ///
    /// Parameters
    /// ----------
    /// x : bool
    ///    the string to tokenize
    ///
    /// Returns
    /// -------
    /// tokens : List[str]
    ///    computed tokens
    fn tokenize<'py>(&self, py: Python<'py>, x: &str) -> PyResult<(&'py PyList)> {
        let res: Vec<&str> = self.inner.tokenize(x).collect();
        let list = PyList::new(py, res);
        Ok(list)
    }
}

/// __init__(self, window_size=4)
///
/// Character tokenizer
///
/// Parameters
/// ----------
/// window_size : str, default=4
///   number of consecutive characters included in a token
///
/// Example
/// -------
/// >>> from vtext.tokenize import CharacterTokenizer
/// >>> tokenizer = CharacterTokenizer(window_size=4)
/// >>> tokenizer.tokenize('fox can\'t')
/// ['fox ', 'ox c', 'x ca', ' can', 'can\'', 'an\'t']
///
#[pyclass]
pub struct CharacterTokenizer {
    pub window_size: usize,
    inner: vtext::tokenize::CharacterTokenizer,
}

#[pymethods]
impl CharacterTokenizer {
    #[new]
    #[args(window_size = 4)]
    fn new(obj: &PyRawObject, window_size: usize) {
        let inner = vtext::tokenize::CharacterTokenizer::new(window_size);

        obj.init(CharacterTokenizer {
            window_size: window_size,
            inner: inner,
        });
    }

    /// tokenize(self, x)
    ///
    /// Tokenize a string
    ///
    /// Parameters
    /// ----------
    /// x : bool
    ///    the string to tokenize
    ///
    /// Returns
    /// -------
    /// tokens : List[str]
    ///    computed tokens
    fn tokenize<'py>(&self, py: Python<'py>, x: &str) -> PyResult<(&'py PyList)> {
        let res: Vec<&str> = self.inner.tokenize(x).collect();
        let list = PyList::new(py, res);
        Ok(list)
    }
}
