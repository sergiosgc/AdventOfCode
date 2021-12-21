#!/usr/bin/env python3
import fileinput
input = [ line.strip() for line in fileinput.input() ]
def player_scores(start_pos, start_dice):
    pos = start_pos - 1
    dice = start_dice
    score = 0
    do_while = True
    while do_while:
        pos = (pos + sum(range(dice,dice+3))) % 10
        score += pos + 1
        dice += 6
        yield score
        do_while = (score < 1000)

scores = [
    [score for score in player_scores(int(input[0].split(":")[1]), 1)],
    [score for score in player_scores(int(input[1].split(":")[1]), 4)]
]
victory_turn = min(len(scores[0]), len(scores[1]))
if len(scores[0]) < len(scores[1]):
    print(scores[1][victory_turn-2]*(victory_turn * 2 * 3 - 3))
else:
    print(scores[0][victory_turn-1]*(victory_turn * 2 * 3))