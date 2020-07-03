#[cfg(test)]
mod tests;

use std::cmp::{min, max};
use std::collections::VecDeque;
use std::iter;

use std::iter::Peekable;

fn pad_items<'a>(
    items: Box<dyn Iterator<Item = &'a str> + 'a>,
    n: usize,
    pad_left: Option<&'a str>,
    pad_right: Option<&'a str>,
) -> Box<dyn Iterator<Item = &'a str> + 'a> {
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

    all_chained
}

struct SkipVecIter {
    prev: Vec<usize>,
    n: usize,
    k: usize,
    first: bool,
}

impl SkipVecIter {
    pub fn new(n: usize, k: usize) -> SkipVecIter {
        SkipVecIter {
            prev: vec![0; n],
            n,
            k,
            first: true
        }
    }

    pub fn new_empty() -> SkipVecIter {
        SkipVecIter {
            prev: Vec::new(),
            n: 0,
            k: 0,
            first: false
        }
    }
}

impl Iterator for SkipVecIter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.prev.clone());
        } else {
            for i in (0..self.n).rev() {
                let e = self.prev[i];
                if e < self.k {
                    self.prev[i] += 1;
                    for j in i+1..self.n {
                        self.prev[j] = 0;
                    }
                    let sum: usize = self.prev.iter().sum();
                    if sum <= self.k {
                        return Some(self.prev.clone());
                    } else {
                        return self.next()
                    }
                }
            }
            return None;
        }
    }
}

enum IterMode {
    Start,
    PadLeft,
    Main,
    MainEnd,
    PadRight,
}

struct KSkipNGramsIter<'a> {
    // Params
    items: Box<dyn Iterator<Item = &'a str> + 'a>,
    min_n: usize,
    max_n: usize,
    min_k: usize,
    max_k: usize,
    pad_left: Option<&'a str>,
    pad_right: Option<&'a str>,

    // Iterator state
    window: VecDeque<&'a str>,
    window_end: VecDeque<&'a str>,
    n: usize, // length outputted last
    k: usize,
    p: usize,
    combinations: Peekable<SkipVecIter>,
    mode: IterMode,
    first: bool,
}

impl<'a> Iterator for KSkipNGramsIter<'a> {
    type Item = Vec<&'a str>;

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
                if self.min_n != self.max_n || self.max_k > 0  {
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
        self.window_end = self.window.clone();
        self.window.pop_front();
        self.first = true;
    }

    fn start_mode_pad_right(&mut self) {
        self.mode = IterMode::PadRight;
        self.window = self.window_end.clone();
        self.first = true;
    }

    // Next gram
    fn next_gram_pad_left(&mut self) -> Option<Vec<&'a str>> {
        self.next_params_pad_left()?;

        // let slice_idx = (self.k..self.window.len())
        //     .step_by(self.k + 1)
        //     .take(self.n - self.p);
        // let grams = self.construct_grams_vec(slice_idx);
        // return Some(grams);


