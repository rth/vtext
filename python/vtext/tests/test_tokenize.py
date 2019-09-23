# Copyright 2019 vtext developers
#
# Licensed under the Apache License, Version 2.0,
# <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
# modified, or distributed except according to those terms.

import pytest
import hypothesis
import hypothesis.strategies as st

from vtext.tokenize import (
    UnicodeSegmentTokenizer,
    RegexpTokenizer,
    CharacterTokenizer,
    VTextTokenizer,
)


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


@hypothesis.given(st.text())
def test_regex_tokenize_any(txt):
    tokenizer = RegexpTokenizer()
    tokenizer.tokenize(txt)


@hypothesis.given(st.text())
def test_character_tokenize_any(txt):
    tokenizer = CharacterTokenizer()
    tokenizer.tokenize(txt)


@hypothesis.given(st.text())
def test_unicode_segment_tokenize_any(txt):
    tokenizer = UnicodeSegmentTokenizer()
    tokenizer.tokenize(txt)


@hypothesis.given(st.text())
def test_vtext_tokenize_any(txt):
    tokenizer = VTextTokenizer("en")
    tokenizer.tokenize(txt)
