use std::collections::HashMap;
use std::fs;

type Seats = HashMap<(i32, i32), bool>;

fn parse(input: &str) -> Seats {
    let mut seats = HashMap::new();

    for (i, r) in input.lines().enumerate() {
        for (j, tile) in r.chars().enumerate() {
            match tile {
                'L' => seats.insert((i as i32, j as i32), false),
                '#' => seats.insert((i as i32, j as i32), true),
                _ => continue,
            };
        }
    }

    seats
}

fn occupy_all_seats(seats: &mut Seats) {
    for s in seats.values_mut() {
        *s = true;
    }
}

fn count_adjacent_occupied_seats(seats: &Seats, pos: (i32, i32)) -> usize {
    let (x, y) = pos;

    let adjacent_pos = vec![
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y),
        (x + 1, y - 1),
        (x, y - 1),
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
    ];

    adjacent_pos
        .into_iter()
        .filter(|pos| seats.contains_key(pos) && *seats.get(pos).unwrap())
        .count()
}

fn count_visible_occupied_seats(seats: &Seats, pos: (i32, i32)) -> usize {
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
        for i in 1..=20 {
            let target = (x + i * dir_x, y + i * dir_y);
            if seats.contains_key(&target) {
                if *seats.get(&target).unwrap() {
                    count += 1;
                }
                break;
            }
        }
    }

    count
}

fn simulate_once<F>(seats: &mut Seats, threshold: usize, count_func: F) -> bool
where
    F: Fn(&Seats, (i32, i32)) -> usize,
{
    let cur_seats = seats.clone();
    let mut changed = false;

    for (&pos, &occupied) in cur_seats.iter() {
        match count_func(&cur_seats, pos) {
            0 if !occupied => {
                seats.insert(pos, true);
                changed = true;
            }
            val if occupied && val >= threshold => {
                seats.insert(pos, false);
                changed = true;
            }
            _ => continue,
        }
    }

    changed
}

fn count_occupied_seats(seats: &Seats) -> usize {
    seats.values().filter(|&occ| *occ == true).count()
}

struct Solution;

impl Solution {
    fn part1(mut seats: &mut Seats) -> usize {
        loop {
            if !simulate_once(&mut seats, 4, count_adjacent_occupied_seats) {
                break;
            }
        }
        count_occupied_seats(seats)
    }

    fn part2(mut seats: &mut Seats) -> usize {
        loop {
            if !simulate_once(&mut seats, 5, count_visible_occupied_seats) {
                break;
            }
        }
        count_occupied_seats(seats)
    }
}

fn main() {
    let input = fs::read_to_string("./input/day11.txt").expect("File not found!");
    let mut seats = parse(&input);

    let mut seats_b = seats.clone();

    occupy_all_seats(&mut seats);
    println!("p1: {}", Solution::part1(&mut seats));

    occupy_all_seats(&mut seats_b);
    println!("p2: {}", Solution::part2(&mut seats_b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
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
        let mut seats = parse(&input1);
        occupy_all_seats(&mut seats);
        let mut seats_p2 = seats.clone();

        assert_eq!(Solution::part1(&mut seats), 37);
        assert_eq!(Solution::part2(&mut seats_p2), 26);
    }
}
