use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = usize;

/// Expected output
type Answer = usize;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    lines: Vec<Input>,
}

/// Implement parsing a Puzzle struct from an input string
///
/// TODO(macro): Add derive macro for standard implementation of FromStr
impl FromStr for Puzzle
where
    Puzzle: AdventOfCode,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Standard parsing of input
        parsing::lines_of_inputs::<Puzzle>(s)
            // Creation using the From<Vec<Input>> input
            .map(|lines: Vec<Input>| -> Self { lines.into() })
    }
}

/// Collect a Vec<Input> input a structured Puzzle
impl From<Vec<Input>> for Puzzle {
    fn from(input: Vec<Input>) -> Self {
        Puzzle { lines: input }
    }
}

/// Solver implementation
impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        (0..self.lines.len() - 1)
            .filter(|i| self.lines[*i] < self.lines[*i + 1])
            .count()
    }

    fn part_two(&self) -> Self::Answer {
        let w = |i| self.lines[i..i + 3].iter().sum::<Answer>();
        (0..self.lines.len() - 3)
            .filter(|i| w(*i) < w(*i + 1))
            .count()
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 01)
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
    mod advent_2021_01 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};

        // SECTION: PART ONE

        /// Solver: Puzzle::part_one
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
                199
                200
                208
                210
                200
                207
                240
                269
                260
                263
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 7);
        }

        // !SECTION

        // SECTION: PART TWO

        /// Solver: Puzzle::part_two
        /// Test Case: sample_1
        #[test]
        fn part_two_sample_1() {
            let puzzle: Puzzle = r#"
                199
                200
                208
                210
                200
                207
                240
                269
                260
                263
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 5);
        }

        // !SECTION
    }
}
