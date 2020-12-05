"""Advent of Code 2020, Day 5: {TITLE_HERE}."""

import logging
import pprint
import math
import itertools, functools

import attr, click
import pandas as pd, numpy as np

import advent.inputs as inputs


AOC_DAY = 5
INPUT_FILE = inputs.file_of_day(AOC_DAY)


def binary_space(str, posibilities, minus, plus):
    if len(posibilities) == 0:
        raise StopIteration
    if len(posibilities) == 1:
        return posibilities[0]
    n = str[0]
    cut = len(posibilities) // 2
    if n == minus:
        posibilities = posibilities[:cut]
    else:
        posibilities = posibilities[cut:]
    return binary_space(str[1:], posibilities, minus, plus)


@attr.s(auto_attribs=True)
class BoardingPass:
    coded_position: str

    @property
    def row(self):
        return binary_space(
            self.coded_position[0:7],
            list(sorted(range(128))),
            "F",
            "B",
        )

    @property
    def column(self):
        return binary_space(
            self.coded_position[7:],
            list(sorted(range(8))),
            "L",
            "R",
        )

    @property
    def id(self):
        return self.row * 8 + self.column

    @classmethod
    def from_input(day=AOC_DAY):
        yield from map(BoardingPass, inputs.lines_of_day(AOC_DAY))


def part1():
    """Solution for part 1 of day 5."""
    logging.info("SOLVING DAY 5 PART 1")

    result = max(p.id for p in BoardingPass.from_input())

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 896  # Valid result for my input


def part2():
    """Solution for part 2 of day 5."""
    logging.info("SOLVING DAY 5 PART 2")

    passes = {p.id: p for p in BoardingPass.from_input()}
    seats = set(range(128 * 8)) - set(passes.keys())
    pprint.pprint(seats)

    result = None
    for i, p in sorted(passes.items()):
        if i + 1 not in passes and i + 2 in passes:
            result = p.id + 1
            break

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 659  # Valid result for my input
