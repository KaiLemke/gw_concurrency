//! Calculating intcodes depending on opcodes in intcodes

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
    /// If any of the specified indices is out of bounds `Err("index out of bounds")` is returned.
    ///
    /// # Examples
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
    /// let exp = [1, 0, 0, 2, 2, 0, 2, 6, 99]
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
    pub fn execute(&self, cmd_list: &mut Vec<usize>) -> Result<Option<usize>, String> {
        match self {
            Self::Add(idx) => {
                let _ = cmd_list[..]
                    .get(idx + 3)
                    .ok_or_else(|| "execute(): index out of bounds")?;
                let idx_op1 = cmd_list[idx + 1];
                let idx_op2 = cmd_list[idx + 2];
                let idx_dst = cmd_list[idx + 3];
                cmd_list[idx_dst] = cmd_list[idx_op1] + cmd_list[idx_op2];
                Ok(Some(idx + 4))
            }
            Self::Mul(idx) => {
                let _ = cmd_list[..]
                    .get(idx + 3)
                    .ok_or_else(|| "execute(): index out of bounds")?;
                let idx_op1 = cmd_list[idx + 1];
                let idx_op2 = cmd_list[idx + 2];
                let idx_dst = cmd_list[idx + 3];
                cmd_list[idx_dst] = cmd_list[idx_op1] * cmd_list[idx_op2];
                Ok(Some(idx + 4))
            }
            Self::Halt => Ok(None),
        }
    }

    fn parse(idx: usize, opcode: usize) -> Result<Self, &'static str> {
        match opcode {
            1 => Ok(OpCode::Add(idx)),
            2 => Ok(OpCode::Mul(idx)),
            99 => Ok(OpCode::Halt),
            _ => Err("invalid op code"),
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
    /// If `cmd_list` has no index `idx` `Err("index out of bounds")` is retuned.
    ///
    /// If there is no `OpCode` variant identified by the given number `Err("invalid op code")` is returned.
    ///
    /// # Examples
    /// ```
    /// use opcode::OpCode;
    ///
    /// let mut tst_vec = vec![1, 2, 3, 4, 2, 3, 4, 5, 99];
    /// assert_eq!(OpCode::new(0, &mut tst_vec), Ok(OpCode::Add(0)));
    /// assert_eq!(OpCode::new(4, &mut tst_vec), Ok(OpCode::Mul(4)));
    /// assert_eq!(OpCode::new(8, &mut tst_vec), Ok(OpCode::Halt));
    /// assert_eq!(OpCode::new(2, &mut tst_vec), Err("invalid op code"));
    /// assert_eq!(OpCode::new(100, &mut tst_vec), Err("index out of bounds"));
    /// ```
    pub fn new(idx: usize, cmd_list: &Vec<usize>) -> Result<Self, &str> {
        let cmd_list_slice = cmd_list[..].get(idx).ok_or_else(|| "index out of bounds")?;
        Self::parse(idx, *cmd_list_slice)
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
        assert_eq!(OpCode::parse(0, 1337), Err("invalid op code"));
    }
}
