use crate::intcode::Intcode;

fn part1(data: &[i64]) -> Result<i64, String> {
    let mut prog = Intcode::empty_io(data.to_vec());
    prog.run_with(&[(1, 12), (2, 2)])?;
    Ok(prog.data[0])
}

fn part2(data: &[i64]) -> Result<i64, String> {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut prog = Intcode::empty_io(data.to_vec());
            if prog.run_with(&[(1, noun), (2, verb)]).is_ok() && prog.data[0] == 19_690_720 {
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
        .map_err(|_| "Invalid input")?;
    let out1 = part1(&data)?;
    let out2 = part2(&data)?;
    Ok(format!("{} {}", out1, out2))
}
