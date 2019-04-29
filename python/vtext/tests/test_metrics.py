from vtext.metrics.string import dice_similarity, jaro_similarity


def test_dice_similarity():
    assert dice_similarity("healed", "sealed") == 0.8


def test_jaro_similarity():
    assert jaro_similarity("healed", "sealed")
