use std::fs;

type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Grid {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn occupy_all_seats(grid: &mut Grid) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'L' {
                grid[i][j] = '#';
            }
        }
    }
}

fn count_adjacent_occupied_seats(grid: &Grid, pos: (i32, i32)) -> usize {
    let (x, y) = pos;

    let adjacent_pos: Vec<(i32, i32)> = [
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y),
        (x + 1, y - 1),
        (x, y - 1),
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
    ]
    .iter()
    .filter(|pos| {
        pos.0 >= 0 && pos.0 < grid.len() as i32 && pos.1 >= 0 && pos.1 < grid[0].len() as i32
    })
    .copied()
    .collect();

    adjacent_pos
        .into_iter()
        .filter(|pos| grid[pos.0 as usize][pos.1 as usize] == '#')
        .count()
}

fn count_visible_occupied_seats(grid: &Grid, pos: (i32, i32)) -> usize {
    let (x, y) = pos;
    let mut count = 0;

    let directions = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    for (dir_x, dir_y) in directions.iter() {
        let mut xi = x + dir_x;
        let mut yi = y + dir_y;
        while (0..grid.len()).contains(&(xi as usize))
            && (0..grid[0].len()).contains(&(yi as usize))
        {
            match grid[xi as usize][yi as usize] {
                '#' => {
                    count += 1;
                    break;
                }
                'L' => break,
                _ => (),
            }

            xi += dir_x;
            yi += dir_y;
        }
    }

    count
}

fn simulate_once<F>(grid: &mut Grid, count_func: F, threshold: usize) -> bool
where
    F: Fn(&Grid, (i32, i32)) -> usize,
{
    let cur_grid = grid.clone();
    let mut changed = false;

    for (i, row) in cur_grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            match c {
                'L' if count_func(&cur_grid, (i as i32, j as i32)) == 0 => {
                    grid[i][j] = '#';
                    changed = true;
                }
                '#' if count_func(&cur_grid, (i as i32, j as i32)) >= threshold => {
                    grid[i][j] = 'L';
                    changed = true;
                }
                c => grid[i][j] = c,
            }
        }
    }

    changed
}

fn count_occupied_seats(grid: &Grid) -> usize {
    grid.iter().flatten().filter(|&c| *c == '#').count()
}

struct Solution;

impl Solution {
    fn part1(mut grid: &mut Grid) -> usize {
        loop {
            if !simulate_once(&mut grid, count_adjacent_occupied_seats, 4) {
                break;
            }
        }
        count_occupied_seats(grid)
    }

    fn part2(mut grid: &mut Grid) -> usize {
        loop {
            if !simulate_once(&mut grid, count_visible_occupied_seats, 5) {
                break;
            }
        }
        count_occupied_seats(grid)
    }
}

fn main() {
    let input = fs::read_to_string("./input/day11.txt").expect("File not found!");
    let mut grid = parse(&input);

    occupy_all_seats(&mut grid);
    let mut grid_p2 = grid.clone();

    println!("p1: {}", Solution::part1(&mut grid));
    println!("p2: {}", Solution::part2(&mut grid_p2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11() {
        let input1 = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let mut grid = parse(&input1);
        occupy_all_seats(&mut grid);
        let mut grid_p2 = grid.clone();

        assert_eq!(Solution::part1(&mut grid), 37);
        assert_eq!(Solution::part2(&mut grid_p2), 26);
    }
}
