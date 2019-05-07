# Copyright 2019 vtext developers
#
# Licensed under the Apache License, Version 2.0,
# <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
# modified, or distributed except according to those terms.

import pytest

from vtext.tokenize import UnicodeSegmentTokenizer
from vtext.tokenize import RegexpTokenizer
from vtext.tokenize import CharacterTokenizer


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


def test_character_tokenizer():
    tokenizer = CharacterTokenizer()
    assert tokenizer.tokenize("fox can't") == [
        "fox ",
        "ox c",
        "x ca",
        " can",
        "can'",
        "an't",
    ]
