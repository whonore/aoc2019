fn part1() -> u64 {
    0
}

fn part2() -> u64 {
    0
}

pub fn run() -> Result<String, String> {
    let _input = include_str!("input/p24.txt");
    let out1 = part1();
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(part1(), 0);
    }
}
