// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

#![feature(specialization)]

#[macro_use]
extern crate ndarray;
extern crate numpy;
#[macro_use]
extern crate pyo3;
extern crate rust_stemmers;
extern crate vtext;

use ndarray::arr1;
use numpy::{IntoPyArray, PyArray1};
use sprs::CsMat;

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::prelude::{pymodinit, ObjectProtocol, Py, PyModule, PyObject, PyResult, Python};
use pyo3::types::{PyIterator, PyString};

use vtext::metrics;
use vtext::tokenize;
use vtext::vectorize;

type PyCsrArray = (Py<PyArray1<i32>>, Py<PyArray1<i32>>, Py<PyArray1<i32>>);

fn iterable_to_collection(text: PyIterator) -> PyResult<Vec<String>> {
    // This should not be necessary, ideally PyIterator should be converted
    // to a Rust iterator

    let mut collection: Vec<String> = Vec::new();

    for document in text {
        let document = document?;
        let document = ObjectProtocol::extract::<String>(document)?;
        collection.push(document);
    }
    Ok(collection)
}

fn result_to_csr(py: Python, x: CsMat<i32>) -> PyResult<PyCsrArray> {
    // TODO: 1. use slices directly instead of creating new arrays
    //       2. Possibly avoid casing
    //          https://github.com/rust-ndarray/ndarray/issues/493#issuecomment-424043912
    let indices = arr1(x.indices()).mapv(|elem| elem as i32);
    let indptr = arr1(x.indptr()).mapv(|elem| elem as i32);
    let data = arr1(x.data());

    Ok((
        indices.into_pyarray(py).to_owned(),
        indptr.into_pyarray(py).to_owned(),
        data.into_pyarray(py).to_owned(),
    ))
}

#[pyclass]
pub struct _HashingVectorizerWrapper {
    inner: vtext::vectorize::HashingVectorizer,
}

#[pymethods]
impl _HashingVectorizerWrapper {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        let estimator = vtext::vectorize::HashingVectorizer::new();
        obj.init(|_token| _HashingVectorizerWrapper { inner: estimator })
    }

    fn transform(&mut self, py: Python, x: PyObject) -> PyResult<PyCsrArray> {
        let text = PyIterator::from_object(py, &x)?;

        let collection = iterable_to_collection(text)?;

        let x = self.inner.fit_transform(&collection);

        result_to_csr(py, x)
    }
}

#[pyclass]
pub struct _CountVectorizerWrapper {
    inner: vtext::vectorize::CountVectorizer,
}

