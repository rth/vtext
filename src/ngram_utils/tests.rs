use crate::ngram_utils::*;
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_padding() {
    let sent = "Mary had a little lamb".split(" ");

    let output: Vec<&str> =
        pad_items(Box::new(sent.clone()), 3, Some("<s>"), Some("</s>")).collect();
    let expected = vec![
        "<s>", "<s>", "Mary", "had", "a", "little", "lamb", "</s>", "</s>",
    ];
    assert_eq!(output, expected);

    let output: Vec<&str> = pad_items(Box::new(sent.clone()), 2, Some("<s>"), None).collect();
    let expected = vec!["<s>", "Mary", "had", "a", "little", "lamb"];
    assert_eq!(output, expected);

    let output: Vec<&str> = pad_items(Box::new(sent.clone()), 2, None, Some("</s>")).collect();
    let expected = vec!["Mary", "had", "a", "little", "lamb", "</s>"];
    assert_eq!(output, expected);
}

#[test]
fn test_bigram() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter = bigram(Box::new(sent), None, None).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["Mary", "had"],
        vec!["had", "a"],
        vec!["a", "little"],
        vec!["little", "lamb"],
    ];

    assert_eq!(output, expected);
}

#[test]
fn test_trigram() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter = ngrams(Box::new(sent.clone()), 3, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["<s>", "<s>", "Mary"],
        vec!["<s>", "Mary", "had"],
        vec!["Mary", "had", "a"],
        vec!["had", "a", "little"],
        vec!["a", "little", "lamb"],
        vec!["little", "lamb", "</s>"],
        vec!["lamb", "</s>", "</s>"],
    ];

    assert_eq!(output, expected);

    let output_iter = ngrams(Box::new(sent.clone()), 3, None, Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["Mary", "had", "a"],
        vec!["had", "a", "little"],
        vec!["a", "little", "lamb"],
        vec!["little", "lamb", "</s>"],
        vec!["lamb", "</s>", "</s>"],
    ];

    assert_eq!(output, expected);
}

#[test]
fn test_ngrams() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter = ngrams(Box::new(sent), 4, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["<s>", "<s>", "<s>", "Mary"],
        vec!["<s>", "<s>", "Mary", "had"],
        vec!["<s>", "Mary", "had", "a"],
        vec!["Mary", "had", "a", "little"],
        vec!["had", "a", "little", "lamb"],
        vec!["a", "little", "lamb", "</s>"],
        vec!["little", "lamb", "</s>", "</s>"],
        vec!["lamb", "</s>", "</s>", "</s>"],
    ];

    assert_eq!(output, expected);
}

#[test]
fn test_everygram() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter = everygrams(Box::new(sent), 1, 3, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["<s>", "Mary"],
        vec!["<s>", "<s>", "Mary"],
        vec!["<s>", "Mary", "had"],
        vec!["Mary"],
        vec!["Mary", "had"],
        vec!["Mary", "had", "a"],
        vec!["had"],
        vec!["had", "a"],
        vec!["had", "a", "little"],
        vec!["a"],
        vec!["a", "little"],
        vec!["a", "little", "lamb"],
        vec!["little"],
        vec!["little", "lamb"],
        vec!["lamb"],
        vec!["lamb", "</s>"],
        vec!["little", "lamb", "</s>"],
        vec!["lamb", "</s>", "</s>"],
    ];

    assert_eq!(output, expected);
}

