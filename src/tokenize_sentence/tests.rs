// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use crate::tokenize::Tokenizer;
use crate::tokenize_sentence::*;

#[test]
fn test_unicode_sentence_tokenizer() {
    let s = "Here is one. Here is another? Bang!! This trailing text is one more";

    let tokenizer = UnicodeSentenceTokenizerParams::default().build().unwrap();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &[
        "Here is one. ",
        "Here is another? ",
        "Bang!! ",
        "This trailing text is one more",
    ];
    assert_eq!(tokens, b);

    let tokenizer = UnicodeSentenceTokenizer::default();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &[
        "Here is one. ",
        "Here is another? ",
        "Bang!! ",
        "This trailing text is one more",
    ];
    assert_eq!(tokens, b);
}

#[test]
fn test_punctuation_sentence_tokenizer() {
    let s = "Here is one. Here is another? Bang!! This trailing text is one more";

    let tokenizer = PunctuationTokenizerParams::default().build().unwrap();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &[
        "Here is one. ",
        "Here is another? ",
        "Bang!",
        "! ",
        "This trailing text is one more",
    ];
    assert_eq!(tokens, b);

    let tokenizer = PunctuationTokenizer::default();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &[
        "Here is one. ",
        "Here is another? ",
        "Bang!",
        "! ",
        "This trailing text is one more",
    ];
    assert_eq!(tokens, b);

    // String with characters longer than one byte and multi-code points
    let s2 = "y̆es? 这是另一个! 후행 텍스트";

    let tokenizer = PunctuationTokenizer::default();
    let tokens: Vec<&str> = tokenizer.tokenize(s2).collect();
    let b: &[_] = &["y̆es? ", "这是另一个! ", "후행 텍스트"];
    assert_eq!(tokens, b);
}