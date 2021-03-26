use std::convert::TryInto;
use std::io;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum ParamMode {
    Position,
    Immediate,
}
use ParamMode::*;

impl ParamMode {
    fn new(op: i64, param: u32) -> Self {
        if (op / 10_i64.pow(param + 1)) % 10 == 1 {
            Immediate
        } else {
            Position
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum BinOp {
    Add,
    Mul,
}
use BinOp::*;

impl BinOp {
    fn eval(&self, v1: i64, v2: i64) -> i64 {
        match self {
            Add => v1 + v2,
            Mul => v1 * v2,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum CmpOp {
    Lt,
    Eq,
}
use CmpOp::*;

impl CmpOp {
    fn eval(&self, v1: i64, v2: i64) -> bool {
        match self {
            Lt => v1 < v2,
            Eq => v1 == v2,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Opcode {
    Arith(BinOp, i64, i64, usize),
    Input(usize),
    Output(i64),
    Jump(bool, i64, usize),
    Compare(CmpOp, i64, i64, usize),
    Halt,
}
use Opcode::*;

impl Opcode {
    fn new(prog: &[i64], ptr: usize) -> Result<Self, String> {
        match prog[ptr] % 100 {
            op @ 1 | op @ 2 => Ok(Arith(
                if op == 1 { Add } else { Mul },
                Self::in_param(prog, ptr, 1),
                Self::in_param(prog, ptr, 2),
                Self::out_param(prog, ptr, 3) as usize,
            )),
            3 => Ok(Input(Self::out_param(prog, ptr, 1))),
            4 => Ok(Output(Self::in_param(prog, ptr, 1))),
            op @ 5 | op @ 6 => Ok(Jump(
                op == 5,
                Self::in_param(prog, ptr, 1),
                Self::in_param(prog, ptr, 2) as usize,
            )),
            op @ 7 | op @ 8 => Ok(Compare(
                if op == 7 { Lt } else { Eq },
                Self::in_param(prog, ptr, 1),
                Self::in_param(prog, ptr, 2),
                Self::out_param(prog, ptr, 3),
            )),
            99 => Ok(Halt),
            op => Err(format!("Invalid opcode {}", op)),
        }
    }

    fn in_param(prog: &[i64], ptr: usize, param: usize) -> i64 {
        match ParamMode::new(prog[ptr], param as u32) {
            Immediate => prog[ptr + param],
            Position => prog[prog[ptr + param] as usize],
        }
    }

    const fn out_param(prog: &[i64], ptr: usize, param: usize) -> usize {
        prog[ptr + param] as usize
    }

    const fn size(&self) -> usize {
        match self {
            Arith(_, _, _, _) | Compare(_, _, _, _) => 4,
            Jump(_, _, _) => 3,
            Input(_) | Output(_) => 2,
            Halt => 1,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Intcode {
    pub prog: Vec<i64>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct IntcodeExec<I, O> {
    pub prog: Vec<i64>,
    ptr: usize,
    stdin: I,
    stdout: O,
}

impl FromStr for Intcode {
    type Err = String;

    fn from_str(prog: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            prog: prog
                .trim()
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "Invalid input")?,
        })
    }
}

impl Intcode {
    #[allow(dead_code)]
    pub fn new(prog: Vec<i64>) -> Self {
        Self { prog }
    }

    pub fn exec(&self) -> IntcodeExec<io::Empty, io::Sink> {
        IntcodeExec {
            prog: self.prog.clone(),
            ptr: 0,
            stdin: io::empty(),
            stdout: io::sink(),
        }
    }
}

impl<I: io::Read, O: io::Write> IntcodeExec<I, O> {
    pub fn read_from<I2: io::Read>(self, stdin: I2) -> IntcodeExec<I2, O> {
        IntcodeExec {
            prog: self.prog,
            ptr: self.ptr,
            stdin,
            stdout: self.stdout,
        }
    }

    pub fn write_to<O2: io::Write>(self, stdout: O2) -> IntcodeExec<I, O2> {
        IntcodeExec {
            prog: self.prog,
            ptr: self.ptr,
            stdin: self.stdin,
            stdout,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.try_for_each(|res| res)
    }

    pub fn run_with(&mut self, vals: &[(usize, i64)]) -> Result<(), String> {
        for (idx, val) in vals {
            self.prog[*idx] = *val;
        }
        self.run()
    }
}

impl<I: io::Read> IntcodeExec<I, Vec<u8>> {
    pub fn read_out(&self) -> Vec<i64> {
        self.stdout
            .chunks(8)
            .map(|bs| i64::from_le_bytes(bs.try_into().unwrap()))
            .collect()
    }
}

impl<I: io::Read, O: io::Write> Iterator for IntcodeExec<I, O> {
    type Item = Result<(), String>;

    fn next(&mut self) -> Option<Self::Item> {
        let op = Opcode::new(&self.prog, self.ptr);
        if let Err(err) = op {
            return Some(Err(err));
        }
        let op = op.unwrap();
        let mut jumped = false;

        match op {
            Arith(binop, v1, v2, out) => self.prog[out] = binop.eval(v1, v2),
            Input(out) => {
                let mut buf = [0; 8];
                if let Err(err) = self
                    .stdin
                    .read_exact(&mut buf)
                    .map_err(|_| "Invalid read".into())
                {
                    return Some(Err(err));
                }
                self.prog[out] = i64::from_le_bytes(buf);
            }
            Output(val) => {
                if let Err(err) = self
                    .stdout
                    .write(&val.to_le_bytes())
                    .map_err(|_| "Invalid write".into())
                {
                    return Some(Err(err));
                }
            }
            Jump(b, v, ptr) => {
                if b ^ (v == 0) {
                    self.ptr = ptr;
                    jumped = true;
                }
            }
            Compare(cmp, v1, v2, out) => self.prog[out] = if cmp.eval(v1, v2) { 1 } else { 0 },
            Halt => {}
        };

        if !jumped {
            self.ptr += op.size();
        }
        if op == Halt {
            None
        } else {
            Some(Ok(()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_io() {
        let mut p = Intcode::new(vec![1, 0, 0, 0, 99]).exec();
        assert!(p.run().is_ok());
        assert_eq!(p.prog, vec![2, 0, 0, 0, 99]);
        let mut p = Intcode::new(vec![2, 3, 0, 3, 99]).exec();
        assert!(p.run().is_ok());
        assert_eq!(p.prog, vec![2, 3, 0, 6, 99]);
        let mut p = Intcode::new(vec![2, 4, 4, 5, 99, 0]).exec();
        assert!(p.run().is_ok());
        assert_eq!(p.prog, vec![2, 4, 4, 5, 99, 9801]);
        let mut p = Intcode::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).exec();
        assert!(p.run().is_ok());
        assert_eq!(p.prog, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_param_mode() {
        let mut p = Intcode::new(vec![1101, 100, -1, 4, 0]).exec();
        assert!(p.run().is_ok());
        assert_eq!(p.prog, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_echo() {
        let input = &1_i64.to_le_bytes()[..];
        let mut p = Intcode::new(vec![3, 0, 4, 0, 99])
            .exec()
            .read_from(input)
            .write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![1]);
    }

    #[test]
    fn test_eq() {
        let eq1 = Intcode::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        let eq2 = Intcode::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let input1 = &8_i64.to_le_bytes()[..];
        let input2 = &7_i64.to_le_bytes()[..];
        let mut p = eq1.exec().read_from(input1).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![1]);
        let mut p = eq1.exec().read_from(input2).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![0]);
        let mut p = eq2.exec().read_from(input1).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![1]);
        let mut p = eq2.exec().read_from(input2).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![0]);
    }

    #[test]
    fn test_lt() {
        let lt1 = Intcode::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        let lt2 = Intcode::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        let input1 = &7_i64.to_le_bytes()[..];
        let input2 = &8_i64.to_le_bytes()[..];
        let mut p = lt1.exec().read_from(input1).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![1]);
        let mut p = lt1.exec().read_from(input2).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![0]);
        let mut p = lt2.exec().read_from(input1).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![1]);
        let mut p = lt2.exec().read_from(input2).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![0]);
    }

    #[test]
    fn test_if() {
        let if1 = Intcode::new(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        let if2 = Intcode::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        let input1 = &1_i64.to_le_bytes()[..];
        let input2 = &0_i64.to_le_bytes()[..];
        let mut p = if1.exec().read_from(input1).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![1]);
        let mut p = if1.exec().read_from(input2).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![0]);
        let mut p = if2.exec().read_from(input1).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![1]);
        let mut p = if2.exec().read_from(input2).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![0]);
    }

    #[test]
    fn test_large() {
        let large = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0\
                     ,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46\
                     ,1101,1000,1,20,4,20,1105,1,46,98,99"
            .parse::<Intcode>()
            .unwrap();
        let input1 = &7_i64.to_le_bytes()[..];
        let input2 = &8_i64.to_le_bytes()[..];
        let input3 = &9_i64.to_le_bytes()[..];
        let mut p = large.exec().read_from(input1).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![999]);
        let mut p = large.exec().read_from(input2).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![1000]);
        let mut p = large.exec().read_from(input3).write_to(vec![]);
        assert!(p.run().is_ok());
        assert_eq!(p.read_out(), vec![1001]);
    }
}
