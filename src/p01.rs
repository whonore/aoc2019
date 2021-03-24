fn calc_fuel(mass: u64) -> u64 {
    mass / 3 - 2
}

fn solve(masses: &[u64]) -> u64 {
    masses.iter().map(|m| calc_fuel(*m)).sum()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p01.txt");
    let masses = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Bad input")?;
    let out1 = part1(&masses);
    let out2 = part2(&[]);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100756), 33583);
    }
}
