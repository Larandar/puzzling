"""Advent of Code 2020, Day 6: {TITLE_HERE}."""

from collections import defaultdict
import logging
import pprint
import math
import itertools, functools
from typing import Counter, Dict, List

import attr, click
import pandas as pd, numpy as np

import advent.inputs as inputs


AOC_DAY = 6


@attr.s(auto_attribs=True)
class Questionaire:

    persons: List[Dict[str, bool]]

    @property
    def questions(self):
        return Counter([k for q in self.responses for k in q])

    @classmethod
    def from_records(cls, records):
        return cls(persons=[set(q) for q in records])

    def questions(self):
        return set(q for p in self.persons for q in p)

    def person_answers(self, p):
        return {q: q in self.persons[p] for q in sorted(self.questions())}

    def question_answers(self, q):
        return [q in p for p in self.persons]

    def consensus(self):
        return self.persons[0].intersection(*self.persons[1:])


def part1():
    """Solution for part 1 of day 6."""
    logging.info("SOLVING DAY 6 PART 1")

    result = (
        inputs.records_of_day(AOC_DAY, multiline=True)
        .map(Questionaire.from_records)
        .sum(lambda q: len(q.questions()))
    )

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 6437  # Valid result for my input


def part2():
    """Solution for part 2 of day 6."""
    logging.info("SOLVING DAY 6 PART 2")
    result = (
        inputs.records_of_day(AOC_DAY, multiline=True)
        .map(Questionaire.from_records)
        .map(lambda s: s.consensus())
        .sum(len)
    )

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 3229  # Valid result for my input
