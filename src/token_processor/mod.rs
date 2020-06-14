// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

/*!
# Token processor modules

This modules includes estimators that operate on tokens, for instance for stop words filtering,
n-gram construction or stemming.
*/

use std::fmt;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::errors::VTextError;

pub trait TokenProcessor: fmt::Debug {
    fn transform<'a>(&'a self, tokens: dyn Iterator<Item = &'a str>) -> Box<dyn Iterator<Item = &'a str> + 'a>;
}

/// Stop words filter
///
#[derive(Clone)]
pub struct StopWordFilter {
    pub params: StopWordFilterParams,
}

/// Builder for the stop words filter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
pub struct StopWordFilterParams {
    stop_words: HashSet<String>,
}

impl StopWordFilterParams {
    pub fn stop_words(&mut self, value: Vec<&str>) -> StopWordFilterParams {
        self.stop_words = value.iter().map(|el| el.to_string()).collect();
        self.clone()
    }
    pub fn build(&mut self) -> Result<StopWordFilter, VTextError> {
        Ok(StopWordFilter {
            params: self.clone(),
        })
    }
}

impl Default for StopWordFilterParams {
    /// Create a new instance
    fn default() -> StopWordFilterParams {
        StopWordFilterParams {
            stop_words: vec!["and", "or", "this"].iter().map(|el| el.to_string()).collect()
        }
    }
}

impl Default for StopWordFilter {
    /// Create a new instance
    fn default() -> StopWordFilter {
        StopWordFilterParams::default().build().unwrap()
    }
}
