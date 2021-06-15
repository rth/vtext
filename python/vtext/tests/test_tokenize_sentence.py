# Copyright 2019 vtext developers
#
# Licensed under the Apache License, Version 2.0,
# <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
# modified, or distributed except according to those terms.

import pytest
import hypothesis
import hypothesis.strategies as st

from vtext.tokenize import BaseTokenizer
from vtext.tokenize_sentence import UnicodeSentenceTokenizer, PunctuationTokenizer

TOKENIZERS = [UnicodeSentenceTokenizer, PunctuationTokenizer]


def _pytest_ids(x):
    if isinstance(x, BaseTokenizer):
        return x.__class__.__name__


def test_unicode_sentence_tokenize():

    tokenizer = UnicodeSentenceTokenizer()
    assert tokenizer.tokenize(
        "Here is one. Here is another? Bang!! This trailing text is one more"
    ) == [
        "Here is one. ",
        "Here is another? ",
        "Bang!! ",
        "This trailing text is one more",
    ]


def test_punctuation_sentence_tokenizer():

    tokenizer = PunctuationTokenizer()
    assert tokenizer.tokenize(
        "Here is one. Here is another? Bang!! This trailing text is one more"
    ) == [
        "Here is one. ",
        "Here is another? ",
        "Bang!",
        "! ",
        "This trailing text is one more",
    ]


@hypothesis.given(st.text())
@pytest.mark.parametrize(
    "tokenizer",
    [UnicodeSentenceTokenizer(), PunctuationTokenizer()],
    ids=_pytest_ids,
)
def test_tokenize_edge_cases(tokenizer, txt):
    tokens = tokenizer.tokenize(txt)
    assert len("".join(tokens)) == len(txt)


@pytest.mark.parametrize(
    "tokenizer, expected",
    [
        (UnicodeSentenceTokenizer(), {}),
        (PunctuationTokenizer(), {"punctuation": [".", "!", "?"]}),
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
