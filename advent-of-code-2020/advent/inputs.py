"""Input function for AoC."""
from advent.const import AOC_INPUT_URL, INPUT_DIR, ROOT_DIR
import requests
import logging
from contextlib import contextmanager
import os
from pathlib import Path

from requests.models import cookiejar_from_dict


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


def lines_of_day(day):
    """Yield each line (striped) from the input of the day."""
    with file_of_day(day).open() as f:
        yield from (l.strip() for l in f.readlines())
