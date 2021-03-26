use std::convert::TryInto;
use std::io;

#[derive(PartialEq, Eq, Debug)]
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

#[derive(PartialEq, Eq, Debug)]
enum Opcode {
    Add(i64, i64, usize),
    Mul(i64, i64, usize),
    Input(usize),
    Output(i64),
    Halt,
}
use Opcode::*;

impl Opcode {
    fn new(data: &[i64], ptr: usize) -> Result<Self, String> {
        match data[ptr] % 100 {
            1 => Ok(Add(
                Self::in_param(data, ptr, 1),
                Self::in_param(data, ptr, 2),
                Self::out_param(data, ptr, 3) as usize,
            )),
            2 => Ok(Mul(
                Self::in_param(data, ptr, 1),
                Self::in_param(data, ptr, 2),
                Self::out_param(data, ptr, 3) as usize,
            )),
            3 => Ok(Input(Self::out_param(data, ptr, 1))),
            4 => Ok(Output(Self::in_param(data, ptr, 1))),
            99 => Ok(Halt),
            op => Err(format!("Invalid opcode {}", op)),
        }
    }

    fn in_param(data: &[i64], ptr: usize, param: usize) -> i64 {
        match ParamMode::new(data[ptr], param as u32) {
            Immediate => data[ptr + param],
            Position => data[data[ptr + param] as usize],
        }
    }

    fn out_param(data: &[i64], ptr: usize, param: usize) -> usize {
        data[ptr + param] as usize
    }

    const fn size(&self) -> usize {
        match self {
            Add(_, _, _) | Mul(_, _, _) => 4,
            Input(_) | Output(_) => 2,
            Halt => 1,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Intcode<I: io::Read, O: io::Write> {
    pub data: Vec<i64>,
    ptr: usize,
    stdin: I,
    stdout: O,
}

impl<I: io::Read, O: io::Write> Intcode<I, O> {
    pub fn new(data: Vec<i64>, stdin: I, stdout: O) -> Self {
        Self {
            data,
            ptr: 0,
            stdin,
            stdout,
        }
    }

    pub fn step(&mut self) -> Result<bool, String> {
        let op = Opcode::new(&self.data, self.ptr)?;
        match op {
            Add(v1, v2, out) => self.data[out] = v1 + v2,
            Mul(v1, v2, out) => self.data[out] = v1 * v2,
            Input(out) => {
                let mut buf = [0; 8];
                self.stdin
                    .read_exact(&mut buf)
                    .map_err(|_| "Invalid read")?;
                self.data[out] = i64::from_le_bytes(buf);
            }
            Output(val) => {
                self.stdout
                    .write(&val.to_le_bytes())
                    .map_err(|_| "Invalid write")?;
            }
            Halt => {}
        }
        self.ptr += op.size();
        Ok(op == Halt)
    }

    pub fn run(&mut self) -> Result<(), String> {
        while !self.step()? {}
        Ok(())
    }

    pub fn run_with(&mut self, vals: &[(usize, i64)]) -> Result<(), String> {
        for (idx, val) in vals {
            self.data[*idx] = *val;
        }
        self.run()
    }
}

impl Intcode<io::Empty, io::Sink> {
    pub fn empty_io(data: Vec<i64>) -> Self {
        Self::new(data, io::empty(), io::sink())
    }
}

impl<I: io::Read> Intcode<I, Vec<u8>> {
    pub fn vec_out(data: Vec<i64>, stdin: I) -> Self {
        Self::new(data, stdin, vec![])
    }

    pub fn read_out(&self) -> Vec<i64> {
        self.stdout
            .chunks(8)
            .map(|bs| i64::from_le_bytes(bs.try_into().unwrap()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_io() {
        let mut p1 = Intcode::empty_io(vec![1, 0, 0, 0, 99]);
        assert!(p1.run().is_ok());
        assert_eq!(p1.data, vec![2, 0, 0, 0, 99]);
        let mut p2 = Intcode::empty_io(vec![2, 3, 0, 3, 99]);
        assert!(p2.run().is_ok());
        assert_eq!(p2.data, vec![2, 3, 0, 6, 99]);
        let mut p3 = Intcode::empty_io(vec![2, 4, 4, 5, 99, 0]);
        assert!(p3.run().is_ok());
        assert_eq!(p3.data, vec![2, 4, 4, 5, 99, 9801]);
        let mut p4 = Intcode::empty_io(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert!(p4.run().is_ok());
        assert_eq!(p4.data, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_param_mode() {
        let mut p1 = Intcode::empty_io(vec![1101, 100, -1, 4, 0]);
        assert!(p1.run().is_ok());
        assert_eq!(p1.data, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_echo() {
        let input = &1_i64.to_le_bytes()[..];
        let mut p1 = Intcode::vec_out(vec![3, 0, 4, 0, 99], input);
        assert!(p1.run().is_ok());
        assert_eq!(p1.read_out(), vec![1]);
    }
}
