"""Commands for handling dev environment."""
from advent.const import DAYS_DIR

COMMON_IMPORTS = """
import logging
import pprint
import math
import itertools, functools

import attr, click
import pandas as pd, numpy as np

import advent.inputs as inputs
"""

COMMON_CODE = """
AOC_DAY = {day}
INPUT_FILE = inputs.file_of_day(AOC_DAY)
"""

PART_FUNCTION = """
def part{part}():
    \"""Solution for part {part} of day {day}.\"""
    logging.info("SOLVING DAY {day} PART {part}")

    result = "Hello, World!"

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    # assert result == XXXX  # Valid result for my input
"""


def create(day):
    """Command to create an environment for the next day."""
    py_file = DAYS_DIR / f"day{day:02d}.py"
    if py_file.exists():
        raise ValueError(f"File {py_file} already exists.")

    with py_file.open("w") as f:
        f.write(
            "\n".join(
                [
                    f'"""Advent of Code 2020, Day {day}: {{TITLE_HERE}}."""',
                    COMMON_IMPORTS,
                    COMMON_CODE.format(day=day),
                    PART_FUNCTION.format(day=day, part=1),
                    PART_FUNCTION.format(day=day, part=2),
                ]
            )
        )
