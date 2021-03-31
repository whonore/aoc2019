const fn calc_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

const fn calc_fuel_all(mass: u64) -> u64 {
    let mut tot = 0;
    let mut mass = mass;
    while mass > 0 {
        mass = calc_fuel(mass);
        tot += mass;
    }
    tot
}

fn part1(masses: &[u64]) -> u64 {
    masses.iter().copied().map(calc_fuel).sum()
}

fn part2(masses: &[u64]) -> u64 {
    masses.iter().copied().map(calc_fuel_all).sum()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p01.txt");
    let masses = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Invalid input")?;
    let out1 = part1(&masses);
    let out2 = part2(&masses);
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

    #[test]
    fn test02() {
        assert_eq!(calc_fuel_all(14), 2);
        assert_eq!(calc_fuel_all(1969), 966);
        assert_eq!(calc_fuel_all(100756), 50346);
    }
}
