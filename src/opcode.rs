//! Calculating intcodes depending on opcodes in intcodes

use displaydoc::Display;
use std::error;

/// Any Errors returned creating `OpCode`s or executing them
#[derive(Debug, Display, PartialEq)]
pub enum Error {
    /// There is no `OpCode` associated with the number found at the given index.
    InvalidOpCode,
    /// The index for the `OpCode` does not exist.
    NoOpCode,
    /// The index for the next `OpCode` does not exist.
    NoNextOpCode,
    /// Not all indices for the execution arguments exist.
    MissingArgs,
    /// The execution arguments point to non-existing indices.
    InvalidArgIndices,
    /// There is no number to look up where to write the calculation result to.
    MissingResult,
    /// The index to write the calculation result to does not exist.
    InvalidResult,
}

impl error::Error for Error {}

/// The result of any operation related to `OpCode`s
pub type Result<T> = std::result::Result<T, Error>;

/// An opcode representing a calculation operation
///
/// Most variants hold an index of the intcode where they were parsed from.
/// This is needed to know where to find arguments for calculation.
#[derive(PartialEq, Eq, Debug)]
pub enum OpCode {
    /// 1 - Add to numbers
    ///
    /// Takes the three indices following the opcode,
    /// adds the number identified by the second to that identified by the first and
    /// inserts it at position specified by the third.
    Add(usize),
    /// 2 - Multiply two numbers
    ///
    /// Takes the three indices following the opcode,
    /// multiplies the number at the first and second and
    /// inserts it at posision specified by the third.
    Mul(usize),
    /// 99 - Stop calculating intcodes and close connection
    Halt,
}

impl OpCode {
    /// Processes a complete `OpCode`.
    ///
    /// Starts at index 0 by calling `new` and `execute`
    /// and goes on with the next index until finding an `Err` or `Ok(None)`.
    ///
    /// # Examples
    /// ```
    /// use opcode::opcode::{Error, OpCode};
    ///
    /// let mut tst_cmd_list = vec![1, 0, 0, 3, 2, 0, 3, 6, 99, 1, 0, 1, 4];
    /// assert_eq!(Ok(vec![1, 0, 0, 2, 2, 0, 2, 6, 99, 1, 0, 1, 4]), OpCode::process(tst_cmd_list));
    ///
    /// let mut out_of_bounds_list = vec![1, 0, 0, 100, 2, 0, 3, 6, 99];
    /// assert_eq!(Err(Error::InvalidResult), OpCode::process(out_of_bounds_list));
    ///
    /// let mut invalid_opcode_list = vec![42, 1, 2, 3, 99];
    /// assert_eq!(Err(Error::InvalidOpCode), OpCode::process(invalid_opcode_list));
    ///
    /// let mut too_short_list = vec![1,0];
    /// assert_eq!(Err(Error::MissingArgs), OpCode::process(too_short_list));
    /// ```
    pub fn process(mut cmd_list: Vec<usize>) -> Result<Vec<usize>> {
        let mut idx = Some(0);
        while idx.is_some() {
            idx = OpCode::new(idx.unwrap(), &cmd_list)?.execute(&mut cmd_list)?;
        }
        Ok(cmd_list)
    }