#[pymethods]
impl _CountVectorizerWrapper {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        let estimator = vtext::vectorize::CountVectorizer::new();
        obj.init(|_token| _CountVectorizerWrapper { inner: estimator })
    }

    fn fit(&mut self, py: Python, x: PyObject) -> PyResult<()> {
        let text = PyIterator::from_object(py, &x)?;

        let collection = iterable_to_collection(text)?;

        self.inner.fit(&collection);
        Ok(())
    }

    fn get_n_features(&self, py: Python) -> PyResult<(usize)> {
        let n_features = self.inner.vocabulary.len();
        Ok(n_features)
    }

    fn transform(&mut self, py: Python, x: PyObject) -> PyResult<PyCsrArray> {
        let text = PyIterator::from_object(py, &x)?;

        let collection = iterable_to_collection(text)?;

        let x = self.inner.transform(&collection);

        result_to_csr(py, x)
    }

    fn fit_transform(&mut self, py: Python, x: PyObject) -> PyResult<PyCsrArray> {
        let text = PyIterator::from_object(py, &x)?;

        let collection = iterable_to_collection(text)?;

        let x = self.inner.fit_transform(&collection);

        result_to_csr(py, x)
    }
}

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
    fn __new__(obj: &PyRawObject, word_bounds: bool) -> PyResult<()> {
        let tokenizer = vtext::tokenize::UnicodeSegmentTokenizer::new(word_bounds);

        obj.init(|_token| UnicodeSegmentTokenizer {
            word_bounds: word_bounds,
            inner: tokenizer,
        })
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
    fn tokenize(&self, py: Python, x: String) -> PyResult<(Vec<String>)> {
        let x = x.to_string();

        let res = self.inner.tokenize(&x);
        let res = res.map(|s| s.to_string()).collect();
        Ok((res))
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
    fn __new__(obj: &PyRawObject, lang: String) -> PyResult<()> {
        let tokenizer = vtext::tokenize::VTextTokenizer::new(&lang);
        obj.init(|_token| VTextTokenizer {
            lang: lang,
            inner: tokenizer,
        })
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
    fn tokenize(&self, py: Python, x: String) -> PyResult<(Vec<String>)> {
        let x = x.to_string();

        let res = self.inner.tokenize(&x);
        let res = res.map(|s| s.to_string()).collect();
        Ok((res))
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
    fn __new__(obj: &PyRawObject, pattern: &str) -> PyResult<()> {
        let inner = vtext::tokenize::RegexpTokenizer::new(pattern.to_owned());

        obj.init(|_token| RegexpTokenizer {
            pattern: pattern.to_string(),
            inner: inner,
        })
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
    fn tokenize(&self, py: Python, x: String) -> PyResult<(Vec<String>)> {
        // TODO: reduce the number of copies here
        let res = self.inner.tokenize(&x);
        let res: Vec<String> = res.map(|s| s.to_string()).collect();
        Ok((res))
    }
}

/// __init__(self, pattern=r'\\b\\w\\w+\\b')
///
/// Character tokenizer
#[pyclass]
pub struct CharacterTokenizer {
    pub window_size: usize,
    inner: vtext::tokenize::CharacterTokenizer,
}

#[pymethods]
impl CharacterTokenizer {
    #[new]
    #[args(window_size = 4)]
    fn __new__(obj: &PyRawObject, window_size: usize) -> PyResult<()> {
        let inner = vtext::tokenize::CharacterTokenizer::new(window_size);

        obj.init(|_token| CharacterTokenizer {
            window_size: window_size,
            inner: inner,
        })
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
    fn tokenize(&self, py: Python, x: String) -> PyResult<(Vec<String>)> {
        // TODO: reduce the number of copies here
        let res = self.inner.tokenize(&x);
        let res: Vec<String> = res.map(|s| s.to_string()).collect();
        Ok((res))
    }
}

/// __init__(self, lang='english')
///
/// Snowball stemmer
///
/// Wraps the rust-stemmers crate that uses an implementation generated
/// by the `Snowball compiler <https://github.com/snowballstem/snowball>`_
/// for Rust.
#[pyclass]
pub struct SnowballStemmer {
    pub lang: String,
    inner: rust_stemmers::Stemmer,
}

#[pymethods]
impl SnowballStemmer {
    #[new]
    #[args(lang = "\"english\"")]
    fn __new__(obj: &PyRawObject, lang: &str) -> PyResult<()> {
        let algorithm = match lang {
            "arabic" => Ok(rust_stemmers::Algorithm::Arabic),
            "danish" => Ok(rust_stemmers::Algorithm::Danish),
            "dutch" => Ok(rust_stemmers::Algorithm::Dutch),
            "english" => Ok(rust_stemmers::Algorithm::English),
            "french" => Ok(rust_stemmers::Algorithm::French),
            "german" => Ok(rust_stemmers::Algorithm::German),
            "greek" => Ok(rust_stemmers::Algorithm::Greek),
            "hungarian" => Ok(rust_stemmers::Algorithm::Hungarian),
            "italian" => Ok(rust_stemmers::Algorithm::Italian),
            "portuguese" => Ok(rust_stemmers::Algorithm::Portuguese),
            "romanian" => Ok(rust_stemmers::Algorithm::Romanian),
            "russian" => Ok(rust_stemmers::Algorithm::Russian),
            "spanish" => Ok(rust_stemmers::Algorithm::Spanish),
            "swedish" => Ok(rust_stemmers::Algorithm::Swedish),
            "tamil" => Ok(rust_stemmers::Algorithm::Tamil),
            "turkish" => Ok(rust_stemmers::Algorithm::Turkish),
            _ => Err(exceptions::ValueError::py_err(format!(
                "lang={} is unsupported!",
                lang
            ))),
        }?;

        let stemmer = rust_stemmers::Stemmer::create(algorithm);

        obj.init(|_token| SnowballStemmer {
            lang: lang.to_string(),
            inner: stemmer,
        })
    }

    /// stem(self, word)
    ///
    /// Stem a string
    ///
    /// Parameters
    /// ----------
    /// word : str
    ///    the string to tokenize
    ///
    /// Returns
    /// -------
    /// word_stemmed : str
    ///      stemmed word
    fn stem(&self, py: Python, word: &str) -> PyResult<(String)> {
        let res = self.inner.stem(word).to_string();
        Ok((res))
    }
}

///  dice_similarity(x, y)
///
///  Sørensen–Dice similarity coefficient
///
///  This similarity tokenizes the input string x, y as 2-char n-grams,
///  into two sets of tokens X, Y then computes,
///
///  similarity(x, y) = 2 * |X ∩ Y| / (|X| + |Y|)
///
///  where |X| is the cardinality of set X.
///
///  Parameters
///  ----------
///  x : str
///     string to compare
///  y : str
///     string to compare
///
///  Result
///  ------
///  similarity : float
///     computed similarity
///
///  Example
///  -------
///  >>> dice_similarity('yesterday', 'today')
///  0.333..
#[pyfunction]
fn dice_similarity(x: &str, y: &str) -> PyResult<f64> {
    Ok(metrics::string::dice_similarity(x, y))
}

///  jaro_similarity(x, y)
///
///  Jaro similarity
///
///  The `Jaro
///  similarity <https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance#Jaro_Similarity>`_
///  accounts for the minimal number of character transpositions needed
///  to change one word in another.
///
///  References
///  ----------
///
///  Matthew A. Jaro (1989). Advances in record linkage methodology
///  as applied to the 1985 census of Tampa Florida. Journal of the
///  American Statistical Association. 84 (406): 414-20.
///
///  Parameters
///  ----------
///  x : str
///     string to compare
///  y : str
///     string to compare
///
///  Result
///  ------
///  similarity : float
///     computed similarity
///
///  Example
///  -------
///  >>> jaro_similarity('yesterday', 'today')
///  0.581..
#[pyfunction]
fn jaro_similarity(x: &str, y: &str) -> PyResult<f64> {
    Ok(metrics::string::jaro_similarity(x, y))
}

///  jaro_winkler_similarity(x, y, p, max_l)
///
///  Jaro Winkler similarity
///
///  The `Jaro-Winkler
///  similarity <https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance>`_
///  accounts for the minimal number of character transpositions needed
///  to change one word in another, and the length of the longest common prefix
///
///  The default values for parameters are ``p=0.1``, ``max_l=4``
///
///  References
///  ----------
///
///  William E. Winkler. 1990. String Comparator Metrics and Enhanced
///  Decision Rules in the Fellegi-Sunter Model of Record Linkage.
///  Proceedings of the Section on Survey Research Methods.
///  American Statistical Association: 354-359.
///
///  Parameters
///  ----------
///  x : str
///     string to compare
///  y : str
///     string to compare
///  p : float, default=0.1
///     the constant scaling factor to overweigh common prefixes
///  max_l : int, default=4
///     the length of common prefix at the start of the string
///
///  Result
///  ------
///  similarity : float
///     computed similarity
///
///  Example
///  -------
///  >>> jaro_winkler_similarity('yesterday', 'today')
///  0.581..
#[pyfunction]
fn jaro_winkler_similarity(x: &str, y: &str, p: f64, max_l: usize) -> PyResult<f64> {
    Ok(metrics::string::jaro_winkler_similarity(x, y, p, max_l))
}

///  edit_distance(x, y, substitution_cost, transpositions)
///
///  Levenshtein edit distance
///
///  It corresponds to the minimum number of single-character edits
///  (insertions, deletions, substitutions, and optionally transpositions)
///  required to change one word into the other.
///
///  Parameters
///  ----------
///  x : str
///     string to compare
///  y : str
///     string to compare
///  substitution_cost : int
///     the cost associated with one character substitutions
///  transpositions : bool
///     if True, transpositions are also taken into account
///
///  Result
///  ------
///  distance : float
///     computed distance
///
///  Example
///  -------
///  >>> edit_distance('yesterday', 'today')
///  4.0
#[pyfunction]
fn edit_distance(
    x: &str,
    y: &str,
    substitution_cost: usize,
    transpositions: bool,
) -> PyResult<f64> {
    Ok(metrics::string::edit_distance(
        x,
        y,
        substitution_cost,
        transpositions,
    ))
}

#[pymodinit]
fn _lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<_HashingVectorizerWrapper>()?;
    m.add_class::<_CountVectorizerWrapper>()?;
    m.add_class::<UnicodeSegmentTokenizer>()?;
    m.add_class::<RegexpTokenizer>()?;
    m.add_class::<VTextTokenizer>()?;
    m.add_class::<CharacterTokenizer>()?;
    m.add_class::<SnowballStemmer>()?;
    m.add_function(wrap_function!(dice_similarity))?;
    m.add_function(wrap_function!(jaro_similarity))?;
    m.add_function(wrap_function!(jaro_winkler_similarity))?;
    m.add_function(wrap_function!(edit_distance))?;
    Ok(())
}
