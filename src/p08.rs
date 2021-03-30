type Layer = Vec<Vec<u32>>;

struct Image {
    width: usize,
    height: usize,
    depth: usize,
    layers: Vec<Layer>,
}

impl Image {
    fn new(width: usize, height: usize, pixels: &str) -> Self {
        Self {
            width,
            height,
            depth: pixels.len() / (width * height),
            layers: pixels
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
                .chunks(width * height)
                .map(|layer| layer.chunks(width).map(|row| row.to_vec()).collect())
                .collect(),
        }
    }

    fn count_digit(&self, layer: usize, digit: u32) -> usize {
        self.layers[layer]
            .iter()
            .map(|row| row.iter().filter(|v| **v == digit).count())
            .sum()
    }
}

fn part1(img: &Image) -> usize {
    (0..img.depth)
        .min_by_key(|layer| img.count_digit(*layer, 0))
        .map(|layer| img.count_digit(layer, 1) * img.count_digit(layer, 2))
        .unwrap()
}

fn part2() -> u64 {
    0
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p08.txt");
    let img = Image::new(25, 6, input);
    let out1 = part1(&img);
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(part1(), 0);
    }
}
