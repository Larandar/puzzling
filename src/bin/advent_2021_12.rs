use std::collections::{HashMap, HashSet};

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
    rooms: HashMap<String, HashSet<String>>,
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
        let mut rooms = HashMap::new();
        input
            .iter()
            .filter_map(|l| l.split_once("-"))
            // make edge reflectives
            .flat_map(|(a, b)| [(a, b), (b, a)])
            .for_each(|(room, link)| {
                if !rooms.contains_key(&room.to_string()) {
                    rooms.insert(room.to_string(), HashSet::from([link.to_string()]));
                } else {
                    rooms
                        .get_mut(&room.to_string())
                        .unwrap()
                        .insert(link.to_string());
                };
            });
        Self { rooms }
    }
}

impl Puzzle {
    fn part_one_paths(&self) -> HashSet<String> {
        #[cfg(test)]
        debug!("Puzzle: {:?}", self);

        let rooms = self.rooms.clone();

        let mut paths: HashSet<String> = HashSet::new();
        let mut visiting: Vec<(String, Vec<String>)> =
            vec![("start".to_string(), vec!["start".to_string()])];

        while !visiting.is_empty() {
            let (current_room, path) = visiting.pop().unwrap();

            for next_room in rooms.get(&current_room).unwrap() {
                // Impasse (we already visited the next room and it's a small one)
                let visited = path.contains(next_room);
                let is_small = next_room == &next_room.to_lowercase();
                if is_small && visited {
                    continue;
                };

                let mut path = path.clone();
                path.push(next_room.clone().to_string());

                if next_room == "end" {
                    // End of path
                    paths.insert(path.iter().join(","));
                } else {
                    // Continue exploration
                    visiting.push((next_room.clone(), path));
                }
            }
        }
        paths
    }
    fn part_two_paths(&self) -> HashSet<String> {
        #[cfg(test)]
        debug!("Puzzle: {:?}", self);

        let rooms = self.rooms.clone();

        let mut paths: HashSet<String> = HashSet::new();
        let mut visiting: Vec<(String, Vec<String>)> =
            vec![("start".to_string(), vec!["start".to_string()])];

        while !visiting.is_empty() {
            let (current_room, path) = visiting.pop().unwrap();

            for next_room in rooms.get(&current_room).unwrap() {
                let visited = path.contains(next_room) || path.contains(&["*", next_room].concat());
                let is_small = next_room == &next_room.to_lowercase();
                let joker_used = path.iter().any(|room| room.starts_with("*"));

                let mut path = path.clone();
                if next_room == "start" {
                    continue;
                }
                // Impasse (we already visited the next room and it's a small one)
                else if is_small && visited && joker_used {
                    continue;
                } else if is_small && visited {
                    path.push(["*", next_room].concat());
                } else {
                    path.push(next_room.clone().to_string());
                };

                if next_room == "end" {
                    // End of path
                    paths.insert(path.iter().join(","));
                } else {
                    // Continue exploration
                    visiting.push((next_room.clone(), path));
                }
            }
        }

        #[cfg(test)]
        if paths.len() < 40 {
            for p in paths.clone() {
                debug!("  > {}", p);
            }
        };
        paths
    }
}

impl AdventOfCode for Puzzle {
    type Input = Input;
    type Answer = Answer;
    type Puzzle = Puzzle;

    fn part_one(&self) -> Self::Answer {
        self.part_one_paths().len()
    }

    fn part_two(&self) -> Self::Answer {
        self.part_two_paths().len()
    }
}

// TODO(macro): make bootstrap code injected by macro
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let puzzle: Puzzle = daily_challenge(2021, 12)
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
    mod advent_2021_12 {
        use std::collections::HashSet;

        use crate::Puzzle;
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        // SECTION: PART ONE

        /// Solver PART one
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
                start-A
                start-b
                A-c
                A-b
                b-d
                A-end
                b-end
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 10);
            assert_eq!(
                puzzle.part_one_paths(),
                HashSet::from([
                    "start,A,b,A,c,A,end".to_string(),
                    "start,A,b,A,end".to_string(),
                    "start,A,b,end".to_string(),
                    "start,A,c,A,b,A,end".to_string(),
                    "start,A,c,A,b,end".to_string(),
                    "start,A,c,A,end".to_string(),
                    "start,A,end".to_string(),
                    "start,b,A,c,A,end".to_string(),
                    "start,b,A,end".to_string(),
                    "start,b,end".to_string()
                ])
            );
        }

        /// Solver PART one
        /// Test Case: sample_2
        #[test]
        fn part_one_sample_2() {
            let puzzle: Puzzle = r#"
                dc-end
                HN-start
                start-kj
                dc-start
                dc-HN
                LN-dc
                HN-end
                kj-sa
                kj-HN
                kj-dc
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 19);
        }

        /// Solver PART one
        /// Test Case: sample_3
        #[test]
        fn part_one_sample_3() {
            let puzzle: Puzzle = r#"
                fs-end
                he-DX
                fs-he
                start-DX
                pj-DX
                end-zg
                zg-sl
                zg-pj
                pj-he
                RW-he
                fs-DX
                pj-RW
                zg-RW
                start-pj
                he-WI
                zg-he
                pj-fs
                start-RW
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 226);
        }

        // !SECTION

        // SECTION: PART TWO

        /// Solver PART two
        /// Test Case: sample_1
        #[test]
        fn part_two_sample_1() {
            let puzzle: Puzzle = r#"
                start-A
                start-b
                A-c
                A-b
                b-d
                A-end
                b-end
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 36);
        }

        /// Solver PART two
        /// Test Case: sample_2
        #[test]
        fn part_two_sample_2() {
            let puzzle: Puzzle = r#"
                dc-end
                HN-start
                start-kj
                dc-start
                dc-HN
                LN-dc
                HN-end
                kj-sa
                kj-HN
                kj-dc
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 103);
        }

        /// Solver PART two
        /// Test Case: sample_3
        #[test]
        fn part_two_sample_3() {
            let puzzle: Puzzle = r#"
                fs-end
                he-DX
                fs-he
                start-DX
                pj-DX
                end-zg
                zg-sl
                zg-pj
                pj-he
                RW-he
                fs-DX
                pj-RW
                zg-RW
                start-pj
                he-WI
                zg-he
                pj-fs
                start-RW
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_two(), 3509);
        }

        // !SECTION
    }
}
