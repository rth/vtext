import pytest

from vtext.tokenize import UnicodeSegmentTokenizer
from vtext.tokenize import RegexpTokenizer


def test_unicode_segment_tokenize():

    tokenizer = UnicodeSegmentTokenizer(word_bounds=False)
    assert tokenizer.tokenize("Today, tomorrow") == ["Today", "tomorrow"]

    tokenizer = UnicodeSegmentTokenizer(word_bounds=True)
    assert tokenizer.tokenize("Today, tomorrow") == ["Today", ",", "tomorrow"]

    with pytest.raises(TypeError):
        UnicodeSegmentTokenizer(word_bounds=1)

    with pytest.raises(TypeError):
        UnicodeSegmentTokenizer().tokenize(2)


def test_regexp_tokenize():

    tokenizer = RegexpTokenizer(pattern=r"\b\w\w+\b")
    assert tokenizer.tokenize("Today, tomorrow") == ["Today", "tomorrow"]

    # check default pattern
    tokenizer = RegexpTokenizer()
    assert tokenizer.tokenize("Today, tomorrow") == ["Today", "tomorrow"]
