#![allow(dead_code, unused_variables)]
use array2d::Array2D;
use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

/// Expected output
type Answer = usize;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    map: Array2D<usize>,
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
            map: Array2D::from_rows(
                &input
                    .iter()
                    .map(|l| {
                        l.chars()
                            .map(|c| c.to_string().parse::<usize>().unwrap())
                            .collect()
                    })
                    .collect::<Vec<Vec<usize>>>(),
            ),
        }
    }
}

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point {
    position: (usize, usize),
    path: Vec<(usize, usize)>,
    danger: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        Reverse(self.danger)
            .cmp(&Reverse(other.danger))
            .then_with(|| Reverse(self.path.len()).cmp(&Reverse(other.path.len())))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Puzzle {
    fn show_path(map: Array2D<usize>, path: Vec<(usize, usize)>) -> String {
        let path: HashSet<_> = HashSet::from_iter(path.iter().cloned());
        let repr = map
            .as_rows()
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, &c)| {
                        if path.contains(&(x, y)) {
                            format!("<b>{}</b>", c).to_string()
                        } else {
                            c.to_string()
                        }
                    })
                    .join("")
            })
            .join("\n");
        paris::formatter::colorize_string(repr)
    }

    // Dijkstra's shortest path algorithm.

    // Start at `start` and use `dist` to track the current shortest distance
    // to each node. This implementation isn't memory-efficient as it may leave duplicate
    // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
    // for a simpler implementation.
    fn safest_path(
        map: Array2D<usize>,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let mut heap = BinaryHeap::new();
        let mut dist = Array2D::filled_with(usize::MAX, map.num_rows(), map.num_columns());

        let num_rows = map.num_rows();
        let num_columns = map.num_columns();
        let neighbours = |(x, y): (usize, usize)| {
            vec![
                (x as isize - 1, y as isize),
                (x as isize + 1, y as isize),
                (x as isize, y as isize - 1),
                (x as isize, y as isize + 1),
            ]
            .iter()
            .filter(|&&(x, y)| {
                x >= 0 && y >= 0 && x < num_rows as isize && y < num_columns as isize
            })
            .map(|&(x, y)| (x as usize, y as usize))
            .collect::<Vec<(usize, usize)>>()
        };

        // We're at `start`, with a zero cost
        heap.push(Point {
            path: vec![start],
            position: start,
            danger: 0,
        });

        let mut safest_yet = None;

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(
            ref point @ Point {
                danger,
                position,
                ref path,
            },
        ) = heap.pop()
        {
            // Alternatively we could have continued to find all shortest paths
            if position == goal {
                if let Some(Point {
                    danger: other,
                    position: _,
                    path: _,
                }) = safest_yet
                {
                    if danger < other {
                        safest_yet = Some(point.clone());
                    }
                } else {
                    safest_yet = Some(point.clone());
                }
                continue;
            }

            // Important as we may have already found a better way
            if danger > *dist.get(position.0, position.1).unwrap() {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for edge in neighbours(position) {
                let next = Point {
                    danger: danger + *map.get(edge.0, edge.1).unwrap(),
                    position: edge,
                    path: path.iter().cloned().chain(std::iter::once(edge)).collect(),
                };

                // If so, add it to the frontier and continue
                if next.danger < *dist.get(edge.0, edge.1).unwrap() {
                    dist.set(edge.0, edge.1, next.danger).unwrap();
                    // Relaxation, we have now found a better way
                    heap.push(next);
                }
            }
        }

        // Goal not reachable
        safest_yet.map(|p| p.path)
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        let path = Puzzle::safest_path(
            self.map.clone(),
            (0, 0),
            (self.map.num_rows() - 1, self.map.num_columns() - 1),
        )
        .unwrap();

        path.iter()
            .skip(1)
            .map(|&(x, y)| self.map.get(x, y).unwrap())
            .sum::<usize>()
    }

    fn part_two(&self) -> Self::Answer {
        let num_rows = self.map.num_rows();
        let num_columns = self.map.num_columns();

        let clamp = |v| if v % 9 > 0 { v % 9 } else { 9 };

        let larger_map = (0..5)
            .flat_map(|i| {
                self.map
                    .as_rows()
                    .iter()
                    .cloned()
                    .map(|row| {
                        (0..5)
                            .flat_map(|j| row.iter().cloned().map(move |c| clamp(c + i + j)))
                            .collect_vec()
                    })
                    .collect_vec()
            })
            .collect_vec();
        let larger_map = Array2D::from_rows(&larger_map[..]);

        let path = Puzzle::safest_path(
            larger_map.clone(),
            (0, 0),
            (larger_map.num_rows() - 1, larger_map.num_columns() - 1),
        )
        .unwrap();

        #[cfg(test)]
        debug!("\n{}", Puzzle::show_path(larger_map.clone(), path.clone()));

        path.iter()
            .skip(1)
            .map(|&(x, y)| larger_map.get(x, y).unwrap())
            .sum::<usize>()
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 15)
        .expect("impossible to fetch daily challenge")
        .parse()
        .expect("failed to parse daily challenge");

    let timeit = Instant::now();
    let part_one = puzzle.part_one();
    println!(
        "Part 1 ({:.04}s): {}",
        timeit.elapsed().as_secs_f64(),
        part_one,
    );

    let timeit = Instant::now();
    let part_two = puzzle.part_two();
    println!(
        "Part 2 ({:.04}s): {}",
        timeit.elapsed().as_secs_f64(),
        part_two,
    );
}

#[cfg(test)]
#[cfg(feature = "advent_2021")]
#[cfg(feature = "completed")]
mod tests {

    #[allow(unused_imports)]
    mod advent_2021_15 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver PART one
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
                1163751742
                1381373672
                2136511328
                3694931569
                7463417111
                1319128137
                1359912421
                3125421639
                1293138521
                2311944581
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 40);
        }

        // !SECTION

        // SECTION: PART TWO

        /// Solver PART one
        /// Test Case: sample_1
        #[test]
        fn part_two_sample_1() {
            let puzzle: Puzzle = r#"
                1163751742
                1381373672
                2136511328
                3694931569
                7463417111
                1319128137
                1359912421
                3125421639
                1293138521
                2311944581
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 315);
        }

        // !SECTION
    }
}
