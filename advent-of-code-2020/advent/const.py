"""All the constants for resolving common AoC functions."""

from pathlib import Path

AOC_YEAR = 2020
AOC_INPUT_URL = f"https://adventofcode.com/{AOC_YEAR}/day/{{day}}/input"

ROOT_DIR = Path(__file__).parent.parent.absolute()

INPUT_DIR = ROOT_DIR / "inputs"
INPUT_DIR.mkdir(exist_ok=True, parents=True)

DAYS_DIR = ROOT_DIR / "advent" / "days"
DAYS_DIR.mkdir(exist_ok=True, parents=True)
