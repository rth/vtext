#[macro_use]
extern crate lazy_static;
extern crate fasthash;
extern crate fnv;
extern crate regex;
extern crate sprs;

use fnv::FnvHashMap;
use regex::Regex;
use sprs::CsMat;


const TOKEN_PATTERN_DEFAULT: &str = r"(?-u:\b)\w\w+(?-u:\b)";

#[cfg(test)]
mod tests;

pub fn analyze(tokens: Vec<&str>) -> Vec<&str> {
    tokens
}

pub fn tokenize(text: &String) -> (Vec<&str>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(TOKEN_PATTERN_DEFAULT).unwrap();
    }

    RE.find_iter(text).map(|m| m.as_str()).collect::<Vec<_>>()
}

pub fn count(tokens: Vec<&str>) -> (usize) {
    let mut counter: FnvHashMap<&str, i32> =
        FnvHashMap::with_capacity_and_hasher(1000, Default::default());
    for el in tokens {
        counter.entry(el).and_modify(|e| *e += 1).or_insert(1);
    }
    counter.len()
}

#[derive(Debug)]
struct _CSRArray {
    pub indices: Vec<usize>,
    pub indptr: Vec<usize>,
    pub values: Vec<i32>,
}


#[derive(Debug)]
pub struct HashingVectorizer {
    lowercase: bool,
    token_pattern: String,
    n_features: u32,
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

    pub fn fit(mut self, X: &[String]) -> Self {
        // Fit method
        //
        // The vectorizer is stateless, this has no effect
        self
    }

    pub fn transform(&self, X: &[String]) -> CsMat<i32> {
        // Transform method

        let mut tf = _CSRArray {
            indices: Vec::new(),
            indptr: Vec::new(),
            values: Vec::new(),
        };

        tf.indptr.push(0);

        let mut size: usize = 0;

        for (document_id, document) in X.iter().enumerate() {
            let tokens = tokenize(&document);
            let n_grams = analyze(tokens);
            for token in n_grams {
                let hash = fasthash::murmur3::hash32(&token);
                let bucket = hash % self.n_features;

                tf.indices.push(bucket as usize);
                tf.values.push(1);
            }
            tf.indptr.push(tf.values.len());
        }
        CsMat::new(
            (tf.indptr.len() - 1, self.n_features as usize),
            tf.indptr,
            tf.indices,
            tf.values,
        )
    }

    pub fn fit_transform(&self, X: &[String]) -> CsMat<i32> {
        // Fit and transform
        //
        self.transform(X)
    }
}
