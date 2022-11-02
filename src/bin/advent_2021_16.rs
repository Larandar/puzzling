#![allow(dead_code, unused_variables)]
use puzzling::advent_of_code::{daily_challenge, parsing, AdventOfCode};
use puzzling::prelude::*;
use std::collections::VecDeque;
use std::str;
/// Input type for each line
type Input = String;
/// Expected output
type Answer = u128;

/// Representation of a complete puzzle
#[derive(Debug, Clone)]
struct Puzzle {
    packet: Vec<u8>,
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
            packet: input
                .iter()
                .flat_map(|l| l.chars().map(|b| b.to_digit(16).unwrap() as u8))
                .collect::<Vec<_>>(),
        }
    }
}

impl Puzzle {
    fn format_packet(packet: &[u8]) -> String {
        packet
            .iter()
            .map(|b| format!("{:04b}", 0xF & b))
            .interleave_shortest(
                vec![
                    " ".to_string(),
                    " ".to_string(),
                    " ".to_string(),
                    "\n".to_string(),
                ]
                .iter()
                .cloned()
                .cycle(),
            )
            .collect::<Vec<String>>()
            .join("")
    }
}

const BWORD_SIZE: u8 = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
enum BuoyancyPacket {
    Literal {
        version: u8,
        value: u128,
    },
    Operator {
        version: u8,
        operator: u8,
        packets: Vec<BuoyancyPacket>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BuoyancyTape {
    tape: VecDeque<u8>,
    remainder: u8,
    read: u128,
}

impl BuoyancyTape {
    fn new(tape: &[u8]) -> Self {
        Self {
            tape: tape.iter().cloned().collect(),
            remainder: BWORD_SIZE,
            read: 0,
        }
    }

    fn next_bits(&mut self, n: u8) -> u128 {
        debug_assert!(n <= 128, "can only read 128 bits at a time");
        debug_assert!(
            n as usize <= (self.tape.len() - 1) * BWORD_SIZE as usize + self.remainder as usize,
            "can't read more bits than are available"
        );
        let mut result: u128 = 0;
        let mut to_take = n;

        while to_take > 0 {
            if self.remainder == 0 {
                self.tape.pop_front().unwrap();
                self.remainder = BWORD_SIZE;
            }

            let taking = std::cmp::min(to_take, self.remainder);
            let allign = self.remainder - taking;
            let mask = (0b1111 >> (BWORD_SIZE - taking)) << allign;

            result <<= taking;
            result |= (mask & self.tape[0] as u128) >> allign;

            to_take -= taking;

            // Keep track of the internal state
            self.remainder -= taking;
            self.read += taking as u128;
        }

        result
    }
}

impl BuoyancyPacket {
    fn read(mut tape: &mut BuoyancyTape) -> Self {
        let version = tape.next_bits(3) as u8;
        let packet_type = tape.next_bits(3) as u8;

        match packet_type {
            4 => {
                let mut value = 0;

                let mut take_next = true;
                while take_next {
                    take_next = tape.next_bits(1) == 1;
                    value <<= 4;
                    value |= 0x0F & tape.next_bits(4);
                }

                Self::Literal { version, value }
            }
            _ => {
                if tape.next_bits(1) == 0 {
                    let mut packets = vec![];

                    let to_read = tape.next_bits(15);
                    let stop_point = tape.read + to_read as u128;

                    while tape.read < stop_point {
                        packets.push(Self::read(&mut tape));
                    }

                    Self::Operator {
                        version,
                        operator: packet_type,
                        packets,
                    }
                } else {
                    let number_of_packets = tape.next_bits(11);
                    let mut packets = vec![];

                    for _ in 0..number_of_packets {
                        packets.push(Self::read(&mut tape));
                    }

                    Self::Operator {
                        version,
                        operator: packet_type,
                        packets,
                    }
                }
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
        debug!("Packet: \n{}", Puzzle::format_packet(&self.packet));

        let mut tape = BuoyancyTape::new(&self.packet.clone());
        let packet = BuoyancyPacket::read(&mut tape);

        #[cfg(test)]
        debug!("Parsed: \n{:?}", packet);

        fn walk(packet: &BuoyancyPacket) -> u128 {
            match packet {
                BuoyancyPacket::Literal { version, value: _ } => *version as u128,
                BuoyancyPacket::Operator {
                    version,
                    operator: _,
                    packets,
                } => *version as u128 + packets.iter().map(walk).sum::<u128>(),
            }
        }

        walk(&packet)
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

    let puzzle: Puzzle = daily_challenge(2021, 16)
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
    mod advent_2021_16 {
        use crate::{BuoyancyPacket, BuoyancyTape, Puzzle};
        use puzzling::{advent_of_code::AdventOfCode, prelude::*};
        use test_log::test;

        #[test]
        fn test_read_literal() {
            let puzzle: Puzzle = r#"
                D2FE28
            "#
            .parse()
            .expect("failed to parse input string");

            let mut tape = BuoyancyTape::new(&puzzle.packet);
            debug!("{:?}", tape);

            assert_eq!(
                BuoyancyPacket::read(&mut tape),
                BuoyancyPacket::Literal {
                    version: 6,
                    value: 2021,
                }
            );
        }

        #[test]
        fn test_read_operator_1() {
            let puzzle: Puzzle = r#"
                38006F45291200
            "#
            .parse()
            .expect("failed to parse input string");

            let mut tape = BuoyancyTape::new(&puzzle.packet);
            debug!("{:?}", tape);

            assert_eq!(
                BuoyancyPacket::read(&mut tape),
                BuoyancyPacket::Operator {
                    version: 1,
                    operator: 6,
                    packets: vec![
                        BuoyancyPacket::Literal {
                            version: 6,
                            value: 10,
                        },
                        BuoyancyPacket::Literal {
                            version: 2,
                            value: 20,
                        },
                    ]
                }
            );
        }

        #[test]
        fn test_read_operator_2() {
            let puzzle: Puzzle = r#"
                EE00D40C823060
            "#
            .parse()
            .expect("failed to parse input string");

            let mut tape = BuoyancyTape::new(&puzzle.packet);
            debug!("{:?}", tape);

            assert_eq!(
                BuoyancyPacket::read(&mut tape),
                BuoyancyPacket::Operator {
                    version: 7,
                    operator: 3,
                    packets: vec![
                        BuoyancyPacket::Literal {
                            version: 2,
                            value: 1,
                        },
                        BuoyancyPacket::Literal {
                            version: 4,
                            value: 2,
                        },
                        BuoyancyPacket::Literal {
                            version: 1,
                            value: 3,
                        },
                    ]
                }
            );
        }

        // SECTION: PART ONE

        /// Solver PART one
        /// Test Case: sample_1
        #[test]
        fn part_one_sample_1() {
            let puzzle: Puzzle = r#"
                8A004A801A8002F478
            "#
            .parse()
            .expect("failed to parse input string");

            debug!("Packet: \n{}", Puzzle::format_packet(&puzzle.packet));

            assert_eq!(puzzle.packet.len(), "8A004A801A8002F478".len());

            let mut tape = BuoyancyTape::new(&puzzle.packet);
            assert_eq!(
                BuoyancyPacket::read(&mut tape),
                BuoyancyPacket::Operator {
                    version: 4,
                    operator: 2,
                    packets: vec![BuoyancyPacket::Operator {
                        version: 1,
                        operator: 2,
                        packets: vec![BuoyancyPacket::Operator {
                            version: 5,
                            operator: 2,
                            packets: vec![BuoyancyPacket::Literal {
                                version: 6,
                                value: 15
                            }]
                        }]
                    }]
                }
            );

            assert_eq!(puzzle.part_one(), 16);
        }

        /// Solver PART one
        /// Test Case: sample_2
        #[test]
        fn part_one_sample_2() {
            let puzzle: Puzzle = r#"
                620080001611562C8802118E34
            "#
            .parse()
            .expect("failed to parse input string");

            debug!("Packet: \n{}", Puzzle::format_packet(&puzzle.packet));

            let mut tape = BuoyancyTape::new(&puzzle.packet);
            assert_eq!(
                BuoyancyPacket::read(&mut tape),
                BuoyancyPacket::Operator {
                    version: 3,
                    operator: 0,
                    packets: vec![
                        BuoyancyPacket::Operator {
                            version: 0,
                            operator: 0,
                            packets: vec![
                                BuoyancyPacket::Literal {
                                    version: 0,
                                    value: 10
                                },
                                BuoyancyPacket::Literal {
                                    version: 5,
                                    value: 11
                                }
                            ]
                        },
                        BuoyancyPacket::Operator {
                            version: 1,
                            operator: 0,
                            packets: vec![
                                BuoyancyPacket::Literal {
                                    version: 0,
                                    value: 12
                                },
                                BuoyancyPacket::Literal {
                                    version: 3,
                                    value: 13
                                }
                            ]
                        }
                    ]
                }
            );

            assert_eq!(puzzle.part_one(), 12);
        }

        /// Solver PART one
        /// Test Case: sample_3
        #[test]
        fn part_one_sample_3() {
            let puzzle: Puzzle = r#"
                C0015000016115A2E0802F182340
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 23);
        }

        /// Solver PART one
        /// Test Case: sample_4
        #[test]
        fn part_one_sample_4() {
            let puzzle: Puzzle = r#"
                A0016C880162017C3686B18A3D4780
            "#
            .parse()
            .expect("failed to parse input string");

            assert_eq!(puzzle.part_one(), 31);
        }

        // !SECTION

        // SECTION: PART TWO

        // !SECTION
    }
}
