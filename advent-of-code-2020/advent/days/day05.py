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


def binary_space(space, instructions, left):
    """Split a space using a binary space indexing."""
    # NOTE: Yes it crash if it get a 0 length space
    if len(space) <= 1:
        return space[0]

    cut = len(space) // 2
    return binary_space(
        space[:cut] if instructions[0] == left else space[cut:],
        instructions=instructions[1:],
        left=left,
    )


@attr.s(auto_attribs=True)
class BoardingPass:
    """Describe a boarding pass."""

    coded_position: str

    row: int = attr.ib(init=False)
    column: int = attr.ib(init=False)

    def __attrs_post_init__(self):
        """Compute raw and column only once after the initialization."""
        self.row = binary_space(
            list(range(128)),
            instructions=self.coded_position[:7],
            left="F",
        )
        self.column = binary_space(
            list(range(8)),
            instructions=self.coded_position[7:],
            left="L",
        )

    @property
    def id(self):
        """Id of the boarding pass."""
        return self.row * 8 + self.column

    @classmethod
    def from_input(cls, day=AOC_DAY):
        """Yield boarding passes from the input of the day."""
        yield from map(cls, inputs.lines_of_day(day))


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

    result = None
    for i, p in sorted(passes.items()):
        if i + 1 not in passes and i + 2 in passes:
            result = p.id + 1
            break

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 659  # Valid result for my input
