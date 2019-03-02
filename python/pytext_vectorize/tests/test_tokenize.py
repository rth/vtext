import pytest

from pytext_vectorize.tokenize import UnicodeSegmentTokenizer


def test_unicode_segment_tokenize():

    tokenizer = UnicodeSegmentTokenizer(word_bounds=False)
    assert tokenizer.tokenize("Today, tomorrow") == ["Today", "tomorrow"]

    tokenizer = UnicodeSegmentTokenizer(word_bounds=True)
    assert tokenizer.tokenize("Today, tomorrow") == ["Today", ",", "tomorrow"]

    with pytest.raises(TypeError):
        UnicodeSegmentTokenizer(word_bounds=1)

    with pytest.raises(TypeError):
        UnicodeSegmentTokenizer().tokenize(2)
