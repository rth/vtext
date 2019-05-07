from time import time
from glob import glob
from pathlib import Path
import re

from nltk.stem import PorterStemmer, SnowballStemmer

import vtext

base_dir = Path(__file__).parent.parent.resolve()

if __name__ == "__main__":
    input_files = list(glob(str(base_dir / "data" / "*" / "*")))
    data = []
    for file_path in input_files[:1000]:
        with open(file_path, "rt") as fh:
            data.append(fh.read())
    assert len(data) > 0

    token_regexp = r"\b\w\w+\b"

    print("# stemming {} documents".format(len(data)))

    for label, func in [
        (r"nltk.stem.PorterStemmer()", PorterStemmer().stem),
        (r"nltk.stem.SnowballStemmer('english')", SnowballStemmer("english").stem),
        (r"nltk.stem.SnowballStemmer('french')", SnowballStemmer("french").stem),
        (
            "vtext.stem.SnowballStemmer('english')",
            vtext.stem.SnowballStemmer("english").stem,
        ),
        (
            "vtext.stem.SnowballStemmer('french')",
            vtext.stem.SnowballStemmer("french").stem,
        ),
    ]:
        tokens = [re.compile(r"\b\w\w+\b").findall(txt) for txt in data]
        tokens_len = sum(len(tok) for tok in tokens)

        t0 = time()
        for doc in tokens:
            for token in doc:
                func(token)

        dt = time() - t0

        print(
            "{:>45}: {:.2f}s [{:.2f} M tokens/s]".format(
                label, dt, 1e-6 * tokens_len / dt
            )
        )