#[test]
fn test_skipgram() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter = skipgrams(Box::new(sent.clone()), 2, 1, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["<s>", "Mary"],
        vec!["<s>", "had"],
        vec!["Mary", "had"],
        vec!["Mary", "a"],
        vec!["had", "a"],
        vec!["had", "little"],
        vec!["a", "little"],
        vec!["a", "lamb"],
        vec!["little", "lamb"],
        vec!["lamb", "</s>"],
        vec!["little", "</s>"],
    ];

    assert_eq!(output, expected);

    let output_iter = skipgrams(Box::new(sent.clone()), 3, 1, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["<s>", "<s>", "Mary"],
        vec!["<s>", "<s>", "had"],
        vec!["<s>", "Mary", "had"],
        vec!["<s>", "Mary", "a"],
        vec!["<s>", "had", "a"],
        vec!["Mary", "had", "a"],
        vec!["Mary", "had", "little"],
        vec!["Mary", "a", "little"],
        vec!["had", "a", "little"],
        vec!["had", "a", "lamb"],
        vec!["had", "little", "lamb"],
        vec!["a", "little", "lamb"],
        vec!["little", "lamb", "</s>"],
        vec!["a", "lamb", "</s>"],
        vec!["a", "little", "</s>"],
        vec!["lamb", "</s>", "</s>"],
        vec!["little", "</s>", "</s>"],
    ];

    assert_eq!(output, expected);
}

#[test]
fn test_skipgram_everygram() {
    let sent = "Mary had a little lamb".split(" ");

    // min_n=2, max_n=3, max_k=1
    let output_iter =
        build_k_skip_n_grams(Box::new(sent.clone()), 2, 3, 1, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<_> = output_iter.collect();
    let output_set: HashSet<Vec<&str>> = HashSet::from_iter(output.iter().cloned());

    // should be equivalent to union of two skipgram outputs n=2,3 (k=1) but expect different ordering
    let output_sg_2: Vec<_> = skipgrams(Box::new(sent.clone()), 2, 1, Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();
    let output_sg_2_set: HashSet<Vec<&str>> = HashSet::from_iter(output_sg_2.iter().cloned());

    let output_sg_3: Vec<_> = skipgrams(Box::new(sent.clone()), 3, 1, Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();
    let output_sg_3_set: HashSet<Vec<&str>> = HashSet::from_iter(output_sg_3.iter().cloned());
    let expected_set: HashSet<_> = output_sg_2_set
        .union(&output_sg_3_set)
        .map(move |x| x.clone())
        .collect();

    // Same output - different order
    assert_eq!(output_set, expected_set);

    // No duplicates from either output expected
    assert_eq!(output.len(), output_sg_2.len() + output_sg_3.len());
}

#[test]
fn test_ngram_edge_cases() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter =
        build_k_skip_n_grams(Box::new(sent.clone()), 1, 1, 0, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["Mary"],
        vec!["had"],
        vec!["a"],
        vec!["little"],
        vec!["lamb"],
    ];

    assert_eq!(output, expected);

    let output_iter =
        build_k_skip_n_grams(Box::new(sent.clone()), 1, 1, 1, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    assert_eq!(output, expected);
}

#[test]
fn test_sample_combinations() {
    let output: Vec<Vec<usize>> = SampleCombinations::new(false, 3, 3).unwrap().collect();

    let expected = vec![vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 3], vec![1, 2, 3]];
    assert_eq!(output, expected);

    let output: Vec<Vec<usize>> = SampleCombinations::new(true, 3, 3).unwrap().collect();
    let expected = vec![vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 3]];
    assert_eq!(output, expected);

    // Single output
    let output: Vec<Vec<usize>> = SampleCombinations::new(false, 1, 2).unwrap().collect();
    let expected = vec![vec![0, 1]];
    assert_eq!(output, expected);

    let output: Vec<Vec<usize>> = SampleCombinations::new(true, 1, 2).unwrap().collect();
    let expected = vec![vec![0, 1]];
    assert_eq!(output, expected);

    let output: Vec<Vec<usize>> = SampleCombinations::new(true, 2, 3).unwrap().collect();
    let expected = vec![vec![0, 1, 2]];
    assert_eq!(output, expected);

    let output: Vec<Vec<usize>> = SampleCombinations::new(false, 0, 1).unwrap().collect();
    let expected = vec![vec![0]];
    assert_eq!(output, expected);

    let output: Vec<Vec<usize>> = SampleCombinations::new(true, 0, 1).unwrap().collect();
    let expected = vec![vec![0]];
    assert_eq!(output, expected);
}
