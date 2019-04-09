import re
import conllu
from time import time
from pathlib import Path

import pandas as pd
import numpy as np

from vtext.tokenize import UnicodeSegmentTokenizer, VTextTokenizer

base_dir = Path("/home/rth/nlp/ud-treebanks-v2.3/")



def tokens_similarity(tokens_ref, tokens):
    return len([tok for tok in tokens if tok in tokens_ref]) / len(tokens_ref)



def evaluate_tokenizer(treebank, tokenizer):
    scores = []
    for sentence in treebank:
        txt = sentence.metadata['text']
        tokens = tokenizer(txt)
        tokens_ref = [el['form'] for el in sentence]
        similarity = tokens_similarity(tokens_ref, tokens)
        #if similarity != 1:
        #    print(f'Expected: {tokens_ref}')
        #    print(f'Got:      {tokens}')
        scores.append(similarity)
    scores = np.mean(scores)
    return scores


tb_list = [('English-GUM', 'UD_English-GUM/en_gum-ud-train.conllu'),
           ('English-EWT', 'UD_English-EWT/en_ewt-ud-train.conllu'),
           #('UD_French-GSD', 'UD_French-GSD/fr_gsd-ud-train.conllu'),
           #('Japanese-PUD', 'UD_Japanese-PUD/ja_pud-ud-test.conllu')
           ]

out = []
for tb_name, tb_path in tb_list:

    with (base_dir / tb_path).open('rt') as fh:
        t0 = time()
        tb = conllu.parse(fh.read())
        print(f'Loaded {tb_name} in {time() - t0:.2f}s')
    for name, tokenizer in [#('whitespace', lambda x: x.split(' ')),
                            ('regexp', re.compile(r'\b\w\w+\b').findall),
                            ('UnicodeSegment', UnicodeSegmentTokenizer(word_bounds=True).tokenize),
                            ("VTextTokenizer", VTextTokenizer("en").tokenize)]:
        t0 = time()
        res = evaluate_tokenizer(tb, tokenizer)
        print(f'{tb_name} done with {name} in {time() - t0:.2f}s')
        out.append({'treebank': tb_name, 'tokenizer': name, 'score': res})

out = pd.DataFrame(out).set_index(['tokenizer', 'treebank']).score.unstack().T.round(3)
print(out)
