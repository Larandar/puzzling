use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    ThreeFiveSum { n: u128 },
    EvenFibonacci { n: u128 },
    LargestPrimeFactor { n: u128 },
    LargestPalindromeProduct { n: u128 },
    NthPrime { n: u128 },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::ThreeFiveSum { n } => three_five_sum(n),
        Commands::EvenFibonacci { n } => even_fibonacci(n),
        Commands::LargestPrimeFactor { n } => largest_prime_factor(n),
        Commands::LargestPalindromeProduct { n } => largest_palindrome_product(n),
        Commands::NthPrime { n } => nth_prime(n),
    };

    println!("{:?} => {}", cli.command, result);
}

/// Source: https://projecteuler.net/problem=1
fn three_five_sum(number: u128) -> u128 {
    (0..number).filter(|&n| n % 3 == 0 || n % 5 == 0).sum()
}

/// Source: https://projecteuler.net/problem=2
fn even_fibonacci(number: u128) -> u128 {
    euler::series::fibonacci()
        .take_while(|&n| n < number)
        .filter(|&n| n % 2 == 0)
        .sum::<u128>()
}

/// Source: https://projecteuler.net/problem=3
fn largest_prime_factor(number: u128) -> u128 {
    euler::primes::factors(number).max().unwrap_or(number)
}

/// Source: https://projecteuler.net/problem=4
fn largest_palindrome_product(number: u128) -> u128 {
    unimplemented!()
}

/// Source: https://projecteuler.net/problem=7
fn nth_prime(number: u128) -> u128 {
    euler::primes::primes()
        .take(number as usize)
        .last()
        .unwrap_or(0)
}
