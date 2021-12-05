use crate::prelude::*;

pub trait AdventOfCode {
    type Input: FromStr;
    type Answer;
    type Puzzle: Sized + From<Vec<Self::Input>>;

    fn part_one(&self) -> Self::Answer;
    fn part_two(&self) -> Self::Answer;
}
