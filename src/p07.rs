use itertools::Itertools;

use crate::intcode::Intcode;

fn run_amp(prog: &Intcode, phases: &[i64]) -> Result<i64, String> {
    phases.iter().try_fold(0, |input, phase| {
        prog.exec()
            .read_vec(&[*phase, input])
            .write_to(vec![])
            .run_to_out()
            .and_then(|res| res.ok_or_else(|| "No return value".into()))
    })
}

fn run_amp_feedback(prog: &Intcode, phases: &[i64]) -> Result<i64, String> {
    let mut amps = phases
        .iter()
        .map(|phase| prog.exec().read_vec(&[*phase]).write_to(vec![]))
        .collect::<Vec<_>>();
    let mut input = 0;
    loop {
        for amp in amps.iter_mut() {
            amp.read_next(&[input]);
            if let Some(out) = amp.run_to_out()? {
                input = out;
            } else {
                return Ok(input);
            }
        }
    }
}

fn part1(prog: &Intcode) -> Result<i64, String> {
    (0..=4)
        .permutations(5)
        .map(|phases| run_amp(prog, &phases))
        .collect::<Result<Vec<_>, _>>()
        .map(|outs| outs.into_iter().max().unwrap())
}

fn part2(prog: &Intcode) -> Result<i64, String> {
    (5..=9)
        .permutations(5)
        .map(|phases| run_amp_feedback(prog, &phases))
        .collect::<Result<Vec<_>, _>>()
        .map(|outs| outs.into_iter().max().unwrap())
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p07.txt");
    let prog = input.parse()?;
    let out1 = part1(&prog)?;
    let out2 = part2(&prog)?;
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

    #[test]
    fn test02() {
        let p = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
                 27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            .parse()
            .unwrap();
        assert_eq!(part2(&p), Ok(139629729));
        let p = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
                 -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
                 53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            .parse()
            .unwrap();
        assert_eq!(part2(&p), Ok(18216));
    }
}
