fn digits(x: u64) -> Vec<u32> {
    x.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn valid1(min: u64, max: u64, pass: u64) -> bool {
    let ds = digits(pass);
    ds.len() == 6
        && min <= pass
        && pass <= max
        && ds.windows(2).any(|xs| xs[0] == xs[1])
        && ds.windows(2).all(|xs| xs[0] <= xs[1])
}

fn two_match(digits: &[u32]) -> bool {
    let mut last = None;
    let mut found = 1;
    for d in digits {
        if Some(d) == last {
            found += 1
        } else if found == 2 {
            break;
        } else {
            found = 1
        }
        last = Some(d)
    }
    found == 2
}

fn valid2(min: u64, max: u64, pass: u64) -> bool {
    let ds = digits(pass);
    ds.len() == 6
        && min <= pass
        && pass <= max
        && two_match(&ds)
        && ds.windows(2).all(|xs| xs[0] <= xs[1])
}

fn count_valid<F>(min: u64, max: u64, valid: F) -> usize
where
    F: Fn(u64, u64, u64) -> bool,
{
    (min..max).filter(|pass| valid(min, max, *pass)).count()
}

fn part1(min: u64, max: u64) -> usize {
    count_valid(min, max, valid1)
}

fn part2(min: u64, max: u64) -> usize {
    count_valid(min, max, valid2)
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
    let out2 = part2(minmax[0], minmax[1]);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid1() {
        assert!(valid1(100000, 999999, 111111));
        assert!(!valid1(100000, 999999, 223450));
        assert!(!valid1(100000, 999999, 123789));
    }

    #[test]
    fn test_valid2() {
        assert!(valid2(100000, 999999, 112233));
        assert!(!valid2(100000, 999999, 123444));
        assert!(valid2(100000, 999999, 111122));
    }

    #[test]
    fn test_two_match() {
        assert!(two_match(&[1, 1]));
        assert!(two_match(&[1, 1, 2]));
        assert!(two_match(&[2, 1, 1, 2]));
        assert!(!two_match(&[1, 1, 1]));
        assert!(!two_match(&[1, 1, 1, 1]));
        assert!(two_match(&[1, 1, 1, 2, 2]));
        assert!(two_match(&[2, 2, 1, 1, 1]));
        assert!(two_match(&[1, 1, 1, 2, 2, 1, 1, 1]));
    }
}
