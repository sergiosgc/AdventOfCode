#!/usr/bin/env python3
import fileinput
input = [ line.strip() for line in fileinput.input() ]
win_counts_memoize = {}
def win_counts(_one, _two, dice, ones_turn):
    if ones_turn:
        one = { "score": _one['score'] + (_one['pos'] -1 + sum(dice)) % 10 + 1, "pos": (_one['pos'] -1 + sum(dice)) % 10 + 1}
        two = { "score": _two['score'], "pos": _two['pos'] }
    else:
        one = { "score": _one['score'], "pos": _one['pos'] }
        two = { "score": _two['score'] + (_two['pos'] -1 + sum(dice)) % 10 + 1, "pos": (_two['pos'] -1 + sum(dice)) % 10 + 1}
    key = "%s-%s-%s-%s-%s" % (one["score"], one["pos"], two["score"], two["pos"], "1" if ones_turn else "0")
    if one['score'] > 20:
        return [1,0]
    if two['score'] > 20:
        return [0,1]
    if key in win_counts_memoize:
        return win_counts_memoize[key]
    win_counts_memoize[key] = list(map(sum, zip(*[win_counts(one, two, (d1,d2,d3), not ones_turn) for d1 in range(1,4) for d2 in range(1,4) for d3 in range(1,4)])))
    return win_counts_memoize[key]

print(max(list(map(sum, zip(*[win_counts(
    {"score": 0, "pos": int(input[0].split(":")[1]) }, 
    {"score": 0, "pos": int(input[1].split(":")[1]) }, 
    (d1,d2, d3), True) for d1 in range(1,4) for d2 in range(1,4) for d3 in range(1,4)])))))