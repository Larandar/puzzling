"""Input function for AoC."""
from functional.pipeline import Sequence
from advent.const import AOC_INPUT_URL, INPUT_DIR, ROOT_DIR
import requests
import logging
from contextlib import contextmanager
import os
from pathlib import Path

from functional import pseq, seq


def input_of_day(day):
    """Return the content of the day."""
    if "AOC_SESSION" not in os.environ:
        raise RuntimeError("AOC_SESSION environment variable need to be set.")

    return requests.get(
        AOC_INPUT_URL.format(day=day),
        cookies=dict(session=os.environ["AOC_SESSION"]),
    ).content


def file_of_day(day, ext="txt"):
    """Return a file path on disk of the input."""
    filepath = INPUT_DIR / f"{day:02d}.{ext}"

    if not filepath.exists():
        with filepath.open("wb") as f:
            f.write(input_of_day(day))

        logging.info(
            "File was not previously loaded, saved at: %s",
            filepath.relative_to(ROOT_DIR),
        )

    return filepath


def lines_of_day(day, split=False):
    """Yield each line (striped) from the input of the day."""
    with file_of_day(day).open() as f:
        lines = f.readlines()

    # We only want to remove end of files to not change the input
    lines = map(lambda l: l.strip("\r\n"), lines)

    if split is not False:
        lines = map(lambda l: l.split(split), lines)

    yield from lines


def matrix_of_day(day, mapping={}):
    """Return a list (lines) of list (columns)."""
    return [[mapping.get(c, c) for c in l] for l in lines_of_day(day)]


def _grouped_lines(lines):
    """Group lines seperated by empty lines."""
    group = []
    for l in lines:
        if not l and group:
            yield group
            group = []
        else:
            group.append(l)

    # Don't forget to yield the last one, common error
    if group:
        yield group


def records_of_day(day, multiline=False) -> Sequence:
    """Return a sequence of records."""
    lines = seq.open(file_of_day(day)).map(lambda l: l.strip())

    if multiline:
        lines = _grouped_lines(lines)

    return pseq(lines)
