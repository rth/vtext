extern crate text_vectorize;

use std::fs;
use std::io::prelude::*;
use text_vectorize::CountVectorizer;

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

    let mut vect = CountVectorizer::new();

    let X = vect.fit_transform(&documents);
}
