#[cfg(test)]
mod tests;

use std::cmp::min;
use std::collections::VecDeque;
use std::iter;

use crate::errors::EstimatorErr;
#[cfg(feature = "python")]
use dict_derive::{FromPyObject, IntoPyObject};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
pub struct KSkipNGramsParams {
    pub min_n: usize,
    pub max_n: usize,
    pub max_k: usize,
}

impl KSkipNGramsParams {
    pub fn new(min_n: usize, max_n: usize, max_k: usize) -> KSkipNGramsParams {
        KSkipNGramsParams {
            min_n,
            max_n,
            max_k,
        }
    }

    pub fn build(&mut self) -> KSkipNGrams {
        KSkipNGrams {
            params: self.clone(),
        }
    }
}

/// Transforms a given sequence of `items` into k-skip-n-grams iterator.
///
/// Use convenience methods for common use cases:  `new_bigram`, `new_trigram`, `new_ngrams`,
/// `new_everygrams`, `new_skipgrams`. Otherwise build new using `new`.
pub struct KSkipNGrams {
    pub params: KSkipNGramsParams,
}

/// Core methods to build `KSkipNGrams`
impl KSkipNGrams {
    /// Generate all bigrams from a sequence of `items`, an iterator.
    ///
    /// Example:
    ///
    /// ```
    /// use vtext::token_processing::*;
    /// let sent = "One Two Three Four".split(" ");
    /// let gramizer = KSkipNGrams::new_bigram();
    /// let grams: Vec<_> = gramizer.transform(Box::new(sent), None, None).unwrap().collect();
    /// assert_eq!(grams, vec![vec!["One", "Two"], vec!["Two", "Three"], vec!["Three", "Four"]]);
    /// ```
    pub fn new_bigram() -> KSkipNGrams {
        KSkipNGramsParams::new(2, 2, 0).build()
    }

    /// Generate n-grams from a sequence of `items`, an iterator.
    ///
    /// Example:
    /// ```
    /// use vtext::token_processing::*;
    /// let sent = "One Two Three Four".split(" ");
    /// let gramizer = KSkipNGrams::new_ngrams(3);
    /// let grams: Vec<_> = gramizer.transform(Box::new(sent), None, None).unwrap().collect();
    /// assert_eq!(grams, vec![vec!["One", "Two", "Three"], vec!["Two", "Three", "Four"]]);
    /// ```
    ///
    /// Paramaters:
    ///  * `n` - The degree of the ngrams
    pub fn new_ngrams(n: usize) -> KSkipNGrams {
        KSkipNGramsParams::new(n, n, 0).build()
    }

    /// Generate all n-grams between `min_n` and `max_n` from a sequence of `items`, an iterator.
    ///
    /// Example:
    /// ```
    /// use vtext::token_processing::*;
    /// let sent = "One Two Three".split(" ");
    /// let gramizer = KSkipNGrams::new_everygrams(1, 3);
    /// let grams: Vec<_> = gramizer.transform(Box::new(sent), None, None).unwrap().collect();
    /// assert_eq!(grams, vec![
    /// vec!["One"], vec!["Two"], vec!["Three"], vec!["One", "Two"], vec!["Two", "Three"],
    /// vec!["One", "Two", "Three"]]);
    /// ```
    ///
    /// Paramaters:
    ///  * `min_n` - The minimum degree of the ngram
    ///  * `max_n` - The maximum degree of the ngram
    pub fn new_everygrams(min_n: usize, max_n: usize) -> KSkipNGrams {
        KSkipNGramsParams::new(min_n, max_n, 0).build()
    }

