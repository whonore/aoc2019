use std::collections::HashSet;
use std::str::FromStr;

type Point = (i64, i64);

const fn dist(p: Point) -> u64 {
    (p.0.abs() + p.1.abs()) as u64
}

#[derive(PartialEq, Eq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use Dir::*;

impl Dir {
    const fn go(&self, loc: Point, amt: u64) -> Point {
        match self {
            Up => (loc.0, loc.1 + amt as i64),
            Down => (loc.0, loc.1 - amt as i64),
            Left => (loc.0 - amt as i64, loc.1),
            Right => (loc.0 + amt as i64, loc.1),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Wire {
    path: Vec<(Dir, u64)>,
}

impl FromStr for Wire {
    type Err = String;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        let path = path
            .trim()
            .split(',')
            .map(|dir_amt| {
                let dir = match dir_amt.chars().next() {
                    Some('U') => Ok(Up),
                    Some('D') => Ok(Down),
                    Some('L') => Ok(Left),
                    Some('R') => Ok(Right),
                    _ => Err("Invalid dir"),
                }?;
                let amt = dir_amt[1..].parse().map_err(|_| "Invalid amount")?;
                Ok((dir, amt))
            })
            .collect::<Result<_, Self::Err>>()?;
        Ok(Self { path })
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Segment(Point, Point);

impl Iterator for Segment {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.0).0 < (self.1).0 {
            (self.0).0 += 1;
            Some(self.0)
        } else if (self.0).0 > (self.1).0 {
            (self.0).0 -= 1;
            Some(self.0)
        } else if (self.0).1 < (self.1).1 {
            (self.0).1 += 1;
            Some(self.0)
        } else if (self.0).1 > (self.1).1 {
            (self.0).1 -= 1;
            Some(self.0)
        } else {
            None
        }
    }
}

impl Wire {
    fn segs(&self) -> Vec<Segment> {
        self.path
            .iter()
            .scan((0, 0), |loc, (dir, amt)| {
                let start = *loc;
                *loc = dir.go(*loc, *amt);
                Some(Segment(start, *loc))
            })
            .collect()
    }

    fn intersect(&self, other: &Self) -> HashSet<Point> {
        let ps1 = self
            .segs()
            .iter()
            .copied()
            .flatten()
            .collect::<HashSet<Point>>();
        let ps2 = other
            .segs()
            .iter()
            .copied()
            .flatten()
            .collect::<HashSet<Point>>();
        ps1.intersection(&ps2).copied().collect()
    }

    fn steps_to(&self, p: Point) -> Option<usize> {
        self.segs()
            .iter()
            .copied()
            .flatten()
            .position(|x| x == p)
            .map(|idx| idx + 1)
    }
}

fn part1(wire1: &Wire, wire2: &Wire) -> u64 {
    wire1
        .intersect(wire2)
        .iter()
        .copied()
        .map(dist)
        .min()
        .unwrap_or(0)
}

fn part2(wire1: &Wire, wire2: &Wire) -> u64 {
    wire1
        .intersect(wire2)
        .iter()
        .map(|p| (wire1.steps_to(*p).unwrap() + wire2.steps_to(*p).unwrap()) as u64)
        .min()
        .unwrap_or(0)
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p03.txt");
    let wires = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = part1(&wires[0], &wires[1]);
    let out2 = part2(&wires[0], &wires[1]);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segs() {
        assert_eq!(
            Wire {
                path: vec![(Up, 7), (Right, 6), (Down, 4), (Left, 4)]
            }
            .segs(),
            vec![
                Segment((0, 0), (0, 7)),
                Segment((0, 7), (6, 7)),
                Segment((6, 7), (6, 3)),
                Segment((6, 3), (2, 3))
            ]
        );
    }

    #[test]
    fn test_intersect() {
        let w1 = Wire {
            path: vec![(Right, 8), (Up, 5), (Left, 5), (Down, 3)],
        };
        let w2 = Wire {
            path: vec![(Up, 7), (Right, 6), (Down, 4), (Left, 4)],
        };
        assert_eq!(
            w1.intersect(&w2),
            [(3, 3), (6, 5)].iter().copied().collect()
        )
    }

    #[test]
    fn test01() {
        assert_eq!(
            part1(
                &Wire {
                    path: vec![(Right, 8), (Up, 5), (Left, 5), (Down, 3)]
                },
                &Wire {
                    path: vec![(Up, 7), (Right, 6), (Down, 4), (Left, 4)]
                }
            ),
            6
        );
        assert_eq!(
            part1(
                &Wire {
                    path: vec![
                        (Right, 75),
                        (Down, 30),
                        (Right, 83),
                        (Up, 83),
                        (Left, 12),
                        (Down, 49),
                        (Right, 71),
                        (Up, 7),
                        (Left, 72)
                    ]
                },
                &Wire {
                    path: vec![
                        (Up, 62),
                        (Right, 66),
                        (Up, 55),
                        (Right, 34),
                        (Down, 71),
                        (Right, 55),
                        (Down, 58),
                        (Right, 83)
                    ]
                }
            ),
            159
        );
        assert_eq!(
            part1(
                &Wire {
                    path: vec![
                        (Right, 98),
                        (Up, 47),
                        (Right, 26),
                        (Down, 63),
                        (Right, 33),
                        (Up, 87),
                        (Left, 62),
                        (Down, 20),
                        (Right, 33),
                        (Up, 53),
                        (Right, 51)
                    ]
                },
                &Wire {
                    path: vec![
                        (Up, 98),
                        (Right, 91),
                        (Down, 20),
                        (Right, 16),
                        (Down, 67),
                        (Right, 40),
                        (Up, 7),
                        (Right, 15),
                        (Up, 6),
                        (Right, 7)
                    ]
                }
            ),
            135
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            part2(
                &Wire {
                    path: vec![(Right, 8), (Up, 5), (Left, 5), (Down, 3)]
                },
                &Wire {
                    path: vec![(Up, 7), (Right, 6), (Down, 4), (Left, 4)]
                }
            ),
            30
        );
        assert_eq!(
            part2(
                &Wire {
                    path: vec![
                        (Right, 75),
                        (Down, 30),
                        (Right, 83),
                        (Up, 83),
                        (Left, 12),
                        (Down, 49),
                        (Right, 71),
                        (Up, 7),
                        (Left, 72)
                    ]
                },
                &Wire {
                    path: vec![
                        (Up, 62),
                        (Right, 66),
                        (Up, 55),
                        (Right, 34),
                        (Down, 71),
                        (Right, 55),
                        (Down, 58),
                        (Right, 83)
                    ]
                }
            ),
            610
        );
        assert_eq!(
            part2(
                &Wire {
                    path: vec![
                        (Right, 98),
                        (Up, 47),
                        (Right, 26),
                        (Down, 63),
                        (Right, 33),
                        (Up, 87),
                        (Left, 62),
                        (Down, 20),
                        (Right, 33),
                        (Up, 53),
                        (Right, 51)
                    ]
                },
                &Wire {
                    path: vec![
                        (Up, 98),
                        (Right, 91),
                        (Down, 20),
                        (Right, 16),
                        (Down, 67),
                        (Right, 40),
                        (Up, 7),
                        (Right, 15),
                        (Up, 6),
                        (Right, 7)
                    ]
                }
            ),
            410
        );
    }
}
