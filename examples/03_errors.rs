//! # 03 Error handling: no exceptions. Errors are ordinary values.
//!
//! - `Result<T, E>` = `Ok(T)` or `Err(E)`. The `?` operator returns early on Err.
//! - `panic!` / `.unwrap()` / `.expect()` are for bugs and impossible states (and tests/prototypes).
//! - Real projects: libraries define an error enum (crate `thiserror`), apps use `anyhow::Result`.
//!   Std-only here so you see what those crates generate for you.
//!
//! cargo run  --example 03_errors
//! cargo test --example 03_errors
#![allow(unused)]

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    Missing(String),
    BadNumber(ParseIntError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Missing(k) => write!(f, "missing key: {k}"),
            ConfigError::BadNumber(e) => write!(f, "bad number: {e}"),
        }
    }
}
impl std::error::Error for ConfigError {}

// `?` calls `From::from` on the error, so this impl lets ParseIntError convert automatically.
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

fn get_port(config: &str) -> Result<u16, ConfigError> {
    for line in config.lines() {
        if let Some(v) = line.strip_prefix("port=") {
            return Ok(v.trim().parse()?); // ParseIntError -> ConfigError via From
        }
    }
    Err(ConfigError::Missing("port".into()))
}

// main can return a Result: on Err it prints the error and exits with code 1.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", get_port("host=x\nport=8080"));
    println!("{:?}", get_port("host=x"));
    match get_port("port=abc") {
        Ok(p) => println!("port {p}"),
        Err(e) => println!("error: {e}"),
    }

    // Converting between Option and Result
    let n: Option<i32> = "42".parse().ok();
    let r: Result<i32, &str> = n.ok_or("none");
    println!("{n:?} {r:?}");

    // Combinators: transform without unpacking
    let doubled = "21".parse::<i32>().map(|x| x * 2).unwrap_or(0);
    println!("{doubled}");

    let port = get_port("port=3000")?; // `?` works in main too
    println!("got {port}");
    Ok(())
}

// ======================= EXERCISES =======================

#[derive(Debug, PartialEq)]
enum ParseError {
    NoEquals,
    EmptyKey,
    BadValue,
}

/// "age=30" -> Ok(("age", 30)).  "age30" -> NoEquals.  "=3" -> EmptyKey.  "a=x" -> BadValue.
/// Hints: `split_once('=').ok_or(...)?`, `.parse().map_err(|_| ...)?`
fn parse_pair(s: &str) -> Result<(String, i32), ParseError> {
    todo!()
}

/// Sum the values of every line. The first error wins.
/// Hint: a loop with `?`, or `.map(parse_pair).collect::<Result<Vec<_>, _>>()?`
fn sum_values(text: &str) -> Result<i32, ParseError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs() {
        assert_eq!(parse_pair("age=30"), Ok(("age".into(), 30)));
        assert_eq!(parse_pair("age30"), Err(ParseError::NoEquals));
        assert_eq!(parse_pair("=3"), Err(ParseError::EmptyKey));
        assert_eq!(parse_pair("a=x"), Err(ParseError::BadValue));
    }

    #[test]
    fn sums() {
        assert_eq!(sum_values("a=1\nb=2\nc=3"), Ok(6));
        assert_eq!(sum_values("a=1\nbad\nc=x"), Err(ParseError::NoEquals));
    }
}
