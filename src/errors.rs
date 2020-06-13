use std::error::Error;
use std::fmt;
use regex;
use thiserror::Error;

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
