use std::collections::HashMap;
use std::ops::Neg;

use crate::intcode::Intcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

impl From<i64> for Turn {
    fn from(t: i64) -> Self {
        match t {
            0 => Self::Left,
            1 => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    const fn turn(self, t: Turn) -> Self {
        match (self, t) {
            (Self::Up, Turn::Left) | (Self::Down, Turn::Right) => Self::Left,
            (Self::Down, Turn::Left) | (Self::Up, Turn::Right) => Self::Right,
            (Self::Left, Turn::Left) | (Self::Right, Turn::Right) => Self::Down,
            (Self::Right, Turn::Left) | (Self::Left, Turn::Right) => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Black = 0,
    White = 1,
}
use Color::*;

impl From<i64> for Color {
    fn from(c: i64) -> Self {
        match c {
            0 => Black,
            1 => White,
            _ => unreachable!(),
        }
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Black => White,
            White => Black,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Robot {
    x: i64,
    y: i64,
    dir: Dir,
    visited: HashMap<(i64, i64), Color>,
}

impl Default for Robot {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Dir::Up,
            visited: HashMap::new(),
        }
    }
}

impl Robot {
    fn color(&self) -> Color {
        self.visited
            .get(&(self.x, self.y))
            .copied()
            .unwrap_or(Black)
    }

    fn step(&mut self, c: Color, t: Turn) {
        self.visited.insert((self.x, self.y), c);
        self.dir = self.dir.turn(t);
        match self.dir {
            Dir::Up => self.y += 1,
            Dir::Down => self.y -= 1,
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1,
        }
    }

    fn run(&mut self, prog: &Intcode) -> Result<(), String> {
        let mut prog = prog.exec().read_vec(&[]).write_to(vec![]);
        loop {
            prog.read_next(&[self.color() as i64]);
            if let Some(color) = prog.run_to_out()? {
                let turn = prog
                    .run_to_out()?
                    .ok_or("Program didn't return a direction")?;
                self.step(Color::from(color), Turn::from(turn));
            } else {
                break;
            }
        }
        Ok(())
    }
}

fn part1(prog: &Intcode) -> Result<usize, String> {
    let mut robot = Robot::default();
    robot.run(prog)?;
    Ok(robot.visited.len())
}

fn part2(prog: &Intcode) -> Result<String, String> {
    let mut robot = Robot::default();
    robot.visited.insert((0, 0), White);
    robot.run(prog)?;

    let xmin = robot.visited.keys().min_by_key(|(x, _)| x).unwrap().0;
    let xmax = robot.visited.keys().max_by_key(|(x, _)| x).unwrap().0;
    let ymin = robot.visited.keys().min_by_key(|(_, y)| y).unwrap().1;
    let ymax = robot.visited.keys().max_by_key(|(_, y)| y).unwrap().1;
    let width = 1 + (xmax - xmin) as usize;
    let height = 1 + (ymax - ymin) as usize;

    let mut grid = [" "]
        .repeat(width * height)
        .chunks(width)
        .map(|row| row.to_vec())
        .collect::<Vec<Vec<_>>>();
    for ((x, y), color) in robot.visited {
        if color == White {
            grid[(y - ymin) as usize][(x - xmin) as usize] = "\u{2588}";
        }
    }

    grid.reverse();
    Ok(grid
        .iter()
        .map(|row| row.join(""))
        .collect::<Vec<_>>()
        .join("\n"))
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p11.txt");
    let prog = input.parse()?;
    let out1 = part1(&prog)?;
    let out2 = part2(&prog)?;
    Ok(format!("{}\n{}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut robot = Robot::default();
        dbg!(robot.visited.clone());
        robot.step(White, Turn::Left);
        dbg!(robot.visited.clone());
        robot.step(Black, Turn::Left);
        dbg!(robot.visited.clone());
        robot.step(White, Turn::Left);
        dbg!(robot.visited.clone());
        robot.step(White, Turn::Left);
        dbg!(robot.visited.clone());
        robot.step(Black, Turn::Right);
        dbg!(robot.visited.clone());
        robot.step(White, Turn::Left);
        dbg!(robot.visited.clone());
        robot.step(White, Turn::Left);
        dbg!(robot.visited.clone());
        assert_eq!(robot.visited.len(), 6);
    }
}
