use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
}

// hexgrid coordinates
//      -z
//  +y      +x
//       X
//  -x      -y
//      +z
type Position = (i32, i32, i32); // x, y, z

fn parse(input: &str) -> HashMap<Position, Color> {
    let mut hexgrid = HashMap::new();

    hexgrid.insert((0, 0, 0), Color::White);

    for line in input.trim().lines() {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        let mut dir_iter = line.chars().peekable();

        while let Some(dir) = dir_iter.peek() {
            match dir {
                'e' => {
                    x += 1;
                    y -= 1;
                    dir_iter.next();
                }
                'w' => {
                    x -= 1;
                    y += 1;
                    dir_iter.next();
                }
                's' => {
                    dir_iter.next();
                    if let Some(d) = dir_iter.peek() {
                        match d {
                            'e' => {
                                y -= 1;
                                z += 1;
                                dir_iter.next();
                            }
                            'w' => {
                                x -= 1;
                                z += 1;
                                dir_iter.next();
                            }
                            _ => panic!("unknown direction: {}!", d),
                        }
                    }
                }
                'n' => {
                    dir_iter.next();
                    if let Some(d) = dir_iter.peek() {
                        match d {
                            'e' => {
                                x += 1;
                                z -= 1;
                                dir_iter.next();
                            }
                            'w' => {
                                y += 1;
                                z -= 1;
                                dir_iter.next();
                            }
                            _ => panic!("unknown direction: {}!", d),
                        }
                    }
                }
                _ => panic!("unknown direction: {}!", dir),
            }
        }

        assert_eq!(x + y + z, 0);

        let current_entry = hexgrid.entry((x, y, z)).or_insert(Color::White);

        if *current_entry == Color::White {
            *current_entry = Color::Black;
        } else {
            *current_entry = Color::White;
        }
    }

    hexgrid
}

fn count_black_tiles(hexgrid: &HashMap<Position, Color>) -> usize {
    hexgrid.values().filter(|&c| *c == Color::Black).count()
}

fn get_adjacent_tiles(
    hexgrid: &HashMap<Position, Color>,
    from: &Position,
) -> HashMap<Position, Color> {
    let adj_directions = [
        (1, -1, 0),
        (0, -1, 1),
        (-1, 0, 1),
        (-1, 1, 0),
        (0, 1, -1),
        (1, 0, -1),
    ];

    let (x, y, z) = from;
    let mut adjacent_tiles = HashMap::new();

    for dir in adj_directions.iter() {
        let (xi, yi, zi) = dir;
        let adj_pos = (x + xi, y + yi, z + zi);

        adjacent_tiles.insert(adj_pos, {
            match hexgrid.get(&adj_pos) {
                None => Color::White,
                Some(&color) => color,
            }
        });
    }

    assert_eq!(adjacent_tiles.len(), 6);
    adjacent_tiles
}

fn calc_new_tile_color(color: Color, num_adjacent_black_tiles: usize) -> Color {
    assert!(num_adjacent_black_tiles <= 6);

    match color {
        Color::Black
            if num_adjacent_black_tiles == 0
                || (3..=6).any(|c| c == num_adjacent_black_tiles) =>
        {
            Color::White
        }

        Color::White if num_adjacent_black_tiles == 2 => Color::Black,
        _ => color,
    }
}

fn process_tile(
    cur_grid: &HashMap<Position, Color>,
    next_grid: &mut HashMap<Position, Color>,
    tile: (&Position, &Color),
) -> HashMap<Position, Color> {
    let (tile_pos, tile_color) = tile;

    let adj_tiles = get_adjacent_tiles(cur_grid, tile_pos);
    let num_adj_black_tiles = count_black_tiles(&adj_tiles);
    let new_tile_color = calc_new_tile_color(*tile_color, num_adj_black_tiles);

    if new_tile_color == Color::Black {
        next_grid.insert(*tile_pos, Color::Black);
    }

    adj_tiles
}

fn flip_tiles(hexgrid: &HashMap<Position, Color>, cycles: usize) -> HashMap<Position, Color> {
    let mut cur_grid = hexgrid.clone();

    for _ in 0..cycles {
        let mut next_grid = HashMap::new();
        for tile in cur_grid.iter() {
            let adj_tiles = process_tile(&cur_grid, &mut next_grid, tile);
            let ext_tiles: HashMap<_, Color> = adj_tiles
                .into_iter()
                .filter(|(tile_pos, _)| !cur_grid.contains_key(tile_pos))
                .collect();

            for ext_tile in ext_tiles.iter() {
                process_tile(&cur_grid, &mut next_grid, ext_tile);
            }
        }
        cur_grid = next_grid.clone();
    }

    cur_grid
}

struct Solution;

impl Solution {
    fn part1(hexgrid: &HashMap<Position, Color>) -> usize {
        count_black_tiles(hexgrid)
    }

    fn part2(hexgrid: &HashMap<Position, Color>) -> usize {
        let final_hexgrid = flip_tiles(hexgrid, 100);
        count_black_tiles(&final_hexgrid)
    }
}

fn main() {
    let input = fs::read_to_string("./input/day24.txt").expect("File not found!");
    let hexgrid: HashMap<Position, Color> = parse(&input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&hexgrid),
        timer.elapsed()
    );

    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&hexgrid),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day24() {
        let input = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        let hexgrid: HashMap<Position, Color> = parse(input);
        assert_eq!(Solution::part1(&hexgrid), 10);
        assert_eq!(Solution::part2(&hexgrid), 2208);
    }
}
