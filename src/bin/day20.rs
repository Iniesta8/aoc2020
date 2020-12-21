use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<char>>,
    // top_edge: Vec<char>,
    // bottom_edge: Vec<char>,
    // left_edge: Vec<char>,
    // right_edge: Vec<char>,
}

impl Tile {
    fn from_raw_data(data: &str) -> Self {
        dbg!(&data);
        let lines: Vec<&str> = data.lines().collect();

        let id: usize = lines[0]
            .replace(':', " ")
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let pixels: Vec<Vec<char>> = lines
            .iter()
            .skip(1)
            .map(|line| line.chars().collect())
            .collect();

        Self {
            id: id,
            pixels: pixels,
            // top_edge: vec![],
            // bottom_edge: vec![],
            // left_edge: vec![],
            // right_edge: vec![],
        }
    }

    // fn get_edges(&self) {
    // self.top_edge = self.pixels[0].clone();
    // self.bottom_edge = self.pixels[self.pixels[0].len() - 1].clone();
    //
    // self.left_edge = self.pixels.iter().rev().map(|row| row[0]).collect();
    // self.right_edge = self
    // .pixels
    // .iter()
    // .rev()
    // .map(|row| row[row.len() - 1])
    // .collect();
    // }
}

struct Solution;

impl Solution {
    fn part1() -> usize {
        unimplemented!()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day20.txt").expect("File not found!");
    let tiles: Vec<Tile> = input
        .trim()
        .split("\n\n")
        .map(Tile::from_raw_data)
        .collect();

    dbg!(&tiles.len()); // 144 -> square of 12x12

    let timer = Instant::now();
    println!("p1: {} (runtime: {:?})", Solution::part1(), timer.elapsed());
}
