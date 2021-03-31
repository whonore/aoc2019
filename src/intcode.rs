use std::collections::HashMap;
use std::io;
use std::ops::Index;
use std::str::FromStr;

fn ints_to_bytes(xs: &[i64]) -> Vec<u8> {
    xs.iter().flat_map(|x| x.to_le_bytes().to_vec()).collect()
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}
use ParamMode::*;

impl ParamMode {
    fn new(op: i64, param: u32) -> Self {
        match (op / 10_i64.pow(param + 1)) % 10 {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => unreachable!(),
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
    const fn eval(self, v1: i64, v2: i64) -> i64 {
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
    const fn eval(self, v1: i64, v2: i64) -> bool {
        match self {
            Lt => v1 < v2,
            Eq => v1 == v2,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Opcode {
    Arith(BinOp, i64, i64, u64),
    Input(u64),
    Output(i64),
    Jump(bool, i64, u64),
    Compare(CmpOp, i64, i64, u64),
    AdjustBase(i64),
    Halt,
}
use Opcode::*;

impl Opcode {
    fn new(mem: &Memory) -> Result<Self, String> {
        match mem.instr() % 100 {
            op @ 1 | op @ 2 => Ok(Arith(
                if op == 1 { Add } else { Mul },
                mem.in_param(1),
                mem.in_param(2),
                mem.out_param(3),
            )),
            3 => Ok(Input(mem.out_param(1))),
            4 => Ok(Output(mem.in_param(1))),
            op @ 5 | op @ 6 => Ok(Jump(op == 5, mem.in_param(1), mem.in_param(2) as u64)),
            op @ 7 | op @ 8 => Ok(Compare(
                if op == 7 { Lt } else { Eq },
                mem.in_param(1),
                mem.in_param(2),
                mem.out_param(3),
            )),
            9 => Ok(AdjustBase(mem.in_param(1))),
            99 => Ok(Halt),
            op => Err(format!("Invalid opcode {}", op)),
        }
    }

    const fn size(&self) -> u64 {
        match self {
            Arith(_, _, _, _) | Compare(_, _, _, _) => 4,
            Jump(_, _, _) => 3,
            Input(_) | Output(_) | AdjustBase(_) => 2,
            Halt => 1,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Memory {
    mem: HashMap<u64, i64>,
    ptr: u64,
    base: i64,
}

impl Memory {
    fn set(&mut self, ptr: u64, val: i64) {
        self.mem.insert(ptr, val);
    }

    fn instr(&self) -> i64 {
        self[self.ptr]
    }

    fn in_param(&self, param: u64) -> i64 {
        match ParamMode::new(self.instr(), param as u32) {
            Immediate => self[self.ptr + param],
            Position => self[self[self.ptr + param] as u64],
            Relative => self[(self.base + self[self.ptr + param]) as u64],
        }
    }

    fn out_param(&self, param: u64) -> u64 {
        match ParamMode::new(self.instr(), param as u32) {
            Immediate => unreachable!(),
            Position => self[self.ptr + param] as u64,
            Relative => (self.base + self[self.ptr + param]) as u64,
        }
    }
}

impl From<Vec<i64>> for Memory {
    fn from(code: Vec<i64>) -> Self {
        Self {
            mem: code
                .iter()
                .enumerate()
                .map(|(idx, v)| (idx as u64, *v))
                .collect(),
            ptr: 0,
            base: 0,
        }
    }
}

impl Index<u64> for Memory {
    type Output = i64;

    fn index(&self, ptr: u64) -> &Self::Output {
        self.mem.get(&ptr).unwrap_or(&0)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Intcode {
    pub code: Vec<i64>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct IntcodeExec<I, O> {
    mem: Memory,
    stdin: I,
    stdout: O,
}

impl FromStr for Intcode {
    type Err = String;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            code: code
                .trim()
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "Invalid input")?,
        })
    }
}

impl From<Vec<i64>> for Intcode {
    fn from(code: Vec<i64>) -> Self {
        Self { code }
    }
}

impl<I, O> Index<u64> for IntcodeExec<I, O> {
    type Output = i64;

    fn index(&self, idx: u64) -> &Self::Output {
        &self.mem[idx]
    }
}

impl Intcode {
    pub fn exec(&self) -> IntcodeExec<io::Empty, io::Sink> {
        IntcodeExec {
            mem: self.code.clone().into(),
            stdin: io::empty(),
            stdout: io::sink(),
        }
    }
}

impl<I: io::Read, O: io::Write> IntcodeExec<I, O> {
    pub fn read_from<I2: io::Read>(self, stdin: I2) -> IntcodeExec<I2, O> {
        IntcodeExec {
            mem: self.mem,
            stdin,
            stdout: self.stdout,
        }
    }

    pub fn write_to<O2: io::Write>(self, stdout: O2) -> IntcodeExec<I, O2> {
        IntcodeExec {
            mem: self.mem,
            stdin: self.stdin,
            stdout,
        }
    }

    pub fn read_vec(self, stdin: &[i64]) -> IntcodeExec<io::Cursor<Vec<u8>>, O> {
        self.read_from(io::Cursor::new(ints_to_bytes(stdin)))
    }

    pub fn run(&mut self) -> Result<Vec<i64>, String> {
        self.collect::<Result<Vec<_>, _>>()
            .map(|outs| outs.iter().copied().filter_map(|out| out).collect())
    }

    pub fn run_with(&mut self, vals: &[(u64, i64)]) -> Result<Vec<i64>, String> {
        for (idx, val) in vals {
            self.mem.set(*idx, *val);
        }
        self.run()
    }

    pub fn run_to_out(&mut self) -> Result<Option<i64>, String> {
        self.find(|res| res.is_err() || res.as_ref().unwrap().is_some())
            .unwrap_or(Ok(None))
    }
}

impl<I: io::Read + io::Write + io::Seek, O: io::Write> IntcodeExec<I, O> {
    pub fn read_next(&mut self, stdin: &[i64]) {
        let pos = self.stdin.seek(io::SeekFrom::Current(0)).unwrap();
        self.stdin.seek(io::SeekFrom::End(0)).unwrap();
        self.stdin.write_all(&ints_to_bytes(stdin)).unwrap();
        self.stdin.seek(io::SeekFrom::Start(pos)).unwrap();
    }
}

impl<I: io::Read, O: io::Write> Iterator for IntcodeExec<I, O> {
    type Item = Result<Option<i64>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let op = Opcode::new(&self.mem);
        if let Err(err) = op {
            return Some(Err(err));
        }
        let op = op.unwrap();
        let mut jumped = false;
        let mut out = None;

        match op {
            Arith(binop, v1, v2, out) => self.mem.set(out, binop.eval(v1, v2)),
            Input(out) => {
                let mut buf = [0; 8];
                if let Err(err) = self
                    .stdin
                    .read_exact(&mut buf)
                    .map_err(|_| "Invalid read".into())
                {
                    return Some(Err(err));
                }
                self.mem.set(out, i64::from_le_bytes(buf));
            }
            Output(val) => {
                out = Some(val);
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
                    self.mem.ptr = ptr;
                    jumped = true;
                }
            }
            Compare(cmp, v1, v2, out) => self.mem.set(out, if cmp.eval(v1, v2) { 1 } else { 0 }),
            AdjustBase(v) => self.mem.base += v,
            Halt => {}
        };

        if !jumped {
            self.mem.ptr += op.size();
        }
        if op == Halt {
            None
        } else {
            Some(Ok(out))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_io() {
        let mut p = Intcode::from(vec![1, 0, 0, 0, 99]).exec();
        assert!(p.run().is_ok());
        assert_eq!(p.mem.mem, Memory::from(vec![2, 0, 0, 0, 99]).mem);
        let mut p = Intcode::from(vec![2, 3, 0, 3, 99]).exec();
        assert!(p.run().is_ok());
        assert_eq!(p.mem.mem, Memory::from(vec![2, 3, 0, 6, 99]).mem);
        let mut p = Intcode::from(vec![2, 4, 4, 5, 99, 0]).exec();
        assert!(p.run().is_ok());
        assert_eq!(p.mem.mem, Memory::from(vec![2, 4, 4, 5, 99, 9801]).mem);
        let mut p = Intcode::from(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).exec();
        assert!(p.run().is_ok());
        assert_eq!(
            p.mem.mem,
            Memory::from(vec![30, 1, 1, 4, 2, 5, 6, 0, 99]).mem
        );
    }

    #[test]
    fn test_param_mode() {
        let mut p = Intcode::from(vec![1101, 100, -1, 4, 0]).exec();
        assert!(p.run().is_ok());
        assert_eq!(p.mem.mem, Memory::from(vec![1101, 100, -1, 4, 99]).mem);
    }

    #[test]
    fn test_echo() {
        let mut p = Intcode::from(vec![3, 0, 4, 0, 99])
            .exec()
            .read_vec(&[1])
            .write_to(vec![]);
        assert_eq!(p.run_to_out(), Ok(Some(1)));
    }

    #[test]
    fn test_eq() {
        let eq1 = Intcode::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        let eq2 = Intcode::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let mut p = eq1.exec().read_vec(&[8]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![1]));
        let mut p = eq1.exec().read_vec(&[7]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![0]));
        let mut p = eq2.exec().read_vec(&[8]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![1]));
        let mut p = eq2.exec().read_vec(&[7]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![0]));
    }

    #[test]
    fn test_lt() {
        let lt1 = Intcode::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        let lt2 = Intcode::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        let mut p = lt1.exec().read_vec(&[7]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![1]));
        let mut p = lt1.exec().read_vec(&[8]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![0]));
        let mut p = lt2.exec().read_vec(&[7]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![1]));
        let mut p = lt2.exec().read_vec(&[8]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![0]));
    }

