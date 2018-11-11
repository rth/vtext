#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate fnv;


use regex::Regex;
use fnv::FnvHashMap;

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


pub fn analyze(tokens: Vec<String>) -> Vec<String> {

    tokens
}


pub fn tokenize(text: &String) -> (Vec<&str>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u:\b)\w\w+(?-u:\b)").unwrap();
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



// fn hash_table(tokens: Vec<String>, indices: Vec<i32>, indptr: Vec<i64>, values: Vec<i32>,
//               n_features: usize)  {
//     /// Add current tokens to the existing  
//     for el in tokens {
// 
//     }
// 
//     (indices, indptr, values)
//     }
