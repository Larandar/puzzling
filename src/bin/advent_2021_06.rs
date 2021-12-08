use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

/// Expected output
type Answer = u128;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    fish_population: [u128; 9],
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
            fish_population: input
                .iter()
                .flat_map(|line| line.split(','))
                .map(|line| line.parse::<usize>().context("parsing fish age").unwrap())
                .fold([0; 9], |acc, fish| {
                    acc.iter()
                        .enumerate()
                        .map(|(age, t)| -> u128 {
                            match fish == age {
                                true => *t + 1,
                                false => *t,
                            }
                        })
                        .collect_vec()
                        .try_into()
                        .unwrap()
                }),
        }
    }
}

/// The population is better expressed as a array where each element is
/// the number of fish and the index the age of those fishes.
fn advance_age(population: [u128; 9]) -> [u128; 9] {
    [
        population[1],
        population[2],
        population[3],
        population[4],
        population[5],
        population[6],
        population[7] + population[0],
        population[8],
        population[0],
    ]
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        (1..=80)
            .fold(self.fish_population, |pop, _| advance_age(pop))
            .iter()
            .sum()
    }

    fn part_two(&self) -> Self::Answer {
        (1..=256)
            .fold(self.fish_population, |pop, _| advance_age(pop))
            .iter()
            .sum()
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
