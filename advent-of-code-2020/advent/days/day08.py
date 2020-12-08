"""Advent of Code 2020, Day 8: {TITLE_HERE}."""

import logging
import pprint
import math
import itertools, functools

import attr, click
import pandas as pd, numpy as np

import advent.inputs as inputs


AOC_DAY = 8
INPUT_FILE = inputs.file_of_day(AOC_DAY)


@attr.s(auto_attribs=True)
class Instruction:

    code: str
    pointer: int

    @classmethod
    def from_record(cls, record):
        code, pointer = record.split(" ")
        return cls(code=code, pointer=int(pointer))


def part1():
    """Solution for part 1 of day 8."""
    logging.info("SOLVING DAY 8 PART 1")

    runned = set()
    acc = 0
    pointer = 0
    instructions = inputs.records_of_day(AOC_DAY).map(Instruction.from_record).list()

    while pointer not in runned:
        i = instructions[pointer]
        runned.add(pointer)
        if i.code == "acc":
            acc += i.pointer
            pointer += 1
        elif i.code == "jmp":
            pointer += i.pointer
        else:
            pointer += 1

    result = acc

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 1766  # Valid result for my input


def part2():
    """Solution for part 2 of day 8."""
    logging.info("SOLVING DAY 8 PART 2")
    to_change = (
        inputs.records_of_day(AOC_DAY)
        .map(Instruction.from_record)
        .enumerate()
        .filter_not(lambda i: i[1].code == "acc")
        .list()
    )

    result = None
    for x, change in to_change:
        print(f"Changing: {x} -> {change}")

        if change.code == "nop":
            change = Instruction("jmp", change.pointer)
        else:
            change = Instruction("nop", 0)

        print(f"\tTo -> {change}")

        runned = set()
        acc = 0
        pointer = 0
        instructions = (
            inputs.records_of_day(AOC_DAY).map(Instruction.from_record).list()
        )
        instructions[x] = change

        while pointer not in runned:
            if pointer == len(instructions):
                print("Success")
                result = acc
                break
            i = instructions[pointer]
            runned.add(pointer)
            if i.code == "acc":
                acc += i.pointer
                pointer += 1
            elif i.code == "jmp":
                pointer += i.pointer
            elif i.code == "nop":
                pointer += 1
            else:
                pointer += 1
        else:
            print(f"Changing: {x} -> {change} did not work")

        if result is not None:
            break

    click.echo(click.style("RESULT >> ", fg="green") + pprint.pformat(result))
    assert result == 1639  # Valid result for my input
