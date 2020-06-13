use std::error::Error;
use std::fmt;
use regex;
use thiserror::Error;
#[cfg(feature = "python")]
use pyo3;


#[derive(Error, Debug)]
pub enum EstimatorErr {
    #[error("Invalid paramer: `{0}`")]
    InvalidParams(String),
    #[error("Invalid regex paramer")]
    RegexErr {
        #[from]
        source: regex::Error
    }
}

#[cfg(feature = "python")]
impl From<EstimatorErr> for pyo3::PyErr {
    fn from(err: EstimatorErr) -> pyo3::PyErr {
        pyo3::PyErr::new::<pyo3::exceptions::ValueError, _>(format!("{}", err))
    }
}
