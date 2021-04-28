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
    fn turn(self, t: Turn) -> Self {
        match (self, t) {
            (Self::Up, Turn::Left) => Self::Left,
            (Self::Down, Turn::Left) => Self::Right,
            (Self::Left, Turn::Left) => Self::Down,
            (Self::Right, Turn::Left) => Self::Up,
            (Self::Up, Turn::Right) => Self::Right,
            (Self::Down, Turn::Right) => Self::Left,
            (Self::Left, Turn::Right) => Self::Up,
            (Self::Right, Turn::Right) => Self::Down,
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
}

fn part1(prog: &Intcode) -> Result<usize, String> {
    let mut robot = Robot::default();
    let mut prog = prog.exec().read_vec(&[]).write_to(vec![]);
    loop {
        prog.read_next(&[robot.color() as i64]);
        if let Some(color) = prog.run_to_out()? {
            let turn = prog
                .run_to_out()?
                .ok_or_else(|| "Program didn't return a direction")?;
            robot.step(Color::from(color), Turn::from(turn));
        } else {
            break;
        }
    }
    Ok(robot.visited.len())
}

fn part2() -> u64 {
    0
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p11.txt");
    let prog = input.parse()?;
    let out1 = part1(&prog)?;
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
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
