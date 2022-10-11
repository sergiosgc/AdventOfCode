#!/usr/bin/env python3
import fileinput
import sys
from pprint import pprint

def between(a, b, c):
    return (a>=b) and (a<=c)

chitons = []
distances = []
for line in fileinput.input():
    chitons.append([int(c) for c in line.rstrip()])
    width = len(chitons[len(chitons) - 1])
    for r in range(1,5):
        chitons[len(chitons)-1].extend( [ 1 if c + 1 > 9 else c + 1 for c in chitons[len(chitons) - 1][width*(r-1):width*r] ] )
width = len(chitons[0])
height = len(chitons)
for r in range(0,4):
    for line in chitons[height*r:height*r+height]:
        chitons.append( [ 1 if c + 1 > 9 else c + 1 for c in line ])
for line in chitons:
    distances.append( [ None for c in line ])
width = len(chitons[0])
height = len(chitons)
distances[height-1][width-1] = 0
queue = [ (width - 1, height - 1)]
while len(queue):
    next_queue = []
    for x,y in queue:
        for coord in [ c for c in [ (x+1,y), (x-1,y), (x,y+1), (x,y-1) ] if between(c[0], 0, width-1) and between(c[1], 0, height-1) ]:
            if distances[coord[1]][coord[0]] is None or distances[coord[1]][coord[0]] > (distances[y][x] + chitons[y][x]):
                distances[coord[1]][coord[0]] = (distances[y][x] + chitons[y][x])
                next_queue.append( (coord[0], coord[1]) )
    queue = next_queue

print(distances[0][0])

