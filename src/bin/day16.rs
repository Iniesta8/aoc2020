use std::fs;
use std::ops::Range;
use std::time::Instant;

#[derive(Debug)]
struct TicketField {
    name: String,
    valid_ranges: Vec<Range<usize>>,
}

type Ticket = Vec<usize>;

#[derive(Debug)]
struct TicketNotes {
    ticket_fields: Vec<TicketField>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn parse_ticket_field(line: &str) -> TicketField {
    let token: Vec<&str> = line.split(": ").collect();
    assert_eq!(token.len(), 2);

    let name = token[0];
    let range_elements: Vec<usize> = token[1]
        .split(|p| p == ' ' || p == '-')
        .map(|s| s.parse::<usize>().ok())
        .flatten()
        .collect();
    let valid_ranges: Vec<Range<usize>> = range_elements
        .chunks(2)
        .map(|chunk| Range {
            start: chunk[0],
            end: chunk[1] + 1,
        })
        .collect();

    TicketField {
        name: name.to_owned(),
        valid_ranges,
    }
}

fn parse(input: &str) -> TicketNotes {
    let mut ticket_fields = vec![];
    let mut my_ticket = vec![];
    let mut nearby_tickets = vec![];

    let mut block = "fields";

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("your ticket:") {
            block = "ticket";
            continue;
        }
        if line.starts_with("nearby tickets:") {
            block = "nearby";
            continue;
        }
        if block == "fields" {
            ticket_fields.push(parse_ticket_field(line));
        } else {
            let ticket: Ticket = line.trim().split(',').map(|f| f.parse().unwrap()).collect();
            match block {
                "ticket" => my_ticket = ticket,
                "nearby" => nearby_tickets.push(ticket),
                _ => unreachable!(),
            }
        }
    }
    TicketNotes {
        ticket_fields,
        my_ticket,
        nearby_tickets,
    }
}

struct Solution;

impl Solution {
    fn part1(notes: &TicketNotes) -> usize {
        let valid_ranges: Vec<Range<usize>> = notes
            .ticket_fields
            .iter()
            .map(|tf| tf.valid_ranges.clone())
            .flatten()
            .collect();

        let nearby_ticket_values: Vec<usize> =
            notes.nearby_tickets.iter().cloned().flatten().collect();

        let mut error_rate: Vec<usize> = vec![];
        for val in nearby_ticket_values.iter() {
            if !valid_ranges.iter().any(|vr| vr.contains(val)) {
                error_rate.push(*val);
            }
        }
        error_rate.iter().sum()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day16.txt").expect("File not found!");
    let notes = parse(&input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&notes),
        timer.elapsed()
    );
    // let timer = Instant::now();
    // println!(
    // "p2: {} (runtime: {:?})",
    // Solution::solve(&notes, 30_000_000),
    // timer.elapsed()
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16_part1() {
        let input = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(Solution::part1(&parse(&input)), 71);
    }
}
