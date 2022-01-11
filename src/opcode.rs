#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_tryfrom() {
        assert_eq!(OpCode::parse(0, 1) , Ok(OpCode::ADD(0)));
        assert_eq!(OpCode::parse(0, 2) , Ok(OpCode::MUL(0)));
        assert_eq!(OpCode::parse(0, 99) , Ok(OpCode::HALT));
        assert_eq!(OpCode::parse(0, 1337), Err("invalid op code"));
    }

    #[test]
    fn tst_new() {
        let mut tst_vec = vec![1, 2, 3, 4, 2, 3, 4, 5, 99];
        assert_eq!(OpCode::new(0, &mut tst_vec) , Ok(OpCode::ADD(0)));
        assert_eq!(OpCode::new(4, &mut tst_vec) , Ok(OpCode::MUL(4)));
        assert_eq!(OpCode::new(8, &mut tst_vec) , Ok(OpCode::HALT));
        assert_eq!(OpCode::new(2, &mut tst_vec) , Err("invalid op code"));
        assert_eq!(OpCode::new(100, &mut tst_vec) , Err("index out of bounds"));        
    }

    #[test]
    fn tst_execute() {
        let mut tst_cmd_list = vec![1, 0, 0, 3, 2, 0, 3, 6, 99];
        
        let exp = [1, 0, 0, 2, 2, 0, 3, 6, 99];
        let operation = OpCode::new(0, &mut tst_cmd_list).unwrap();
        let new_idx = operation.execute(&mut tst_cmd_list).unwrap().unwrap();
        assert_eq!(new_idx, 4);
        assert_eq!(tst_cmd_list[..], exp);

        let exp = [1, 0, 0, 2, 2, 0, 2, 6, 99];
        let operation = OpCode::new(new_idx, &mut tst_cmd_list).unwrap();
        let new_idx = operation.execute(&mut tst_cmd_list).unwrap().unwrap();
        assert_eq!(new_idx, 8);
        assert_eq!(tst_cmd_list[..], exp);

        let operation = OpCode::new(new_idx, &mut tst_cmd_list).unwrap();
        let new_idx = operation.execute(&mut tst_cmd_list).unwrap();
        assert_eq!(new_idx, None);
        assert_eq!(tst_cmd_list[..], exp);
    }
}

impl OpCode {
    pub fn execute(&self, cmd_list :&mut Vec<usize>) -> Result<Option<usize>, String> {
        match self {
            Self::ADD(idx) => {
                let _ = cmd_list[..].get(idx + 3).ok_or_else(|| "execute(): index out of bounds")?;
                let idx_op1 = cmd_list[idx + 1];
                let idx_op2 = cmd_list[idx + 2];
                let idx_dst = cmd_list[idx + 3];
                cmd_list[idx_dst] = cmd_list[idx_op1] + cmd_list[idx_op2];
                Ok(Some(idx + 4))
            },
            Self::MUL(idx) => {
                let _ = cmd_list[..].get(idx + 3).ok_or_else(|| "execute(): index out of bounds")?;
                let idx_op1 = cmd_list[idx + 1];
                let idx_op2 = cmd_list[idx + 2];
                let idx_dst = cmd_list[idx + 3];
                cmd_list[idx_dst] = cmd_list[idx_op1] * cmd_list[idx_op2];
                Ok(Some(idx + 4))
            },
            Self::HALT => {
                Ok(None)
            },
        }
    }

    fn parse(idx: usize, opcode: usize) -> Result<Self, &'static str> {
        match opcode {
            1 => Ok(OpCode::ADD(idx)),
            2 => Ok(OpCode::MUL(idx)),
            99 => Ok(OpCode::HALT),
            _ => Err("invalid op code"),
        }
    }

    pub fn new(idx: usize, cmd_list: &Vec<usize>) -> Result<Self, &str> {
        let cmd_list_slice = cmd_list[..].get(idx).ok_or_else(|| {
            "index out of bounds"
        })?;
        Self::parse(idx, *cmd_list_slice)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum OpCode {
    ADD(usize),
    MUL(usize),
    HALT,
}
