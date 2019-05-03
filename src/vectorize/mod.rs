// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

/*!
# Vectorization module

This module allows computing a sparse document term matrix from a text corpus.

```rust
extern crate vtext;

use vtext::tokenize::{VTextTokenizer,Tokenizer};
use vtext::vectorize::CountVectorizer;

let documents = vec![
    String::from("Some text input"),
    String::from("Another line"),
];

let tokenizer = VTextTokenizer::new("en");

let mut vectorizer = CountVectorizer::new(&tokenizer);
let X = vectorizer.fit_transform(&documents);
// returns a sparse CSR matrix with document-terms counts
*/

use crate::math::CSRArray;
use crate::tokenize;
use crate::tokenize::Tokenizer;
use hashbrown::HashMap;
use ndarray::Array;
use sprs::CsMat;

const TOKEN_PATTERN_DEFAULT: &str = r"\b\w\w+\b";

#[cfg(test)]
mod tests;

/// Sort features by name
///
/// Returns a reordered matrix and modifies the vocabulary in place
fn _sort_features(X: &mut CSRArray, vocabulary: &mut HashMap<String, i32>) {
    let mut vocabulary_sorted: Vec<_> = vocabulary.iter().collect();
    vocabulary_sorted.sort_unstable();
    let mut idx_map: Array<usize, _> = Array::zeros(vocabulary_sorted.len());
    for (idx_new, (_term, idx_old)) in vocabulary_sorted.iter().enumerate() {
        idx_map[**idx_old as usize] = idx_new;
    }
    for idx in 0..X.indices.len() {
        X.indices[idx] = idx_map[X.indices[idx]];
    }
}

/// Sum duplicates
#[inline]
fn _sum_duplicates(tf: &mut CSRArray, indices_local: &[i32], nnz: &mut usize) {
    let mut bucket: i32 = 0;
    let mut index_last = indices_local[0];

    for index_current in indices_local.iter().skip(1) {
        bucket += 1;
        if *index_current != index_last {
            tf.indices.push(index_last as usize);
            tf.data.push(bucket);
            *nnz += 1;
            index_last = *index_current;
            bucket = 0;
        }
    }
    tf.indices
        .push(indices_local[indices_local.len() - 1] as usize);
    if bucket == 0 {
        bucket += 1
    }
    tf.data.push(bucket);
    *nnz += 1;

    tf.indptr.push(*nnz);
}

#[derive(Debug)]
pub struct HashingVectorizer<'b> {
    lowercase: bool,
    tokenizer: &'b Tokenizer,
    n_features: u64,
}

#[derive(Debug)]
pub struct CountVectorizer<'b> {
    lowercase: bool,
    tokenizer: &'b Tokenizer,
    // vocabulary uses i32 indices, to avoid memory copies when converting
    // to sparse CSR arrays in Python with scipy.sparse
    pub vocabulary: HashMap<String, i32>,
}

pub enum Vectorizer {}

impl<'b> CountVectorizer<'b> {
    /// Initialize a CountVectorizer estimator
    pub fn new(tokenizer: &'b Tokenizer) -> Self {
        CountVectorizer {
            lowercase: true,
            tokenizer: tokenizer,
            vocabulary: HashMap::with_capacity_and_hasher(1000, Default::default()),
        }
    }

    /// Fit the estimator
    ///
    /// This lists the vocabulary
    pub fn fit(&mut self, X: &[String]) -> () {
        self._fit_transform(X, false);
    }

    /// Transform
    ///
    /// Converts a sequence of text documents to a CSR Matrix
    pub fn transform(&mut self, X: &[String]) -> CsMat<i32> {
        self._fit_transform(X, true)
    }

    /// Fit and transform (with optional fixed vocabulary)
    fn _fit_transform(&mut self, X: &[String], _fixed_vocabulary: bool) -> CsMat<i32> {
        let mut tf = crate::math::CSRArray {
            indices: Vec::new(),
            indptr: Vec::new(),
            data: Vec::new(),
        };

        tf.indptr.push(0);

        let mut nnz: usize = 0;
        let mut indices_local: Vec<i32> = Vec::new();

        let tokenizer = tokenize::RegexpTokenizer::new(TOKEN_PATTERN_DEFAULT.to_string());

        let pipe = X.iter().map(|doc| doc.to_ascii_lowercase());

        let mut vocabulary_size: i32 = 0;

        for (_document_id, document) in pipe.enumerate() {
            let tokens = tokenizer.tokenize(&document);

            indices_local.clear();
            for token in tokens {
                match self.vocabulary.get(token) {
                    Some(_id) => indices_local.push(*_id),
                    None => {
                        self.vocabulary.insert(token.to_string(), vocabulary_size);
                        indices_local.push(vocabulary_size);
                        vocabulary_size += 1;
                    }
                };
            }
            // this takes 10-15% of the compute time
            indices_local.sort_unstable();

            _sum_duplicates(&mut tf, indices_local.as_slice(), &mut nnz);
        }

        _sort_features(&mut tf, &mut self.vocabulary);

        CsMat::new(
            (tf.indptr.len() - 1, self.vocabulary.len()),
            tf.indptr,
            tf.indices,
            tf.data,
        )
    }

    /// Fit and transform
    ///
    pub fn fit_transform(&mut self, X: &[String]) -> CsMat<i32> {
        self._fit_transform(X, true)
    }
}

impl<'b> HashingVectorizer<'b> {
    /// Create a new HashingVectorizer estimator
    pub fn new(tokenizer: &'b Tokenizer) -> Self {
        HashingVectorizer {
            lowercase: true,
            tokenizer: tokenizer,
            n_features: 1048576,
        }
    }

    /// Fit method
    ///
    /// The vectorizer is stateless, this has no effect
    pub fn fit(self, _X: &[String]) -> Self {
        self
    }

    /// Transform method
    pub fn transform(&self, X: &[String]) -> CsMat<i32> {
        let mut tf = crate::math::CSRArray {
            indices: Vec::new(),
            indptr: Vec::new(),
            data: Vec::new(),
        };

        tf.indptr.push(0);

        let mut indices_local = Vec::new();
        let mut nnz: usize = 0;

        // String.to_lowercase() is very slow
        // https://www.reddit.com/r/rust/comments/6wbru2/performance_issue_can_i_avoid_of_using_the_slow/
        // https://github.com/rust-lang/rust/issues/26244
        // Possibly use: https://github.com/JuliaStrings/utf8proc
        // http://www.unicode.org/faq/casemap_charprop.html
        let pipe = X.iter().map(|doc| doc.to_ascii_lowercase());

        for (_document_id, document) in pipe.enumerate() {
            let tokens = self.tokenizer.tokenize(&document);
            indices_local.clear();
            for token in tokens {
                // set the RNG seeds to get reproducible hashing
                let hash = seahash::hash_seeded(token.as_bytes(), 1, 1000, 200, 89);
                let hash = (hash % self.n_features) as i32;

                indices_local.push(hash);
            }
            // this takes 10-15% of the compute time
            indices_local.sort_unstable();

            _sum_duplicates(&mut tf, indices_local.as_slice(), &mut nnz);
        }

        CsMat::new(
            (tf.indptr.len() - 1, self.n_features as usize),
            tf.indptr,
            tf.indices,
            tf.data,
        )
    }

    /// Fit and transform
    ///
    pub fn fit_transform(&self, X: &[String]) -> CsMat<i32> {
        self.transform(X)
    }
}
