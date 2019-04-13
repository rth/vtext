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

use vtext::tokenize;
use vtext::{CountVectorizer, HashingVectorizer};

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
    inner: HashingVectorizer,
}

#[pymethods]
impl _HashingVectorizerWrapper {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        let estimator = HashingVectorizer::new();
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
    inner: CountVectorizer,
}

#[pymethods]
impl _CountVectorizerWrapper {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        let estimator = CountVectorizer::new();
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

/// Unicode Segmentation tokenizer
///
/// This implementation is a thin wrapper around the
/// `unicode-segmentation` crate
///
/// ## References
///
/// * [Unicode® Standard Annex #29](http://www.unicode.org/reports/tr29/)
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

    /// Tokenize a string
    ///
    /// ## Parameters
    ///  - x : bool
    ///    the string to tokenize
    ///
    /// ## Returns
    ///  - tokens : List<str>
    fn tokenize(&self, py: Python, x: String) -> PyResult<(Vec<String>)> {
        let x = x.to_string();

        let res = self.inner.tokenize(&x);
        let res = res.map(|s| s.to_string()).collect();
        Ok((res))
    }
}

/// VText tokenizer
///
/// This tokenizer a few additional rules on top of word boundaries computed
/// by unicode segmentation.
///
/// Additional language specific rules are implemented for English (en),
/// and French (en). Providing `lang` parameter with any other value, will silently
/// fall back to `lang='any'`.
///
///
/// ## References
///
/// * [Unicode® Standard Annex #29](http://www.unicode.org/reports/tr29/)
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

    /// Tokenize a string
    ///
    /// ## Parameters
    ///  - x : bool
    ///    the string to tokenize
    ///
    /// ## Returns
    ///  - tokens : List<str>
    fn tokenize(&self, py: Python, x: String) -> PyResult<(Vec<String>)> {
        let x = x.to_string();

        let res = self.inner.tokenize(&x);
        let res = res.map(|s| s.to_string()).collect();
        Ok((res))
    }
}

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

    /// Tokenize a string
    ///
    /// ## Parameters
    ///  - x : bool
    ///    the string to tokenize
    ///
    /// ## Returns
    ///  - tokens : List<str>
    fn tokenize(&self, py: Python, x: String) -> PyResult<(Vec<String>)> {
        // TODO: reduce the number of copies here
        let res = self.inner.tokenize(&x);
        let res: Vec<String> = res.map(|s| s.to_string()).collect();
        Ok((res))
    }
}

/// Snowball stemmer
///
/// Wraps the rust-stemmers crate that uses an implementation generated
/// by the [Snowball compiler](https://github.com/snowballstem/snowball)
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

    /// Stem a string
    ///
    /// ## Parameters
    ///
    /// word : str
    ///    the string to tokenize
    ///
    /// ## Returns
    ///
    /// word_stemmed : str
    ///      stemmed word
    fn stem(&self, py: Python, word: &str) -> PyResult<(String)> {
        let res = self.inner.stem(word).to_string();
        Ok((res))
    }
}

#[pymodinit]
fn _lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<_HashingVectorizerWrapper>()?;
    m.add_class::<_CountVectorizerWrapper>()?;
    m.add_class::<UnicodeSegmentTokenizer>()?;
    m.add_class::<RegexpTokenizer>()?;
    m.add_class::<VTextTokenizer>()?;
    m.add_class::<SnowballStemmer>()?;
    Ok(())
}
