use std::collections::HashMap;
use std::str::FromStr;

struct Orbits(HashMap<String, String>);

impl FromStr for Orbits {
    type Err = String;

    fn from_str(orbits: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            orbits
                .lines()
                .map(|orbit| {
                    let objs = orbit.split(')').collect::<Vec<_>>();
                    if objs.len() == 2 {
                        Ok((objs[1].into(), objs[0].into()))
                    } else {
                        Err("Invalid orbit".into())
                    }
                })
                .collect::<Result<_, Self::Err>>()?,
        ))
    }
}

impl Orbits {
    fn depth(&self, obj: &str) -> u64 {
        if self.0.contains_key(obj) {
            1 + self.depth(&self.0[obj])
        } else {
            0
        }
    }
}

fn part1(orbits: &Orbits) -> u64 {
    orbits.0.keys().map(|obj| orbits.depth(obj)).sum()
}

fn part2() -> u64 {
    0
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p06.txt");
    let orbits = input.parse()?;
    let out1 = part1(&orbits);
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let orbits = "COM)B\n\
                      B)C\n\
                      C)D\n\
                      D)E\n\
                      E)F\n\
                      B)G\n\
                      G)H\n\
                      D)I\n\
                      E)J\n\
                      J)K\n\
                      K)L"
        .parse::<Orbits>()
        .unwrap();
        assert_eq!(orbits.depth("D"), 3);
        assert_eq!(orbits.depth("L"), 7);
        assert_eq!(part1(&orbits), 42);
    }
}
