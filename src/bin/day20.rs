use std::{collections::HashMap, time::Instant};
use std::{collections::HashSet, fs};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<char>>,
    top_border: Vec<char>,
    bottom_border: Vec<char>,
    left_border: Vec<char>,
    right_border: Vec<char>,
}

impl Tile {
    fn default() -> Self {
        Self {
            id: 0,
            pixels: vec![vec!['.'; 10]; 10],
            top_border: vec!['.'; 10],
            bottom_border: vec!['.'; 10],
            left_border: vec!['.'; 10],
            right_border: vec!['.'; 10],
        }
    }
    fn from_raw_data(data: &str) -> Self {
        // dbg!(&data);
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

        let top_border = pixels[0].clone();
        let bottom_border = pixels[pixels.len() - 1].clone();
        let left_border = pixels.iter().map(|l| l[0]).collect();
        let right_border = pixels.iter().map(|l| l[l.len() - 1]).collect();

        Self {
            id,
            pixels,
            top_border,
            bottom_border,
            left_border,
            right_border,
        }
    }

    fn flip(&mut self) {
        let origin = self.pixels.clone();
        for y in 0..origin.len() {
            for x in 0..origin[y].len() {
                self.pixels[y][x] = origin[y][origin.len() - x - 1];
            }
        }

        std::mem::swap(&mut self.right_border, &mut self.left_border);
        self.top_border = Self::reverse_border(&self.top_border);
        self.bottom_border = Self::reverse_border(&self.bottom_border);
    }

    fn rotate(&mut self) {
        let origin = self.clone();
        for y in 0..origin.pixels.len() {
            for x in 0..origin.pixels[y].len() {
                self.pixels[y][x] = origin.pixels[origin.pixels.len() - x - 1][y];
            }
        }

        self.right_border = origin.top_border;
        self.top_border = Self::reverse_border(&origin.left_border);
        self.left_border = origin.bottom_border;
        self.bottom_border = Self::reverse_border(&origin.right_border);
    }

    fn reverse_border(border: &[char]) -> Vec<char> {
        border.iter().rev().copied().collect()
    }

    fn options(tile: &Tile) -> Vec<Tile> {
        let tile_r0 = tile.clone();
        let mut tile_r1 = tile_r0.clone();
        tile_r1.rotate();
        let mut tile_r2 = tile_r1.clone();
        tile_r2.rotate();
        let mut tile_r3 = tile_r2.clone();
        tile_r3.rotate();

        let mut tile_f0 = tile_r0.clone();
        tile_f0.flip();
        let mut tile_f1 = tile_r1.clone();
        tile_f1.flip();
        let mut tile_f2 = tile_r2.clone();
        tile_f2.flip();
        let mut tile_f3 = tile_r3.clone();
        tile_f3.flip();

        vec![
            tile_r0, tile_r1, tile_r2, tile_r3, tile_f0, tile_f1, tile_f2, tile_f3,
        ]
    }
}

fn solve_puzzle(tiles: &[Tile]) -> HashMap<(i32, i32), Tile> {
    let mut final_puzzle: HashMap<(i32, i32), Tile> = HashMap::new();

    let mut unplaced_tiles = HashSet::new();
    unplaced_tiles.insert(((0, 0), tiles[0].clone()));

    let mut unused_tiles: HashSet<Tile> = tiles.iter().skip(1).cloned().collect();

    while let Some(((cur_x, cur_y), cur_tile)) = unplaced_tiles.iter().cloned().next() {
        for unused_tile in unused_tiles.clone().iter() {
            for option in Tile::options(&unused_tile) {
                if option.left_border == cur_tile.right_border {
                    unplaced_tiles.insert(((cur_x + 1, cur_y), option));
                    unused_tiles.remove(&unused_tile);
                    break;
                } else if option.right_border == cur_tile.left_border {
                    unplaced_tiles.insert(((cur_x - 1, cur_y), option));
                    unused_tiles.remove(&unused_tile);
                    break;
                } else if option.bottom_border == cur_tile.top_border {
                    unplaced_tiles.insert(((cur_x, cur_y - 1), option));
                    unused_tiles.remove(&unused_tile);
                    break;
                } else if option.top_border == cur_tile.bottom_border {
                    unplaced_tiles.insert(((cur_x, cur_y + 1), option));
                    unused_tiles.remove(&unused_tile);
                    break;
                }
            }
        }
        unplaced_tiles.remove(&((cur_x, cur_y), cur_tile.clone()));
        final_puzzle.insert((cur_x, cur_y), cur_tile);
    }

    final_puzzle
}

fn relocate(puzzle_map: &HashMap<(i32, i32), Tile>) -> Vec<Vec<Tile>> {
    let min_x = puzzle_map.iter().min_by_key(|(pos, _)| pos.0).unwrap().0 .0;
    let max_x = puzzle_map.iter().max_by_key(|(pos, _)| pos.0).unwrap().0 .0;
    let min_y = puzzle_map.iter().min_by_key(|(pos, _)| pos.1).unwrap().0 .1;
    let max_y = puzzle_map.iter().max_by_key(|(pos, _)| pos.1).unwrap().0 .1;

    let mut relocated_puzzle =
        vec![vec![Tile::default(); (max_y - min_y + 1) as usize]; (max_x - min_x + 1) as usize];

    let dx = if min_x <= 0 { -min_x } else { min_x };
    let dy = if min_y <= 0 { -min_y } else { min_y };

    for ((xi, yi), cur_tile) in puzzle_map.iter() {
        relocated_puzzle[(xi + dx) as usize][(yi + dy) as usize] = cur_tile.clone();
    }

    relocated_puzzle
}

struct Solution;

impl Solution {
    fn part1(tiles: &[Tile]) -> usize {
        let solved_puzzle: HashMap<(i32, i32), Tile> = solve_puzzle(&tiles);
        let relocated_puzzle = relocate(&solved_puzzle);

        let puzzle_width = relocated_puzzle[0].len();
        let puzzle_height = relocated_puzzle.len();

        relocated_puzzle[0][0].id
            * relocated_puzzle[0][puzzle_width - 1].id
            * relocated_puzzle[puzzle_height - 1][puzzle_width - 1].id
            * relocated_puzzle[puzzle_height - 1][0].id
    }
}

fn main() {
    let input = fs::read_to_string("./input/day20.txt").expect("File not found!");
    let tiles: Vec<Tile> = input
        .trim()
        .split("\n\n")
        .map(Tile::from_raw_data)
        .collect();

    // dbg!(&tiles.len()); // 144 -> square of 12x12

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&tiles),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day20() {
        let input = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        let tiles: Vec<Tile> = input
            .trim()
            .split("\n\n")
            .map(Tile::from_raw_data)
            .collect();

        assert_eq!(Solution::part1(&tiles), 20899048083289);
    }
}
