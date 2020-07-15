#[cfg(test)]
mod tests;

use std::cmp::{max, min};
use std::collections::VecDeque;
use std::iter;
use std::iter::Peekable;

#[cfg(feature = "python")]
use dict_derive::{FromPyObject, IntoPyObject};
use serde::{Deserialize, Serialize};
use crate::errors::EstimatorErr;

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
    /// use vtext::ngram_utils::*;
    /// let sent = "One Two Three Four".split(" ");
    /// let gramizer = KSkipNGrams::new_bigram();
    /// let grams: Vec<_> = gramizer.transform(Box::new(sent), None, None).unwrap().collect();
    /// assert_eq!(grams, vec![vec!["One", "Two"], vec!["Two", "Three"], vec!["Three", "Four"]]);
    /// ```
    pub fn new_bigram() -> KSkipNGrams {
        KSkipNGramsParams::new(2, 2, 0).build()
    }

    /// Generate all trigrams from a sequence of `items`, an iterator.
    ///
    /// Example:
    /// ```
    /// use vtext::ngram_utils::*;
    /// let sent = "One Two Three Four".split(" ");
    /// let gramizer = KSkipNGrams::new_trigram();
    /// let grams: Vec<_> = gramizer.transform(Box::new(sent), None, None).unwrap().collect();
    /// assert_eq!(grams, vec![vec!["One", "Two", "Three"], vec!["Two", "Three", "Four"]]);
    /// ```
    pub fn new_trigram() -> KSkipNGrams {
        KSkipNGramsParams::new(3, 3, 0).build()
    }

    /// Generate all ngrams from a sequence of `items`, an iterator.
    ///
    /// Example:
    /// ```
    /// use vtext::ngram_utils::*;
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

    /// Generate all ngrams between `min_n` and `max_n` from a sequence of `items`, an iterator.
    ///
    /// Example:
    /// ```
    /// use vtext::ngram_utils::*;
    /// let sent = "One Two Three".split(" ");
    /// let gramizer = KSkipNGrams::new_everygrams(1, 3);
    /// let grams: Vec<_> = gramizer.transform(Box::new(sent), None, None).unwrap().collect();
    /// assert_eq!(grams, vec![
    /// vec!["One"], vec!["One", "Two"], vec!["One", "Two", "Three"], vec!["Two"],
    /// vec!["Two", "Three"], vec!["Three"]]);
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
    /// use vtext::ngram_utils::*;
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
    /// use vtext::ngram_utils::*;
    /// let sent = "One Two Three Four".split(" ");
    /// let gramizer = KSkipNGrams::new(2, 3, 1);
    /// let grams: Vec<_> = gramizer.transform(Box::new(sent), None, None).unwrap().collect();
    /// assert_eq!(grams, vec![vec!["One", "Two"], vec!["One", "Three"], vec!["One", "Two", "Three"],
    /// vec!["One", "Two", "Four"], vec!["One", "Three", "Four"], vec!["Two", "Three"],
    /// vec!["Two", "Four"], vec!["Two", "Three", "Four"], vec!["Three", "Four"]]);
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
/// The iterator consumes the input iterator only once and holds a window of items to generate the
/// grams. The window is stepped forward as it consumes the input. It also correctly generates
/// left or right padding if specified.
pub struct KSkipNGramsIter<'a> {
    // Params
    items: Box<dyn Iterator<Item = &'a str> + 'a>,
    min_n: usize,
    max_n: usize,
    max_k: usize,
    pad_left: Option<&'a str>,
    pad_right: Option<&'a str>,

    // Iterator state
    /// Window which holds items that have been consumed
    window: VecDeque<&'a str>,
    /// Gram length that was yielded last
    n: usize,
    /// Amount of padding included in item yielded last
    p: usize,
    /// Offset used during MainEnd mode
    offset: usize,
    /// k-skip combinations of current window
    sample_iter: Peekable<SampleCombinations>,
    /// Current mode of iterator
    mode: IterMode,
    first: bool,
}

