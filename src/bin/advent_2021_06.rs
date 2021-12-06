use std::collections::HashMap;

use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

/// Expected output
type Answer = usize;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    fishes: Vec<usize>,
}

/// Implement parsing a Puzzle struct from an input string
impl FromStr for Puzzle
where
    Puzzle: AdventOfCode,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Standard parsing of input
        parsing::lines_of_inputs::<Puzzle>(s)?
            .iter()
            .flat_map(|line| line.split(','))
            .map(|line| line.parse::<usize>().context("parsing fish age"))
            .collect::<Result<Vec<usize>>>()
            // Creation using the From<Vec<Input>> input
            .map(|fishes: Vec<usize>| -> Self { Self { fishes } })
    }
}

/// Collect a Vec<Input> input a structured Puzzle
impl From<Vec<Input>> for Puzzle {
    fn from(input: Vec<Input>) -> Self {
        panic!("{:?}", input);
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        (0..80)
            .fold(self.fishes.clone(), |fishes, _| {
                fishes
                    .iter()
                    .flat_map(|fish| match fish {
                        0 => vec![6, 8],
                        x => vec![x - 1],
                    })
                    .collect::<Vec<_>>()
            })
            .len()
    }

    fn part_two(&self) -> Self::Answer {
        let fishes_count: HashMap<usize, usize> = self.fishes.clone().iter().map(|v| *v).counts();

        let fishes = (0..256).fold(fishes_count, |fishes, _| {
            let fishes = fishes
                .iter()
                .flat_map(|(fish_age, count)| match fish_age {
                    0 => vec![(6, 1 * count), (8, 1 * count)],
                    _ => vec![(fish_age - 1, 1 * count)],
                })
                .group_by(|(age, _)| *age);

            let mut next_generation: HashMap<usize, usize> = HashMap::new();
            for (age, group) in &fishes {
                let total: usize = group.map(|(_, count)| count).sum();
                next_generation.insert(age, next_generation.get(&age).unwrap_or(&0) + total);
            }

            next_generation
        });
        fishes.iter().map(|(_, count)| count).sum()
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 06)
        .expect("impossible to fetch daily challenge")
        .parse()
        .expect("failed to parse daily challenge");

    println!("Part 1: {}", puzzle.part_one());
    println!("Part 2: {}", puzzle.part_two());
}

#[cfg(test)]
#[cfg(feature = "advent_2021")]
// #[cfg(feature = "completed")]
mod tests {

    #[allow(unused_imports)]
    mod advent_2021_06 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE
        /// Solver PART one
        /// Test Case: 1_3,4,3,1,2
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"3,4,3,1,2"#.parse().expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 5934);
        }

        // !SECTION

        // SECTION: PART TWO
        /// Solver PART two
        /// Test Case: 1_input
        #[test]
        fn part_two_sample_1() {
            let puzzle: Puzzle = r#"3,4,3,1,2"#.parse().expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 26984457539);
        }

        // !SECTION
    }
}
