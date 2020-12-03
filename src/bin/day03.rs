use std::{fs, io};

#[derive(Debug)]
struct Map {
    trees: Vec<(usize, usize)>,
    width: usize,
    depth: usize,
}

impl Map {
    fn parse_raw_map(raw_map: &String) -> Map {
        let mut width = 0;
        let mut depth = 0;
        let mut trees = vec![];
        for (i, row) in raw_map.lines().enumerate() {
            for (j, c) in row.trim().chars().enumerate() {
                if c == '#' {
                    trees.push((j, i));
                }
                width = j;
            }
            depth = i;
        }

        Map {
            trees,
            width,
            depth,
        }
    }
}

struct Slope {
    xdir: usize,
    ydir: usize,
}

impl Slope {
    fn new(xdir: usize, ydir: usize) -> Self {
        Self { xdir, ydir }
    }
}

fn count_trees_on_slope(map: &Map, slope: Slope) -> usize {
    let mut count = 0;
    let (mut xpos, mut ypos): (usize, usize) = (0, 0);

    while ypos <= map.depth {
        if map.trees.contains(&(xpos, ypos)) {
            count += 1;
        }

        xpos += slope.xdir;
        ypos += slope.ydir;

        if xpos > map.width {
            xpos = xpos - map.width - 1;
        }
    }

    count
}

fn evaluate_all_slopes(map: &Map, slopes: Vec<Slope>) -> usize {
    slopes
        .into_iter()
        .map(|s| count_trees_on_slope(map, s))
        .product()
}

fn main() -> io::Result<()> {
    let raw_map = fs::read_to_string("./input/day03.txt")?;
    let map = Map::parse_raw_map(&raw_map);

    println!("p1: {}", count_trees_on_slope(&map, Slope::new(3, 1)));
    println!(
        "p1: {}",
        evaluate_all_slopes(
            &map,
            vec![
                Slope::new(1, 1),
                Slope::new(3, 1),
                Slope::new(5, 1),
                Slope::new(7, 1),
                Slope::new(1, 2)
            ]
        )
    );

    Ok(())
}