/// Core methods to build `KSkipNGramsIter`
impl<'a> KSkipNGramsIter<'a> {
    /// Build a new `KSkipNGramsIter`.
    ///
    /// Example:
    /// ```
    /// use vtext::ngram_utils::*;
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
        if min_n < 1 {
            return Err(EstimatorErr::InvalidParams(
                "`min_n` must be greater than or equal to 1".to_string(),
            ));
        }
        if min_n > max_n {
            return Err(EstimatorErr::InvalidParams(
                "`max_n` must be greater than or equal to `min_n`".to_string(),
            ));
        }
        let mut max_k = max_k;
        if max_n == 1 {
            max_k = 0; // if n == 1. k has no effect
        }

        let window = Self::build_window(&mut items, max_n, max_k)?;

        Ok(KSkipNGramsIter {
            // Params
            items,
            min_n,
            max_n,
            max_k,
            pad_left,
            pad_right,

            // Iterator state
            window,
            n: 0,
            p: 0,
            offset: 0,
            sample_iter: SampleCombinations::new_empty().peekable(),
            mode: IterMode::Start,
            first: false,
        })
    }

    // Prepare and populate start window
    fn build_window(
        items: &mut Box<dyn Iterator<Item = &'a str> + 'a>,
        max_n: usize,
        max_k: usize,
    ) -> Result<VecDeque<&'a str>, EstimatorErr> {
        let window_size = max_n + max_k;
        let mut window: VecDeque<&'a str> = VecDeque::with_capacity(window_size);

        // Populate window
        let mut i = window_size;
        while i > 0 {
            let next_item = items.next();
            match next_item {
                None => {
                    return Err(EstimatorErr::InvalidInput(
                        "Items length is smaller than `max_n`+`max_k`".to_string(),
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
impl<'a> Iterator for KSkipNGramsIter<'a> {
    type Item = Vec<&'a str>;

    // Next item. Depending on current mode obtain next item.
    // If current mode has been exhausted then switch to next
    fn next(&mut self) -> Option<Self::Item> {
        return match &self.mode {
            IterMode::Start => {
                self.start_mode_pad_left();
                self.next()
            }

            IterMode::PadLeft => {
                if self.pad_left.is_some() && self.max_n > 1 {
                    let next = self.next_gram_pad_left();
                    match &next {
                        Some(_e) => next,
                        None => {
                            self.start_mode_main();
                            self.next()
                        }
                    }
                } else {
                    self.start_mode_main();
                    self.next()
                }
            }

            IterMode::Main => {
                let next = self.next_gram_main();
                match &next {
                    Some(_e) => next,
                    None => {
                        self.start_mode_main_end();
                        self.next()
                    }
                }
            }

            IterMode::MainEnd => {
                if (self.min_n != self.max_n || self.max_k > 0) && self.window.len() > 1 {
                    let next = self.next_gram_main_end();
                    match &next {
                        Some(_e) => next,
                        None => {
                            self.start_mode_pad_right();
                            self.next()
                        }
                    }
                } else {
                    self.start_mode_pad_right();
                    self.next()
                }
            }

            IterMode::PadRight => {
                if self.pad_right.is_some() && self.max_n > 1 {
                    self.next_gram_pad_right()
                } else {
                    return None;
                }
            }
        };
    }
}

/// Internal functions
impl<'a> KSkipNGramsIter<'a> {
    // Switching between modes
    fn start_mode_pad_left(&mut self) {
        self.mode = IterMode::PadLeft;
        self.first = true;
    }

    fn start_mode_main(&mut self) {
        self.mode = IterMode::Main;
        self.first = true;
    }

    fn start_mode_main_end(&mut self) {
        self.mode = IterMode::MainEnd;
        self.first = true;
    }

    fn start_mode_pad_right(&mut self) {
        self.mode = IterMode::PadRight;
        self.first = true;
    }

    // Obtain next gram for PadLeft mode
    fn next_gram_pad_left(&mut self) -> Option<Vec<&'a str>> {
        self.next_params_pad_left()?;

        let slice_idx: Vec<usize> = self.sample_iter.next().unwrap();
        let grams = self.construct_grams_vec(slice_idx);
        Some(grams)
    }

    // Obtain next gram for PadRight mode
    fn next_gram_pad_right(&mut self) -> Option<Vec<&'a str>> {
        self.next_params_pad_right()?;

        let mut sample_idx: Vec<usize> = self.sample_iter.next().unwrap();

        // Mirror index
        for e in sample_idx.iter_mut() {
            *e = self.window.len() - 1 - *e;
        }
        sample_idx.reverse();

        let grams = self.construct_grams_vec(sample_idx);
        Some(grams)
    }

    // Obtain next gram for Main mode
    fn next_gram_main(&mut self) -> Option<Vec<&'a str>> {
        let finished = self.next_state_pad_main();

        if finished.is_none() {
            self.forward_window()?;
            self.first = true;
            return self.next_gram_main();
        }

        let sample_idx = self.sample_iter.next().unwrap();
        let grams = self.construct_grams_vec(sample_idx);
        Some(grams)
    }

    // Obtain next gram for MainEnd mode
    fn next_gram_main_end(&mut self) -> Option<Vec<&'a str>> {
        self.next_state_pad_main_end()?;

        let mut sample_idx = self.sample_iter.next().unwrap();
        // Offset index
        for e in sample_idx.iter_mut() {
            *e += self.offset;
        }
        let grams = self.construct_grams_vec(sample_idx);
        Some(grams)
    }

    // Forward the window by one step
    fn forward_window(&mut self) -> Option<()> {
        // Need to forward window when yielded ngram of max-length and max-skip-size
        let next_item = self.items.next();

        return match next_item {
            None => None,
            Some(s) => {
                self.window.pop_front();
                self.window.push_back(s);
                Some(()) // Successfully forwarded window
            }
        };
    }

    // Increment parameters and sample iterator
    fn next_params_pad_left(&mut self) -> Option<()> {
        // Equivalent to a for-loop:
        // for n in max(self.min_n, 2)..self.max_n+1:
        //      for p in (n-1)..0:   // decreasing
        //          for sample_idx in sample_iter:
        //              next_gram(n, p, sample_idx)
        return if self.first {
            self.n = max(self.min_n, 2);
            self.p = self.n - 1;
            self.sample_iter =
                SampleCombinations::new(false, self.n + self.max_k - self.p - 1, self.n - self.p)
                    .unwrap()
                    .peekable();

            self.first = false;
            Some(())
        } else if self.sample_iter.peek().is_some() {
            Some(())
        } else if self.p > 1 {
            self.p -= 1;

            self.sample_iter =
                SampleCombinations::new(false, self.n + self.max_k - self.p - 1, self.n - self.p)
                    .unwrap()
                    .peekable();

            Some(())
        } else if self.n < self.max_n {
            self.n += 1;
            self.p = self.n - 1;

            self.sample_iter =
                SampleCombinations::new(false, self.n + self.max_k - self.p - 1, self.n - self.p)
                    .unwrap()
                    .peekable();

            Some(())
        } else {
            None
        };
    }

    // Increment parameters and sample iterator
    fn next_params_pad_right(&mut self) -> Option<()> {
        // Equivalent to a for-loop:
        // for n in max(self.min_n, 2)..self.max_n+1:
        //      for p in 1..n:
        //          for sample_idx in sample_iter:
        //              next_gram(n, p, sample_idx)
        return if self.first {
            self.n = max(self.min_n, 2);
            self.p = 1;
            self.first = false;

            self.sample_iter =
                SampleCombinations::new(false, self.n + self.max_k - self.p - 1, self.n - self.p)
                    .unwrap()
                    .peekable();

            Some(())
        } else if self.sample_iter.peek().is_some() {
            Some(())
        } else if self.p < self.n - 1 {
            self.p += 1;

            self.sample_iter =
                SampleCombinations::new(false, self.n + self.max_k - self.p - 1, self.n - self.p)
                    .unwrap()
                    .peekable();

            Some(())
        } else if self.n < self.max_n {
            self.n += 1;
            self.p = 1;

            self.sample_iter =
                SampleCombinations::new(false, self.n + self.max_k - self.p - 1, self.n - self.p)
                    .unwrap()
                    .peekable();

            Some(())
        } else {
            None
        };
    }

    // Increment parameters and sample iterator for each window
    fn next_state_pad_main(&mut self) -> Option<()> {
        // Equivalent to a for-loop:
        // for n in self.min_n..self.max_n + 1:
        //      for sample_idx in sample_iter:
        //          next_gram(n, sample_idx)
        return if self.first {
            self.n = self.min_n;
            self.sample_iter = SampleCombinations::new(true, self.n + self.max_k - 1, self.n)
                .unwrap()
                .peekable();

            self.first = false;
            Some(())
        } else if self.sample_iter.peek().is_some() {
            Some(())
        } else if self.n < min(self.max_n, self.window.len()) {
            self.n += 1;
            self.sample_iter = SampleCombinations::new(true, self.n + self.max_k - 1, self.n)
                .unwrap()
                .peekable();

            Some(())
        } else {
            None
        };
    }

    // Increment parameters and sample iterator for each window
    fn next_state_pad_main_end(&mut self) -> Option<()> {
        // Equivalent to a for-loop:
        // for offset in 1..window.len()-min_n
        //      for n in self.min_n..self.max_n + 1:
        //          for sample_idx in sample_iter:
        //              next_gram(offset, n, sample_idx)
        return if self.first {
            self.n = self.min_n;
            self.offset = 1;
            self.reset_sample_iter_main_end();

            self.first = false;
            Some(())
        } else if self.sample_iter.peek().is_some() {
            Some(())
        } else if self.n < min(self.max_n, self.window.len() - self.offset) {
            self.n += 1;
            self.reset_sample_iter_main_end();

            Some(())
        } else if self.window.len() - self.offset > self.min_n {
            self.offset += 1;
            self.n = self.min_n;
            self.reset_sample_iter_main_end();

            Some(())
        } else {
            None
        };
    }

    fn reset_sample_iter_main_end(&mut self) {
        let window_len = self.window.len() - self.offset;
        let mut k = 0;
        if window_len > self.n {
            k = min(self.max_k, window_len - self.n);
        }
        let max_i = self.n + k - 1;
        self.sample_iter = SampleCombinations::new(true, max_i, self.n)
            .unwrap()
            .peekable();
    }

    // Create output vec from sample index and add padding if necessary
    fn construct_grams_vec(&mut self, sample_idx: Vec<usize>) -> Vec<&'a str> {
        let grams = self.vec_from_idx(sample_idx);

        return match self.mode {
            IterMode::PadLeft => {
                // Add padding to the left
                [
                    iter::repeat(self.pad_left.unwrap()).take(self.p).collect(),
                    grams,
                ]
                .concat()
            }

            IterMode::PadRight => {
                // Add padding to the right
                [
                    grams,
                    iter::repeat(self.pad_right.unwrap()).take(self.p).collect(),
                ]
                .concat()
            }

            _ => grams,
        };
    }

    // Create output vec from sample index
    fn vec_from_idx(&mut self, sample_idx: Vec<usize>) -> Vec<&'a str> {
        let mut grams = Vec::with_capacity(sample_idx.len());
        for idx in sample_idx.iter() {
            grams.push(self.window[*idx].clone());
        }
        grams
    }
}

