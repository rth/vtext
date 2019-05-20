// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

#[derive(Debug)]
pub struct CSRArray {
    pub indices: Vec<usize>,
    pub indptr: Vec<usize>,
    pub data: Vec<i32>,
}
