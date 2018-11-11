#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

fn tokenize(text: &String) -> (Vec<&str>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u:\b)\w\w+(?-u:\b)").unwrap();
    }

    RE.find_iter(text)
        .map(|m| m.as_str())
        .collect::<Vec<_>>()
}

fn analyze(tokens: Vec<String>) -> Vec<String> {

    tokens
    }

fn count(tokens: Vec<&str>) -> (usize) {

    let mut counter: HashMap<&str, i32> = HashMap::with_capacity(1000);
    for el in tokens {
        counter
            .entry(el)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    counter.len()
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

fn main() {
    let _dirs_list = fs::read_dir("./data/").unwrap();

    let mut s = 0;

    // let mut indices: Vec<i32> = Vec::new()
    // let mut idptr: Vec<i64> = Vec::new()
    // let mut values: Vec<i32> = Vec::new()

    for dir_path in _dirs_list {
        let dir_path = dir_path.unwrap();
        if dir_path.path().is_dir() {
            let _file_list = fs::read_dir(dir_path.path()).unwrap();
            for path in _file_list {
                let mut fh = fs::File::open(path.unwrap().path()).expect("file not found");
                let mut contents = String::new();
                fh.read_to_string(&mut contents)
                    .expect("something went wrong");
                let tokens = tokenize(&contents);
                // let word_n_grams  = analyze(tokens);
                s += count(tokens);

            }
        }
    }
    println!("{}", s);
}
