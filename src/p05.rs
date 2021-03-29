use crate::intcode::Intcode;

fn part1(prog: &Intcode) -> Result<i64, String> {
    let mut exec = prog.exec().read_vec(&[1]).write_to(vec![]);
    exec.run()?;
    let out = exec.read_out();
    if out[..out.len().saturating_sub(1)].iter().all(|x| *x == 0) {
        out.last().copied().ok_or_else(|| "No output".into())
    } else {
        Err("Failed diagnostic".into())
    }
}

fn part2(prog: &Intcode) -> Result<i64, String> {
    let mut exec = prog.exec().read_vec(&[5]).write_to(vec![]);
    exec.run()?;
    exec.read_out()
        .first()
        .copied()
        .ok_or_else(|| "No output".into())
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p05.txt");
    let prog = input.parse()?;
    let out1 = part1(&prog)?;
    let out2 = part2(&prog)?;
    Ok(format!("{} {}", out1, out2))
}