    /// Generate all skip-grams with a max total skip of `k` from a sequence of `items`,
    /// an iterator.
    ///
    /// Example:
    /// ```
    /// use vtext::token_processing::*;
    /// let sent = "One Two Three Four Five".split(" ");
    /// let gramizer = KSkipNGrams::new_skipgrams(3, 2);
    /// let grams: Vec<_> = gramizer.transform(Box::new(sent), None, None).unwrap().collect();
    /// assert_eq!(grams, vec![vec!["One", "Two", "Three"], vec!["One", "Two", "Four"],
    /// vec!["One", "Two", "Five"], vec!["One", "Three", "Four"], vec!["One", "Three", "Five"],
    /// vec!["One", "Four", "Five"], vec!["Two", "Three", "Four"], vec!["Two", "Three", "Five"],
    /// vec!["Two", "Four", "Five"], vec!["Three", "Four", "Five"]]);
    /// ```
    ///
    /// Paramaters:
    /// * `n` - The degree of the ngram
    /// * `k` - The degree of the skipgram: the total max skip between items
    pub fn new_skipgrams(n: usize, k: usize) -> KSkipNGrams {
        KSkipNGramsParams::new(n, n, k).build()
    }

    /// Generate all k-skip-n-grams from a sequence of `items`, an iterator.
    ///
    /// Example:
    /// ```
    /// use vtext::token_processing::*;
    /// let sent = "One Two Three Four".split(" ");
    /// let gramizer = KSkipNGrams::new(2, 3, 1);
    /// let grams: Vec<_> = gramizer.transform(Box::new(sent), None, None).unwrap().collect();
    /// assert_eq!(grams, vec![vec!["One", "Two"], vec!["One", "Three"], vec!["Two", "Three"],
    /// vec!["Two", "Four"], vec!["Three", "Four"], vec!["One", "Two", "Three"],
    /// vec!["One", "Two", "Four"], vec!["One", "Three", "Four"], vec!["Two", "Three", "Four"]]);
    /// ```
    ///
    /// Paramaters:
    /// * `min_n` - The minimum degree of the ngram
    /// * `max_n` - The maximum degree of the ngram
    /// * `k` - The degree of the skipgram: the total max skip between items
    pub fn new(min_n: usize, max_n: usize, max_k: usize) -> KSkipNGrams {
        KSkipNGramsParams::new(min_n, max_n, max_k).build()
    }

    /// Transform a sequence of `items`, an iterator to a `KSkipNGramsIter` iterator.
    ///
    /// Parameters:
    /// * `items` - Input iterator
    /// * `pad_left` - Optional string to use as left padding
    /// * `pad_right` - Optional string to use as right padding
    pub fn transform<'a>(
        &'a self,
        items: Box<dyn Iterator<Item = &'a str> + 'a>,
        pad_left: Option<&'a str>,
        pad_right: Option<&'a str>,
    ) -> Result<Box<dyn Iterator<Item = Vec<&'a str>> + 'a>, EstimatorErr> {
        let k_skip_n_grams_iter = KSkipNGramsIter::new(
            items,
            self.params.min_n,
            self.params.max_n,
            self.params.max_k,
            pad_left,
            pad_right,
        )?;
        Ok(Box::new(k_skip_n_grams_iter))
    }
}

/// An iterator which provided with a sequence of `items` transforms into k-skip-n-grams.
///
/// It also correctly generates left or right padding if specified.
pub struct KSkipNGramsIter<'a> {
    iter: Box<dyn Iterator<Item = Vec<&'a str>> + 'a>,
}

/// Core methods to build `KSkipNGramsIter`
impl<'a> KSkipNGramsIter<'a> {
    /// Build a new `KSkipNGramsIter`.
    ///
    /// Example:
    /// ```
    /// use vtext::token_processing::*;
    /// let sent = "One Two Three".split(" ");
    /// let grams_iter = KSkipNGramsIter::new(Box::new(sent), 1, 2, 1, Some("<s>"), Some("</s>"));
    /// let grams: Vec<Vec<&str>> = grams_iter.unwrap().collect();
    /// ```
    ///
    /// Parameters:
    /// * `items` - Input iterator
    /// * `min_n` - The minimum degree of the ngram
    /// * `max_n` - The maximum degree of the ngram
    /// * `max_k` - The maximum-degree of the skipgram: the total max skip between items
    /// * `pad_left` - Optional string to use as left padding
    /// * `pad_right` - Optional string to use as right padding
    pub fn new(
        mut items: Box<dyn Iterator<Item = &'a str> + 'a>,
        min_n: usize,
        max_n: usize,
        max_k: usize,
        pad_left: Option<&'a str>,
        pad_right: Option<&'a str>,
    ) -> Result<KSkipNGramsIter<'a>, EstimatorErr> {
        if min_n > max_n {
            return Err(EstimatorErr::InvalidParams(
                "`min_n` must be equal to or less than `max_n`".to_string(),
            ));
        }

