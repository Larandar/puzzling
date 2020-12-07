"""Advent of Code 2020, Day 3: {TITLE_HERE}."""

import logging
import pprint
import math
import itertools, functools
from typing import List

import attr, click
import pandas as pd, numpy as np

import advent.inputs as inputs


AOC_DAY = 3


@attr.s(auto_attribs=True)
class Map:
    lines: List[List[bool]]

    @classmethod
    def from_input(cls, day=AOC_DAY):
        return cls(inputs.matrix_of_day(day, mapping={".": False, "#": True}))

    def __getitem__(self, s):
        i, j = s
        return self.lines[i][j % len(self.lines[i])]

    def on_path(self, path):
        try:
            yield from (self[i, j] for i, j in path)
        except IndexError:
            pass


def slope(i, j):
    return zip(itertools.count(step=i), itertools.count(step=j))


def count_true(iter):
    return len(list(filter(None, iter)))


def part1():
    """Solution for part 1 of day 3."""
    logging.info("SOLVING DAY 3 PART 1")

    map = Map.from_input()
    trees_on_path = map.on_path(slope(1, 3))
    result = count_true(trees_on_path)

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 274  # Valid result for my input


def part2():
    """Solution for part 2 of day 3."""
    logging.info("SOLVING DAY 3 PART 2")

    map = Map.from_input()
    result = 1
    for s in [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]:
        result *= count_true(map.on_path(slope(*s)))

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 6050183040  # Valid result for my input
