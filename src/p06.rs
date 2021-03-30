use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
struct Orbits(HashMap<String, String>);

#[derive(PartialEq, Eq, Debug)]
struct Parents<'a> {
    orbits: &'a Orbits,
    obj: &'a str,
    dist: u64,
}

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

    fn parents<'a>(&'a self, obj: &'a str) -> Parents<'a> {
        Parents {
            orbits: self,
            obj,
            dist: 0,
        }
    }

    fn distance(&self, obj1: &str, obj2: &str) -> u64 {
        let parents1 = self.parents(obj1).collect::<HashMap<_, _>>();
        self.parents(obj2)
            .filter(|(p, _)| parents1.contains_key(p))
            .min_by_key(|&(_, d)| d)
            .map(|(common, d)| d + parents1[&common])
            .unwrap_or(0)
    }
}

impl Iterator for Parents<'_> {
    type Item = (String, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.orbits.0.contains_key(self.obj) {
            self.obj = &self.orbits.0[self.obj];
            self.dist += 1;
            Some((self.obj.into(), self.dist))
        } else {
            None
        }
    }
}

fn part1(orbits: &Orbits) -> u64 {
    orbits.0.keys().map(|obj| orbits.depth(obj)).sum()
}

fn part2(orbits: &Orbits) -> u64 {
    orbits.distance("YOU", "SAN") - 2
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p06.txt");
    let orbits = input.parse()?;
    let out1 = part1(&orbits);
    let out2 = part2(&orbits);
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

    #[test]
    fn test02() {
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
                      K)L\n\
                      K)YOU\n\
                      I)SAN"
            .parse::<Orbits>()
            .unwrap();
        assert_eq!(part2(&orbits), 4);
    }
}