        let mut iter: Box<dyn Iterator<Item = Vec<&'a str>> + 'a> = Box::new(iter::empty());

        for n in min_n..max_n + 1 {
            let (iter_split_1, iter_split_0) = items.tee();
            items = Box::new(iter_split_0);

            if max_k == 0 {
                let sub_iter = NGramIter::new(Box::new(iter_split_1), n, pad_left, pad_right)?;
                iter = Box::new(iter.chain(sub_iter));
            } else {
                let sub_iter =
                    SkipGramIter::new(Box::new(iter_split_1), n, max_k, pad_left, pad_right)?;
                iter = Box::new(iter.chain(sub_iter));
            }
        }

        Ok(KSkipNGramsIter { iter })
    }
}

/// Iterator functions
impl<'a> Iterator for KSkipNGramsIter<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// An iterator which provided with a sequence of `items` transforms into n-grams.
///
/// The iterator consumes the input iterator only once and holds a window of items to generate the
/// n-grams. The window is stepped forward as it consumes the input. It also correctly generates
/// left or right padding if specified.
pub struct NGramIter<'a> {
    // Params
    items: Box<dyn Iterator<Item = &'a str> + 'a>,

    // Iterator state
    /// Window which holds items that have been consumed
    window: VecDeque<&'a str>,
    first: bool,
}

/// Core method to build `NGramIter`
impl<'a> NGramIter<'a> {
    /// Build a new `NGramIter`.
    ///
    /// Example:
    /// ```
    /// use vtext::token_processing::*;
    /// let sent = "One Two Three".split(" ");
    /// let grams_iter = NGramIter::new(Box::new(sent), 1, Some("<s>"), Some("</s>"));
    /// let grams: Vec<Vec<&str>> = grams_iter.unwrap().collect();
    /// ```
    ///
    /// Parameters:
    /// * `items` - Input iterator
    /// * `min_n` - The degree of the ngrams
    /// * `pad_left` - Optional string to use as left padding
    /// * `pad_right` - Optional string to use as right padding
    pub fn new(
        mut items: Box<dyn Iterator<Item = &'a str> + 'a>,
        n: usize,
        pad_left: Option<&'a str>,
        pad_right: Option<&'a str>,
    ) -> Result<NGramIter<'a>, EstimatorErr> {
        if n < 1 {
            return Err(EstimatorErr::InvalidParams(
                "`min_n` must be greater than or equal to 1".to_string(),
            ));
        }

        if pad_left.is_some() || pad_right.is_some() {
            items = pad_items(items, n, pad_left, pad_right)?;
        }

        let window = Self::build_window(&mut items, n)?;

        Ok(NGramIter {
            // Params
            items,

            // Iterator state
            window,
            first: true,
        })
    }

    /// Prepare and populate start window
    fn build_window(
        items: &mut Box<dyn Iterator<Item = &'a str> + 'a>,
        n: usize,
    ) -> Result<VecDeque<&'a str>, EstimatorErr> {
        let window_size = n;
        let mut window: VecDeque<&'a str> = VecDeque::with_capacity(window_size);

        // Populate window
        let mut i = window_size;
        while i > 0 {
            let next_item = items.next();
            match next_item {
                None => {
                    return Err(EstimatorErr::InvalidInput(
                        "Items length is smaller than `n`".to_string(),
                    ))
                }
                Some(s) => {
                    window.push_back(s);
                }
            }
            i -= 1;
        }
        Ok(window)
    }
}

/// Iterator functions
impl<'a> Iterator for NGramIter<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(Vec::from(self.window.clone()));
        }

        // Forward window or when self.items return None
        let next_item = self.items.next()?;
        self.window.pop_front();
        self.window.push_back(next_item);

        Some(Vec::from(self.window.clone()))
    }
}

