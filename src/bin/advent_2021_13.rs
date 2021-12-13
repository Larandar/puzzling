use std::collections::HashSet;

use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

/// Expected output
type Answer = usize;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Puzzle {
    holes: HashSet<Hole>,
    folds: Vec<Fold>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hole(isize, isize);

impl Hole {
    fn get(&self, axis: char) -> isize {
        match axis {
            'x' => self.0,
            'y' => self.1,
            _ => unreachable!(),
        }
    }

    fn fold(&self, fold: Fold) -> Option<Self> {
        if self.get(fold.0) == fold.1 {
            return None;
        }

        let pos = self.get(fold.0);
        let pos = if pos < fold.1 { pos } else { 2 * fold.1 - pos };

        match fold.0 {
            'x' => Some(Hole(pos, self.1)),
            'y' => Some(Hole(self.0, pos)),
            _ => unreachable!(),
        }
    }
}

impl FromStr for Hole {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").context("spliting hole location")?;
        Ok(Hole(x.parse()?, y.parse()?))
    }
}

#[derive(Debug, Clone, Copy)]
struct Fold(char, isize);

impl FromStr for Fold {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, pos) = s[11..]
            .split_once("=")
            .context("parsing folding instruction")?;
        let axis = axis.chars().nth(0).context("parsing folding axis")?;
        if !matches!(axis, 'x' | 'y') {
            return Err(anyhow!("unknown axis {}", axis));
        }
        Ok(Fold(axis, pos.parse().context("parsing folding position")?))
    }
}

/// Collect a Vec<Input> input a structured Puzzle
impl From<Vec<Input>> for Puzzle {
    fn from(input: Vec<Input>) -> Self {
        let holes = input
            .iter()
            .take_while(|line| line.len() > 0)
            .map(|l| l.parse::<Hole>().unwrap())
            .collect();

        let folds = input
            .iter()
            .skip_while(|line| line.len() > 0)
            .skip(1)
            .map(|line| {
                let mut chars = line[11..].chars();
                let axis = chars.next().unwrap();
                let pos = chars.skip(1).collect::<String>().parse::<isize>().unwrap();
                Fold(axis, pos)
            })
            .collect();

        Self { holes, folds }
    }
}

impl Puzzle {
    fn fold(&self) -> Result<Self> {
        let fold = self
            .folds
            .first()
            .context("no folding instruction")?
            .clone();

        let holes = self
            .holes
            .iter()
            .cloned()
            .filter_map(|h| h.fold(fold))
            .collect();

        let folds = self.folds.iter().skip(1).cloned().collect_vec();
        Ok(Self { holes, folds })
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        #[cfg(test)]
        debug!("Puzzle: {:?}", self);

        self.fold().unwrap().holes.len()
    }

    fn part_two(&self) -> Self::Answer {
        #[cfg(test)]
        debug!("Puzzle: {:?}", self);

        let mut paper = self.clone();
        while !paper.folds.is_empty() {
            paper = paper.fold().unwrap()
        }

        let num_rows = paper.holes.iter().map(|h| h.get('y')).max().unwrap() + 1;
        let num_columns = paper.holes.iter().map(|h| h.get('x')).max().unwrap() + 1;

        let mut grid = vec![vec![' '; num_columns as usize]; num_rows as usize];
        paper
            .holes
            .iter()
            .for_each(|Hole(x, y)| grid[*y as usize][*x as usize] = '#');

        grid.iter()
            .for_each(|row| println!("{}", row.iter().collect::<String>()));
        0
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 13)
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
    mod advent_2021_13 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver PART one
        /// Test Case: sample_1
        #[test]
        #[ignore]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0

            fold along y=7
            fold along x=5
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 17);
        }

        #[test]
        fn part_one_sample_2() {
            let puzzle: Puzzle = r#"
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0

            fold along x=5
            fold along y=7
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 17);
        }

        // !SECTION

        // SECTION: PART TWO

        // !SECTION
    }
}
