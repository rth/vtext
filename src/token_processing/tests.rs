use crate::token_processing::*;
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_bigram() {
    let sent = "Mary had a little lamb".split(" ");

    let gramizer = KSkipNGrams::new_bigram();
    let grams: Vec<Vec<&str>> = gramizer
        .transform(Box::new(sent.clone()), None, None)
        .unwrap()
        .collect();

    let expected = vec![
        vec!["Mary", "had"],
        vec!["had", "a"],
        vec!["a", "little"],
        vec!["little", "lamb"],
    ];

    assert_eq!(grams, expected);
}

#[test]
fn test_trigram() {
    let sent = "Mary had a little lamb".split(" ");

    let gramizer = KSkipNGrams::new_ngrams(3);
    let grams: Vec<Vec<&str>> = gramizer
        .transform(Box::new(sent.clone()), Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();

    let expected = vec![
        vec!["<s>", "<s>", "Mary"],
        vec!["<s>", "Mary", "had"],
        vec!["Mary", "had", "a"],
        vec!["had", "a", "little"],
        vec!["a", "little", "lamb"],
        vec!["little", "lamb", "</s>"],
        vec!["lamb", "</s>", "</s>"],
    ];

    assert_eq!(grams, expected);

    let gramizer = KSkipNGrams::new_ngrams(3);
    let grams: Vec<Vec<&str>> = gramizer
        .transform(Box::new(sent.clone()), None, Some("</s>"))
        .unwrap()
        .collect();

    let expected = vec![
        vec!["Mary", "had", "a"],
        vec!["had", "a", "little"],
        vec!["a", "little", "lamb"],
        vec!["little", "lamb", "</s>"],
        vec!["lamb", "</s>", "</s>"],
    ];

    assert_eq!(grams, expected);
}

#[test]
fn test_ngrams() {
    let sent = "Mary had a little lamb".split(" ");

    let gramizer = KSkipNGrams::new_ngrams(4);
    let grams: Vec<Vec<&str>> = gramizer
        .transform(Box::new(sent.clone()), Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();

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

    assert_eq!(grams, expected);
}

#[test]
fn test_everygram() {
    let sent = "Mary had a little lamb".split(" ");

    let gramizer = KSkipNGrams::new_everygrams(1, 3);
    let grams: Vec<Vec<&str>> = gramizer
        .transform(Box::new(sent.clone()), Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();

    let expected = vec![
        vec!["Mary"],
        vec!["had"],
        vec!["a"],
        vec!["little"],
        vec!["lamb"],
        vec!["<s>", "Mary"],
        vec!["Mary", "had"],
        vec!["had", "a"],
        vec!["a", "little"],
        vec!["little", "lamb"],
        vec!["lamb", "</s>"],
        vec!["<s>", "<s>", "Mary"],
        vec!["<s>", "Mary", "had"],
        vec!["Mary", "had", "a"],
        vec!["had", "a", "little"],
        vec!["a", "little", "lamb"],
        vec!["little", "lamb", "</s>"],
        vec!["lamb", "</s>", "</s>"],
    ];

    assert_eq!(grams, expected);
}

#[test]
fn test_skipgram() {
    let sent = "Mary had a little lamb".split(" ");

    let gramizer = KSkipNGrams::new_skipgrams(2, 1);
    let grams: Vec<Vec<&str>> = gramizer
        .transform(Box::new(sent.clone()), Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();

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
        vec!["little", "</s>"],
        vec!["lamb", "</s>"],
    ];

    assert_eq!(grams, expected);

    let gramizer = KSkipNGrams::new_skipgrams(3, 1);
    let grams: Vec<Vec<&str>> = gramizer
        .transform(Box::new(sent.clone()), Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();

    let expected = vec![
        vec!["<s>", "<s>", "Mary"],
        vec!["<s>", "<s>", "had"],
        vec!["<s>", "Mary", "had"],
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
        vec!["a", "little", "</s>"],
        vec!["a", "lamb", "</s>"],
        vec!["little", "lamb", "</s>"],
        vec!["little", "lamb", "</s>"],
        vec!["little", "</s>", "</s>"],
        vec!["lamb", "</s>", "</s>"],
    ];

    assert_eq!(grams, expected);

    let sent = "Mary had a little lamb, whose fleece ...".split(" ");

    let gramizer = KSkipNGrams::new_skipgrams(3, 2);
    let grams: Vec<Vec<&str>> = gramizer
        .transform(Box::new(sent.clone()), None, None)
        .unwrap()
        .collect();

    let expected = vec![
        vec!["Mary", "had", "a"],
        vec!["Mary", "had", "little"],
        vec!["Mary", "had", "lamb,"],
        vec!["Mary", "a", "little"],
        vec!["Mary", "a", "lamb,"],
        vec!["Mary", "little", "lamb,"],
        vec!["had", "a", "little"],
        vec!["had", "a", "lamb,"],
        vec!["had", "a", "whose"],
        vec!["had", "little", "lamb,"],
        vec!["had", "little", "whose"],
        vec!["had", "lamb,", "whose"],
        vec!["a", "little", "lamb,"],
        vec!["a", "little", "whose"],
        vec!["a", "little", "fleece"],
        vec!["a", "lamb,", "whose"],
        vec!["a", "lamb,", "fleece"],
        vec!["a", "whose", "fleece"],
        vec!["little", "lamb,", "whose"],
        vec!["little", "lamb,", "fleece"],
        vec!["little", "lamb,", "..."],
        vec!["little", "whose", "fleece"],
        vec!["little", "whose", "..."],
        vec!["little", "fleece", "..."],
        vec!["lamb,", "whose", "fleece"],
        vec!["lamb,", "whose", "..."],
        vec!["lamb,", "fleece", "..."],
        vec!["whose", "fleece", "..."],
    ];

    assert_eq!(grams, expected);
}

#[test]
fn test_skipgram_everygram() {
    let sent = "Mary had a little lamb, whose fleece ...".split(" ");

    // min_n=2, max_n=4, max_k=3
    let gramizer = KSkipNGrams::new(2, 4, 3);
    let output: Vec<_> = gramizer
        .transform(Box::new(sent.clone()), Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();
    let output_set: HashSet<Vec<&str>> = HashSet::from_iter(output.iter().cloned());

    // Equivalent to union of three skip-gram outputs n=2,3,4 (k=3) but with different ordering
    let gramizer_sg_2 = KSkipNGrams::new_skipgrams(2, 3);
    let output_sg_2: Vec<_> = gramizer_sg_2
        .transform(Box::new(sent.clone()), Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();
    let output_sg_2_set: HashSet<Vec<&str>> = HashSet::from_iter(output_sg_2.iter().cloned());

    let gramizer_sg_3 = KSkipNGrams::new_skipgrams(3, 3);
    let output_sg_3: Vec<_> = gramizer_sg_3
        .transform(Box::new(sent.clone()), Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();
    let output_sg_3_set: HashSet<Vec<&str>> = HashSet::from_iter(output_sg_3.iter().cloned());

    let gramizer_sg_4 = KSkipNGrams::new_skipgrams(4, 3);
    let output_sg_4: Vec<_> = gramizer_sg_4
        .transform(Box::new(sent.clone()), Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();
    let output_sg_4_set: HashSet<Vec<&str>> = HashSet::from_iter(output_sg_4.iter().cloned());

    let expected_set: HashSet<_> = output_sg_2_set
        .union(&output_sg_3_set)
        .map(move |x| x.clone())
        .collect::<HashSet<_>>()
        .union(&output_sg_4_set)
        .map(move |x| x.clone())
        .collect();

    // Same output - different order
    assert_eq!(output_set, expected_set);

    // No duplicates from either output expected
    assert_eq!(
        output.len(),
        output_sg_2.len() + output_sg_3.len() + output_sg_4.len()
    );
}

#[test]
fn test_ngram_edge_cases() {
    let sent = "Mary had a little lamb".split(" ");

    let gramizer = KSkipNGrams::new(1, 1, 0);
    let grams: Vec<Vec<&str>> = gramizer
        .transform(Box::new(sent.clone()), Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();

    let expected = vec![
        vec!["Mary"],
        vec!["had"],
        vec!["a"],
        vec!["little"],
        vec!["lamb"],
    ];

    assert_eq!(grams, expected);

    let gramizer = KSkipNGrams::new(1, 1, 1);
    let grarms: Vec<Vec<&str>> = gramizer
        .transform(Box::new(sent.clone()), Some("<s>"), Some("</s>"))
        .unwrap()
        .collect();

    assert_eq!(grarms, expected);
}

#[test]
fn test_sample_combinations() {
    let output: Vec<Vec<usize>> = SampleCombinations::new(false, 3, 3).unwrap().collect();

    let expected = vec![vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 3], vec![1, 2, 3]];
    assert_eq!(output, expected);

    let output: Vec<Vec<usize>> = SampleCombinations::new(true, 3, 3).unwrap().collect();
    let expected = vec![vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 3]];
    assert_eq!(output, expected);

    let output: Vec<Vec<usize>> = SampleCombinations::new(true, 4, 3).unwrap().collect();
    let expected = vec![
        vec![0, 1, 2],
        vec![0, 1, 3],
        vec![0, 1, 4],
        vec![0, 2, 3],
        vec![0, 2, 4],
        vec![0, 3, 4],
    ];
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

#[test]
fn test_padding() {
    let iter = "Mary had a little lamb".split(" ");

    let output_iter = pad_items(Box::new(iter), 3, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<&str> = output_iter.collect();

    let expected = vec![
        "<s>", "<s>", "Mary", "had", "a", "little", "lamb", "</s>", "</s>",
    ];

    assert_eq!(output, expected);
}
