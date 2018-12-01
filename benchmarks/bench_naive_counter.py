from glob import glob
import re
from collections import Counter
import os

token_pattern = r"\b\w\w+\b"


def vectorize(text):
    tokens = re.compile(token_pattern).findall(text)
    df = Counter(tokens)
    return len(df)


if __name__ == "__main__":
    input_files = list(glob("./data/*/*"))
    s = 0
    for file_path in input_files:
        with open(file_path, "rt", encoding="utf8") as fh:
            try:
                s += vectorize(fh.read())
            except:  # noqa
                os.unlink(file_path)
    print(s)
