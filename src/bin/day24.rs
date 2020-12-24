use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(PartialEq)]
enum Color {
    White,
    Black,
}

// Hexgrid coordinates
//       -z
//  +y        +x
//        .
//  -x        -y
//       +z

type Tile = (i32, i32, i32); // x, y, z

fn parse(input: &str) -> HashMap<Tile, Color> {
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
        println!("identified tile: ({}, {}, {})", x, y, z);

        let current_entry = hexgrid.entry((x, y, z)).or_insert(Color::White);

        if *current_entry == Color::White {
            *current_entry = Color::Black;
        } else {
            *current_entry = Color::White;
        }
    }

    hexgrid
}

struct Solution;

impl Solution {
    fn part1(hexgrid: &HashMap<Tile, Color>) -> usize {
        hexgrid.values().filter(|&c| *c == Color::Black).count()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day24.txt").expect("File not found!");
    let hexgrid: HashMap<Tile, Color> = parse(&input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&hexgrid),
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
        let hexgrid: HashMap<Tile, Color> = parse(&input);
        assert_eq!(Solution::part1(&hexgrid), 10);
    }
}
