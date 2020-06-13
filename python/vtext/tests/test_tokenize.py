# Copyright 2019 vtext developers
#
# Licensed under the Apache License, Version 2.0,
# <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
# modified, or distributed except according to those terms.

import pytest
import hypothesis
import hypothesis.strategies as st

from vtext.tokenize import (
    UnicodeWordTokenizer,
    RegexpTokenizer,
    CharacterTokenizer,
    VTextTokenizer,
    BaseTokenizer,
)

TOKENIZERS = [
    RegexpTokenizer,
    CharacterTokenizer,
    UnicodeWordTokenizer,
    VTextTokenizer,
]


def _pytest_ids(x):
    if isinstance(x, BaseTokenizer):
        return x.__class__.__name__


def test_unicode_segment_tokenize():

    tokenizer = UnicodeWordTokenizer(word_bounds=False)
    assert tokenizer.tokenize("Today, tomorrow") == ["Today", "tomorrow"]

    tokenizer = UnicodeWordTokenizer(word_bounds=True)
    assert tokenizer.tokenize("Today, tomorrow") == ["Today", ",", "tomorrow"]

    with pytest.raises(TypeError):
        UnicodeWordTokenizer(word_bounds=1)

    with pytest.raises(TypeError):
        UnicodeWordTokenizer().tokenize(2)


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
@pytest.mark.parametrize(
    "tokenizer",
    [
        RegexpTokenizer(),
        CharacterTokenizer(),
        UnicodeWordTokenizer(),
        VTextTokenizer("en"),
        VTextTokenizer("fr"),
    ],
    ids=_pytest_ids,
)
def test_tokenize_edge_cases(tokenizer, txt):
    tokenizer.tokenize(txt)


@pytest.mark.parametrize(
    "tokenizer, expected",
    [
        (RegexpTokenizer(), {"pattern": r"\b\w\w+\b"}),
        (CharacterTokenizer(), {"window_size": 4}),
        (UnicodeWordTokenizer(), {"word_bounds": True}),
        (VTextTokenizer("en"), {"lang": "en"}),
        (VTextTokenizer("fr"), {"lang": "fr"}),
    ],
    ids=_pytest_ids,
)
def test_tokenize_get_params(tokenizer, expected):
    params = tokenizer.get_params()
    assert params == expected


@pytest.mark.parametrize("Tokenizer", TOKENIZERS)
def test_tokenize_api(Tokenizer):
    assert issubclass(Tokenizer, BaseTokenizer)
    # check that we can initialize it without positional args
    Tokenizer()
