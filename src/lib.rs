#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate fnv;


use regex::Regex;
use fnv::FnvHashMap;


const TOKEN_PATTERN_DEFAULT: &str = r"(?-u:\b)\w\w+(?-u:\b)";


#[cfg(test)]
mod tests;

#[derive(Debug)]
struct CSRArray {
    indices: Vec<i32>,
    indptr: Vec<i64>,
    values: Vec<i32>
}

#[derive(Debug)]
struct Vectorizer {
    count: usize
}


#[derive(Debug)]
struct HashingVectorizer {
    lowercase: bool,
    token_pattern: String
}

pub fn analyze(tokens: Vec<String>) -> Vec<String> {

    tokens
}


pub fn tokenize(text: &String) -> (Vec<&str>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(TOKEN_PATTERN_DEFAULT).unwrap();
    }

    RE.find_iter(text)
        .map(|m| m.as_str())
        .collect::<Vec<_>>()
}

pub fn count(tokens: Vec<&str>) -> (usize) {

    let mut counter: FnvHashMap<&str, i32> = FnvHashMap::with_capacity_and_hasher(1000, Default::default());
    for el in tokens {
        counter
            .entry(el)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    counter.len()
}


impl Vectorizer{
    fn vectorize(&mut self, document: &String) -> usize {
        let tokens = tokenize(&document);
        self.count += count(tokens);
        1
    }
}


impl HashingVectorizer {
    fn new() -> Self {
        // Create a new HashingVectorizer class
        HashingVectorizer {
            lowercase: true,
            token_pattern: String::from(TOKEN_PATTERN_DEFAULT)
        }
    }

    fn fit(mut self, X: &[String]) -> Self {
        // Fit method
        //
        // The vectorizer is stateless, this has no effect
        self
    }

    fn transform(&self, X: &[String]) -> usize {
        // Transform method
        //
        let mut s: usize = 0;
        for document in X.iter() {
            let tokens = tokenize(&document);
            s += count(tokens);
        }
        s
    }

    fn fit_transform(&self, X: &[String]) -> usize {
        // Fit method
        //
        // The vectorizer is stateless, this has no effect
        self.transform(X)
    }

}



// fn hash_table(tokens: Vec<String>, indices: Vec<i32>, indptr: Vec<i64>, values: Vec<i32>,
//               n_features: usize)  {
//     /// Add current tokens to the existing  
//     for el in tokens {
// 
//     }
// 
//     (indices, indptr, values)
//     }
