use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

/// Expected output
type Answer = usize;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    balls: Vec<usize>,
    boards: Vec<Bingo>,
}

#[derive(Debug, Clone, Copy)]
struct Bingo {
    board: [[Option<usize>; 5]; 5],
}

impl Bingo {
    pub fn mark(&self, number: usize) -> Bingo {
        Bingo {
            board: self
                .board
                .clone()
                .iter()
                .map(|l| {
                    l.clone()
                        .iter()
                        .map(|c| {
                            if c.is_some() && c.unwrap() == number {
                                None
                            } else {
                                c.clone()
                            }
                        })
                        .collect_vec()
                        .try_into()
                        .unwrap()
                })
                .collect_vec()
                .try_into()
                .unwrap(),
        }
    }

    pub fn is_winning(&self) -> bool {
        self.board.iter().any(|l| l.iter().all(Option::is_none))
            || (0..5).any(|i| self.board.iter().all(|l| l[i].is_none()))
    }

    pub fn points(&self) -> usize {
        self.board
            .iter()
            .map(|l| -> usize { l.iter().map(|c| c.unwrap_or(0)).sum() })
            .sum()
    }
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
        let mut input = input.into_iter();

        let balls = input
            .next()
            .unwrap()
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();

        let boards_sections = parsing::sections(input.skip(1).collect());

        let boards = boards_sections
            .into_iter()
            .map(|board| {
                board
                    .iter()
                    .map(|l| {
                        parsing::eager_split(l.clone())
                            .iter()
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect()
                    })
                    .collect()
            })
            .map(|board: Vec<Vec<usize>>| -> [[Option<usize>; 5]; 5] {
                board
                    .iter()
                    .map(|l| -> [Option<usize>; 5] {
                        l.iter().map(|v| Some(*v)).collect_vec().try_into().unwrap()
                    })
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .map(|board| Bingo { board })
            .collect_vec();

        Self { balls, boards }
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        let mut boards = self.boards.clone();
        let mut winning_ball = 0;
        for ball in self.balls.clone() {
            winning_ball = ball;
            boards = boards.iter().map(|b| b.mark(ball)).collect();
            if boards.iter().any(|b| b.is_winning()) {
                break;
            }
        }
        let winner = *boards.iter().filter(|b| b.is_winning()).next().unwrap();
        winner.points() * winning_ball
    }

    fn part_two(&self) -> Self::Answer {
        let mut boards = self
            .boards
            .clone()
            .iter()
            .enumerate()
            .map(|(i, b)| (i, *b))
            .collect_vec();
        let mut winning_ball = 0;
        for ball in self.balls.clone() {
            winning_ball = ball;
            boards = boards.iter().map(|(i, b)| (*i, b.mark(ball))).collect();

            if boards.len() == 1 && boards.last().unwrap().1.is_winning() {
                break;
            }

            boards = boards
                .clone()
                .iter()
                .map(|(i, b)| (*i, *b))
                .filter(|(_, b)| !b.is_winning())
                .collect();
        }
        println!("{:?}", boards);
        let last = boards.last().unwrap().1;
        last.points() * winning_ball
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 04)
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
    mod advent_2021_04 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver: Puzzle::part_one
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
                7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                22 13 17 11  0
                 8  2 23  4 24
                21  9 14 16  7
                 6 10  3 18  5
                 1 12 20 15 19

                 3 15  0  2 22
                 9 18 13 17  5
                19  8  7 25 23
                20 11 10 24  4
                14 21 16 12  6

                14 21 17 24  4
                10 16 15  9 19
                18  8 23 26 20
                22 11 13  6  5
                 2  0 12  3  7
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 4512);
        }

        // !SECTION

        // SECTION: PART TWO

        /// Solver: Puzzle::part_two
        /// Test Case: sample_1
        #[test]
        fn part_two_sample_1() {
            let puzzle: Puzzle = r#"
                7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                22 13 17 11  0
                 8  2 23  4 24
                21  9 14 16  7
                 6 10  3 18  5
                 1 12 20 15 19

                 3 15  0  2 22
                 9 18 13 17  5
                19  8  7 25 23
                20 11 10 24  4
                14 21 16 12  6

                14 21 17 24  4
                10 16 15  9 19
                18  8 23 26 20
                22 11 13  6  5
                 2  0 12  3  7
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 1924);
        }

        // !SECTION
    }
}
