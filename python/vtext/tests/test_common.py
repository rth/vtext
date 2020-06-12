import pickle
import pytest
from vtext.tokenize import CharacterTokenizer, RegexpTokenizer, UnicodeSegmentTokenizer, VTextTokenizer
from vtext.tokenize_sentence import UnicodeSentenceTokenizer 


TOKENIZERS = [
    CharacterTokenizer, RegexpTokenizer, UnicodeSegmentTokenizer, VTextTokenizer
    ]

SENTENCE_TOKENIZERS = [UnicodeSentenceTokenizer]

@pytest.mark.parametrize('Estimator', TOKENIZERS + SENTENCE_TOKENIZERS)
def test_pickle(Estimator):
    est = Estimator()

    out = pickle.dumps(est)

    pickle.loads(out)
