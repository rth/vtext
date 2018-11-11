extern crate text_vectorize;

use std::fs;
use std::io::prelude::*;
use text_vectorize::{analyze, tokenize, count};



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
