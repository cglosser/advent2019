use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day08 {}

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> String {
        let image = Image::new(
            Shape { cols: 25, rows: 6 },
            input
                .trim()
                .chars()
                .map(|c| match c {
                    '0' => Color::Black,
                    '1' => Color::White,
                    _ => Color::Transparent,
                })
                .collect_vec(),
        );

        image.one_two_product().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let image = Image::new(
            Shape { cols: 25, rows: 6 },
            input
                .trim()
                .chars()
                .map(|c| match c {
                    '0' => Color::Black,
                    '1' => Color::White,
                    _ => Color::Transparent,
                })
                .collect_vec(),
        );

        let s = image
            .stacked()
            .chunks(image.shape.cols)
            .map(|row| {
                row.iter()
                    .map(|px| match px {
                        Color::White => 'X',
                        _ => ' ',
                    })
                    .join("")
            })
            .join("\n");

        format!("\n{}\n", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Black = 0,
    White = 1,
    Transparent = 2,
}

struct Shape {
    cols: usize,
    rows: usize,
}

struct Image {
    shape: Shape,
    pixels: Vec<Color>,
}

impl Image {
    fn new(shape: Shape, pixels: Vec<Color>) -> Self {
        Image {
            shape: shape,
            pixels: pixels,
        }
    }

    fn one_two_product(&self) -> i32 {
        let size = self.shape.rows * self.shape.cols;
        let min_zero_layer = self
            .pixels
            .chunks(size as usize)
            .min_by_key(|&a| a.iter().filter(|&px| *px == Color::Black).count())
            .unwrap();

        (min_zero_layer
            .iter()
            .filter(|&px| *px == Color::White)
            .count()
            * min_zero_layer
                .iter()
                .filter(|&px| *px == Color::Transparent)
                .count()) as i32
    }

    fn stacked(&self) -> Vec<Color> {
        let size = self.shape.cols * self.shape.rows;
        let mut px: Vec<Color> = vec![Color::Transparent; size];

        for pixel_idx in 0..size {
            for layer_idx in 0..(self.pixels.len() / size) {
                let i = layer_idx * size + pixel_idx;
                if self.pixels[i] != Color::Transparent {
                    px[pixel_idx] = self.pixels[i];
                    break;
                }
            }
        }

        px
    }
}