/// An iterator which provided with a sequence of `items` transforms into k-skip-grams.
///
/// The iterator consumes the input iterator only once and holds a window of items to generate the
/// k-skip-grams. The window is stepped forward as it consumes the input. It also correctly
/// generates left or right padding if specified.
pub struct SkipGramIter<'a> {
    // Params
    items: Box<dyn Iterator<Item = &'a str> + 'a>,
    n: usize,
    max_k: usize,

    // Iterator state
    /// Window which holds items that have been consumed
    window: VecDeque<&'a str>,
    sample_iter: SampleCombinations,
}

/// Core methods to build `SkipGramIter`
impl<'a> SkipGramIter<'a> {
    /// Build a new `SkipGramIter`.
    ///
    /// Example:
    /// ```
    /// use vtext::token_processing::*;
    /// let sent = "One Two Three".split(" ");
    /// let grams_iter = SkipGramIter::new(Box::new(sent), 1, 2, Some("<s>"), Some("</s>"));
    /// let grams: Vec<Vec<&str>> = grams_iter.unwrap().collect();
    /// ```
    ///
    /// Parameters:
    /// * `items` - Input iterator
    /// * `n` - The degree of the ngram
    /// * `max_k` - The maximum-degree of the skipgram: the total max skip between items
    /// * `pad_left` - Optional string to use as left padding
    /// * `pad_right` - Optional string to use as right padding
    pub fn new(
        mut items: Box<dyn Iterator<Item = &'a str> + 'a>,
        n: usize,
        max_k: usize,
        pad_left: Option<&'a str>,
        pad_right: Option<&'a str>,
    ) -> Result<SkipGramIter<'a>, EstimatorErr> {
        if n < 1 {
            return Err(EstimatorErr::InvalidParams(
                "`min_n` must be greater than or equal to 1".to_string(),
            ));
        }

        if pad_left.is_some() || pad_right.is_some() {
            items = pad_items(items, n, pad_left, pad_right)?;
        }

        let window = Self::build_window(&mut items, n, max_k)?;
        let sample_iter = SampleCombinations::new(true, n + max_k - 1, n)?;

        Ok(SkipGramIter {
            // Params
            items,
            n,
            max_k,

            // Iterator state
            window,
            sample_iter,
        })
    }

    // Prepare and populate start window
    fn build_window(
        items: &mut Box<dyn Iterator<Item = &'a str> + 'a>,
        n: usize,
        max_k: usize,
    ) -> Result<VecDeque<&'a str>, EstimatorErr> {
        let window_size = n + max_k;
        let mut window: VecDeque<&'a str> = VecDeque::with_capacity(window_size);

        // Populate window
        let mut i = window_size;
        while i > 0 {
            let next_item = items.next();
            match next_item {
                None => {
                    return Err(EstimatorErr::InvalidInput(
                        "Items length is smaller than `n`+`max_k`".to_string(),
                    ))
                }
                Some(s) => {
                    window.push_back(s);
                }
            }
            i -= 1;
        }
        Ok(window)
    }
}

/// Iterator functions
impl<'a> Iterator for SkipGramIter<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_sample = self.sample_iter.next();

        match next_sample {
            // Generate and return samples using self.sample_iter
            Some(sample_idx) => {
                let mut sample = Vec::with_capacity(sample_idx.len());
                for idx in sample_idx.into_iter() {
                    sample.push(self.window[idx]);
                }
                Some(sample)
            }

            // Sample_iter finished so attempt to forward window
            None => {
                // Try to forward window
                let next_item = self.items.next();
                match next_item {
                    // Forward window
                    Some(item) => {
                        self.window.pop_front();
                        self.window.push_back(item);

                        self.sample_iter =
                            SampleCombinations::new(true, self.n + self.max_k - 1, self.n).unwrap();

                        self.next()
                    }

                    // self.items finished. So reduce window size iteratively to n and then finish
                    None => {
                        if self.window.len() > self.n {
                            // reduce window
                            self.window.pop_front();
                        } else {
                            // finished
                            return None;
                        }

                        // Generate samples from smaller window
                        let k = min(self.max_k, self.window.len() - self.n);
                        self.sample_iter =
                            SampleCombinations::new(true, self.n + k - 1, self.n).unwrap();
                        self.next()
                    }
                }
            }
        }
    }
}

