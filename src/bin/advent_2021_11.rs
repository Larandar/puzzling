use array2d::Array2D;
use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

/// Expected output
type Answer = usize;

/// Representation of a complete puzzle
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum Octopus {
    Charging(u8),
    Flashing,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Puzzle {
    cavern: Array2D<Octopus>,
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
            cavern: Array2D::from_iter_row_major(
                input
                    .iter()
                    .flat_map(|l| l.trim().chars())
                    .map(|o| Octopus::Charging(o.to_string().parse().unwrap())),
                input.len(),
                input.first().unwrap().len(),
            ),
        }
    }
}

#[allow(dead_code)]
fn tick(cavern: Array2D<Octopus>) -> (Array2D<Octopus>, usize) {
    let num_rows = cavern.num_rows();
    let num_columns = cavern.num_columns();
    let neighbors = |i: usize| {
        let (x, y) = ((i / num_rows) as isize, (i % num_columns) as isize);
        [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
        .iter()
        .filter(|(x, y)| {
            (&0 <= x && x < &(num_columns as isize)) && (&0 <= y && y < &(num_columns as isize))
        })
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect_vec()
    };

    #[cfg(test)]
    debug!("Starting with cavern: \n{}", repr(cavern.clone()));

    let mut cavern = Array2D::from_iter_row_major(
        cavern.elements_row_major_iter().map(|o| match o {
            Octopus::Charging(x) => Octopus::Charging(x + 1),
            Octopus::Flashing => Octopus::Flashing,
        }),
        num_columns,
        num_rows,
    );

    while cavern
        .elements_row_major_iter()
        .any(|o| matches!(o, Octopus::Charging(x) if x > &9))
    {
        cavern
            .clone()
            .elements_row_major_iter()
            .enumerate()
            .filter(|(_, o)| matches!(o, Octopus::Charging(x) if x > &9))
            .map(|(i, _)| i)
            // Flashing octopi
            .for_each(|i| {
                cavern
                    .set(i / num_rows, i % num_columns, Octopus::Flashing)
                    .unwrap();
                neighbors(i).iter().for_each(|(x, y)| {
                    cavern.get_mut(*x, *y).map(|x| {
                        if let Octopus::Charging(v) = x {
                            *v += 1
                        }
                    });
                });
            });
    }

    let flashes = cavern
        .clone()
        .elements_row_major_iter()
        .enumerate()
        .filter(|(_, o)| matches!(o, Octopus::Flashing))
        .map(|(i, _)| i)
        .map(|i| {
            cavern
                .set(i / num_rows, i % num_columns, Octopus::Charging(0))
                .unwrap()
        })
        .count();

    #[cfg(test)]
    debug!("Flashes: {} => \n{}", flashes, repr(cavern.clone()));
    (cavern, flashes)
}

#[allow(dead_code)]
fn repr(cavern: Array2D<Octopus>) -> String {
    cavern
        .as_rows()
        .iter()
        .map(|row| {
            row.iter()
                .map(|o| match o {
                    &Octopus::Charging(x) => format!("{}", x),
                    &Octopus::Flashing => "F".to_string(),
                })
                .join("")
        })
        .join("\n")
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        #[cfg(test)]
        debug!("Puzzle: {:?}", self);

        (1..=100)
            .fold((self.cavern.clone(), 0), |(cavern, past_flashes), _| {
                let (cavern, flashes) = tick(cavern);
                (cavern, past_flashes + flashes)
            })
            .1
    }

    fn part_two(&self) -> Self::Answer {
        #[cfg(test)]
        debug!("Puzzle: {:?}", self);
        let mut cavern = self.cavern.clone();
        let mut step = 0;
        loop {
            let (next, _) = tick(cavern);
            step += 1;
            if next
                .elements_row_major_iter()
                .tuple_windows()
                .all(|(prev, item)| prev == item)
            {
                break step;
            }
            cavern = next;
        }
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 11)
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
    mod advent_2021_11 {
        use crate::{tick, Octopus, Puzzle};
        use array2d::Array2D;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver PART one
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
                5483143223
                2745854711
                5264556173
                6141336146
                6357385478
                4167524645
                2176841721
                6882881134
                4846848554
                5283751526
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 1656);
        }

        // !SECTION

        // SECTION: PART TWO

        // !SECTION
    }
}
