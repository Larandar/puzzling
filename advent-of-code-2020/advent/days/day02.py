"""Advent of Code 2020, Day 2: {TITLE_HERE}."""

import logging
import pprint
from collections import Counter

import click
import attr

import advent.inputs as inputs


AOC_DAY = 2


@attr.s(auto_attribs=True)
class Policy:

    a: int
    b: int
    letters: set
    passwd: str

    @classmethod
    def from_input(cls):
        yield from (
            cls(
                *map(int, idxs.split("-")),
                letters=set(letters.strip(":")),
                passwd=passwd,
            )
            for idxs, letters, passwd in inputs.lines_of_day(AOC_DAY, split=" ")
        )

    @property
    def is_valid(self) -> bool:
        count = Counter(self.passwd)
        return all(self.a <= count[l] <= self.b for l in self.letters)

    @property
    def is_toboggan_valid(self) -> bool:
        return (self.passwd[self.a - 1] in self.letters) != (
            self.passwd[self.b - 1] in self.letters
        )


def part1():
    """Solution for part 1 of day 2."""
    logging.info("SOLVING DAY 2 PART 1")

    valid_policies = filter(lambda p: p.is_valid, Policy.from_input())
    result = len(list(valid_policies))

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 564  # Valid result for my input


def part2():
    """Solution for part 2 of day 2."""
    logging.info("SOLVING DAY 2 PART 2")

    valid_policies = filter(lambda p: p.is_toboggan_valid, Policy.from_input())
    result = len(list(valid_policies))

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 325  # Valid result for my input
