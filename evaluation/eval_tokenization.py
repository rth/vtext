import re
from glob import glob
from time import time
from pathlib import Path

import conllu
import pandas as pd
import numpy as np

from vtext.tokenize import UnicodeSegmentTokenizer, VTextTokenizer

try:
    import sacremoses
except ImportError:
    sacremoses = None

try:
    import spacy
except ImportError:
    spacy = None

base_dir = Path(__file__).parent.parent
base_dir = base_dir / "ud-treebanks-v2.3/"


def tokens_similarity(tokens_ref, tokens):
    return len([tok for tok in tokens if tok in tokens_ref]) / len(tokens_ref)


def evaluate_tokenizer(treebank, tokenizer):
    scores = []
    for sentence in treebank:
        txt = sentence.metadata["text"]
        tokens = [str(el) for el in tokenizer(txt)]
        tokens_ref = [el["form"] for el in sentence]
        similarity = tokens_similarity(tokens_ref, tokens)
        # if similarity != 1:
        #    print(f"Expected: {tokens_ref}")
        #    print(f"Got:      {tokens}")
        scores.append(similarity)
    scores = np.mean(scores)
    return scores


tb_list = [
    ("en", "GUM"),
    ("en", "EWT"),
    ("fr", "Sequoia"),
    ("de", "GSD"),
    # ("ru", "GSD"),
]


def whitespace_split(x):
    return x.split(" ")


tok_db = [
    # ("whitespace", lambda lang: whitespace_split),
    ("regexp", lambda lang: re.compile(r"\b\w\w+\b").findall),
    (
        "unicode-segmentation",
        lambda lang: UnicodeSegmentTokenizer(word_bounds=True).tokenize,
    ),
    ("vtext", lambda lang: VTextTokenizer(lang).tokenize),
]

if sacremoses is not None:
    tok_db.append(("MosesTokenizer", lambda lang: sacremoses.MosesTokenizer().tokenize))

if spacy is not None:
    tok_db.append(
        ("spacy", lambda lang: spacy.load(lang, parser=False, entity=False).tokenizer)
    )

out = []
for lang, tb_name in tb_list:
    tb_pattern = base_dir / "*" / f"{lang}_{tb_name.lower()}-ud-test.conllu"
    tb_path = list(glob(str(tb_pattern)))
    if len(tb_path) != 1:
        raise ValueError(tb_path)
    tb_path = tb_path[0]

    with (base_dir / tb_path).open("rt") as fh:
        t0 = time()
        tb = conllu.parse(fh.read())
        print(f"Loaded {tb_name} in {time() - t0:.2f}s")
    for name, get_tokenizer in tok_db:
        tokenizer = get_tokenizer(lang)
        t0 = time()
        res = evaluate_tokenizer(tb, tokenizer)
        print(f"{tb_name} done with {name} in {time() - t0:.2f}s")
        out.append({"treebank": tb_name, "lang": lang, "tokenizer": name, "score": res})

out = (
    pd.DataFrame(out)
    .set_index(["lang", "treebank", "tokenizer"])
    .score.unstack(-1)
    .round(3)
)
print(out)
