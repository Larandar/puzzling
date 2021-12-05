use crate::prelude::*;

// TODO(parsing): add header parsing
// TODO(parsing): add section parsing

/// Advent of Code input is usually a list of lines
pub fn lines_of_inputs<Puzzle>(input: &str) -> Result<Vec<Puzzle::Input>>
where
    Puzzle: super::AdventOfCode,
    <<Puzzle as super::AdventOfCode>::Input as std::str::FromStr>::Err: Into<anyhow::Error>,
{
    // To make it easy for the test we remove block indentation from the input
    unindent::unindent(input)
        // Inputs are not always coherent, with sometime a few empty lines at the end
        .split_terminator("\n")
        // Parsed line by line
        .map(|x| -> Result<Puzzle::Input, _> { x.parse() })
        // We wrap the error into an anyhow::Error
        .map(|v| v.map_err(|e| anyhow!(e)))
        // Little known fact: Iterator<Result<_>>.collect() -> Result<Iterator<_>>
        .collect::<Result<_>>()
}
