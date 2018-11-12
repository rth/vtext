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
use regex::Regex;

const TOKEN_PATTERN_DEFAULT: &str = r"(?-u:\b)\w\w+(?-u:\b)";

#[cfg(test)]
mod tests;

mod math;

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
pub struct HashingVectorizer {
    lowercase: bool,
    token_pattern: String,
    n_features: u32,
}

#[derive(Debug)]
pub struct CountVectorizer {
    lowercase: bool,
    token_pattern: String,
    vocabulary: FnvHashMap<String, i32>
}

pub enum Vectorizer {

}

impl CountVectorizer {
    pub fn new() -> Self {
        // Create a new CountVectorizer estimator
        CountVectorizer {
            lowercase: true,
            token_pattern: String::from(TOKEN_PATTERN_DEFAULT),
            vocabulary: FnvHashMap::with_capacity_and_hasher(1000, Default::default())
        }
    }

    pub fn fit(&mut self, X: &[String]) -> () {
        // Fit method
        //
        self._fit_transform(X, false);
    }

    pub fn transform(&mut self, X: &[String]) -> CSRArray {
        // Transform method

        self._fit_transform(X, true)
    }

    pub fn _fit_transform(&mut self, X: &[String], fixed_vocabulary: bool) -> CSRArray {
        // Transform method
        let mut tf = ::math::CSRArray {
            indices: Vec::new(),
            indptr: Vec::new(),
            data: Vec::new(),
        };

        tf.indptr.push(0);

        let mut size: usize = 0;

        // we use a localy scoped vocabulary
        let mut vocabulary: FnvHashMap<&str, i32> = FnvHashMap::with_capacity_and_hasher(1000, Default::default());

        let mut counter: FnvHashMap<i32, i32> = 
            FnvHashMap::with_capacity_and_hasher(1000, Default::default());

        for (document_id, document) in X.iter().enumerate() {
            let tokens = tokenize(&document);
            let n_grams = analyze(tokens);
            for token in n_grams {
                let vocabulary_size = (vocabulary.len() + 1) as i32;
                let token_id = vocabulary.entry(token).or_insert(vocabulary_size);
                counter.entry(*token_id).and_modify(|e| *e += 1).or_insert(1);
            }
            //// Here we use a counter to sum duplicates tokens, this means that we
            //// re-hash the hashed values, but it means that we don't need to handle
            //// duplicates later on.
            //// The alternative is to insert them into indices vector as they are,
            //// and let the sparse library matrix to sort indices and sum duplicates
            //// as this is done in `scipy.sparse`.
            for (key, value) in counter.drain() {
                tf.indices.push(key as usize);
                tf.data.push(value);
            }
            tf.indptr.push(tf.data.len());
        }


        // Copy to the vocabulary in the struct and make it own data
        self.vocabulary.clear();
        for (key, value) in vocabulary.drain() {
            self.vocabulary.insert(key.to_string(), value);
        }

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


    pub fn fit(mut self, X: &[String]) -> Self {
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

        let mut size: usize = 0;

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