        let mut slice_idx: Vec<usize> = Vec::with_capacity(self.n);
        let mut i = 0;
        let spacing = self.combinations.next().unwrap();
        println!("");
        for (j, &e) in spacing.clone().iter().enumerate() { // TODO remove clone
            if j == 0 {
                i += e;
            } else {
                i += e+1;
            }
            slice_idx.push(i);
        }
        println!("LP {:?} {:?} {}", spacing, slice_idx, self.window.len());
        let grams = self.construct_grams_vec(slice_idx);
        return Some(grams);
    }

    fn next_gram_pad_right(&mut self) -> Option<Vec<&'a str>> {
        self.next_params_pad_right()?;

        // let slice_idx = (0..self.window.len() - self.k)
        //     .rev()
        //     .step_by(self.k + 1)
        //     .take(self.n - self.p)
        //     .rev();
        // let grams = self.construct_grams_vec(slice_idx.collect::<Vec<_>>());
        // return Some(grams);


        let mut slice_idx: Vec<usize> = Vec::with_capacity(self.n);

        let spacing = self.combinations.next().unwrap();
        let end_idx = self.window.len()-1;
        let mut i = end_idx;
        for (j, &e) in spacing.clone().iter().enumerate() { // TODO remove clone
            if j == 0 {
                i -= e;
            } else {
                i -= e+1;
            }
            slice_idx.push(i);
        }

        slice_idx.reverse();

        for i in slice_idx.clone() {
            if i > end_idx {
                println!();
            }
        }

        println!("LP {:?} {:?} {}", spacing, slice_idx, self.window.len());
        let grams = self.construct_grams_vec(slice_idx);
        return Some(grams);
    }

    fn next_gram_main(&mut self) -> Option<Vec<&'a str>> {
        let finished = self.next_state_pad_main();

        if finished.is_none() {
            self.forward_window()?;
            self.first = true;
            return self.next_gram_main();
        }

        // Get slice
        //let slice_idx = (0..self.window.len()).step_by(self.k + 1).take(self.n);

        let mut slice_idx: Vec<usize> = Vec::with_capacity(self.n);
        slice_idx.push(0);
        let mut i = 0;
        let spacing = self.combinations.next().unwrap();

        for e in spacing.clone() { // TODO remove clone
            i += e+1;
            slice_idx.push(i);
        }
        println!("{:?} {:?} {}", spacing, slice_idx, self.window.len());
        let grams = self.construct_grams_vec(slice_idx);
        return Some(grams);
    }

    fn next_gram_main_end(&mut self) -> Option<Vec<&'a str>> {
        let finished = self.next_state_pad_main();

        if finished.is_none() {
            // if self.window.len() >= 4 {
            //     self.pop_window()?;
            //     self.first = true;
            //     return self.next_gram_main_end();
            // } else if self.window.len() == 1 {
            //     return None
            // } else {
            //     let grams = Vec::from(self.window.clone());
            //     self.pop_window();
            //     return Some(grams)
            // }

            self.pop_window()?;
            self.first = true;
            return self.next_gram_main_end();
        }

        // Get slice
        // let slice_idx = (0..self.window.len()).step_by(self.k + 1).take(self.n);
        // let grams = self.construct_grams_vec(slice_idx);

        // return if grams.len() == self.n { // `take` takes n or less
        //     Some(grams)
        // } else {
        //     None
        // }

        let mut slice_idx: Vec<usize> = Vec::with_capacity(self.n);
        slice_idx.push(0);
        let mut i = 0;
        for e in self.combinations.next().unwrap() {
            i += e+1;
            slice_idx.push(i);
        }
        println!("{:?}, {}", slice_idx, self.window.len());
        let grams = self.construct_grams_vec(slice_idx);

        return if grams.len() == self.n { // TODO: why?
            Some(grams)
        } else {
            None
        }

    }

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
        }
    }

    fn pop_window(&mut self) -> Option<()> {
        // Pop item from window
        return if self.window.len() >= 2 {
            self.window.pop_front();
            Some(())
        } else {
            None
        };
    }

    fn next_params_pad_left(&mut self) -> Option<()> {
        // Equivalent to a for-loop:
        // for n in max(self.min_n, 2)..self.max_n + 1
        // --     for k in self.min_k..self.max_k + 1
        //          for p in (n-1)..0   // decreasing
        //              for combi in combinations:
        //                  next_gram(n, k, p)
        return if self.first {
            self.n = max(self.min_n, 2);
            //self.k = self.min_k;
            self.p = self.n - 1;

            let pick_n = self.n - self.p;
            let skip_total = self.max_k;
            self.combinations = SkipVecIter::new(pick_n, skip_total).peekable();

            self.first = false;
            Some(())
        } else if self.combinations.peek().is_some() {
            Some(())
        } else if self.p > 1 {
            self.p -= 1;

            let pick_n = self.n - self.p;
            let skip_total = self.max_k;
            self.combinations = SkipVecIter::new(pick_n, skip_total).peekable();

            Some(())
        } else if self.n < self.max_n {
            self.n += 1;
            //self.k = self.min_k;
            self.p = self.n - 1;

            let pick_n = self.n - self.p;
            let skip_total = self.max_k;
            self.combinations = SkipVecIter::new(pick_n, skip_total).peekable();

            Some(())
        } else {
            None
        }
    }

    fn next_params_pad_right(&mut self) -> Option<()> {
        // Equivalent to a for-loop:
        // for n in max(self.min_n, 2)..self.max_n + 1
        //      for k in self.min_k..self.max_k + 1
        //          for p in 1..n
        //              next_gram(n, k, p)
        return if self.first {
            self.n = max(self.min_n, 2);
            //self.k = self.min_k;
            self.p = 1;
            self.first = false;

            let pick_n = self.n - self.p;
            let skip_total = self.max_k;
            self.combinations = SkipVecIter::new(pick_n, skip_total).peekable();

            Some(())
        } else if self.combinations.peek().is_some() {
            Some(())
        } else if self.p < self.n - 1 {
            self.p += 1;

            let pick_n = self.n - self.p;
            let skip_total = self.max_k;
            self.combinations = SkipVecIter::new(pick_n, skip_total).peekable();

            Some(())
        } else if self.n < self.max_n {
            self.n += 1;
            //self.k = self.min_k;
            self.p = 1;

            let pick_n = self.n - self.p;
            let skip_total = self.max_k;
            self.combinations = SkipVecIter::new(pick_n, skip_total).peekable();

            Some(())
        } else {
            None
        }
    }

    fn next_state_pad_main(&mut self) -> Option<()> {
        // Equivalent to a for-loop:
        // for n in self.min_n..self.max_n + 1
        //      for combi in combination(window[1:], n)
        //          next_gram = window[0] + combi
        return if self.first {
            self.n = self.min_n;

            let pick_n = min(self.max_n, self.window.len()) - 1;
            let skip_total = min(self.window.len()-pick_n-1, self.max_k);
            self.combinations = SkipVecIter::new(pick_n, skip_total).peekable();

            self.first = false;
            Some(())
        } else if self.combinations.peek().is_some() {
            Some(())
        } else if self.n < min(self.max_n, self.window.len()) {
            self.n += 1;

            let pick_n = min(self.max_n, self.window.len()) - 1;
            let skip_total = min(self.window.len()-pick_n-1, self.max_k);
            self.combinations = SkipVecIter::new(pick_n, skip_total).peekable();

            Some(())
        } else {
            None
        }
    }

    // fn next_state_pad_main_end(&mut self) -> Option<()> {
    //     // Equivalent to a for-loop:
    //     // for n in self.min_n..self.max_n + 1
    //     //      for k in self.min_k..self.max_k + 1
    //     //          next_gram(n, k, p)
    //     return if self.first {
    //         self.n = self.min_n;
    //         self.k = self.min_k;
    //         self.first = false;
    //         Some(())
    //     } else if self.k < self.max_k {
    //         self.k += 1;
    //         Some(())
    //     } else if self.n < min(self.max_n, self.window.len()) {
    //         self.k = self.min_k;
    //         self.n += 1;
    //         Some(())
    //     } else {
    //         None
    //     }
    // }

    fn construct_grams_vec(
        &mut self,
        slice_idx: Vec<usize>,
    ) -> Vec<&'a str> {
        let grams = self.vec_from_idx(slice_idx);

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

    fn vec_from_idx(&mut self, slice_idx: Vec<usize>) -> Vec<&'a str> {
        let mut grams = Vec::with_capacity(slice_idx.len());
        for idx in slice_idx.iter() {
            grams.push(self.window[*idx].clone());
        }
        grams
    }
}

