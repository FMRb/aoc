import functools
from collections import Counter

cardValue = {
    "A": 13, "K": 12, "Q":11, "J": 10, "T": 9, "9":8, "8":7, "7":6, "6":5, "5":4, "4":3, "3":2, "2":1
}

cardValueWithJoker = {
    "A": 13, "K": 12, "Q":11, "J": 0, "T": 9, "9":8, "8":7, "7":6, "6":5, "5":4, "4":3, "3":2, "2":1
}

def is_five_of_kind(cards, joker=False):
    if joker:
        nc = set(filter(lambda c: c != "J", cards))
        return len(nc) == 0 or len(nc) == 1


    return len(set(cards)) == 1

def is_four_of_kind(cards, joker=False):
    if joker:
        nc = set(filter(lambda c: c != "J", cards))
        if len(nc) == 2:
            a, b = nc
            return cards.count(a) == 1 or cards.count(b) == 1
        return False
    s_cards = set(cards)
    if len(s_cards) == 2:
        for c in s_cards:
            if cards.count(c) == 4:
                return True
    return False


def is_full_house(cards, joker=False):
    if joker:
        nc = set(filter(lambda c: c != "J", cards))
        return len(nc) == 2 and not is_four_of_kind(cards, joker)
    return len(set(cards)) == 2 and not is_four_of_kind(cards)

def is_three_of_kind(cards, joker=False):
    if joker:
        # AJQTJ JAAQT JJAQT AAAQT
        # AAQQT AAQQT 
        nc = set(filter(lambda c: c != "J", cards))
        if len(nc) == 3:
            a, b, c = nc
            return (
                (cards.count(a) >= 1 and cards.count(b) == 1 and cards.count(c) == 1) or 
                (cards.count(a) == 1 and cards.count(b) >= 1 and cards.count(c) == 1) or 
                (cards.count(a) == 1 and cards.count(b) == 1 and cards.count(c) >= 1)
            )
        return False
    s_cards = set(cards)
    if len(s_cards) == 3:
        for c in s_cards:
            if cards.count(c) == 3:
                return True
    return False

def is_two_pairs(cards, joker=False):
    # AAQQT A
    if joker:
        nc = set(filter(lambda c: c != "J", cards))
        return len(nc) == 3 and not is_three_of_kind(cards, joker)
    s_cards = set(cards)
    return len(s_cards) == 3 and not is_three_of_kind(cards)

def is_one_pair(cards, joker=False):
    # AQTKJ AAQTK
    if joker:
        nc = set(filter(lambda c: c != "J", cards))
        return len(nc) == 4
    return len(set(cards)) == 4

def compare_higher_cards(card_a, card_b, joker=False):
    for i in range(len(card_a)):
        v_a = cardValueWithJoker[card_a[i]] if joker else cardValue[card_a[i]]
        v_b = cardValueWithJoker[card_b[i]] if joker else cardValue[card_b[i]]
        if v_a == v_b:
            continue
        return v_a - v_b
    return 0

def compare_hands(hand_a, hand_b, joker=False):
    cards_a, bid_a = hand_a
    cards_b, bid_b = hand_b

    if is_five_of_kind(cards_a,joker) and is_five_of_kind(cards_b,joker):
        return compare_higher_cards(cards_a, cards_b,joker)
    if is_five_of_kind(cards_a,joker) or is_five_of_kind(cards_b,joker):
        return is_five_of_kind(cards_a,joker) - is_five_of_kind(cards_b,joker)

    if is_four_of_kind(cards_a,joker) and is_four_of_kind(cards_b,joker):
        return compare_higher_cards(cards_a, cards_b,joker)
    if is_four_of_kind(cards_a,joker) or is_four_of_kind(cards_b,joker):
        return is_four_of_kind(cards_a,joker) - is_four_of_kind(cards_b,joker)

    if is_full_house(cards_a,joker) and is_full_house(cards_b,joker):
        return compare_higher_cards(cards_a, cards_b,joker)
    if is_full_house(cards_a,joker) or is_full_house(cards_b,joker):
        return is_full_house(cards_a,joker) - is_full_house(cards_b,joker)

    if is_three_of_kind(cards_a,joker) and is_three_of_kind(cards_b,joker):
        return compare_higher_cards(cards_a, cards_b,joker)
    if is_three_of_kind(cards_a,joker) or is_three_of_kind(cards_b,joker):
        return is_three_of_kind(cards_a,joker) - is_three_of_kind(cards_b,joker)

    if is_two_pairs(cards_a,joker) and is_two_pairs(cards_b,joker):
        return compare_higher_cards(cards_a, cards_b,joker)
    if is_two_pairs(cards_a,joker) or is_two_pairs(cards_b,joker):
        return is_two_pairs(cards_a,joker) - is_two_pairs(cards_b,joker)

    if is_one_pair(cards_a,joker) and is_one_pair(cards_b,joker):
        return compare_higher_cards(cards_a, cards_b,joker)
    if is_one_pair(cards_a,joker) or is_one_pair(cards_b,joker):
        return is_one_pair(cards_a,joker) - is_one_pair(cards_b,joker)

    return compare_higher_cards(cards_a, cards_b,joker)
    
def part1(input):
    sorted_hands = sorted(input, key=functools.cmp_to_key(compare_hands))

    result = 0
    for i in range(len(sorted_hands)):
        result += sorted_hands[i][1] * (i+1)
        #print("".join(sorted_hands[i][0]))


    print("Part 1:", result)

def part2(input):
    sorted_hands = sorted(input, key=functools.cmp_to_key(lambda a, b: compare_hands(a, b, True)))

    result = 0
    for i in range(len(sorted_hands)):
        result += sorted_hands[i][1] * (i+1)

    print("Part 1:", result)

    pass


if __name__ == "__main__":
    input = []
    for line in open(0).read().splitlines():
        cards, bid = line.split()
        cards = [*cards]
        bid = int(bid)
        input.append((cards, bid))

    part1(input)
    part2(input)

