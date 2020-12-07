"""Advent of Code 2020, Day 1: {TITLE_HERE}."""

import logging
import pprint

import click
import pandas as pd

import advent.inputs as inputs


AOC_DAY = 1
INPUT_FILE = inputs.file_of_day(AOC_DAY, ext="csv")

REPORT_DF = pd.read_csv(INPUT_FILE, names=["expenses"])
EXPENSES = REPORT_DF["expenses"]


def smart_report(nb_count, values=EXPENSES):
    if nb_count <= 0:
        raise ValueError
    if nb_count == 1:
        yield from (([i], [v], v, v) for i, v in enumerate(values))
        return
    for i, v in enumerate(values):
        yield from (
            ([i, *idx], [v, *vals], s + v, p * v)
            for idx, vals, s, p in smart_report(nb_count - 1, values[i:])
        )


def part1():
    """Solution for part 1 of day 1."""
    logging.info("SOLVING DAY 1 PART 1")

    matching_reports = ((vals, p) for _, vals, s, p in smart_report(2) if s == 2020)
    vals, result = next(matching_reports)

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat((vals, result)))
    assert result == 145875  # Valid result for my input


def part2():
    """Solution for part 2 of day 1."""
    logging.info("SOLVING DAY 1 PART 2")

    matching_reports = ((vals, p) for _, vals, s, p in smart_report(3) if s == 2020)
    vals, result = next(matching_reports)

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat((vals, result)))
    assert result == 69596112  # Valid result for my input
