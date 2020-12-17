use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    assert_eq!(neighbors.len(), 26);
    neighbors
}

fn count_cubes_by_state(
    cubes: &HashMap<(i32, i32, i32), CubeState>,
    state_to_count: CubeState,
) -> usize {
    cubes
        .iter()
        .filter(|(_, &state)| state == state_to_count)
        .count()
}

fn conway(cubes: &HashMap<(i32, i32, i32), CubeState>, cycles: usize) -> usize {
    let mut cur_cubes = cubes.clone();

    for _ in 0..cycles {
        let mut new_cubes: HashMap<(i32, i32, i32), CubeState> = HashMap::new();
        for cube in cur_cubes.iter() {
            let (cube_pos, cube_state) = cube;
            let neighbors = get_neighbors(&cur_cubes, cube_pos);
            let num_active_neighbors = count_cubes_by_state(&neighbors, CubeState::Active);

            new_cubes.insert(
                *cube_pos,
                match cube_state {
                    CubeState::Active if num_active_neighbors == 2 || num_active_neighbors == 3 => {
                        CubeState::Active
                    }
                    CubeState::Active => CubeState::Inactive,
                    CubeState::Inactive if num_active_neighbors == 3 => CubeState::Active,
                    CubeState::Inactive => CubeState::Inactive,
                },
            );
            let extended_cubes: HashMap<(i32, i32, i32), CubeState> = neighbors
                .into_iter()
                .filter(|cube| !cur_cubes.contains_key(&cube.0))
                .collect();

            for ext_cube in extended_cubes.iter() {
                let (ext_cube_pos, ext_cube_state) = ext_cube;
                let neighbors = get_neighbors(&cur_cubes, ext_cube_pos);
                let num_active_neighbors = count_cubes_by_state(&neighbors, CubeState::Active);

                new_cubes.insert(
                    *ext_cube_pos,
                    match ext_cube_state {
                        CubeState::Active
                            if num_active_neighbors == 2 || num_active_neighbors == 3 =>
                        {
                            CubeState::Active
                        }
                        CubeState::Active => CubeState::Inactive,
                        CubeState::Inactive if num_active_neighbors == 3 => CubeState::Active,
                        CubeState::Inactive => CubeState::Inactive,
                    },
                );
            }
        }

        cur_cubes = new_cubes.clone();
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
    fn test_day17_part1() {
        let input = "\
.#.
..#
###";
        assert_eq!(Solution::part1(&parse(&input)), 112);
    }
}
