#[derive(PartialEq)]
enum Opcode {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
}
use Opcode::*;

impl Opcode {
    fn new(data: &[usize], ptr: usize) -> Result<Self, String> {
        match data[ptr] {
            1 => Ok(Add(data[data[ptr + 1]], data[data[ptr + 2]], data[ptr + 3])),
            2 => Ok(Mul(data[data[ptr + 1]], data[data[ptr + 2]], data[ptr + 3])),
            99 => Ok(Halt),
            op => Err(format!("Invalid opcode {}", op)),
        }
    }

    fn size(&self) -> usize {
        match self {
            Add(_, _, _) => 4,
            Mul(_, _, _) => 4,
            Halt => 1,
        }
    }
}

struct Intcode {
    data: Vec<usize>,
    ptr: usize,
}

impl Intcode {
    fn new(data: Vec<usize>) -> Self {
        Self { data, ptr: 0 }
    }

    fn step(&mut self) -> Result<bool, String> {
        let op = Opcode::new(&self.data, self.ptr)?;
        match op {
            Add(op1, op2, out) => self.data[out] = op1 + op2,
            Mul(op1, op2, out) => self.data[out] = op1 * op2,
            _ => {}
        }
        self.ptr += op.size();
        Ok(op == Halt)
    }

    fn run(&mut self) -> Result<(), String> {
        while !self.step()? {}
        Ok(())
    }

    fn run_with(&mut self, noun: usize, verb: usize) -> Result<(), String> {
        self.data[1] = noun;
        self.data[2] = verb;
        self.run()
    }
}

fn part1(data: &[usize]) -> Result<usize, String> {
    let mut prog = Intcode::new(data.to_vec());
    prog.run_with(12, 2)?;
    Ok(prog.data[0])
}

fn part2(data: &[usize]) -> Result<usize, String> {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut prog = Intcode::new(data.to_vec());
            if prog.run_with(noun, verb).is_ok() && prog.data[0] == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err("No solution found".into())
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p02.txt");
    let data = input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Bad input")?;
    let out1 = part1(&data)?;
    let out2 = part2(&data)?;
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut p1 = Intcode::new(vec![1, 0, 0, 0, 99]);
        assert!(p1.run().is_ok());
        assert_eq!(p1.data, vec![2, 0, 0, 0, 99]);
        let mut p2 = Intcode::new(vec![2, 3, 0, 3, 99]);
        assert!(p2.run().is_ok());
        assert_eq!(p2.data, vec![2, 3, 0, 6, 99]);
        let mut p3 = Intcode::new(vec![2, 4, 4, 5, 99, 0]);
        assert!(p3.run().is_ok());
        assert_eq!(p3.data, vec![2, 4, 4, 5, 99, 9801]);
        let mut p4 = Intcode::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert!(p4.run().is_ok());
        assert_eq!(p4.data, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
