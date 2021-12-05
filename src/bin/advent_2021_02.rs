use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
#[derive(Debug, Clone)]
enum Input {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((o, d)) = s.split(" ").tuples().next() {
            let d = d.parse()?;
            match o.to_lowercase().as_str() {
                "forward" => Ok(Input::Forward(d)),
                "down" => Ok(Input::Down(d)),
                "up" => Ok(Input::Up(d)),
                _ => Err(anyhow!("unknow order: {}", s)),
            }
        } else {
            Err(anyhow!("unknow order: {}", s))
        }
    }
}

/// Expected output
type Answer = isize;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    orders: Vec<Input>,
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
        Puzzle { orders: input }
    }
}

/// Solver implementation
impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        let (x, depth) = self.orders.iter().fold((0, 0), |(x, depth), o| match o {
            Input::Forward(v) => (x + v, depth),
            Input::Down(v) => (x, depth + v),
            Input::Up(v) => (x, depth - v),
        });
        x * depth
    }

    fn part_two(&self) -> Self::Answer {
        let (_, x, depth) = self
            .orders
            .iter()
            .fold((0, 0, 0), |(aim, x, depth), o| match o {
                Input::Forward(v) => (aim, x + v, depth + aim * v),
                Input::Down(v) => (aim - v, x, depth),
                Input::Up(v) => (aim + v, x, depth),
            });
        -x * depth
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 02)
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
        fn part_one_sample() {
            let puzzle: Puzzle = r#"
                forward 5
                down 5
                forward 8
                up 3
                down 8
                forward 2
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 150);
        }

        // !SECTION

        // SECTION: PART TWO

        /// Solver: Puzzle::part_two
        /// Test Case: sample_1
        #[test]
        fn part_two_sample() {
            let puzzle: Puzzle = r#"
                forward 5
                down 5
                forward 8
                up 3
                down 8
                forward 2
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 900);
        }

        // !SECTION
    }
}
