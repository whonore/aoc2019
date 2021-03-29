use itertools::Itertools;

use crate::intcode::Intcode;

fn run_amp(prog: &Intcode, phases: &[i64]) -> Result<i64, String> {
    phases.iter().try_fold(0, |input, phase| {
        prog.exec()
            .read_vec(&[*phase, input])
            .write_to(vec![])
            .run_return()
    })
}

fn part1(prog: &Intcode) -> Result<i64, String> {
    (0..=4)
        .permutations(5)
        .map(|phases| run_amp(prog, &phases))
        .collect::<Result<Vec<_>, _>>()
        .map(|outs| outs.into_iter().max().unwrap())
}

fn part2() -> u64 {
    0
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p07.txt");
    let prog = input.parse()?;
    let out1 = part1(&prog)?;
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let p = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            .parse()
            .unwrap();
        assert_eq!(part1(&p), Ok(43210));
        let p = "3,23,3,24,1002,24,10,24,1002,23,-1,23,\
                 101,5,23,23,1,24,23,23,4,23,99,0,0"
            .parse()
            .unwrap();
        assert_eq!(part1(&p), Ok(54321));
        let p = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
                 1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            .parse()
            .unwrap();
        assert_eq!(part1(&p), Ok(65210));
    }
}
