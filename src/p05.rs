use crate::intcode::Intcode;

fn part1(data: &[i64]) -> Result<i64, String> {
    let input = &1_i64.to_le_bytes()[..];
    let mut prog = Intcode::vec_out(data.to_vec(), input);
    prog.run()?;
    let out = prog.read_out();
    if out[..out.len().saturating_sub(1)].iter().all(|x| *x == 0) {
        out.last().copied().ok_or_else(|| "No output".into())
    } else {
        Err("Failed diagnostic".into())
    }
}

fn part2(data: &[i64]) -> Result<i64, String> {
    Ok(0)
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p05.txt");
    let data = input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Invalid input")?;
    let out1 = part1(&data)?;
    let out2 = part2(&data)?;
    Ok(format!("{} {}", out1, out2))
}
