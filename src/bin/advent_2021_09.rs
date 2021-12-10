use std::collections::HashSet;

use array2d::Array2D;
use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

/// Expected output
type Answer = usize;

/// Representation of a complete puzzle
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Puzzle {
    heightmap: Array2D<usize>,
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
        let rows = input
            .iter()
            .map(|s| {
                s.chars()
                    .map(|d| -> usize { d.to_digit(10).unwrap() as usize })
                    .collect_vec()
            })
            .collect_vec();
        Self {
            heightmap: Array2D::from_iter_row_major(
                rows.iter().flat_map(|r| r.iter()).map(|v| *v),
                rows.len(),
                rows.first().unwrap().len(),
            ),
        }
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        let num_rows = self.heightmap.num_rows() as isize;
        let num_columns = self.heightmap.num_columns() as isize;

        #[cfg(test)]
        debug!(
            "Map: \n{}",
            self.heightmap
                .as_rows()
                .iter()
                .map(|c| c.iter().join(" "))
                .join("\n")
        );

        let neighbors = |(x, y): (usize, usize)| -> Vec<(usize, usize)> {
            vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
                .iter()
                .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
                .filter(|(dx, dy)| dx >= &0 && dx < &num_rows && dy >= &0 && dy < &num_columns)
                .map(|(dx, dy)| (dx as usize, dy as usize))
                .collect_vec()
        };

        let low_points = (0..num_rows)
            .cartesian_product(0..num_columns)
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|(x, y)| {
                let h = self.heightmap.get(*x, *y).unwrap();
                neighbors((*x, *y))
                    .iter()
                    .all(|(dx, dy)| self.heightmap.get(*dx, *dy).unwrap() > h)
            })
            .collect_vec();

        low_points
            .iter()
            .map(|(x, y)| 1 + self.heightmap.get(*x, *y).unwrap())
            .sum()
    }

    #[allow(unused_variables)]
    fn part_two(&self) -> Self::Answer {
        #[cfg(test)]
        debug!("Puzzle: {:?}", self);

        let num_rows = self.heightmap.num_rows() as isize;
        let num_columns = self.heightmap.num_columns() as isize;

        #[cfg(test)]
        debug!(
            "Map: \n{}",
            self.heightmap
                .as_rows()
                .iter()
                .map(|c| c.iter().join(" "))
                .join("\n")
        );

        let neighbors = |(x, y): (usize, usize)| -> Vec<(usize, usize)> {
            vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
                .iter()
                .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
                .filter(|(dx, dy)| dx >= &0 && dx < &num_rows && dy >= &0 && dy < &num_columns)
                .map(|(dx, dy)| (dx as usize, dy as usize))
                .collect_vec()
        };

        let low_points = (0..num_rows)
            .cartesian_product(0..num_columns)
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|(x, y)| {
                let h = self.heightmap.get(*x, *y).unwrap();
                neighbors((*x, *y))
                    .iter()
                    .all(|(dx, dy)| self.heightmap.get(*dx, *dy).unwrap() > h)
            })
            .collect_vec();

        #[cfg(test)]
        debug!("Low points: {:?}", low_points);

        let mut basins: Vec<HashSet<(usize, usize)>> = vec![];
        low_points.iter().for_each(|(x0, y0)| {
            let mut basin = HashSet::new();

            let mut stack = vec![(*x0, *y0)];
            while !stack.is_empty() {
                let (x, y) = stack.pop().unwrap();
                let h = *self.heightmap.get(x, y).unwrap();
                basin.insert((x, y));

                neighbors((x, y))
                    .iter()
                    .filter(|(xi, yi)| {
                        basins.iter().all(|b| !b.contains(&(*xi, *yi)))
                            && !basin.contains(&(*xi, *yi))
                    })
                    .filter(|(xi, yi)| self.heightmap.get(*xi, *yi).unwrap() < &9)
                    .for_each(|(xi, yi)| stack.push((*xi, *yi)))
            }

            basins.push(basin);
        });

        #[cfg(test)]
        debug!(
            "Basins sizes: {:?}",
            basins.iter().map(|b| b.len()).sorted().rev().collect_vec()
        );

        basins
            .iter()
            .map(|b| b.len())
            .sorted()
            .rev()
            .take(3)
            .product()
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 09)
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
    mod advent_2021_09 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver PART one
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
                2199943210
                3987894921
                9856789892
                8767896789
                9899965678
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 15);
        }

        // !SECTION

        // SECTION: PART TWO

        /// Solver PART two
        /// Test Case: sample_1
        #[test]
        fn part_two_sample_1() {
            let puzzle: Puzzle = r#"
                2199943210
                3987894921
                9856789892
                8767896789
                9899965678
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 1134);
        }

        // !SECTION
    }
}
