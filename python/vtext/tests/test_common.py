# Copyright 2019 vtext developers
#
# Licensed under the Apache License, Version 2.0,
# <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
# modified, or distributed except according to those terms.

import pickle
import pytest
from vtext.tokenize import (
    CharacterTokenizer,
    RegexpTokenizer,
    UnicodeWordTokenizer,
    VTextTokenizer,
)
from vtext.tokenize_sentence import UnicodeSentenceTokenizer, PunctuationTokenizer
from vtext.stem import SnowballStemmer


TOKENIZERS = [
    CharacterTokenizer,
    RegexpTokenizer,
    UnicodeWordTokenizer,
    VTextTokenizer,
]

SENTENCE_TOKENIZERS = [UnicodeSentenceTokenizer, PunctuationTokenizer]
STEMMERS = [SnowballStemmer]


@pytest.mark.parametrize("Estimator", TOKENIZERS + SENTENCE_TOKENIZERS + STEMMERS)
def test_pickle(Estimator):
    est = Estimator()
    params_ref = est.get_params()

    out = pickle.dumps(est)

    est2 = pickle.loads(out)
    assert est2.get_params() == params_ref


def test_pickle_non_default_params():
    # check that pickling correctly stores estimator parameters
    est = CharacterTokenizer(window_size=10)
    est2 = pickle.loads(pickle.dumps(est))
    assert est2.get_params()["window_size"] == 10