/// Represents the different modes of `KSkipNGramsIter`
enum IterMode {
    Start,
    PadLeft,
    Main,
    MainEnd,
    PadRight,
}

/// An iterator which generates the list of combinations of `n` items in a range upto `max_i`.
/// It is possible to fix the first item at index 0 (i.e. `fix_0` == true)
///
/// Examples:
/// ```text
/// use vtext::ngram_utils::*;
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
    pub fn new(fix_0: bool, max_i: usize, n: usize) -> Result<SampleCombinations, EstimatorErr> {
        let min_i;
        if fix_0 {
            min_i = 1;
        } else {
            min_i = 0;
        }

        if max_i + 1 < n {
            return Err(EstimatorErr::InvalidParams("`max_i`+1 must be less than `n`".to_string()));
        }

        let position: Vec<usize> = (0..n).collect();

        let mut last = false;
        if n == max_i + 1 {
            last = true;
        }

        Ok(SampleCombinations {
            min_i,
            max_i,
            n,
            position,
            first: true,
            last,
        })
    }

    /// Produce dummy `SampleCombinations`. Will panic if `next` is executed.
    pub fn new_empty() -> SampleCombinations {
        SampleCombinations {
            min_i: 0,
            max_i: 0,
            n: 0,
            position: Vec::new(),
            first: false,
            last: false,
        }
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
