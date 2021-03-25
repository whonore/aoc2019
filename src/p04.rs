fn digits(x: u64) -> Vec<u32> {
    x.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

#[derive(PartialEq, Eq, Debug)]
struct Criteria {
    min: u64,
    max: u64,
}

impl Criteria {
    fn new(min: u64, max: u64) -> Self {
        Self { min, max }
    }

    fn valid(&self, pass: u64) -> bool {
        let ds = digits(pass);
        ds.len() == 6
            && self.min <= pass
            && pass <= self.max
            && ds.windows(2).any(|xs| xs[0] == xs[1])
            && ds.windows(2).all(|xs| xs[0] <= xs[1])
    }

    fn count(&self) -> usize {
        (self.min..self.max)
            .filter(|pass| self.valid(*pass))
            .count()
    }
}

fn part1(min: u64, max: u64) -> usize {
    Criteria::new(min, max).count()
}

fn part2() -> u64 {
    0
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p04.txt");
    let minmax = input
        .trim()
        .split('-')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Invalid input")?;
    let out1 = part1(minmax[0], minmax[1]);
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let crit = Criteria::new(100000, 999999);
        assert!(crit.valid(111111));
        assert!(!crit.valid(223450));
        assert!(!crit.valid(123789));
    }
}
