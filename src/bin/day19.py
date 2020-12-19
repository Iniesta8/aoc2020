#!/usr/bin/env python3 
import regex

f = open("./input/day19_2.txt", 'r')
data = f.read()

rules, data = data.split("\n\n")
messages = data.split('\n')

r = open("./data/day19_2_regex.txt", 'r')
re = r.read().strip()

count = 0
for msg in messages:
    count += regex.match(re, msg) is not None

print(count)