/// An iterator which generates the list of combinations of `n` items in a range upto `max_i`.
/// It is possible to fix the first item at index 0 (i.e. `fix_0` == true)
///
/// Examples:
/// ```text
/// use vtext::token_processing::*;
/// let output: Vec<_> = SampleCombinations::new(false, 3, 3).unwrap().collect();
/// let expected = vec![
///     vec![0, 1, 2],
///     vec![0, 1, 3],
///     vec![0, 2, 3],
///     vec![1, 2, 3]
/// ];
/// assert_eq!(output, expected);
///
/// let output: Vec<_> = SampleCombinations::new(true, 3, 3).unwrap().collect();
/// let expected = vec![
///     vec![0, 1, 2],
///     vec![0, 1, 3],
///     vec![0, 2, 3]
/// ];
/// assert_eq!(output, expected);
/// ```
struct SampleCombinations {
    // Params
    min_i: usize,
    max_i: usize,
    n: usize,

    // State
    position: Vec<usize>,
    first: bool,
    last: bool,
}

impl SampleCombinations {
    /// New `SampleCombinations`
    ///
    /// Parameters:
    /// * `fix_0` - fix the first element at 0?
    /// * `max_i` - the maximum index for the output elements
    /// * `n` - number of items per combination
    fn new(fix_0: bool, max_i: usize, n: usize) -> Result<SampleCombinations, EstimatorErr> {
        let min_i = if fix_0 { 1 } else { 0 };

        if max_i + 1 < n {
            return Err(EstimatorErr::InvalidParams(
                "`max_i`+1 must be less than `n`".to_string(),
            ));
        }

        let position: Vec<usize> = (0..n).collect();

        let last = n == max_i + 1;

        Ok(SampleCombinations {
            min_i,
            max_i,
            n,
            position,
            first: true,
            last,
        })
    }
}

impl Iterator for SampleCombinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.position.clone());
        }
        if self.last {
            return None;
        }

        for i in (self.min_i..self.position.len()).rev() {
            let e = self.position[i];
            if e < self.max_i - (self.n - i - 1) {
                let mut e_1 = e;
                for j in i..self.position.len() {
                    e_1 += 1;
                    self.position[j] = e_1;
                }
                if i == self.min_i && e + 1 == self.max_i {
                    self.last = true;
                }
                return Some(self.position.clone());
            }
        }
        None // Will never reach
    }
}

/// Pad an integrator left and/or right with tokens.
///
/// Example:
/// ```
/// use vtext::token_processing::*;
/// let sent = "One Two Three".split(" ");
/// let sent_padded: Vec<_> = pad_items(Box::new(sent), 3, Some("<s>"), Some("</s>")).unwrap().collect();
/// ```
///
/// Parameters:
/// * `items` - Input iterator
/// * `n` - The degree of the ngram
/// * `pad_left` - Optional string to use as left padding
/// * `pad_right` - Optional string to use as right padding
pub fn pad_items<'a>(
    items: Box<dyn Iterator<Item = &'a str> + 'a>,
    n: usize,
    pad_left: Option<&'a str>,
    pad_right: Option<&'a str>,
) -> Result<Box<dyn Iterator<Item = &'a str> + 'a>, EstimatorErr> {
    if n < 1 {
        return Err(EstimatorErr::InvalidParams(
            "`n` must be greater than or equal to 1".to_string(),
        ));
    }

    let left_chained: Box<dyn Iterator<Item = &'a str>>;
    let all_chained: Box<dyn Iterator<Item = &'a str>>;

    match pad_left {
        Some(s) => {
            let pad_left_iter = iter::repeat(s).take(n - 1);
            left_chained = Box::new(pad_left_iter.chain(items));
        }
        None => {
            left_chained = items;
        }
    }

    match pad_right {
        Some(s) => {
            let pad_right_iter = iter::repeat(s).take(n - 1);
            all_chained = Box::new(left_chained.chain(pad_right_iter));
        }
        None => {
            all_chained = left_chained;
        }
    }

    Ok(all_chained)
}
