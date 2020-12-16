use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;
use std::time::Instant;

#[derive(Debug)]
struct TicketField {
    name: String,
    valid_ranges: Vec<RangeInclusive<usize>>,
}

type Ticket = Vec<usize>;

#[derive(Debug)]
struct TicketNotes {
    ticket_fields: Vec<TicketField>,
    my_ticket: Ticket,
    nearby_tickets: HashSet<Ticket>,
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
    let valid_ranges: Vec<RangeInclusive<usize>> = range_elements
        .chunks(2)
        .map(|chunk| RangeInclusive::new(chunk[0], chunk[1]))
        .collect();

    TicketField {
        name: name.to_owned(),
        valid_ranges,
    }
}

fn parse(input: &str) -> TicketNotes {
    let mut ticket_fields = vec![];
    let mut my_ticket = vec![];
    let mut nearby_tickets = HashSet::new();
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
            if block == "ticket" {
                my_ticket = ticket;
            } else {
                nearby_tickets.insert(ticket);
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
        let valid_ranges: Vec<RangeInclusive<usize>> = notes
            .ticket_fields
            .iter()
            .map(|tf| tf.valid_ranges.clone())
            .flatten()
            .collect();

        notes
            .nearby_tickets
            .iter()
            .cloned()
            .flatten()
            .filter(|val| !valid_ranges.iter().cloned().any(|vr| vr.contains(val)))
            .sum()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16() {
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
