from time import time
from glob import glob
from pathlib import Path
import re

from vtext.tokenize_sentence import UnicodeSentenceTokenizer, PunctuationTokenizer

base_dir = Path(__file__).parent.parent.resolve()

if __name__ == "__main__":
    input_files = list(glob(str(base_dir / "data" / "*" / "*")))
    data = []
    for file_path in input_files:
        with open(file_path, "rt") as fh:
            data.append(fh.read())
    assert len(data) > 0

    dataset_size = 91  # MB for 20 newsgroup dataset

    print("# Tokenizing {} documents".format(len(data)))

    def regexp_tokenizer(txt):
        return list(re.split("(?<=[!.?])", txt))

    db = [
        (r"Python re.split('(?<=[!.?])', ...)", regexp_tokenizer),
        (
            "UnicodeSentenceTokenizer()",
            UnicodeSentenceTokenizer().tokenize,
        ),
        (
            "PunctuationTokenizer()",
            PunctuationTokenizer().tokenize,
        ),
    ]

    for label, func in db:
        t0 = time()

        out = []
        for idx, doc in enumerate(data):
            out.append(func(doc))

        dt = time() - t0

        n_tokens = sum(len(tok) for tok in out)

        print(
            "{:>45}: {:.2f}s {:.1f} MB/s, {:.0f} sentences".format(
                label, dt, dataset_size / dt, n_tokens
            )
        )