    /// Executes the command identified by the `OpCode`.
    ///
    /// # Arguments
    /// * `cmd_list` - A mutable pointer to an intcode. See game description for details.
    ///
    /// # Returns
    /// If `OpCode::Halt` is specified and everything goes well, `Ok(None)` is returned.
    ///
    /// For the other variants of `OpCode` the according operation is calculated.
    /// If everything goes well, the index of the next opcode in `cmd_list` is returned as `Ok` variant.
    ///
    /// # Errors
    /// If any of the specified indices is out of bounds `Err(Error::IndexOutOfBounds)` is returned.
    ///
    /// # Examples
    ///
    /// A successful processing could look like this:
    /// ```
    /// use opcode::OpCode;
    ///
    /// let mut tst_cmd_list = vec![1, 0, 0, 3, 2, 0, 3, 6, 99];
    ///
    /// // Start with the first opcode.
    /// let operation = OpCode::new(0, &mut tst_cmd_list).unwrap();
    /// let new_idx = operation.execute(&mut tst_cmd_list).unwrap().unwrap();
    /// // Now the next opcode is at index 4 and the result is at index 3.
    /// assert_eq!(new_idx, 4);
    /// assert_eq!(tst_cmd_list[..], [1, 0, 0, 2, 2, 0, 3, 6, 99]);
    ///
    /// // Go on with the new opcode at `new_idx`.
    /// let exp = [1, 0, 0, 2, 2, 0, 2, 6, 99];
    /// let operation = OpCode::new(new_idx, &mut tst_cmd_list).unwrap();
    /// let new_idx = operation.execute(&mut tst_cmd_list).unwrap().unwrap();
    /// // Now the next opcode is at index 8 and the result is at index 6.
    /// assert_eq!(new_idx, 8);
    /// assert_eq!(tst_cmd_list[..], exp);
    ///
    /// // Go on with the new opcode at `new_idx`.
    /// let operation = OpCode::new(new_idx, &mut tst_cmd_list).unwrap();
    /// let new_idx = operation.execute(&mut tst_cmd_list).unwrap();
    /// // This time the intcode has not changed and there is no new index.
    /// assert_eq!(new_idx, None);
    /// assert_eq!(tst_cmd_list[..], exp);
    /// ```
    pub fn execute(&self, cmd_list: &mut Vec<usize>) -> Result<Option<usize>> {
        match self {
            Self::Add(idx) => {
                let (args, result, next_opcode) = Self::check_indices(cmd_list, *idx, 2)?;
                cmd_list[result] = args.iter().copied().map(|i| cmd_list[i]).sum();
                Ok(Some(next_opcode))
            }
            Self::Mul(idx) => {
                let (args, result, next_opcode) = Self::check_indices(cmd_list, *idx, 2)?;
                cmd_list[result] = args.iter().copied().map(|i| cmd_list[i]).product();
                Ok(Some(next_opcode))
            }
            Self::Halt => Ok(None),
        }
    }

    /// Checks that the indices of `cmd_list` for execution exist and returns them.
    ///
    /// # Returns
    /// The return tuple consists of the following:
    /// * The list of arguments for execution. For `OpCode::Add`, e.g., this will be a vector of two elements.
    /// * The index where the calculation result should be written to.
    /// * The next `OpCode`'s index.
    fn check_indices(
        cmd_list: &mut Vec<usize>,
        opcode: usize,
        num_args: usize,
    ) -> Result<(Vec<usize>, usize, usize)> {
        // If we have 2 args then first arg is opcode + 1, second opcode + 2,
        // so we look up where to write the result to at opcode + 3.
        let result = opcode + num_args + 1;
        // The next opcode can be found 1 position further.
        let next_opcode = opcode + num_args + 2;

        // Check that the indices of the arguments are in bounds.
        let mut args = Vec::new();
        for arg in opcode + 1..result {
            let &arg = cmd_list.get(arg).ok_or(Error::MissingArgs)?;
            let _ = cmd_list.get(arg).ok_or(Error::InvalidArgIndices)?;
            args.push(arg);
        }

        // Check that the index exists where we want to write the result to:
        // 1. Get the value at opcode + num_args + 1
        // 2. Check that the value is a valid index
        let &result = cmd_list.get(result).ok_or(Error::MissingResult)?;
        let _ = cmd_list.get(result).ok_or(Error::InvalidResult)?;

        // Check that the next opcode's position exists.
        let _ = cmd_list.get(next_opcode).ok_or(Error::NoNextOpCode)?;

        Ok((args, result, next_opcode))
    }

    fn parse(idx: usize, opcode: usize) -> Result<Self> {
        match opcode {
            1 => Ok(OpCode::Add(idx)),
            2 => Ok(OpCode::Mul(idx)),
            99 => Ok(OpCode::Halt),
            _ => Err(Error::InvalidOpCode),
        }
    }

