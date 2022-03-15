use bimap::BiMap;
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
    instructions: Vec<Input>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum ChunkToken {
    Valid,
    Incomplete(String),
    Corrupted(char, char),
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
            instructions: input,
        }
    }
}

fn parse(chunk: String) -> ChunkToken {
    #[cfg(test)]
    debug!("parsing: {}", chunk);

    let clossing_pair: BiMap<char, char> =
        BiMap::from_iter(vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    if !chunk
        .chars()
        .all(|c| clossing_pair.contains_left(&c) || clossing_pair.contains_right(&c))
    {
        panic!("non valid instruction");
    }

    let mut chunk = chunk.chars().into_iter().rev().collect_vec();
    let mut stack = vec![];

    while !(stack.is_empty() && chunk.is_empty()) {
        if chunk.is_empty() {
            return ChunkToken::Incomplete(
                stack
                    .iter()
                    .rev()
                    .filter_map(|c| clossing_pair.get_by_left(c))
                    .join(""),
            );
        } else if stack.is_empty() {
            stack.push(chunk.pop().expect("at least one character in chunk"))
        }

        let left = *stack.last().unwrap();

        if !clossing_pair.contains_left(&left) {
            return ChunkToken::Corrupted(' ', left);
        };

        let right = chunk.pop().unwrap();
        if clossing_pair.contains_left(&right) {
            stack.push(right);
            continue;
        }

        if *clossing_pair.get_by_left(&left).unwrap() == right {
            stack.pop();
        } else {
            return ChunkToken::Corrupted(*clossing_pair.get_by_left(&left).unwrap(), right);
        };
    }

    ChunkToken::Valid
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        #[cfg(test)]
        debug!("Puzzle: \n{}", self.instructions.iter().join("\n"));

        #[cfg(test)]
        debug!(
            "Parsing: {:?}",
            self.instructions
                .iter()
                .map(|l| parse(l.to_string()))
                .collect_vec()
        );

        self.instructions
            .iter()
            .map(|l| parse(l.to_string()))
            .filter_map(|r| match r {
                ChunkToken::Corrupted(_, found) => Some(match found {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                }),
                _ => None,
            })
            .sum()
    }

    fn part_two(&self) -> Self::Answer {
        #[cfg(test)]
        debug!("Puzzle: \n{}", self.instructions.iter().join("\n"));

        #[cfg(test)]
        debug!(
            "Parsing: {:?}",
            self.instructions
                .iter()
                .map(|l| parse(l.to_string()))
                .collect_vec()
        );

        let scores = self
            .instructions
            .iter()
            .map(|l| parse(l.to_string()))
            .filter_map(|r| match r {
                ChunkToken::Incomplete(missing) => Some(missing.chars().fold(0, |a, c| {
                    a * 5
                        + match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => unreachable!(),
                        }
                })),
                _ => None,
            })
            .sorted()
            .collect_vec();

        scores[scores.len() / 2]
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 10)
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
    mod advent_2021_10 {
        use crate::{parse, ChunkToken, Puzzle};
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver PART one
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_0() {
            let puzzle: Puzzle = r#"
                [({(<(())[]>[[{[]{<()<>>
                [(()[<>])]({[<{<<[]>>(
                {([(<{}[<>[]}>{[]{[(<()>
                (((({<>}<{<{<>}{[]{[]{}
                [[<[([]))<([[{}[[()]]]
                [{[{({}]{}}([{[{{{}}([]
                {<[[]]>}<{[{[{[]{()[[[]
                [<(<(<(<{}))><([]([]()
                <{([([[(<>()){}]>(<<{{
                <{([{{}}[<[[[<>{}]]]>[]]
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 26397);
        }

        #[test]
        fn parse_samples_corrupted() {
            assert_eq!(
                parse("{([(<{}[<>[]}>{[]{[(<()>".to_string()),
                ChunkToken::Corrupted(']', '}')
            );
            assert_eq!(
                parse("[[<[([]))<([[{}[[()]]]".to_string()),
                ChunkToken::Corrupted(']', ')')
            );
            assert_eq!(
                parse("[{[{({}]{}}([{[{{{}}([]".to_string()),
                ChunkToken::Corrupted(')', ']')
            );
            assert_eq!(
                parse("[<(<(<(<{}))><([]([]()".to_string()),
                ChunkToken::Corrupted('>', ')')
            );
            assert_eq!(
                parse("<{([([[(<>()){}]>(<<{{".to_string()),
                ChunkToken::Corrupted(']', '>')
            );
        }

        // !SECTION

        // SECTION: PART TWO

        /// Solver PART two
        /// Test Case: sample_0
        #[test]
        fn part_two_sample_0() {
            let puzzle: Puzzle = r#"
                [({(<(())[]>[[{[]{<()<>>
                [(()[<>])]({[<{<<[]>>(
                {([(<{}[<>[]}>{[]{[(<()>
                (((({<>}<{<{<>}{[]{[]{}
                [[<[([]))<([[{}[[()]]]
                [{[{({}]{}}([{[{{{}}([]
                {<[[]]>}<{[{[{[]{()[[[]
                [<(<(<(<{}))><([]([]()
                <{([([[(<>()){}]>(<<{{
                <{([{{}}[<[[[<>{}]]]>[]]
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 288957);
        }

        /// Solver PART two
        /// Test Case: sample_1
        #[test]
        fn part_two_sample_1() {
            let puzzle: Puzzle = r#"
                [({(<(())[]>[[{[]{<()<>>
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 288957);
        }

        #[test]
        fn parse_samples_incomplete() {
            assert_eq!(
                parse("[({(<(())[]>[[{[]{<()<>>".to_string()),
                ChunkToken::Incomplete("}}]])})]".to_string())
            );
            assert_eq!(
                parse("[(()[<>])]({[<{<<[]>>(".to_string()),
                ChunkToken::Incomplete(")}>]})".to_string())
            );
            assert_eq!(
                parse("(((({<>}<{<{<>}{[]{[]{}".to_string()),
                ChunkToken::Incomplete("}}>}>))))".to_string())
            );
            assert_eq!(
                parse("{<[[]]>}<{[{[{[]{()[[[]".to_string()),
                ChunkToken::Incomplete("]]}}]}]}>".to_string())
            );
            assert_eq!(
                parse("<{([{{}}[<[[[<>{}]]]>[]]".to_string()),
                ChunkToken::Incomplete("])}>".to_string())
            );
        }
        // !SECTION
    }
}
