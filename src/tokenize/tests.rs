// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use crate::tokenize::*;

#[test]
fn test_regexp_tokenizer() {
    let s = "fox can't jump 32.3 feet, right?";

    let tokenizer = RegexpTokenizer::default();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &["fox", "can", "jump", "32", "feet", "right"];
    assert_eq!(tokens, b);
}

#[test]
fn test_regexp_tokenizer_error() {
    let tokenizer = RegexpTokenizerParams::default().pattern("(").build();

    assert!(tokenizer.is_err());
}

#[test]
fn test_unicode_tokenizer() {
    let s = "The quick (\"brown\") fox can't jump 32.3 feet, right?";

    let tokenizer = UnicodeWordTokenizerParams::default()
        .word_bounds(false)
        .build()
        .unwrap();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &[
        "The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "right",
    ];
    assert_eq!(tokens, b);

    let tokenizer = UnicodeWordTokenizer::default();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &[
        "The", "quick", "(", "\"", "brown", "\"", ")", "fox", "can't", "jump", "32.3", "feet", ",",
        "right", "?",
    ];
    assert_eq!(tokens, b);
}

#[test]
fn test_vtext_tokenizer_all_lang() {
    let tokenizer = VTextTokenizer::default();

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
    let tokenizer = VTextTokenizer::default();

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
    let tokenizer = VTextTokenizerParams::default().lang("fr").build().unwrap();

    for (s, tokens_ref) in [("l'image", vec!["l'", "image"])].iter() {
        let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
        assert_eq!(&tokens, tokens_ref);
    }
}

#[test]
fn test_vtext_tokenizer_invalid_lang() {
    let tokenizer = VTextTokenizerParams::default()
        .lang("unknown")
        .build()
        .unwrap();
    assert_eq!(tokenizer.params.lang, "any");
}

#[test]
fn test_character_tokenizer() {
    let s = "fox can't";

    let tokenizer = CharacterTokenizer::default();
    let tokens: Vec<&str> = tokenizer.tokenize(s).collect();
    let b: &[_] = &["fox ", "ox c", "x ca", " can", "can'", "an't"];
    assert_eq!(tokens, b);
}

#[test]
fn test_tokenizer_defaults() {
    let tokenizer = UnicodeWordTokenizer::default();
    assert_eq!(tokenizer.params.word_bounds, true);
}

#[test]
fn test_treebank_word_tokenizer() {
    let tokenizer = NTLKWordTokenizer::default();

    // test cases from NLTK
    // https://github.com/nltk/nltk/blob/develop/nltk/test/tokenize.doctest
    let s = "On a $50,000 mortgage of 30 years at 8 percent, the monthly payment would be $366.88.";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &[
        "On", "a", "$", "50,000", "mortgage", "of", "30", "years", "at", "8", "percent", ",",
        "the", "monthly", "payment", "would", "be", "$", "366.88", ".",
    ];
    assert_eq!(tokens, b);

    let s = "\"We beat some pretty good teams to get here,\" Slocum said.";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &[
        "``", "We", "beat", "some", "pretty", "good", "teams", "to", "get", "here", ",", "''",
        "Slocum", "said", ".",
    ];
    assert_eq!(tokens, b);

    let s = "Well, we couldn't have this predictable, cliche-ridden, \"Touched by an Angel\" (a show creator John Masius worked on) wanna-be if she didn't.";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &[
        "Well",
        ",",
        "we",
        "could",
        "n\"t",
        "have",
        "this",
        "predictable",
        ",",
        "cliche-ridden",
        ",",
        "``",
        "Touched",
        "by",
        "an",
        "Angel",
        "\"\"",
        "(",
        "a",
        "show",
        "creator",
        "John",
        "Masius",
        "worked",
        "on",
        ")",
        "wanna-be",
        "if",
        "she",
        "did",
        "n\"t",
        ".",
    ];
    // TODO
    // assert_eq!(tokens, b);

    let s = "I cannot cannot work under these conditions!";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &[
        "I",
        "can",
        "not",
        "can",
        "not",
        "work",
        "under",
        "these",
        "conditions",
        "!",
    ];
    assert_eq!(tokens, b);

    let s = "The company spent $30,000,000 last year.";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &[
        "The",
        "company",
        "spent",
        "$",
        "30,000,000",
        "last",
        "year",
        ".",
    ];
    assert_eq!(tokens, b);

    let s = "The company spent 40.75% of its income last year.";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &[
        "The", "company", "spent", "40.75", "%", "of", "its", "income", "last", "year", ".",
    ];
    assert_eq!(tokens, b);

    let s = "He arrived at 3:00 pm.";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &["He", "arrived", "at", "3:00", "pm", "."];
    assert_eq!(tokens, b);

    let s = "I bought these items: books, pencils, and pens.";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &[
        "I", "bought", "these", "items", ":", "books", ",", "pencils", ",", "and", "pens", ".",
    ];
    assert_eq!(tokens, b);

    let s = "Though there were 150, 100 of them were old.";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &[
        "Though", "there", "were", "150", ",", "100", "of", "them", "were", "old", ".",
    ];
    assert_eq!(tokens, b);

    let s = "There were 300,000, but that wasn't enough.";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &[
        "There", "were", "300,000", ",", "but", "that", "was", "n't", "enough", ".",
    ];
    assert_eq!(tokens, b);

    // Handling of unicode
    let s = "«Now that I can do.»";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &["«", "Now", "that", "I", "can", "do", ".", "»"];
    assert_eq!(tokens, b);

    let s = "The unicode 201C and 201D \u{201c}LEFT(RIGHT) DOUBLE QUOTATION MARK\u{201d} is also OPEN_PUNCT and CLOSE_PUNCT.";
    let tokens = tokenizer.tokenize(s);
    let b: &[_] = &[
        "The",
        "unicode",
        "201C",
        "and",
        "201D",
        "\u{201c}",
        "LEFT",
        "(",
        "RIGHT",
        ")",
        "DOUBLE",
        "QUOTATION",
        "MARK",
        "\u{201d}",
        "is",
        "also",
        "OPEN_PUNCT",
        "and",
        "CLOSE_PUNCT",
        ".",
    ];
    assert_eq!(tokens, b);
}
