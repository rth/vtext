// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.
use crate::token_processor::*;

#[test]
fn test_regexp_tokenizer() {
    let stop_words = vec!["and", "or"];
    let tokens = vec!["Today", "and", "tomorrow"];

    let filter = StopWordFilterParams::default()
        .stop_words(stop_words)
        .build()
        .unwrap();

    let tokens_out: Vec<&str> = filter.transform(tokens.iter().cloned()).collect();
    assert_eq!(tokens_out, vec!["Today", "tomorrow"]);
}
