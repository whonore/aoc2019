use std::fmt;
use std::ops::BitOr;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Pixel {
    Black,
    White,
    Transparent,
}
use Pixel::*;

impl Pixel {
    fn new(pix: u32) -> Self {
        match pix {
            0 => Black,
            1 => White,
            2 => Transparent,
            _ => unreachable!(),
        }
    }
}

impl BitOr for Pixel {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Transparent, _) => rhs,
            (_, _) => self,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Layer(Vec<Vec<Pixel>>);

impl BitOr for Layer {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(row1, row2)| row1.iter().zip(row2.iter()).map(|(x, y)| *x | *y).collect())
                .collect(),
        )
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Layers {
    width: usize,
    height: usize,
    depth: usize,
    layers: Vec<Layer>,
}

#[derive(PartialEq, Eq, Debug)]
struct Image {
    width: usize,
    height: usize,
    pixels: Layer,
}

impl Layers {
    fn new(width: usize, height: usize, pixels: &str) -> Self {
        Self {
            width,
            height,
            depth: pixels.len() / (width * height),
            layers: pixels
                .trim()
                .chars()
                .map(|c| Pixel::new(c.to_digit(10).unwrap()))
                .collect::<Vec<_>>()
                .chunks(width * height)
                .map(|layer| Layer(layer.chunks(width).map(|row| row.to_vec()).collect()))
                .collect(),
        }
    }

    fn count_pixel(&self, layer: usize, pix: Pixel) -> usize {
        self.layers[layer]
            .0
            .iter()
            .map(|row| row.iter().filter(|v| **v == pix).count())
            .sum()
    }

    fn decode(&self) -> Image {
        Image {
            width: self.width,
            height: self.height,
            pixels: self
                .layers
                .clone()
                .into_iter()
                .reduce(|x, y| x | y)
                .unwrap(),
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.pixels.0 {
            for pix in row {
                write!(
                    f,
                    "{}",
                    match pix {
                        White => "\u{2588}",
                        _ => " ",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(layers: &Layers) -> usize {
    (0..layers.depth)
        .min_by_key(|layer| layers.count_pixel(*layer, Black))
        .map(|layer| layers.count_pixel(layer, White) * layers.count_pixel(layer, Transparent))
        .unwrap()
}

fn part2(layers: &Layers) -> String {
    format!("{}", layers.decode())
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p08.txt");
    let img = Layers::new(25, 6, input);
    let out1 = part1(&img);
    let out2 = part2(&img);
    Ok(format!("{}\n{}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(
            Layers::new(2, 2, "0222112222120000").decode().pixels,
            Layer(vec![vec![Black, White], vec![White, Black]])
        );
    }
}
