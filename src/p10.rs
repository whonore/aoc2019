use std::collections::HashSet;
use std::ops::{Add, Sub};
use std::str::FromStr;

fn in_range(x: isize, y: isize, z: isize) -> bool {
    let min = x.min(z);
    let max = x.max(z);
    (min <= y && y < max) || (min < y && y <= max)
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn colinear(self, end: Self, other: Self) -> bool {
        let diff1 = end - self;
        let diff2 = other - end;
        diff1.y * diff2.x == diff2.y * diff1.x
    }

    fn between(self, p1: Self, p2: Self) -> bool {
        self.colinear(p1, p2) && (in_range(p1.x, self.x, p2.x) || in_range(p1.y, self.y, p2.y))
    }

    fn angle(self, _other: Self) -> f64 {
        0.0
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Map(HashSet<Point>);

impl FromStr for Map {
    type Err = String;

    fn from_str(map: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            map.lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| {
                        if c == '#' {
                            Some(Point::new(x as isize, y as isize))
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        ))
    }
}

impl Map {
    fn visible_from(&self, p1: Point) -> HashSet<Point> {
        let mut visible = self.0.clone();
        loop {
            let mut remove = HashSet::new();
            for p2 in &visible {
                if *p2 == p1 {
                    continue;
                }
                remove = visible
                    .iter()
                    .filter(|p3| p2 != *p3 && p2.between(p1, **p3))
                    .copied()
                    .collect();
                if !remove.is_empty() {
                    break;
                }
            }

            if remove.is_empty() {
                break;
            } else {
                visible = visible.difference(&remove).copied().collect();
            }
        }
        visible
    }

    fn vaporize_from(&self, p: Point) -> Vaporize {
        Vaporize::new(self.clone(), p)
    }
}

struct Vaporize {
    map: Map,
    start: Point,
    visible: Vec<Point>,
}

impl Vaporize {
    fn new(map: Map, start: Point) -> Self {
        Self {
            map,
            start,
            visible: vec![],
        }
    }
}

impl Iterator for Vaporize {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.visible.is_empty() {
            let start = self.start;
            self.visible = self.map.visible_from(start).iter().copied().collect();
            self.visible.sort_unstable_by(|p1, p2| {
                start.angle(*p1).partial_cmp(&start.angle(*p2)).unwrap()
            });
            self.visible.reverse();
        }

        if let Some(v) = self.visible.pop() {
            self.map.0.remove(&v);
            Some(v)
        } else {
            None
        }
    }
}

fn part1(map: &Map) -> (Point, usize) {
    map.0
        .iter()
        .map(|p| (*p, map.visible_from(*p).len() - 1))
        .max_by_key(|(_, cnt)| *cnt)
        .unwrap()
}

fn part2(map: &Map, p: Point) -> isize {
    map.vaporize_from(p)
        .nth(199)
        .map(|p| p.x * 100 + p.y)
        .unwrap()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p10.txt");
    let map = input.parse()?;
    let (p, out1) = part1(&map);
    let out2 = part2(&map, p);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_from() {
        let map = ".#..#\n\
                   .....\n\
                   #####\n\
                   ....#\n\
                   ...##"
            .parse::<Map>()
            .unwrap();
        assert_eq!(map.visible_from(Point::new(1, 0)).len(), 8);
        assert_eq!(map.visible_from(Point::new(4, 0)).len(), 8);
        assert_eq!(map.visible_from(Point::new(0, 2)).len(), 7);
        assert_eq!(map.visible_from(Point::new(1, 2)).len(), 8);
        assert_eq!(map.visible_from(Point::new(2, 2)).len(), 8);
        assert_eq!(map.visible_from(Point::new(3, 2)).len(), 8);
        assert_eq!(map.visible_from(Point::new(4, 2)).len(), 6);
        assert_eq!(map.visible_from(Point::new(4, 3)).len(), 8);
        assert_eq!(map.visible_from(Point::new(3, 4)).len(), 9);
        assert_eq!(map.visible_from(Point::new(4, 4)).len(), 8);
    }

    #[ignore]
    #[test]
    fn test_vaporize() {
        let map = ".#....#####...#..\n\
                   ##...##.#####..##\n\
                   ##...#...#.#####.\n\
                   ..#.....#...###..\n\
                   ..#.#.....#....##"
            .parse::<Map>()
            .unwrap();
        let mut vapor = map.vaporize_from(Point::new(8, 3));
        assert_eq!(vapor.next(), Some(Point::new(8, 1)));
        assert_eq!(vapor.next(), Some(Point::new(9, 0)));
        assert_eq!(vapor.next(), Some(Point::new(9, 1)));
        assert_eq!(vapor.next(), Some(Point::new(10, 0)));
        assert_eq!(vapor.next(), Some(Point::new(11, 1)));
        assert_eq!(vapor.next(), Some(Point::new(12, 1)));
        assert_eq!(vapor.next(), Some(Point::new(11, 2)));
        assert_eq!(vapor.next(), Some(Point::new(15, 1)));
        assert_eq!(vapor.next(), Some(Point::new(12, 2)));
        assert_eq!(vapor.next(), Some(Point::new(13, 2)));
        assert_eq!(vapor.next(), Some(Point::new(14, 2)));
        assert_eq!(vapor.next(), Some(Point::new(15, 2)));
        assert_eq!(vapor.next(), Some(Point::new(11, 3)));
    }

    #[ignore]
    #[test]
    fn test01() {
        let map = ".#..#\n\
                   .....\n\
                   #####\n\
                   ....#\n\
                   ...##"
            .parse::<Map>()
            .unwrap();
        assert_eq!(part1(&map), (Point::new(3, 4), 8));
        let map = "......#.#.\n\
                   #..#.#....\n\
                   ..#######.\n\
                   .#.#.###..\n\
                   .#..#.....\n\
                   ..#....#.#\n\
                   #..#....#.\n\
                   .##.#..###\n\
                   ##...#..#.\n\
                   .#....####"
            .parse()
            .unwrap();
        assert_eq!(part1(&map), (Point::new(5, 8), 33));
        let map = "#.#...#.#.\n\
                   .###....#.\n\
                   .#....#...\n\
                   ##.#.#.#.#\n\
                   ....#.#.#.\n\
                   .##..###.#\n\
                   ..#...##..\n\
                   ..##....##\n\
                   ......#...\n\
                   .####.###."
            .parse()
            .unwrap();
        assert_eq!(part1(&map), (Point::new(1, 2), 35));
        let map = ".#..#..###\n\
                   ####.###.#\n\
                   ....###.#.\n\
                   ..###.##.#\n\
                   ##.##.#.#.\n\
                   ....###..#\n\
                   ..#.#..#.#\n\
                   #..#.#.###\n\
                   .##...##.#\n\
                   .....#.#.."
            .parse()
            .unwrap();
        assert_eq!(part1(&map), (Point::new(6, 3), 41));
        let map = ".#..##.###...#######\n\
                   ##.############..##.\n\
                   .#.######.########.#\n\
                   .###.#######.####.#.\n\
                   #####.##.#.##.###.##\n\
                   ..#####..#.#########\n\
                   ####################\n\
                   #.####....###.#.#.##\n\
                   ##.#################\n\
                   #####.##.###..####..\n\
                   ..######..##.#######\n\
                   ####.##.####...##..#\n\
                   .#####..#.######.###\n\
                   ##...#.##########...\n\
                   #.##########.#######\n\
                   .####.#.###.###.#.##\n\
                   ....##.##.###..#####\n\
                   .#.#.###########.###\n\
                   #.#.#.#####.####.###\n\
                   ###.##.####.##.#..##"
            .parse()
            .unwrap();
        assert_eq!(part1(&map), (Point::new(11, 13), 210));
    }
}
