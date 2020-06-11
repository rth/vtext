import pickle
import pytest
from vtext.tokenize import CharacterTokenizer, RegexpTokenizer, UnicodeSegmentTokenizer, VTextTokenizer


@pytest.mark.parametrize('Estimator', [
    UnicodeSegmentTokenizer,
    #CharacterTokenizer, #RegexpTokenizer, UnicodeSegmentTokenizer, VTextTokenizer
])
def test_pickle(Estimator):
    est = Estimator()

    out = pickle.dumps(est)

    pickle.loads(out)
