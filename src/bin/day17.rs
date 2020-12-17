use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
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

fn get_neighbors_3d(
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

fn get_neighbors_4d(
    cubes: &HashMap<(i32, i32, i32, i32), CubeState>,
    cube_pos: &(i32, i32, i32, i32),
) -> HashMap<(i32, i32, i32, i32), CubeState> {
    let mut neighbors = HashMap::new();
    let (x, y, z, w) = cube_pos;

    for xi in -1..=1 {
        for yi in -1..=1 {
            for zi in -1..=1 {
                for wi in -1..=1 {
                    if xi == 0 && yi == 0 && zi == 0 && wi == 0 {
                        continue;
                    }
                    let neighbor = (x + xi, y + yi, z + zi, w + wi);
                    neighbors.insert(neighbor, {
                        match cubes.get(&neighbor) {
                            None => CubeState::Inactive,
                            Some(&state) => state,
                        }
                    });
                }
            }
        }
    }

    assert_eq!(neighbors.len(), 80);
    neighbors
}

fn calc_new_cube_state(state: CubeState, num_active_neighbors: usize) -> CubeState {
    match state {
        CubeState::Active if num_active_neighbors == 2 || num_active_neighbors == 3 => {
            CubeState::Active
        }
        CubeState::Active => CubeState::Inactive,
        CubeState::Inactive if num_active_neighbors == 3 => CubeState::Active,
        CubeState::Inactive => CubeState::Inactive,
    }
}

fn process_cube<P, F>(
    cur_cubes: &HashMap<P, CubeState>,
    new_cubes: &mut HashMap<P, CubeState>,
    cube: (&P, &CubeState),
    get_neighbors: F,
) -> HashMap<P, CubeState>
where
    F: Fn(&HashMap<P, CubeState>, &P) -> HashMap<P, CubeState> + Copy,
    P: Copy + Hash + Eq,
{
    let (cube_pos, cur_cube_state) = cube;
    let neighbors = get_neighbors(&cur_cubes, &cube_pos);
    let num_active_neighbors = neighbors
        .iter()
        .filter(|(_, &state)| state == CubeState::Active)
        .count();

    new_cubes.insert(
        *cube_pos,
        calc_new_cube_state(*cur_cube_state, num_active_neighbors),
    );

    neighbors
}

fn conway<P, F>(cubes: &HashMap<P, CubeState>, get_neighbors: F, cycles: usize) -> usize
where
    F: Fn(&HashMap<P, CubeState>, &P) -> HashMap<P, CubeState> + Copy,
    P: Copy + Hash + Eq,
{
    let mut cur_cubes = cubes.clone();

    for _ in 0..cycles {
        let mut new_cubes = HashMap::new();
        for cube in cur_cubes.iter() {
            let neighbors = process_cube(&cur_cubes, &mut new_cubes, cube, get_neighbors);
            let extended_cubes: HashMap<_, CubeState> = neighbors
                .into_iter()
                .filter(|cube| !cur_cubes.contains_key(&cube.0))
                .collect();

            for ext_cube in extended_cubes.iter() {
                process_cube(&cur_cubes, &mut new_cubes, ext_cube, get_neighbors);
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
        conway(&cubes, get_neighbors_3d, 6)
    }

    fn part2(cubes: &HashMap<(i32, i32, i32, i32), CubeState>) -> usize {
        conway(&cubes, get_neighbors_4d, 6)
    }
}

fn main() {
    let input = fs::read_to_string("./input/day17.txt").expect("File not found!");
    let cubes_3d = parse(&input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&cubes_3d),
        timer.elapsed()
    );

    let cubes_4d = cubes_3d
        .iter()
        .map(|(&(x, y, z), &state)| ((x, y, z, 0), state))
        .collect();

    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&cubes_4d),
        timer.elapsed()
    );
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
        let cubes_3d = parse(&input);
        assert_eq!(Solution::part1(&cubes_3d), 112);

        let cubes_4d = cubes_3d
            .iter()
            .map(|(&(x, y, z), &state)| ((x, y, z, 0), state))
            .collect();

        assert_eq!(Solution::part2(&cubes_4d), 848);
    }
}
