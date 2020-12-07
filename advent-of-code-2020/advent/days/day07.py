"""Advent of Code 2020, Day 7: {TITLE_HERE}."""

import logging
import pprint
import math, re
import itertools, functools

import attr, click
import pandas as pd, numpy as np

import advent.inputs as inputs


AOC_DAY = 7


BAG_REGEX = re.compile(r"^\s*((?P<count>\d+)\s)?(?P<color>\w+ \w+)( bags?)?.?$")


@attr.s(auto_attribs=True, hash=True)
class Bag:
    color: str
    count: str = attr.ib(default=1, eq=False, hash=False)

    @classmethod
    def from_expression(cls, exp):
        match = BAG_REGEX.match(exp)
        return cls(
            color=match.group("color"),
            count=int(match.group("count") or 1),
        )


def parse_bags(day=AOC_DAY):
    for b, _, contain in map(
        lambda l: l.partition(" contain "), inputs.lines_of_day(day)
    ):
        bag = Bag.from_expression(b)
        if contain == "no other bags.":
            yield bag, set()
        else:
            yield bag, set(map(Bag.from_expression, contain.split(",")))


def bag_map(day=AOC_DAY):
    return dict(parse_bags(day))


def part1():
    """Solution for part 1 of day 7."""
    logging.info("SOLVING DAY 7 PART 1")

    mapping = bag_map()
    backtrack = [Bag("shiny gold")]
    can_contain = set()
    while len(backtrack):
        n = backtrack.pop()
        for b, contains in mapping.items():
            if n in contains:
                backtrack.append(b)
                can_contain.add(b)

    result = len(can_contain)

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    # assert result == XXXX  # Valid result for my input


def part2():
    """Solution for part 2 of day 7."""
    logging.info("SOLVING DAY 7 PART 2")

    mapping = bag_map()

    def inside(bag):
        return bag.count + bag.count * sum(map(inside, mapping.get(bag, [])))

    result = inside(Bag("shiny gold")) - 1

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    # assert result == XXXX  # Valid result for my input
