#![feature(never_type, once_cell)]

// Global exports
pub mod config;
pub mod logging;
pub mod prelude;

// Per challenge source modules
#[cfg(feature = "advent")]
pub mod advent_of_code;
