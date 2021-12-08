use regex::Regex;

use crate::prelude::*;

// TODO(parsing): add header parsing
// TODO(parsing): add section parsing

/// Advent of Code input is usually a list of lines
pub fn lines<T: FromStr>(input: &str) -> Result<Vec<T>>
where
    <T as FromStr>::Err: Into<anyhow::Error>,
{
    // To make it easy for the test we remove block indentation from the input
    unindent::unindent(input)
        .split_terminator("\n")
        // Parsed line by line
        .map(|x| -> Result<T, _> { x.parse() })
        // We wrap the error into an anyhow::Error
        .map(|v| v.map_err(|e| anyhow!(e)))
        // Little known fact: Iterator<Result<_>>.collect() -> Result<Iterator<_>>
        .collect::<Result<_>>()
}

/// Sectionned input by empty line
pub fn sections(input: Vec<String>) -> Vec<Vec<String>> {
    let mut section_id = 0;
    input
        .iter()
        .group_by(|v| {
            if v.len() == 0 {
                section_id += 1
            }
            section_id
        })
        .into_iter()
        .map(|(_, section)| {
            section
                // Empty line
                .filter(|l| l.len() > 0)
                // Remove the reference
                .map(|x| x.clone())
                .collect()
        })
        .collect()
}

/// Split a string using on any number whitespace
/// Also trim the string to prevent empty elements
pub fn eager_split(input: String) -> Vec<String> {
    let whitespace: Regex = Regex::new(r"\s+").unwrap();
    whitespace.split(&input.trim()).map(String::from).collect()
}
