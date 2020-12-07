"""Advent of Code 2020, Day 6: {TITLE_HERE}."""

import logging
import pprint
import math
import itertools, functools
from typing import Dict, List

import attr, click
import pandas as pd, numpy as np

import advent.inputs as inputs


AOC_DAY = 6


def responses(day=AOC_DAY):
    current = set()
    for l in inputs.lines_of_day(day):
        if not l:
            yield current
            current = set()
        else:
            current |= set(l)
    yield current


@attr.s(auto_attribs=True)
class Questionaire:

    responses: List[Dict[str, bool]]

    @classmethod
    def from_imput(cls, day=AOC_DAY):
        input_lines = []
        for l in inputs.lines_of_day(day):
            if not l:
                yield cls._from_input(input_lines)
                input_lines = []
            else:
                input_lines.append(l)
        yield cls._from_input(input_lines)

    @classmethod
    def _from_input(cls, lines):
        q = set().union(*lines)

        return cls([{k: k in l for k in q} for l in lines])


def part1():
    """Solution for part 1 of day 6."""
    logging.info("SOLVING DAY 6 PART 1")

    result = sum(map(len, responses()))

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 6437  # Valid result for my input


def part2():
    """Solution for part 2 of day 6."""
    logging.info("SOLVING DAY 6 PART 2")
    questionaires = Questionaire.from_imput()
    groups = [
        [set(k for k, v in q0.items() if v) for q0 in q.responses]
        for q in questionaires
    ]
    all_yes = [qs[0].intersection(*qs) for qs in groups]

    pprint.pprint(all_yes)
    result = sum(map(len, all_yes))
    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    # assert result == XXXX  # Valid result for my input
