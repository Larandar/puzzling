use array2d::Array2D;

use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
#[derive(Debug, Clone, Copy)]
struct Input((usize, usize), (usize, usize));

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" -> ").context("parse a -> b")?;
        let (x1, y1) = left.split_once(",").context("parse x1,y1")?;
        let (x2, y2) = right.split_once(",").context("parse x2,y2")?;
        Ok(Input(
            (x1.parse()?, y1.parse()?),
            (x2.parse()?, y2.parse()?),
        ))
    }
}

/// Expected output
type Answer = usize;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    lines: Vec<Input>,
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
        Self { lines: input }
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        // First we need to get the size of the map
        let max_size = self
            .lines
            .iter()
            .flat_map(|Input((x1, y1), (x2, y2))| vec![x1, y1, x2, y2])
            .max()
            .expect("could not find max size for puzzle")
            + 1;

        // Let just make the map square
        let mut ocean_floor: Array2D<usize> = Array2D::filled_with(0, max_size, max_size);

        // Inclusive, both way, range util
        let range = |ref a: usize, ref b: usize| -> Vec<usize> {
            if a < b {
                (*a..=*b).collect_vec()
            } else {
                (*b..=*a).rev().collect_vec()
            }
        };

        // Each line is a pair of coords
        self.lines
            .iter()
            .flat_map(|Input((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    // Vertical line
                    range(*y1, *y2).iter().map(|y| (*x1, *y)).collect_vec()
                } else if y1 == y2 {
                    // Horizontal line
                    range(*x1, *x2).iter().map(|x| (*x, *y1)).collect_vec()
                } else {
                    // Diagonal line (not implemented in part_one)
                    vec![]
                }
            })
            // Mark each position
            .for_each(|(x, y)| ocean_floor[(x, y)] += 1);

        #[cfg(test)]
        ocean_floor
            .rows_iter()
            .for_each(|row| debug!("{:?}", row.collect_vec()));

        ocean_floor
            .elements_row_major_iter()
            .filter(|e| **e >= 2)
            .count()
    }

    fn part_two(&self) -> Self::Answer {
        // First we need to get the size of the map
        let max_size = self
            .lines
            .iter()
            .flat_map(|Input((x1, y1), (x2, y2))| vec![x1, y1, x2, y2])
            .max()
            .expect("could not find max size for puzzle")
            + 1;

        // Let just make the map square
        let mut ocean_floor = Array2D::filled_with(0, max_size, max_size);

        // Inclusive, both way, range util
        let range = |ref a: usize, ref b: usize| -> Vec<usize> {
            if a < b {
                (*a..=*b).collect_vec()
            } else {
                (*b..=*a).rev().collect_vec()
            }
        };

        // Each line is a pair of coords
        self.lines
            .iter()
            .flat_map(|Input((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    // Vertical line
                    range(*y1, *y2).iter().map(|y| (*x1, *y)).collect_vec()
                } else if y1 == y2 {
                    // Horizontal line
                    range(*x1, *x2).iter().map(|x| (*x, *y1)).collect_vec()
                } else {
                    // Diagonal line
                    range(*x1, *x2)
                        .iter()
                        .zip(range(*y1, *y2).iter())
                        .map(|(x, y)| (*x, *y))
                        .collect_vec()
                }
            })
            // Mark each position
            .for_each(|(x, y)| ocean_floor[(x, y)] += 1);

        #[cfg(test)]
        ocean_floor
            .rows_iter()
            .for_each(|row| debug!("{:?}", row.collect_vec()));

        ocean_floor
            .elements_row_major_iter()
            .filter(|e| **e >= 2)
            .count()
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 05)
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
    mod advent_2021_05 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver: Puzzle::part_one
        /// Test Case: sample
        #[test]
        fn part_one_sample() {
            let puzzle: Puzzle = r#"
                0,9 -> 5,9
                8,0 -> 0,8
                9,4 -> 3,4
                2,2 -> 2,1
                7,0 -> 7,4
                6,4 -> 2,0
                0,9 -> 2,9
                3,4 -> 1,4
                0,0 -> 8,8
                5,5 -> 8,2
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 5);
        }

        // !SECTION

        // SECTION: PART TWO

        /// Solver PART two
        /// Test Case: sample
        #[test]
        fn part_two_sample() {
            let puzzle: Puzzle = r#"
                0,9 -> 5,9
                8,0 -> 0,8
                9,4 -> 3,4
                2,2 -> 2,1
                7,0 -> 7,4
                6,4 -> 2,0
                0,9 -> 2,9
                3,4 -> 1,4
                0,0 -> 8,8
                5,5 -> 8,2
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 12);
        }

        // !SECTION
    }
}
