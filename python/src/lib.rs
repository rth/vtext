#![feature(specialization)]

#[macro_use]
extern crate ndarray;
extern crate numpy;
extern crate pyo3;
extern crate text_vectorize;

use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::{pymodinit, ObjectProtocol, Py, PyModule, PyObject, PyResult, Python};
use pyo3::types::PyIterator;
use text_vectorize::CountVectorizer;

use text_vectorize::tokenize;

#[pymodinit]
fn _lib(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "count_vectorize")]
    fn count_vectorize(py: Python, x: PyObject) -> PyResult<(Py<PyArray1<i32>>)> {
        let text = PyIterator::from_object(py, &x)?;

        let mut collection: Vec<String> = Vec::new();

        for document in text {
            let document = document?;
            let document = ObjectProtocol::extract::<String>(document)?;
            collection.push(document);
        }

        let mut vect = CountVectorizer::new();
        let x = vect.fit_transform(&collection);

        println!("{:?}", collection);
        println!("{:?}", x);

        Ok((
            x.indices.into_pyarray(py).to_owned(),
            x.indptr.into_pyarray(py).to_owned(),
            x.data.into_pyarray(py).to_owned(),
        ))
    }

    Ok(())
}
