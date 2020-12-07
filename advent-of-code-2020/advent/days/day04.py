"""Advent of Code 2020, Day 4: {TITLE_HERE}."""

import logging
import pprint
import math, re
import itertools, functools

import attr, click
from functional import pseq as seq
import pandas as pd, numpy as np

import advent.inputs as inputs
import advent.parsing as parsing


AOC_DAY = 4


def passport(records):
    p = {}
    for r in records:
        p |= {k: v for k, v in (parsing.split_on(":") | r.split(" "))}
    return p


def part1():
    """Solution for part 1 of day 4."""
    logging.info("SOLVING DAY 4 PART 1")

    required_fields = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
    result = (
        inputs.records_of_day(AOC_DAY, multiline=True)
        .map(passport)
        .filter_not(lambda p: required_fields - p.keys())
        .len()
    )

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 202  # Valid result for my input


def part2():
    """Solution for part 2 of day 4."""
    logging.info("SOLVING DAY 4 PART 2")

    rules = {
        "byr": lambda x: 1920 <= int(x) <= 2002,
        "iyr": lambda x: 2010 <= int(x) <= 2020,
        "eyr": lambda x: 2020 <= int(x) <= 2030,
        "hgt": lambda x: (x[-2:] == "cm" and 150 <= int(x[:-2]) <= 193)
        or (x[-2:] == "in" and 59 <= int(x[:-2]) <= 76),
        "hcl": lambda x: re.match(r"^[#][0-9a-f]{6}$", x),
        "ecl": lambda x: x in set("amb blu brn gry grn hzl oth".split()),
        "pid": lambda x: re.match(r"^[0-9]{9}$", x),
    }

    def is_valid(p):
        try:
            return all(rule(p[k]) for k, rule in rules.items())
        except (KeyError, ValueError) as e:
            return False

    result = (
        inputs.records_of_day(AOC_DAY, multiline=True).map(passport).count(is_valid)
    )

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 137  # Valid result for my input
