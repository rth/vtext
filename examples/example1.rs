#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Debug)]
enum StringWrap<'a> {
    Slice(&'a str),
    String(String),
}

trait PipelineComponent<'a> {
    type InItem;
    type OutItem;

    fn transform<T>(
        &'a self,
        items: T
    ) -> Box<dyn Iterator<Item = Self::OutItem> + 'a>
    where
        T: Iterator<Item = Self::InItem> + 'a;
}

struct DummyTokenizer {
    split_on: String,
}

impl<'a> DummyTokenizer {
    fn transform_text(&'a self, text: &'a str) -> Box<dyn Iterator<Item = StringWrap<'a>> + 'a> {
        let iter = text.split(&self.split_on).map(|x| StringWrap::Slice(x));
        Box::new(iter)
    }
}

impl<'a> PipelineComponent<'a> for DummyTokenizer {
    type InItem = &'a str;
    type OutItem = StringWrap<'a>;

    fn transform<T>(
        &'a self,
        items: T
    ) -> Box<dyn Iterator<Item = Self::OutItem> + 'a>
        where
        T: Iterator<Item = Self::InItem> + 'a
    {
        let iter = items.flat_map(move |s| self.transform_text(s));
        Box::new(iter)
    }
}

struct DummyFilter {
    word: String,
}

impl<'a> PipelineComponent<'a> for DummyFilter {
    type InItem = StringWrap<'a>;
    type OutItem = Self::InItem;

    fn transform<T>(
        &'a self,
        items: T
    ) -> Box<dyn Iterator<Item = Self::OutItem> + 'a>
        where
        T: Iterator<Item = Self::InItem> + 'a

    {
        let iter = items.filter(move |x| match x {
            StringWrap::Slice(s) => s.clone() != self.word,
            StringWrap::String(s) => s.clone() != self.word,
        });
        Box::new(iter)
    }
}

struct DummyStemmer {}

impl<'a> PipelineComponent<'a> for DummyStemmer {
    type InItem = StringWrap<'a>;
    type OutItem = Self::InItem;

    fn transform<T>(
        &'a self,
        items: T
    ) -> Box<dyn Iterator<Item = Self::OutItem> + 'a>
        where
        T: Iterator<Item = Self::InItem> + 'a
    {
        // Outputs a StringWrap::string
        let iter = items.map(|x| match x {
            StringWrap::Slice(s) => StringWrap::String([s, "ing"].join("")),
            StringWrap::String(s) => StringWrap::String([s, "ing".to_string()].join("")),
        });
        Box::new(iter)
    }
}

fn main() {
    // API example
    let text = "Marry had a little lamb.";

    // Pipeline components
    let tokenizer = DummyTokenizer {
        split_on: " ".to_string(),
    };
    let filter = DummyFilter {
        word: "lamb".to_string(),
    };
    let stemmer = DummyStemmer {};

    // Pipeline
    let output = tokenizer.transform_text(text);
    let output = filter.transform(output);
    let output: Vec<_> = stemmer.transform(output).collect();

    println!("{:?}", output);
}
