from time import time
from glob import glob
from pathlib import Path

import nltk
import nltk.metrics.distance

import vtext

base_dir = Path(__file__).parent.parent.resolve()

try:
    import Levenshtein
except ImportError:
    Levenshtein = None

if __name__ == "__main__":
    input_files = list(glob(str(base_dir / "data" / "comp.graphics" / "*")))
    data = []
    for file_path in input_files:
        with open(file_path, "rt") as fh:
            data.append(fh.read())

    tokens = []
    for document in data:
        for word in document.split():
            if len(word) > 1:
                tokens.append(word)

    print("# vectorizing {} documents:".format(len(data)))

    tokens = tokens[:20000]
    db = [
        ("vtext dice_similarity", vtext.metrics.string.dice_similarity),
        ("vtext jaro_similarity", vtext.metrics.string.jaro_similarity),
        ("NLTK edit_distance", nltk.edit_distance),
    ]
    if Levenshtein is not None:
        db.extend(
            [
                ("python-Levenshtein Levenshtein", Levenshtein.distance),
                ("python-Levenshtein jaro", Levenshtein.jaro),
                ("python-Levenshtein jaro_winkler", Levenshtein.jaro_winkler),
            ]
        )

    for label, func in db:

        t0 = time()

        for x, y in zip(tokens, tokens[1:]):
            func(x, y)

        dt = time() - t0

        print(
            "{:>40}: {:.2f}s [ {:.1f} · 10³ tokens/s]".format(
                label, dt, len(tokens) / (dt * 1e3)
            )
        )
