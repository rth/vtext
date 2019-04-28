from vtext.metrics.string import dice_similarity


def test_dice_similarity():
    assert dice_similarity("healed", "sealed") == 0.8
