// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use crate::tokenize::*;

#[test]
fn test_regexp_tokenizer() {
    let s = "fox can't jump 32.3 feet, right?";

    let tokenizer = RegexpTokenizerParams::default().build().unwrap();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &["fox", "can", "jump", "32", "feet", "right"];
    assert_eq!(tokens, b);
}

#[test]
fn test_unicode_tokenizer() {
    let s = "The quick (\"brown\") fox can't jump 32.3 feet, right?";

    let tokenizer = UnicodeSegmentTokenizerParams::default()
        .word_bounds(false)
        .build()
        .unwrap();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &[
        "The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "right",
    ];
    assert_eq!(tokens, b);

    let tokenizer = UnicodeSegmentTokenizerParams::default().build().unwrap();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &[
        "The", "quick", "(", "\"", "brown", "\"", ")", "fox", "can't", "jump", "32.3", "feet", ",",
        "right", "?",
    ];
    assert_eq!(tokens, b);
}

#[test]
fn test_vtext_tokenizer_all_lang() {
    let tokenizer = VTextTokenizer::new("en");

    for (s, tokens_ref) in [
        // float numbers
        ("23.2 meters", vec!["23.2", "meters"]),
        ("11,2 m", vec!["11,2", "m"]),
        // repeated punctuation
        ("1 ..", vec!["1", ".."]),
        ("I ...", vec!["I", "..."]),
        (", o ! o", vec![",", "o", "!", "o"]),
        ("... ok.", vec!["...", "ok", "."]),
        // dash separated words
        ("porte-manteau", vec!["porte-manteau"]),
        // emails
        ("name@domain.com", vec!["name@domain.com"]),
        // fractions
        ("1/2", vec!["1/2"]),
        ("and/or", vec!["and", "/", "or"]),
        // time
        ("8:30", vec!["8:30"]),
        ("B&B", vec!["B&B"]),
        // TODO ("Hello :)", vec!["Hello", ":)"])
        // TODO ("http://www.youtube.com/watch?v=q2lDF0XU3NI",
        // vec!["http://www.youtube.com/watch?v=q2lDF0XU3NI"])
    ]
    .iter()
    {
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(&tokens, tokens_ref);
    }
}

#[test]
fn test_vtext_tokenizer_en() {
    let tokenizer = VTextTokenizer::new("en");

    for (s, tokens_ref) in [
        ("We can't", vec!["We", "ca", "n't"]),
        ("it's", vec!["it", "'s"]),
        ("it’s", vec!["it", "’s"]),
        // TODO ("N.Y.", vec!["N.Y."])
    ]
    .iter()
    {
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(&tokens, tokens_ref);
    }
}

#[test]
fn test_vtext_tokenizer_fr() {
    let tokenizer = VTextTokenizer::new("fr");

    for (s, tokens_ref) in [("l'image", vec!["l'", "image"])].iter() {
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(&tokens, tokens_ref);
    }
}

#[test]
fn test_vtext_tokenizer_invalid_lang() {
    let tokenizer = VTextTokenizer::new("unknown");
    assert_eq!(tokenizer.lang, "any");
}

#[test]
fn test_character_tokenizer() {
    let s = "fox can't";

    let tokenizer = CharacterTokenizer::new(4);
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &["fox ", "ox c", "x ca", " can", "can'", "an't"];
    assert_eq!(tokens, b);
}

#[test]
fn test_tokenizer_defaults() {
    let tokenizer = UnicodeSegmentTokenizerParams::default().build().unwrap();
    assert_eq!(tokenizer.word_bounds, true);
}