    /// Parses a posisition of an intcode as `OpCode`.
    ///
    /// # Arguments
    /// * `idx` - The index of `cmd_list` that is to be parsed
    /// * `cmd_list` - A pointer to an intcode
    ///
    /// # Returns
    /// If the number at `idx` is a valid opcode the according `OpCode` variant is returned as `Ok`.
    ///
    /// # Errors
    /// If `cmd_list` has no index `idx` `Err(Error::IndexOutOfBounds)` is retuned.
    ///
    /// If there is no `OpCode` variant identified by the given number `Err(Error::InvalidOpCode)` is returned.
    ///
    /// # Examples
    /// ```
    /// use opcode::opcode::{Error, OpCode};
    ///
    /// let mut tst_vec = vec![1, 2, 3, 4, 2, 3, 4, 5, 99];
    /// assert_eq!(OpCode::new(0, &mut tst_vec), Ok(OpCode::Add(0)));
    /// assert_eq!(OpCode::new(4, &mut tst_vec), Ok(OpCode::Mul(4)));
    /// assert_eq!(OpCode::new(8, &mut tst_vec), Ok(OpCode::Halt));
    /// assert_eq!(OpCode::new(2, &mut tst_vec), Err(Error::InvalidOpCode));
    /// assert_eq!(OpCode::new(100, &mut tst_vec), Err(Error::NoOpCode));
    /// assert_eq!(OpCode::new(0, &mut vec![]), Err(Error::NoOpCode));
    /// ```
    pub fn new(idx: usize, cmd_list: &[usize]) -> Result<Self> {
        let &opcode = cmd_list.get(idx).ok_or(Error::NoOpCode)?;
        Self::parse(idx, opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_parse() {
        assert_eq!(OpCode::parse(0, 1), Ok(OpCode::Add(0)));
        assert_eq!(OpCode::parse(0, 2), Ok(OpCode::Mul(0)));
        assert_eq!(OpCode::parse(0, 99), Ok(OpCode::Halt));
        assert_eq!(OpCode::parse(0, 1337), Err(Error::InvalidOpCode));
    }

    #[test]
    fn tst_check_indices_ok() {
        let mut cmd_list: Vec<usize> = vec![0, 1, 2, 3, 3, 2, 1];
        let (args, result, next_opcode) = OpCode::check_indices(&mut cmd_list, 0, 4).unwrap();
        assert_eq!(vec![1, 2, 3, 3], args);
        assert_eq!(2, result);
        assert_eq!(6, next_opcode);
    }

    #[test]
    fn tst_check_indices_args_indices() {
        let mut cmd_list: Vec<usize> = vec![0, 0, 0];
        assert_eq!(
            Err(Error::MissingArgs),
            OpCode::check_indices(&mut cmd_list, 0, 3)
        );
    }

    #[test]
    fn tst_check_indices_args_values() {
        let mut cmd_list: Vec<usize> = vec![0, 8, 0, 0];
        assert_eq!(
            Err(Error::InvalidArgIndices),
            OpCode::check_indices(&mut cmd_list, 0, 1)
        );
    }

    #[test]
    fn tst_check_indices_next_opcode() {
        let mut cmd_list: Vec<usize> = vec![0, 0];
        assert_eq!(
            Err(Error::NoNextOpCode),
            OpCode::check_indices(&mut cmd_list, 0, 0)
        );
    }

    #[test]
    fn tst_check_indices_result() {
        let mut cmd_list: Vec<usize> = vec![0];
        assert_eq!(
            Err(Error::MissingResult),
            OpCode::check_indices(&mut cmd_list, 0, 0)
        );

        let mut cmd_list: Vec<usize> = vec![0, 3, 1];
        assert_eq!(
            Err(Error::InvalidResult),
            OpCode::check_indices(&mut cmd_list, 0, 0)
        );
    }
}
