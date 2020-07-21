# Copyright 2019 vtext developers
#
# Licensed under the Apache License, Version 2.0,
# <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
# modified, or distributed except according to those terms.

import pytest
import hypothesis
import hypothesis.strategies as st

from vtext.token_processing import KSkipNGrams


def test_unicode_segment_tokenize():

    gramizer = KSkipNGrams(min_n=2, max_n=2, max_k=0)
    assert gramizer.transform(["One", "Two", "Three"]) == [
        ["One", "Two"],
        ["Two", "Three"],
    ]

    with pytest.raises(TypeError):
        KSkipNGrams()

    # n == 0
    with pytest.raises(ValueError):
        KSkipNGrams(min_n=0, max_n=0, max_k=0).transform(["One", "Two", "Three"])

    # min_n > max_n
    with pytest.raises(ValueError):
        KSkipNGrams(min_n=1, max_n=0, max_k=0).transform(["One", "Two", "Three"])

    # max_k < 0
    with pytest.raises(OverflowError):
        KSkipNGrams(min_n=1, max_n=1, max_k=-1).transform(["One", "Two", "Three"])


@hypothesis.given(st.lists(st.text(min_size=0))
def test_tokenize_edge_cases(txt):
    KSkipNGrams(min_n=1, max_n=1, max_k=1).transform(list(txt))
