#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

use itertools::MinMaxResult;
use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

/// Expected output
type Answer = usize;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    starting_polymer: String,
    rules: HashMap<(char, char), char>,
}

/// Implement parsing a Puzzle struct from an input string
impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Standard parsing of input
        parsing::lines::<Input>(s)
            // Creation using the From<Vec<Input>> input
            .map(|lines: Vec<Input>| -> Self { lines.into() })
    }
}

/// Collect a Vec<Input> input a structured Puzzle
impl From<Vec<Input>> for Puzzle {
    fn from(input: Vec<Input>) -> Self {
        let starting_polymer = input[0].clone();
        let rules = input[2..]
            .iter()
            .map(|line| {
                let mut chars = line.chars();
                let a = chars.next().unwrap();
                let b = chars.next().unwrap();
                let c = chars.skip(4).next().unwrap();
                ((a, b), c)
            })
            .collect();
        Self {
            starting_polymer,
            rules,
        }
    }
}

impl Puzzle {
    fn polymerize(&self, polymer: String) -> String {
        let mut start = polymer.chars().nth(0).unwrap().to_string().clone();
        let polymer: String = polymer
            .chars()
            .collect_vec()
            .par_windows(2)
            .map(|w| (w[0], w[1]))
            .map(|t| (*self.rules.get(&t).unwrap(), t.1))
            .fold(
                || String::new(),
                |mut acc, (a, b)| {
                    acc.push(a);
                    acc.push(b);
                    acc
                },
            )
            .collect();

        start.push_str(polymer.as_str());
        start
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        #[cfg(test)]
        debug!("Puzzle: {:?}", self);

        let polymer = self.starting_polymer.clone();

        if let MinMaxResult::MinMax(min, max) = (0..10)
            .fold(polymer, |polymer, _| self.polymerize(polymer))
            .chars()
            .counts()
            .into_iter()
            .minmax_by_key(|&(_, count)| count)
        {
            debug!("Minmax: {:?} > {:?}", max, min);
            max.1 - min.1
        } else {
            0
        }
    }

    fn part_two(&self) -> Self::Answer {
        #[cfg(test)]
        debug!("Puzzle: {:?}", self);

        let start = self.starting_polymer.chars().nth(0).unwrap();
        let polymer: HashMap<(char, char), usize> = self
            .starting_polymer
            .clone()
            .chars()
            .tuple_windows()
            .counts();

        let polymer = (0..40).fold(polymer, |previous, _| {
            let mut polymer = HashMap::new();

            previous
                .into_iter()
                .flat_map(|((a, b), count)| {
                    let insert = *self.rules.get(&(a, b)).unwrap();
                    vec![((a, insert), count), ((insert, b), count)]
                })
                .for_each(|(key, count)| {
                    *polymer.entry(key).or_insert(0) += count;
                });

            polymer
        });
        debug!("Polymer: {:?}", polymer);

        let elements = polymer.into_iter().map(|((_, e), c)| (e, c)).fold(
            vec![(start, 1)]
                .iter()
                .cloned()
                .collect::<HashMap<char, usize>>(),
            |mut acc, (e, count)| {
                *acc.entry(e).or_insert(0) += count;
                acc
            },
        );

        debug!("Elements: {:?}", elements);
        match elements.into_iter().minmax_by_key(|(_, count)| *count) {
            MinMaxResult::NoElements => todo!(),
            MinMaxResult::OneElement(_) => todo!(),
            MinMaxResult::MinMax((_, a), (_, b)) => b - a,
        }
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 14)
        .expect("impossible to fetch daily challenge")
        .parse()
        .expect("failed to parse daily challenge");

    println!("Part 1: {}", puzzle.part_one());
    println!("Part 2: {}", puzzle.part_two());
}

#[cfg(test)]
#[cfg(feature = "advent_2021")]
#[cfg(feature = "completed")]
mod tests {

    #[allow(unused_imports)]
    mod advent_2021_14 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver PART one
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
            NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 1588);
        }

        // !SECTION

        // SECTION: PART TWO

        // !SECTION
    }
}
