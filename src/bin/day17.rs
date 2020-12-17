use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum CubeState {
    Active,
    Inactive,
}

impl From<char> for CubeState {
    fn from(c: char) -> Self {
        match c {
            '#' => CubeState::Active,
            '.' => CubeState::Inactive,
            _ => panic!("unknown cube state: {}", c),
        }
    }
}

fn parse(input: &str) -> HashMap<(i32, i32, i32), CubeState> {
    let mut cubes = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, cube) in line.chars().enumerate() {
            cubes.insert((x as i32, y as i32, 0), CubeState::from(cube));
        }
    }
    cubes
}

fn get_neighbors(
    cubes: &HashMap<(i32, i32, i32), CubeState>,
    cube_pos: &(i32, i32, i32),
) -> HashMap<(i32, i32, i32), CubeState> {
    let adj_indices = [
        (0, 0),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    let mut neighbors = HashMap::new();
    let (x, y, z) = cube_pos;

    for zi in -1..=1 {
        for adj_i in adj_indices
            .iter()
            .filter(|(xi, yi)| !(zi == 0 && *xi == 0 && *yi == 0))
        {
            let (xi, yi) = adj_i;
            let neighbor = (x + xi, y + yi, z + zi);
            neighbors.insert(neighbor, {
                match cubes.get(&neighbor) {
                    None => CubeState::Inactive,
                    Some(&state) => state,
                }
            });
        }
    }
    neighbors
}

fn conway(cubes: &HashMap<(i32, i32, i32), CubeState>, cycles: usize) -> usize {
    let mut cur_cubes = cubes.clone();

    for _ in 0..cycles {
        let mut new_cubes = HashMap::new();
        for cube in cur_cubes.iter() {
            let (cube_pos, cube_state) = cube;
            let neighbors = get_neighbors(&cur_cubes, cube_pos);

            let num_active_adj_cubes = neighbors
                .iter()
                .filter(|(_, &state)| state == CubeState::Active)
                .count();

            for inactive_neighbor in neighbors
                .iter()
                .filter(|cube| !cur_cubes.contains_key(cube.0))
            {
                new_cubes.insert(inactive_neighbor.0, inactive_neighbor.1);
            }

            let (new_cube_pos, new_cube_state) = match cube_state {
                CubeState::Active if num_active_adj_cubes == 2 || num_active_adj_cubes == 3 => {
                    (cube_pos, CubeState::Active)
                }
                CubeState::Active => (cube_pos, CubeState::Inactive),
                CubeState::Inactive if num_active_adj_cubes == 2 || num_active_adj_cubes == 3 => {
                    (cube_pos, CubeState::Active)
                }
                CubeState::Inactive => (cube_pos, CubeState::Inactive),
            };

            new_cubes.insert(new_cube_pos, &new_cube_state);
        }

        cur_cubes = *new_cubes.iter().copied().collect();
    }

    cur_cubes
        .values()
        .filter(|&state| *state == CubeState::Active)
        .count()
}

struct Solution;

impl Solution {
    fn part1(cubes: &HashMap<(i32, i32, i32), CubeState>) -> usize {
        conway(&cubes, 6)
    }
}

fn main() {
    let input = fs::read_to_string("./input/day17.txt").expect("File not found!");
    let cubes = parse(&input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&cubes),
        timer.elapsed()
    );
    // let timer = Instant::now();
    // println!(
    // "p2: {} (runtime: {:?})",
    // Solution::solve(&starting_numbers, 30_000_000),
    // timer.elapsed()
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_part1() {
        let input = "\
.#.
..#
###";
        assert_eq!(Solution::part1(&parse(&input)), 121);
    }
}