    #[test]
    fn test_if() {
        let if1 = Intcode::from(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        let if2 = Intcode::from(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        let mut p = if1.exec().read_vec(&[1]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![1]));
        let mut p = if1.exec().read_vec(&[0]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![0]));
        let mut p = if2.exec().read_vec(&[1]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![1]));
        let mut p = if2.exec().read_vec(&[0]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![0]));
    }

    #[test]
    fn test_large() {
        let large = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0\
                     ,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46\
                     ,1101,1000,1,20,4,20,1105,1,46,98,99"
            .parse::<Intcode>()
            .unwrap();
        let mut p = large.exec().read_vec(&[7]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![999]));
        let mut p = large.exec().read_vec(&[8]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![1000]));
        let mut p = large.exec().read_vec(&[9]).write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![1001]));
    }

    #[test]
    fn test_relative() {
        let code = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut p = Intcode::from(code.clone()).exec().write_to(vec![]);
        assert_eq!(p.run(), Ok(code));
    }

    #[test]
    fn test_big_number() {
        let mut p = Intcode::from(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0])
            .exec()
            .write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![1219070632396864]));
        let mut p = Intcode::from(vec![104, 1125899906842624, 99])
            .exec()
            .write_to(vec![]);
        assert_eq!(p.run(), Ok(vec![1125899906842624]));
    }
}