fn build_window<'a>(
    items: &mut Box<dyn Iterator<Item = &'a str> + 'a>,
    max_n: usize,
    max_k: usize,
) -> Result<VecDeque<&'a str>, &'static str> {
    let window_size = (max_n - 1) * (max_k + 1) + 1;
    let mut window: VecDeque<&'a str> = VecDeque::with_capacity(window_size);

    // Populate window
    let mut i = window_size;
    while i > 0 {
        let next_item = items.next();
        match next_item {
            None => {
                return Err("Items length is smaller than what is required by `max_n` and `max_k`")
            }
            Some(s) => {
                window.push_back(s);
            }
        }
        i -= 1;
    }
    Ok(window)
}

fn build_k_skip_n_grams_iter<'a>(
    mut items: Box<dyn Iterator<Item = &'a str> + 'a>,
    min_n: usize,
    max_n: usize,
    min_k: usize,
    max_k: usize,
    pad_left: Option<&'a str>,
    pad_right: Option<&'a str>,
) -> Result<Box<dyn Iterator<Item = Vec<&'a str>> + 'a>, &'a str> {
    if min_n < 1 {
        return Err("`min_n` must be greater than or equal to 1");
    }
    if min_n > max_n {
        return Err("`max_n` must be greater than or equal to `min_n`");
    }
    if min_k > max_k {
        return Err("`max_k` must be greater than or equal to `min_k`");
    }

    let window = build_window(&mut items, max_n, max_k)?;

    Ok(Box::new(KSkipNGramsIter {
        // Params
        items,
        min_n,
        max_n,
        min_k,
        max_k,
        pad_left,
        pad_right,

        // Iterator state
        window,
        window_end: VecDeque::new(),
        n: 0, // length outputted last
        k: 0,
        combinations: SkipVecIter::new_empty().peekable(),
        p: 0,
        mode: IterMode::Start,
        first: false,
    }))
}

fn bigram<'a>(
    items: Box<dyn Iterator<Item = &'a str> + 'a>,
    pad_left: Option<&'a str>,
    pad_right: Option<&'a str>,
) -> Result<Box<dyn Iterator<Item = Vec<&'a str>> + 'a>, &'a str> {
    build_k_skip_n_grams_iter(items, 2, 2, 0, 0, pad_left, pad_right)
}

fn ngrams<'a>(
    items: Box<dyn Iterator<Item = &'a str> + 'a>,
    n: usize,
    pad_left: Option<&'a str>,
    pad_right: Option<&'a str>,
) -> Result<Box<dyn Iterator<Item = Vec<&'a str>> + 'a>, &'a str> {
    build_k_skip_n_grams_iter(items, n, n, 0, 0, pad_left, pad_right)
}

fn everygrams<'a>(
    items: Box<dyn Iterator<Item = &'a str> + 'a>,
    min_length: usize,
    max_length: usize,
    pad_left: Option<&'a str>,
    pad_right: Option<&'a str>,
) -> Result<Box<dyn Iterator<Item = Vec<&'a str>> + 'a>, &'a str> {
    build_k_skip_n_grams_iter(items, min_length, max_length, 0, 0, pad_left, pad_right)
}

fn skipgrams<'a>(
    items: Box<dyn Iterator<Item = &'a str> + 'a>,
    n: usize,
    k: usize,
    pad_left: Option<&'a str>,
    pad_right: Option<&'a str>,
) -> Result<Box<dyn Iterator<Item = Vec<&'a str>> + 'a>, &'a str> {
    build_k_skip_n_grams_iter(items, n, n, 0, k, pad_left, pad_right)
}
