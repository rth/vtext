// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use crate::tokenize::Tokenizer;
use crate::tokenize_sentence::{UnicodeSentenceTokenizerParams, UnicodeSentenceTokenizer};

#[test]
fn test_unicode_sentence_tokenizer() {
    let s = "Here is one. Here is another! This trailing text is one more";

    let tokenizer = UnicodeSentenceTokenizerParams::default()
        .build()
        .unwrap();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &[
        "Here is one. ",
        "Here is another! ",
        "This trailing text is one more"
    ];
    assert_eq!(tokens, b);

    let tokenizer = UnicodeSentenceTokenizer::default();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &[
        "Here is one. ",
        "Here is another! ",
        "This trailing text is one more"
    ];
    assert_eq!(tokens, b);
}

