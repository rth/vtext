#![feature(specialization)]

#[macro_use]
extern crate ndarray;
extern crate numpy;
#[macro_use]
extern crate pyo3;
extern crate text_vectorize;

use ndarray::arr1;
use numpy::{IntoPyArray, PyArray1};
use sprs::CsMat;

use pyo3::prelude::*;
use pyo3::prelude::{pymodinit, ObjectProtocol, Py, PyModule, PyObject, PyResult, Python};
use pyo3::types::{PyIterator, PyString};

use text_vectorize::tokenize;
use text_vectorize::{CountVectorizer, HashingVectorizer};

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
}

#[pymethods]
impl UnicodeSegmentTokenizer {
    #[new]
    #[args(word_bounds = true)]
    fn __new__(obj: &PyRawObject, word_bounds: bool) -> PyResult<()> {
        obj.init(|_token| UnicodeSegmentTokenizer {
            word_bounds: word_bounds,
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
        let tokenizer = text_vectorize::tokenize::UnicodeSegmentTokenizer::new(self.word_bounds);

        let x = x.to_string();

        let res = tokenizer.tokenize(&x);
        let res = res.iter().map(|s| s.to_string()).collect();
        Ok((res))
    }
}

/// Tokenize a document using regular expressions
#[pyclass]
pub struct RegexpTokenizer {
    pub pattern: String,
    inner: text_vectorize::tokenize::RegexpTokenizer,
}

#[pymethods]
impl RegexpTokenizer {
    #[new]
    //    #[args(pattern = "\\b\\w\\w+\\b".to_string())]
    fn __new__(obj: &PyRawObject, pattern: String) -> PyResult<()> {

        let inner = text_vectorize::tokenize::RegexpTokenizer::new(pattern.to_owned());

        obj.init(|_token| RegexpTokenizer {
            pattern: pattern,
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
        let x = x.to_string();

        // let res = tokenizer.tokenize(&x);
        // let res = res.iter().map(|s| s.to_string()).collect();
        let res: Vec<String> = vec!["test".to_string()];
        Ok((res))
    }
}

#[pymodinit]
fn _lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<_HashingVectorizerWrapper>()?;
    m.add_class::<_CountVectorizerWrapper>()?;
    m.add_class::<UnicodeSegmentTokenizer>()?;
    m.add_class::<RegexpTokenizer>()?;

    Ok(())
}
