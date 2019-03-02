#![allow(non_snake_case)]

extern crate text_vectorize;

use std::fs;
use std::io::prelude::*;
use std::time::SystemTime;
use text_vectorize::{CountVectorizer, HashingVectorizer};

fn main() {
    let _dirs_list = fs::read_dir("./data/").unwrap();

    // let mut indices: Vec<i32> = Vec::new()
    // let mut idptr: Vec<i64> = Vec::new()
    // let mut values: Vec<i32> = Vec::new()

    let mut documents: Vec<String> = Vec::new();

    for dir_path in _dirs_list {
        let dir_path = dir_path.unwrap();
        if dir_path.path().is_dir() {
            let _file_list = fs::read_dir(dir_path.path()).unwrap();
            for path in _file_list {
                let mut fh = fs::File::open(path.unwrap().path()).expect("file not found");
                let mut contents = String::new();
                fh.read_to_string(&mut contents)
                    .expect("something went wrong");
                documents.push(contents)
            }
        }
    }

    let t0 = SystemTime::now();

    let mut vect = CountVectorizer::new();
    let _X = vect.fit_transform(&documents);

    let n_documents = documents.len();

    let t_end = SystemTime::now();
    let dt = t_end.duration_since(t0).unwrap();
    println!(
        "CountVectorizer: vectorized {} documents in {:?}",
        n_documents, dt
    );

    let t0 = SystemTime::now();

    let vect = HashingVectorizer::new();
    let _X = vect.fit_transform(&documents);

    let t_end = SystemTime::now();
    let dt = t_end.duration_since(t0).unwrap();
    println!(
        "HashingVectorizer: vectorized {} documents in {:?}",
        n_documents, dt
    );
}
