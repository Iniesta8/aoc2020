use std::{collections::HashMap, fmt::Display, time::Instant};
use std::{collections::HashSet, fs};

enum ImageProcessingMode {
    Flip,
    Rotate,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<char>>,
    top_border: Vec<char>,
    bottom_border: Vec<char>,
    left_border: Vec<char>,
    right_border: Vec<char>,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "id: {}", self.id)?;
        for i in 0..self.pixels.len() {
            for j in 0..self.pixels[0].len() {
                write!(f, "{}", self.pixels[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Tile {
    fn default() -> Self {
        Self {
            id: 0,
            pixels: vec![vec![' '; 10]; 10],
            top_border: vec![' '; 10],
            bottom_border: vec![' '; 10],
            left_border: vec![' '; 10],
            right_border: vec![' '; 10],
        }
    }
    fn from_raw_data(data: &str) -> Self {
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
        let origin = self.clone();
        self.pixels = process_image(ImageProcessingMode::Flip, &origin.pixels);
        std::mem::swap(&mut self.right_border, &mut self.left_border);
        self.top_border = Self::reverse_border(&origin.top_border);
        self.bottom_border = Self::reverse_border(&origin.bottom_border);
    }

    fn rotate(&mut self) {
        let origin = self.clone();
        self.pixels = process_image(ImageProcessingMode::Rotate, &origin.pixels);
        self.right_border = origin.top_border;
        self.top_border = Self::reverse_border(&origin.left_border);
        self.left_border = origin.bottom_border;
        self.bottom_border = Self::reverse_border(&origin.right_border);
    }

    fn reverse_border(border: &[char]) -> Vec<char> {
        border.iter().rev().copied().collect()
    }

    fn orientation_options(tile: &Tile) -> Vec<Tile> {
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

    fn remove_borders(tile: &Tile) -> Vec<Vec<char>> {
        let mut borderless = vec![vec![' '; tile.pixels[0].len() - 2]; tile.pixels.len() - 2];

        for y in 1..tile.pixels.len() - 1 {
            for x in 1..tile.pixels[0].len() - 1 {
                borderless[x - 1][y - 1] = tile.pixels[x][y];
            }
        }

        borderless
    }
}

fn process_image(mode: ImageProcessingMode, origin: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut processed = match mode {
        ImageProcessingMode::Flip => origin.to_vec(),
        ImageProcessingMode::Rotate => {
            vec![vec![' '; origin.len()]; origin[0].len()]
        }
    };

    for y in 0..processed.len() {
        for x in 0..processed[0].len() {
            match mode {
                ImageProcessingMode::Flip => {
                    processed[y][x] = origin[y][processed[y].len() - x - 1]
                }
                ImageProcessingMode::Rotate => {
                    processed[y][x] = origin[processed[y].len() - x - 1][y]
                }
            }
        }
    }
    processed
}

fn solve_puzzle(tiles: &[Tile]) -> HashMap<(i32, i32), Tile> {
    let mut final_puzzle: HashMap<(i32, i32), Tile> = HashMap::new();

    let mut unplaced_tiles = HashSet::new();
    unplaced_tiles.insert(((0, 0), tiles[0].clone()));

    let mut unused_tiles: HashSet<Tile> = tiles.iter().skip(1).cloned().collect();

    while let Some(((cur_x, cur_y), cur_tile)) = unplaced_tiles.iter().next().cloned() {
        for unused_tile in unused_tiles.clone().iter() {
            for orientation in Tile::orientation_options(unused_tile) {
                if orientation.left_border == cur_tile.right_border {
                    unplaced_tiles.insert(((cur_x + 1, cur_y), orientation));
                    unused_tiles.remove(unused_tile);
                    break;
                } else if orientation.right_border == cur_tile.left_border {
                    unplaced_tiles.insert(((cur_x - 1, cur_y), orientation));
                    unused_tiles.remove(unused_tile);
                    break;
                } else if orientation.bottom_border == cur_tile.top_border {
                    unplaced_tiles.insert(((cur_x, cur_y - 1), orientation));
                    unused_tiles.remove(unused_tile);
                    break;
                } else if orientation.top_border == cur_tile.bottom_border {
                    unplaced_tiles.insert(((cur_x, cur_y + 1), orientation));
                    unused_tiles.remove(unused_tile);
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
        vec![vec![Tile::default(); (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    for ((xi, yi), cur_tile) in puzzle_map.iter() {
        relocated_puzzle[(yi - min_y) as usize][(xi - min_x) as usize] = cur_tile.clone();
    }

    relocated_puzzle
}

#[derive(Clone)]
struct SeaMonster {
    pixels: Vec<Vec<char>>,
}

impl SeaMonster {
    fn new() -> Self {
        Self {
            pixels: vec![
                "                  # ".chars().collect(),
                "#    ##    ##    ###".chars().collect(),
                " #  #  #  #  #  #   ".chars().collect(),
            ],
        }
    }

    fn flip(&mut self) {
        self.pixels = process_image(ImageProcessingMode::Flip, &self.pixels.clone());
    }

    fn rotate(&mut self) {
        self.pixels = process_image(ImageProcessingMode::Rotate, &self.pixels.clone());
    }

    fn height(&self) -> usize {
        self.pixels.len()
    }

    fn width(&self) -> usize {
        self.pixels[0].len()
    }

    fn coords(&self) -> Vec<(usize, usize)> {
        let mut coords = vec![];
        for y in 0..self.pixels.len() {
            for x in 0..self.pixels[0].len() {
                if self.pixels[y][x] == '#' {
                    coords.push((x, y));
                }
            }
        }
        coords
    }

    fn orientation_options() -> Vec<SeaMonster> {
        let sea_monster_r0 = SeaMonster::new();
        let mut sea_monster_r1 = sea_monster_r0.clone();
        sea_monster_r1.rotate();
        let mut sea_monster_r2 = sea_monster_r1.clone();
        sea_monster_r2.rotate();
        let mut sea_monster_r3 = sea_monster_r2.clone();
        sea_monster_r3.rotate();

        let mut sea_monster_f0 = sea_monster_r0.clone();
        sea_monster_f0.flip();
        let mut sea_monster_f1 = sea_monster_r1.clone();
        sea_monster_f1.flip();
        let mut sea_monster_f2 = sea_monster_r2.clone();
        sea_monster_f2.flip();
        let mut sea_monster_f3 = sea_monster_r3.clone();
        sea_monster_f3.flip();

        vec![
            sea_monster_r0,
            sea_monster_r1,
            sea_monster_r2,
            sea_monster_r3,
            sea_monster_f0,
            sea_monster_f1,
            sea_monster_f2,
            sea_monster_f3,
        ]
    }
}

type Picture = Vec<Vec<char>>;

fn create_picture(solved_puzzle: &HashMap<(i32, i32), Tile>) -> Picture {
    let relocated_puzzle = relocate(solved_puzzle);

    let mut sub_pictures: Vec<Vec<Picture>> = vec![];
    for row in relocated_puzzle.iter() {
        let mut tmp: Vec<Picture> = vec![];
        for tile in row.iter() {
            tmp.push(Tile::remove_borders(tile));
        }
        sub_pictures.push(tmp);
    }

    let tile_width = sub_pictures[0][0][0].len();
    let tile_height = sub_pictures[0][0].len();
    let puzzle_width = sub_pictures[0].len();
    let puzzle_height = sub_pictures.len();

    let mut picture = vec![vec![' '; tile_width * puzzle_width]; tile_height * puzzle_height];

    for (i, r) in sub_pictures.iter().enumerate() {
        for (j, tile) in r.iter().enumerate() {
            for (off_i, tr) in tile.iter().enumerate() {
                for (off_j, p) in tr.iter().enumerate() {
                    picture[i * tile_height + off_i][j * tile_width + off_j] = *p;
                }
            }
        }
    }

    picture
}

fn count_sea_monsters(picture: &[Vec<char>]) -> usize {
    let mut count = 0;
    for monster_orientation in SeaMonster::orientation_options() {
        for i in 0..picture.len() - monster_orientation.height() {
            for j in 0..picture[0].len() - monster_orientation.width() {
                if monster_orientation
                    .coords()
                    .iter()
                    .all(|(xm, ym)| picture[i + ym][j + xm] == '#')
                {
                    count += 1;
                }
            }
        }
    }

    count
}

struct Solution;

impl Solution {
    fn part1(tiles: &[Tile]) -> usize {
        let solved_puzzle: HashMap<(i32, i32), Tile> = solve_puzzle(tiles);
        let relocated_puzzle = relocate(&solved_puzzle);

        let puzzle_width = relocated_puzzle[0].len();
        let puzzle_height = relocated_puzzle.len();

        relocated_puzzle[0][0].id
            * relocated_puzzle[0][puzzle_width - 1].id
            * relocated_puzzle[puzzle_height - 1][puzzle_width - 1].id
            * relocated_puzzle[puzzle_height - 1][0].id
    }

    fn part2(tiles: &[Tile]) -> usize {
        let solved_puzzle = solve_puzzle(tiles);
        let picture = create_picture(&solved_puzzle);
        let num_sea_monsters = count_sea_monsters(&picture);

        let num_pixel_sea_monster = 15;

        picture.iter().flatten().filter(|&p| *p == '#').count()
            - num_sea_monsters * num_pixel_sea_monster
    }
}

fn main() {
    let input = fs::read_to_string("./input/day20.txt").expect("File not found!");
    let tiles: Vec<Tile> = input
        .trim()
        .split("\n\n")
        .map(Tile::from_raw_data)
        .collect();

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&tiles),
        timer.elapsed()
    );

    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&tiles),
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
        assert_eq!(Solution::part2(&tiles), 273);
    }
}
