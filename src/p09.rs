fn solve() -> u64 {
    0
}

pub fn run() -> Result<String, String> {
    let _input = include_str!("input/p09.txt");
    let out1 = solve();
    let out2 = solve();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(solve(), 0);
    }
}
