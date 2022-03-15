use bimap::BiMap;
use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;

/// Input type for each line
type Input = String;

#[derive(Debug, Clone)]
struct Instruction(Vec<String>, Vec<String>);

/// Expected output
type Answer = usize;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    lines: Vec<Instruction>,
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
    fn from(lines: Vec<Input>) -> Self {
        let into_instructions = |s: String| -> Vec<String> {
            s.trim()
                .split_terminator(" ")
                .map(|s| s.chars().sorted().join("").to_string())
                .collect_vec()
        };

        Self {
            lines: lines
                .iter()
                .map(|line| line.split_once("|").unwrap())
                .map(|(left, right)| (left.to_string(), right.to_string()))
                .map(|(left, right)| Instruction(into_instructions(left), into_instructions(right)))
                .collect_vec(),
        }
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        self.lines
            .iter()
            .flat_map(|i| i.1.iter())
            .filter_map(|d| match d.len() {
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                7 => Some(8),
                _ => None,
            })
            .count()
    }

    fn part_two(&self) -> Self::Answer {
        let first_guess = |d: &String| match d.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        };

        let shared_segments = |a: &String, b: &String| a.chars().filter(|c| b.contains(*c)).count();

        let compute = |Instruction(digits, output)| {
            let mut roseta = BiMap::new();
            digits.iter().for_each(|d| {
                if let Some(i) = first_guess(d) {
                    roseta.insert(i, d.clone());
                }
            });

            let one = &roseta.get_by_left(&1).unwrap().clone();
            let four = &roseta.get_by_left(&4).unwrap().clone();
            let seven = &roseta.get_by_left(&7).unwrap().clone();

            let nine = *digits
                .iter()
                .filter(|d| {
                    d.len() == 6
                        && shared_segments(*d, four) == 4
                        && shared_segments(*d, seven) == 3
                })
                .collect_vec()
                .first()
                .unwrap();
            roseta.insert(9, nine.clone());

            let zero = *digits
                .iter()
                .filter(|d| d.len() == 6 && (*d).ne(nine) && shared_segments(*d, one) == 2)
                .collect_vec()
                .first()
                .unwrap();
            roseta.insert(0, zero.clone());

            digits
                .iter()
                .filter(|d| {
                    d.len() == 5 && shared_segments(*d, zero) == 4 && shared_segments(*d, one) == 1
                })
                .for_each(|d| {
                    if shared_segments(d, &four) == 2 {
                        roseta.insert(2, d.clone())
                    } else {
                        roseta.insert(5, d.clone())
                    };
                });

            let previous_roseta = roseta.clone();
            digits
                .iter()
                .filter(|d| !previous_roseta.contains_right(*d))
                .for_each(|d| {
                    match d.len() {
                        5 => roseta.insert(3, d.clone()),
                        6 => roseta.insert(6, d.clone()),
                        _ => unreachable!(),
                    };
                });

            output
                .iter()
                .map(|d| roseta.get_by_right(d).unwrap())
                .fold(0, |acc, d| 10 * acc + d)
        };

        self.lines.iter().map(|i| compute(i.clone())).sum()
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 08)
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
    mod advent_2021_08 {
        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver PART ${1|one,two|/}
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
                be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
                edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
                fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
                fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
                aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
                fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
                dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
                bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
                egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
                gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 26);
        }

        // !SECTION

        // SECTION: PART TWO

        /// Solver PART ${1|one,two|/}
        /// Test Case: sample_0
        #[test]
        fn part_two_sample_0() {
            let puzzle: Puzzle = r#"
                acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 5353);
        }

        /// Solver PART ${1|one,two|/}
        /// Test Case: sample_1
        #[test]
        fn part_two_sample_1() {
            let puzzle: Puzzle = r#"
                be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
                edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
                fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
                fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
                aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
                fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
                dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
                bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
                egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
                gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 61229);
        }

        // !SECTION
    }
}
