use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

/// Expected output
type Answer = u128;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    lines: Vec<Input>,
}

/// Implement parsing a Puzzle struct from an input string
///
/// TODO(macro): Add derive macro for standard implementation of FromStr
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
        // NOTE: default implementation
        Self { lines: input }
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        let v = (0..self.lines.first().unwrap().len())
            .map(|i| {
                self.lines
                    .iter()
                    .map(|l| l.chars().nth(i).unwrap())
                    .join(&"")
            })
            .map(|s| {
                if s.matches("1").count() > s.matches("0").count() {
                    &"1"
                } else {
                    &"0"
                }
            })
            .join(&"");
        let gamma: u128 = u128::from_str_radix(v.as_str(), 2).unwrap();
        let epsilon: u128 = u128::from_str_radix(
            v.replace("0", "_")
                .replace("1", "0")
                .replace("_", "1")
                .as_str(),
            2,
        )
        .unwrap();
        gamma * epsilon
    }

    fn part_two(&self) -> Self::Answer {
        let mut oxy_rating = self.lines.clone();
        for i in 0..oxy_rating.first().unwrap().len() {
            let bit_crit = oxy_rating
                .iter()
                .map(|l| l.chars().nth(i).unwrap())
                .sorted()
                .group_by(|x| x.clone())
                .into_iter()
                .map(|(key, group)| (key, group.count()))
                .max_by_key(|&(_, count)| count)
                .unwrap()
                .0;
            oxy_rating = oxy_rating
                .iter()
                .filter(|l| l.chars().nth(i).unwrap().eq(&bit_crit))
                .map(|v| v.clone())
                .collect_vec();
            if oxy_rating.len() == 1 {
                break;
            }
        }
        let oxy_rating = Answer::from_str_radix(oxy_rating.first().unwrap().as_str(), 2).unwrap();

        let mut co2_rating = self.lines.clone();
        for i in 0..co2_rating.first().unwrap().len() {
            let bit_crit = co2_rating
                .iter()
                .map(|l| l.chars().nth(i).unwrap())
                .sorted()
                .group_by(|x| x.clone())
                .into_iter()
                .map(|(key, group)| (key, group.count()))
                .min_by_key(|&(_, count)| count)
                .unwrap()
                .0;
            co2_rating = co2_rating
                .iter()
                .filter(|l| l.chars().nth(i).unwrap().eq(&bit_crit))
                .map(|v| v.clone())
                .collect_vec();
            if co2_rating.len() == 1 {
                break;
            }
        }
        let co2_rating = Answer::from_str_radix(co2_rating.first().unwrap().as_str(), 2).unwrap();

        oxy_rating * co2_rating
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 03)
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
    mod advent_2021_03 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver: Puzzle::part_one
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
                00100
                11110
                10110
                10111
                10101
                01111
                00111
                11100
                10000
                11001
                00010
                01010
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 198);
        }

        // !SECTION

        // SECTION: PART TWO

        /// Solver: Puzzle::part_two
        /// Test Case: sample_1
        #[test]
        fn part_two_sample_1() {
            let puzzle: Puzzle = r#"
                00100
                11110
                10110
                10111
                10101
                01111
                00111
                11100
                10000
                11001
                00010
                01010
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 230);
        }

        // !SECTION
    }
}
