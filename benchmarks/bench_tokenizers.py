from time import time
from glob import glob
from pathlib import Path
import re

from vtext.tokenize import (
    RegexpTokenizer,
    UnicodeWordTokenizer,
    VTextTokenizer,
    CharacterTokenizer,
    NTLKWordTokenizer
)

try:
    import sacremoses
except ImportError:
    sacremoses = None

try:
    import spacy
except ImportError:
    spacy = None

try:
    import blingfire
except ImportError:
    blingfire = None

try:
    import nltk
except ImportError:
    nltk = None


base_dir = Path(__file__).parent.parent.resolve()

if __name__ == "__main__":
    input_files = list(glob(str(base_dir / "data" / "*" / "*")))
    data = []
    for file_path in input_files:
        with open(file_path, "rt") as fh:
            data.append(fh.read())
    assert len(data) > 0

    token_regexp = r"\b\w\w+\b"

    dataset_size = 91  # MB for 20 newsgroup dataset

    print("# Tokenizing {} documents".format(len(data)))

    def pyre_tokenizer(txt):
        return list(re.compile(token_regexp).findall(txt))

    db = [
        (r"Python re.findall(r'\b\w\w+\b', ...)", pyre_tokenizer),
        (
            r"RegexpTokenizer(r'\b\w\w+\b')",
            RegexpTokenizer(pattern=token_regexp).tokenize,
        ),
        (
            "UnicodeWordTokenizer(word_bounds=False)",
            UnicodeWordTokenizer(word_bounds=False).tokenize,
        ),
        (
            "UnicodeWordTokenizer(word_bounds=True)",
            UnicodeWordTokenizer(word_bounds=True).tokenize,
        ),
        ("VTextTokenizer('en')", VTextTokenizer("en").tokenize),
        ("CharacterTokenizer(4)", CharacterTokenizer(4).tokenize),
        ("NTLKWordTokenizer()", NTLKWordTokenizer().tokenize)
    ]

    if sacremoses is not None:
        db.append(("MosesTokenizer()", sacremoses.MosesTokenizer().tokenize))
    if spacy is not None:
        from spacy.lang.en import English

        db.append(("Spacy en", English().tokenizer))

    if blingfire is not None:
        db.append(("BlingFire en", lambda x: blingfire.text_to_words(x).split(" ")))

    if nltk is not None:
        db.append(('NLTK NLTKWordTokenizer', nltk.tokenize.NLTKWordTokenizer().tokenize))

    for label, func in db:
        t0 = time()

        out = []

        for idx, doc in enumerate(data):
            out.append(func(doc))

        dt = time() - t0

        n_tokens = sum(len(tok) for tok in out)

        print(
            "{:>45}: {:.2f}s [{:.1f} MB/s, {:.0f} kWPS]".format(
                label, dt, dataset_size / dt, n_tokens * 1e-3 / dt
            )
        )
