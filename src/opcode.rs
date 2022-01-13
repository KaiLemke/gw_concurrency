//! Calculating intcodes depending on opcodes in intcodes

use displaydoc::Display;
use std::error;

/// Any Errors returned creating `OpCode`s or executing them
#[derive(Debug, Display, PartialEq, Eq)]
pub enum Error {
    /// There is no `OpCode` associated with the number found at the given index.
    InvalidOpCode,
    /// The given intcode does not contain all given indices.
    IndexOutOfBounds,
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
                let &idx_op1 = cmd_list.get(idx + 1).ok_or(Error::IndexOutOfBounds)?;
                let &idx_op2 = cmd_list.get(idx + 2).ok_or(Error::IndexOutOfBounds)?;
                let &idx_dst = cmd_list.get(idx + 3).ok_or(Error::IndexOutOfBounds)?;
                let _ =  cmd_list.get(idx_dst).ok_or(Error::IndexOutOfBounds)?;
                cmd_list[idx_dst] = cmd_list[idx_op1] + cmd_list[idx_op2];
                Ok(Some(idx + 4))
            }
            Self::Mul(idx) => {
                let &idx_op1 = cmd_list.get(idx + 1).ok_or(Error::IndexOutOfBounds)?;
                let &idx_op2 = cmd_list.get(idx + 2).ok_or(Error::IndexOutOfBounds)?;
                let &idx_dst = cmd_list.get(idx + 3).ok_or(Error::IndexOutOfBounds)?;
                let _ =  cmd_list.get(idx_dst).ok_or(Error::IndexOutOfBounds)?;
                cmd_list[idx_dst] = cmd_list[idx_op1] * cmd_list[idx_op2];
                Ok(Some(idx + 4))
            }
            Self::Halt => Ok(None),
        }
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
    /// assert_eq!(OpCode::new(100, &mut tst_vec), Err(Error::IndexOutOfBounds));
    /// assert_eq!(OpCode::new(0, &mut vec![]), Err(Error::IndexOutOfBounds));
    /// ```
    pub fn new(idx: usize, cmd_list: &[usize]) -> Result<Self> {
        let &opcode = cmd_list.get(idx).ok_or(Error::IndexOutOfBounds)?;
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
    fn tst_add_too_short() {
        let mut cmd_list = vec![1, 2];
        let opcode = OpCode::new(0, &cmd_list).unwrap();
        assert_eq!(Err(Error::IndexOutOfBounds), opcode.execute(&mut cmd_list));

        cmd_list = vec![1, 2, 3];
        let opcode = OpCode::new(0, &cmd_list).unwrap();
        assert_eq!(Err(Error::IndexOutOfBounds), opcode.execute(&mut cmd_list));

        cmd_list = vec![1, 2, 3, 42];
        let opcode = OpCode::new(0, &cmd_list).unwrap();
        assert_eq!(Err(Error::IndexOutOfBounds), opcode.execute(&mut cmd_list));
    }

    #[test]
    fn tst_mul_too_short() {
        let mut cmd_list = vec![2, 2];
        let opcode = OpCode::new(0, &cmd_list).unwrap();
        assert_eq!(Err(Error::IndexOutOfBounds), opcode.execute(&mut cmd_list));

        cmd_list = vec![2, 2, 3];
        let opcode = OpCode::new(0, &cmd_list).unwrap();
        assert_eq!(Err(Error::IndexOutOfBounds), opcode.execute(&mut cmd_list));

        cmd_list = vec![2, 2, 3, 42];
        let opcode = OpCode::new(0, &cmd_list).unwrap();
        assert_eq!(Err(Error::IndexOutOfBounds), opcode.execute(&mut cmd_list));
    }
}
