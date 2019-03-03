/*!
# text-vectorize

Text vectorizers and TF-IDF transforms

# Introduction



# Features

 - bag of word, and character 1-gram vectorization of text documents
 - optional hashing vectorization using fast
 - API and implementation inspired by `CountVectorizer` and `HashingVectorizer`
   estimators in [scikit-learn](https://scikit-learn.org/).

# Example

```rust


let documents = vec![
    String::from("the moon in the sky"),
    String::from("The sky sky sky is blue"),
];


```
*/

#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
extern crate fasthash;
extern crate fnv;
extern crate regex;
#[macro_use]
extern crate ndarray;
extern crate rayon;
extern crate sprs;

use crate::math::CSRArray;
use fnv::FnvHashMap;
use ndarray::Array;
use sprs::CsMat;

use rayon::prelude::*;

const TOKEN_PATTERN_DEFAULT: &str = r"\b\w\w+\b";

#[cfg(test)]
mod tests;

mod math;
pub mod tokenize;

/// Sort features by name
///
/// Returns a reordered matrix and modifies the vocabulary in place
fn _sort_features(X: &mut CSRArray, vocabulary: &mut FnvHashMap<String, i32>) {
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
fn _sum_duplicates(tf: &mut CSRArray, indices_local: &[u32], nnz: &mut usize) {
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
pub struct HashingVectorizer {
    lowercase: bool,
    token_pattern: String,
    n_features: u32,
}

#[derive(Debug)]
pub struct CountVectorizer {
    lowercase: bool,
    token_pattern: String,
    pub vocabulary: FnvHashMap<String, i32>,
}

pub enum Vectorizer {}

impl CountVectorizer {
    /// Initialize a CountVectorizer estimator
    pub fn new() -> Self {
        CountVectorizer {
            lowercase: true,
            token_pattern: String::from(TOKEN_PATTERN_DEFAULT),
            vocabulary: FnvHashMap::with_capacity_and_hasher(1000, Default::default()),
        }
    }

    /// Fit the extimator
    ///
    /// This lears the vocabulary
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

        // we use a localy scoped vocabulary
        let mut vocabulary: FnvHashMap<String, i32> =
            FnvHashMap::with_capacity_and_hasher(1000, Default::default());

        let mut nnz: usize = 0;
        let mut indices_local = Vec::new();

        let tokenizer = tokenize::RegexpTokenizer::new(TOKEN_PATTERN_DEFAULT.to_string());

        // TODO: there should be a simpler way of handling this?
        let tokenize = |doc: String| -> Vec<String> {
            tokenizer.tokenize(&doc).map(|x| x.to_string()).collect()
        };

        let pipe: Vec<Vec<String>> = X
            .par_iter()
            .map(|doc| doc.to_ascii_lowercase())
            .map(|doc| tokenize(doc))
            .collect();

        for (_document_id, tokens) in pipe.iter().enumerate() {
            indices_local.clear();
            for token in tokens {
                let vocabulary_size = vocabulary.len() as i32;
                // TODO: don't convert to Sting here
                let token_id = vocabulary
                    .entry(token.to_string())
                    .or_insert(vocabulary_size);
                indices_local.push(*token_id as u32);
            }
            // this takes 10-15% of the compute time
            indices_local.sort_unstable();

            _sum_duplicates(&mut tf, &indices_local, &mut nnz);
        }

        // Copy to the vocabulary in the struct and make it own data
        for (key, value) in vocabulary.drain() {
            self.vocabulary.insert(key.to_owned(), value);
        }

        _sort_features(&mut tf, &mut self.vocabulary);

        CsMat::new(
            (tf.indptr.len() - 1, self.vocabulary.len()),
            tf.indptr,
            tf.indices,
            tf.data,
        )
    }

    pub fn fit_transform(&mut self, X: &[String]) -> CsMat<i32> {
        // Fit and transform
        //
        self._fit_transform(X, true)
    }
}

impl HashingVectorizer {
    pub fn new() -> Self {
        // Create a new HashingVectorizer estimator
        HashingVectorizer {
            lowercase: true,
            token_pattern: String::from(TOKEN_PATTERN_DEFAULT),
            n_features: 1048576,
        }
    }

    pub fn fit(self, _X: &[String]) -> Self {
        // Fit method
        //
        // The vectorizer is stateless, this has no effect
        self
    }

    pub fn transform(&self, X: &[String]) -> CsMat<i32> {
        // Transform method

        let mut tf = crate::math::CSRArray {
            indices: Vec::new(),
            indptr: Vec::new(),
            data: Vec::new(),
        };

        tf.indptr.push(0);

        let mut indices_local = Vec::new();
        let mut nnz: usize = 0;

        let tokenizer = tokenize::RegexpTokenizer::new(TOKEN_PATTERN_DEFAULT.to_string());

        // TODO: there should be a simpler way of handling this?
        let tokenize = |doc: String| -> Vec<String> {
            tokenizer.tokenize(&doc).map(|x| x.to_string()).collect()
        };
        // String.to_lowercase() is very slow
        // https://www.reddit.com/r/rust/comments/6wbru2/performance_issue_can_i_avoid_of_using_the_slow/
        // https://github.com/rust-lang/rust/issues/26244
        // Possibly use: https://github.com/JuliaStrings/utf8proc
        // http://www.unicode.org/faq/casemap_charprop.html
        //
        let pipe: Vec<Vec<String>> = X
            .par_iter()
            .map(|doc| doc.to_ascii_lowercase())
            .map(|doc| tokenize(doc))
            .collect();

        for (_document_id, tokens) in pipe.iter().enumerate() {
            indices_local.clear();
            for token in tokens {
                let hash = fasthash::murmur3::hash32(&token);
                let hash = hash % self.n_features;

                indices_local.push(hash);
            }
            // this takes 10-15% of the compute time
            indices_local.sort_unstable();

            _sum_duplicates(&mut tf, &indices_local, &mut nnz);
        }

        CsMat::new(
            (tf.indptr.len() - 1, self.n_features as usize),
            tf.indptr,
            tf.indices,
            tf.data,
        )
    }

    pub fn fit_transform(&self, X: &[String]) -> CsMat<i32> {
        // Fit and transform
        //
        self.transform(X)
    }
}
