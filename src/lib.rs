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

#[macro_use]
extern crate lazy_static;
extern crate fasthash;
extern crate fnv;
extern crate regex;
#[macro_use]
extern crate ndarray;
extern crate sprs;

use fnv::FnvHashMap;
use math::CSRArray;
use ndarray::Array;
use regex::Regex;

const TOKEN_PATTERN_DEFAULT: &str = r"\b\w\w+\b";

#[cfg(test)]
mod tests;

mod math;

/// Analyze tokens
///
/// Given a list of tokens (words or character groups) in a document,  
/// this corresponding word or character n-grams.
pub fn analyze<'a>(tokens: impl Iterator<Item = &'a str>) -> impl Iterator<Item = &'a str> {
    tokens
}

/// Tokenize text
///
/// Convert a text document into a vector of string tokens.
pub fn tokenize<'a>(text: &'a String) -> impl Iterator<Item = &'a str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(TOKEN_PATTERN_DEFAULT).unwrap();
    }

    RE.find_iter(text).map(|m| m.as_str())
}

/// Sort features by name
///
/// Returns a reordered matrix and modifies the vocabulary in place
fn _sort_features(X: &mut CSRArray, vocabulary: &mut FnvHashMap<String, i32>) {
    let mut vocabulary_sorted: Vec<_> = vocabulary.iter().collect();
    vocabulary_sorted.sort_unstable();
    let mut idx_map: Array<usize, _> = Array::zeros(vocabulary_sorted.len());
    for (idx_new, (term, idx_old)) in vocabulary_sorted.iter().enumerate() {
        idx_map[**idx_old as usize] = idx_new;
    }
    for idx in 0..X.indices.len() {
        X.indices[idx] = idx_map[X.indices[idx]];
    }
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
    vocabulary: FnvHashMap<String, i32>,
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
    pub fn transform(&mut self, X: &[String]) -> CSRArray {
        self._fit_transform(X, true)
    }

    /// Fit and transform (with optional fixed vocabulary)
    fn _fit_transform(&mut self, X: &[String], fixed_vocabulary: bool) -> CSRArray {
        let mut tf = ::math::CSRArray {
            indices: Vec::new(),
            indptr: Vec::new(),
            data: Vec::new(),
        };

        tf.indptr.push(0);

        // we use a localy scoped vocabulary
        let mut vocabulary: FnvHashMap<String, i32> =
            FnvHashMap::with_capacity_and_hasher(1000, Default::default());

        let mut counter: FnvHashMap<i32, i32> =
            FnvHashMap::with_capacity_and_hasher(1000, Default::default());

        for (document_id, document) in X.iter().enumerate() {
            let document = document.to_ascii_lowercase();

            let tokens = tokenize(&document);

            let n_grams = analyze(tokens);
            for token in n_grams {
                let vocabulary_size = vocabulary.len() as i32;
                let token_id = vocabulary
                    .entry(token.to_owned())
                    .or_insert(vocabulary_size);
                counter
                    .entry(*token_id)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            // Here we use a counter to sum duplicates tokens, this re-hashes already
            // the hashed values, but it means that we don't need to handle
            // duplicates later on.
            // The alternative is to insert them into indices vector as they are,
            // and let the sparse library matrix to sort indices and sum duplicates
            // as this is done in `scipy.sparse`.
            for (key, value) in counter.drain() {
                tf.indices.push(key as usize);
                tf.data.push(value);
            }
            tf.indptr.push(tf.data.len());
        }

        // Copy to the vocabulary in the struct and make it own data
        for (key, value) in vocabulary.drain() {
            self.vocabulary.insert(key.to_owned(), value);
        }

        _sort_features(&mut tf, &mut self.vocabulary);

        tf.sort_indices();
        tf
    }

    pub fn fit_transform(&mut self, X: &[String]) -> CSRArray {
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

    pub fn fit(self, X: &[String]) -> Self {
        // Fit method
        //
        // The vectorizer is stateless, this has no effect
        self
    }

    pub fn transform(&self, X: &[String]) -> CSRArray {
        // Transform method

        let mut tf = ::math::CSRArray {
            indices: Vec::new(),
            indptr: Vec::new(),
            data: Vec::new(),
        };

        tf.indptr.push(0);

        let mut counter: FnvHashMap<u32, i32> =
            FnvHashMap::with_capacity_and_hasher(1000, Default::default());

        for (document_id, document) in X.iter().enumerate() {
            // String.to_lowercase() is very slow
            // https://www.reddit.com/r/rust/comments/6wbru2/performance_issue_can_i_avoid_of_using_the_slow/
            // https://github.com/rust-lang/rust/issues/26244
            // Possibly use: https://github.com/JuliaStrings/utf8proc
            let document = document.to_ascii_lowercase();

            let tokens = tokenize(&document);
            let n_grams = analyze(tokens);
            for token in n_grams {
                let hash = fasthash::murmur3::hash32(&token);
                let hash = hash % self.n_features;

                counter.entry(hash).and_modify(|e| *e += 1).or_insert(1);
            }

            // Here we use a counter to sum duplicates tokens, this means that we
            // re-hash the hashed values, which is not great performance wise,
            // but it means that we don't need to handle duplicates later on.
            // The alternative is to insert them into indices vector as they are,
            // and let the sparse library matrix to sort indices and sum duplicates
            // as this is done in `scipy.sparse`.
            for (key, value) in counter.drain() {
                tf.indices.push(key as usize);
                tf.data.push(value);
            }
            tf.indptr.push(tf.data.len());
        }
        // this takes ~10% of the compute time
        tf.sort_indices();
        tf
    }

    pub fn fit_transform(&self, X: &[String]) -> CSRArray {
        // Fit and transform
        //
        self.transform(X)
    }
}
