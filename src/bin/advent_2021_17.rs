#![allow(dead_code, unused_variables)]
#![allow(incomplete_features)]
#![feature(
    never_type,
    generators,
    generator_trait,
    type_alias_impl_trait,
    let_chains
)]

use std::fmt::{Display, Formatter};
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

/// Expected output
type Answer = isize;

type Trajectory = impl Generator<Yield = (isize, isize), Return = !> + Unpin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TargetArea {
    x: (isize, isize),
    y: (isize, isize),
}

impl Display for TargetArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x={}..{}, y={}..{}",
            self.x.0, self.x.1, self.y.0, self.y.1
        )
    }
}

impl TargetArea {
    fn new(x: (isize, isize), y: (isize, isize)) -> Self {
        Self {
            x: (std::cmp::min(x.0, x.1), std::cmp::max(x.0, x.1)),
            y: (std::cmp::min(y.0, y.1), std::cmp::max(y.0, y.1)),
        }
    }

    /// Return wether a point is inside the area
    fn is_within(&self, x: isize, y: isize) -> bool {
        (self.x.0 <= x && x <= self.x.1) && (self.y.0 <= y && y <= self.y.1)
    }

    /// Quick evaluation of if there is a chance of collision eventually (verry lazy)
    fn is_unreachable(&self, x: isize, y: isize) -> bool {
        std::cmp::max(self.x.0.abs(), self.x.1.abs()) * 2 < x.abs()
            || y < -std::cmp::max(self.y.0.abs(), self.y.1.abs()) * 2
    }

    fn aim_with(&self, mut trajectory: &mut Trajectory) -> Option<Vec<(isize, isize)>> {
        let mut points = vec![];

        while let GeneratorState::Yielded((x, y)) = Pin::new(&mut trajectory).resume(()) {
            debug!(
                "{} -> {} {:?}",
                self,
                if self.is_within(x, y) { 'X' } else { ' ' },
                (x, y)
            );

            if self.is_unreachable(x, y) {
                return None;
            }

            points.push((x, y));
            if self.is_within(x, y) {
                break;
            }
        }

        Some(points)
    }
}

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    target_area: TargetArea,
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
        let (x, y) = input[0]["target area: ".len()..].split_once(',').unwrap();
        let (x1, x2) = x.trim()["x=".len()..].split_once("..").unwrap();
        let (y1, y2) = y.trim()["y=".len()..].split_once("..").unwrap();
        Self {
            target_area: TargetArea::new(
                (x1.parse().unwrap(), x2.parse().unwrap()),
                (y1.parse().unwrap(), y2.parse().unwrap()),
            ),
        }
    }
}

impl Puzzle {
    fn launch_pod(vx: isize, vy: isize) -> Trajectory {
        move || -> ! {
            let (mut x, mut vx) = (0, vx);
            let (mut y, mut vy) = (0, vy);
            loop {
                yield (x, y);
                x += vx;
                y += vy;

                vx += if vx == 0 { 0 } else { -vx / vx.abs() };
                vy += -1;
            }
        }
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        #[cfg(test)]
        debug!("Puzzle: {:?}", self);

        let (mut vx, mut vy) = (0, 1);
        return 0;
        unreachable!();
        loop {
            let mut trajectory = Self::launch_pod(0, 1);
            if let Some(trajectory) = self.target_area.aim_with(&mut trajectory) {
                break trajectory.iter().cloned().map(|(_, y)| y).max().unwrap();
            }
        }
    }

    fn part_two(&self) -> Self::Answer {
        #[cfg(test)]
        debug!("Puzzle: {:?}", self);

        todo!()
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 17)
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
    mod advent_2021_17 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver PART one
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
                target area: x=20..30, y=-10..-5
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 45);
        }

        // !SECTION

        // SECTION: PART TWO

        // !SECTION
    }
}
