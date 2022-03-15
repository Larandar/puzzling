use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

/// Expected output
type Answer = usize;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    crabs: Vec<usize>,
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
        Self {
            crabs: input
                .iter()
                .flat_map(|line| line.split(','))
                .map(|line| {
                    line.parse::<usize>()
                        .context("parsing crab heights")
                        .unwrap()
                })
                .collect(),
        }
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        let (min, max) = match self.crabs.iter().minmax() {
            itertools::MinMaxResult::NoElements => todo!(),
            itertools::MinMaxResult::OneElement(a) => (0, *a),
            itertools::MinMaxResult::MinMax(a, b) => (*a, *b),
        };

        (min..=max)
            .map(|pos| {
                let cost = self
                    .crabs
                    .iter()
                    .map(|c| ((*c as i64) - (pos as i64)).abs() as usize)
                    .sum();
                (pos, cost)
            })
            .min_by_key(|(_, cost)| *cost)
            .unwrap_or((0, 0))
            .1
    }

    fn part_two(&self) -> Self::Answer {
        let (min, max) = match self.crabs.iter().minmax() {
            itertools::MinMaxResult::NoElements => unreachable!(),
            itertools::MinMaxResult::OneElement(_) => unreachable!(),
            itertools::MinMaxResult::MinMax(a, b) => (*a, *b),
        };

        (min..=max)
            .map(|pos| {
                let cost = self
                    .crabs
                    .iter()
                    .map(|c| ((*c as i64) - (pos as i64)).abs() as usize)
                    .map(|c| -> usize { c * (c + 1) / 2 })
                    .sum();
                (pos, cost)
            })
            .min_by_key(|(_, cost)| *cost)
            .unwrap_or((0, 0))
            .1
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 07)
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
    mod advent_2021_07 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE
        /// Solver PART one
        /// Test Case: NAME_
        #[test]
        fn part_one_sample_name() {
            let puzzle: Puzzle = r#"16,1,2,0,4,2,7,1,2,14"#
                .parse()
                .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 37);
        }

        // !SECTION

        // SECTION: PART TWO

        /// Solver PART one
        /// Test Case: NAME_
        #[test]
        fn part_two_sample_name() {
            let puzzle: Puzzle = r#"16,1,2,0,4,2,7,1,2,14"#
                .parse()
                .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 168);
        }

        // !SECTION
    }
}
