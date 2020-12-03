use std::fs;
use std::str::FromStr;

struct Map {
    trees: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut trees = vec![];

        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    trees.push((j, i));
                } else if c != '.' {
                    return Err(format!("unknown symbol '{}'", c));
                }
                width = j;
            }
            height = i;
        }

        Ok(Map {
            trees,
            width,
            height,
        })
    }
}

fn count_trees_on_slope(map: &Map, slope: (usize, usize)) -> usize {
    let mut count = 0;
    let mut xpos = 0;
    let mut ypos = 0;
    let (xdir, ydir) = slope;

    while ypos <= map.height {
        if map.trees.contains(&(xpos, ypos)) {
            count += 1;
        }
        xpos = (xpos + xdir) % (map.width + 1);
        ypos += ydir;
    }
    count
}

fn evaluate_slopes(map: &Map, slopes: Vec<(usize, usize)>) -> usize {
    slopes
        .into_iter()
        .map(|s| count_trees_on_slope(map, s))
        .product()
}

fn main() {
    let raw_map = fs::read_to_string("./input/day03.txt").expect("File not found!");
    let map = match Map::from_str(&raw_map) {
        Err(e) => {
            eprintln!("Error on parsing map: {}", e);
            std::process::exit(-1);
        }
        Ok(m) => m,
    };

    println!("p1: {}", count_trees_on_slope(&map, (3, 1)));
    println!(
        "p2: {}",
        evaluate_slopes(&map, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
    );
}
