"""Advent of Code 2020."""

import pprint
import logging
import importlib
import click
import advent.dev_env


@click.command()
@click.argument("day", type=int)
@click.argument("part", type=int)
def run_aoc(day, part):
    """Run AoC day corresponding function."""
    try:
        module = importlib.import_module(f"advent.days.day{day:02d}")
        command = getattr(module, f"part{part}")
    except (ImportError, AttributeError):
        click.secho(
            f"Day not implemented advent.days.day{day:02d}.part{part}",
            fg="red",
        )
        if click.confirm("Do you want to create it?"):
            advent.dev_env.create(day)
        raise click.Abort

    command()


if __name__ == "__main__":
    run_aoc()
