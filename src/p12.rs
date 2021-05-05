use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::Sum;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

fn cmp(x: i64, y: i64) -> i64 {
    match x.cmp(&y) {
        Ordering::Greater => -1,
        Ordering::Less => 1,
        Ordering::Equal => 0,
    }
}

fn gcd(x: u64, y: u64) -> u64 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

fn lcm(x: u64, y: u64) -> u64 {
    (x * y) / gcd(x, y)
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

type Vector = Point;

impl FromStr for Point {
    type Err = String;

    fn from_str(trip: &str) -> Result<Self, Self::Err> {
        let pts = trip
            .trim_matches(&['<', '>'][..])
            .split(',')
            .map(|comp| {
                comp.split('=')
                    .nth(1)
                    .map(|x| x.parse::<i64>().map_err(|_| format!("Bad int {}", x)))
                    .unwrap_or_else(|| Err(format!("Bad field {}", comp)))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            x: pts[0],
            y: pts[1],
            z: pts[2],
        })
    }
}

impl From<(i64, i64, i64)> for Point {
    fn from(pt: (i64, i64, i64)) -> Self {
        Self {
            x: pt.0,
            y: pt.1,
            z: pt.2,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sum for Point {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Point::default(), |acc, pt| acc + pt)
    }
}

impl Point {
    fn cmp(&self, other: &Self) -> Self {
        Self {
            x: cmp(self.x, other.x),
            y: cmp(self.y, other.y),
            z: cmp(self.z, other.z),
        }
    }

    fn abs_sum(&self) -> u64 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u64
    }
}

#[derive(Debug, Clone, Copy)]
struct Body {
    pos: Point,
    vel: Vector,
}

impl Body {
    fn new(pos: Point) -> Self {
        Self {
            pos,
            vel: Point::default(),
        }
    }

    fn cmp_pos(&self, other: &Body) -> Point {
        self.pos.cmp(&other.pos)
    }

    fn step(&mut self) {
        self.pos += self.vel;
    }

    fn potential(&self) -> u64 {
        self.pos.abs_sum()
    }

    fn kinetic(&self) -> u64 {
        self.vel.abs_sum()
    }

    fn energy(&self) -> u64 {
        self.potential() * self.kinetic()
    }
}

fn step(moons: &mut [Body]) {
    let diffs = moons
        .iter()
        .map(|moon| moons.iter().map(|other| moon.cmp_pos(other)).sum())
        .collect::<Vec<_>>();
    for (moon, diff) in moons.iter_mut().zip(diffs) {
        moon.vel += diff;
    }
    for moon in moons.iter_mut() {
        moon.step();
    }
}

fn step_until_repeat<F>(moons: &mut [Body], axis: F) -> u64
where
    F: Fn(&Body) -> (i64, i64),
{
    let mut seen = HashSet::<Vec<(i64, i64)>>::new();
    for cnt in 0.. {
        let axes = moons.iter().map(|moon| axis(moon)).collect::<Vec<_>>();
        if seen.contains(&axes) {
            return cnt;
        }
        seen.insert(axes);
        step(moons);
    }
    unreachable!();
}

fn part1(mut moons: Vec<Body>, steps: u64) -> u64 {
    for _ in 0..steps {
        step(&mut moons);
    }
    moons.iter().map(|moon| moon.energy()).sum()
}

fn part2(mut moons: Vec<Body>) -> u64 {
    let x_cnt = step_until_repeat(&mut moons.clone(), |moon| (moon.pos.x, moon.vel.x));
    let y_cnt = step_until_repeat(&mut moons.clone(), |moon| (moon.pos.y, moon.vel.y));
    let z_cnt = step_until_repeat(&mut moons, |moon| (moon.pos.z, moon.vel.z));
    [x_cnt, y_cnt, z_cnt].iter().copied().fold(1, lcm)
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p12.txt");
    let moons = input
        .lines()
        .map(|line| Ok(Body::new(line.parse()?)))
        .collect::<Result<Vec<_>, String>>()?;
    let out1 = part1(moons.clone(), 1000);
    let out2 = part2(moons);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let moons = vec![
            Body::new(Point::from((-1, 0, 2))),
            Body::new(Point::from((2, -10, -7))),
            Body::new(Point::from((4, -8, 8))),
            Body::new(Point::from((3, 5, -1))),
        ];
        assert_eq!(part1(moons, 10), 179);
        let moons = vec![
            Body::new(Point::from((-8, -10, 0))),
            Body::new(Point::from((5, 5, 10))),
            Body::new(Point::from((2, -7, 3))),
            Body::new(Point::from((9, -8, -3))),
        ];
        assert_eq!(part1(moons, 100), 1940);
    }

    #[test]
    fn test02() {
        let moons = vec![
            Body::new(Point::from((-8, -10, 0))),
            Body::new(Point::from((5, 5, 10))),
            Body::new(Point::from((2, -7, 3))),
            Body::new(Point::from((9, -8, -3))),
        ];
        assert_eq!(part2(moons), 4_686_774_924);
    }
}
