//! Reading and writing intcodes

use std::num::ParseIntError;
use std::{error, fmt};

/// An intcode is a non-empty list of integers.
pub type IntCode = Vec<usize>;

/// Any errors that occur when parsing a string to an intcode
#[derive(Debug, PartialEq)]
pub enum Error {
    /// String does not look like a `Vec`
    ParseVec,
    /// The list does not contain integers only
    ParseInt(ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}

/// Parses a string to an `IntCode`.
///
/// # Arguments
/// * `s` - A string matching the `Debug` implementation of `IntCode`
///
/// # Examples
/// ```
/// use opcode::intcode::parse;
///
/// let v = vec![1, 2, 3, 4, 5];
/// assert_eq!(v, parse(format!("{:?}", v).as_str()).unwrap());
///
/// assert!(parse("1, 2, 3, 4, 5").is_err());
/// assert!(parse("[]").is_err());
/// ```
pub fn parse(s: &str) -> Result<IntCode, Error> {
    let mut ints = Vec::new();
    for s in s
        .strip_prefix('[')
        .map(|s| s.strip_suffix(']'))
        .flatten()
        .map(|s| s.split(", ").collect::<Vec<&str>>())
        .ok_or(Error::ParseVec)?
    {
        ints.push(s.parse::<usize>().map_err(Error::ParseInt)?);
    }

    Ok(ints)
}
