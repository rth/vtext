from time import time
from glob import glob
from pathlib import Path

from vtext.tokenize import RegexpTokenizer
from vtext.token_processing import KSkipNGrams

try:
    import nltk
    from nltk.util import everygrams as nltk_everygrams, skipgrams as nltk_skipgrams
except ImportError:
    nltk = None


base_dir = Path(__file__).parent.parent.resolve()

LIMIT = None

if __name__ == "__main__":
    input_files = list(glob(str(base_dir / "data" / "*" / "*")))
    if LIMIT is not None:
        input_files = input_files[:LIMIT]
    data = []
    for file_path in input_files:
        with open(file_path, "rt") as fh:
            data.append(fh.read())
    assert len(data) > 0

    token_regexp = r"\b\w\w+\b"

    dataset_size = 91  # MB for 20 newsgroup dataset

    print("# Testing {} documents".format(len(data)))

    db = [
        (r"vtext: everygram", KSkipNGrams(min_n=1, max_n=3, max_k=0).transform,),
        ("nltk: everygram", lambda seq: nltk_everygrams(seq, 1, 3),),
        (r"vtext: skipgram", KSkipNGrams(min_n=3, max_n=3, max_k=2).transform,),
        ("nltk: skipgram", lambda seq: nltk_skipgrams(seq, n=3, k=2),),
    ]

    tokenizer = RegexpTokenizer(pattern=token_regexp)

    # Tokenize
    doc_tokens = [tokenizer.tokenize(doc) for doc in data]

    for label, func in db:
        t0 = time()

        out = []
        for idx, doc in enumerate(doc_tokens):
            out.append(func(doc))

        dt = time() - t0

        # number of input tokens
        n_tokens = sum(len(tok) for tok in doc_tokens)

        print(
            "{:>45}: {:.2f}s [{:.1f} MB/s, {:.0f} kWPS]".format(
                label, dt, dataset_size / dt, n_tokens * 1e-3 / dt
            )
        )
