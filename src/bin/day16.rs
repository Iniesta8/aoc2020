use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::RangeInclusive;
use std::time::Instant;

type Ticket = Vec<usize>;

#[derive(Debug)]
struct TicketNotes {
    ticket_fields: HashMap<String, Vec<RangeInclusive<usize>>>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn parse_ticket_field(line: &str) -> (String, Vec<RangeInclusive<usize>>) {
    let token: Vec<&str> = line.split(": ").collect();
    assert_eq!(token.len(), 2);

    let name = token[0];
    let range_elements: Vec<usize> = token[1]
        .split(|p| p == ' ' || p == '-')
        .flat_map(str::parse)
        .collect();
    let valid_ranges: Vec<RangeInclusive<usize>> = range_elements
        .chunks(2)
        .map(|chunk| chunk[0]..=chunk[1])
        .collect();

    (name.to_owned(), valid_ranges)
}

fn parse(input: &str) -> TicketNotes {
    let mut ticket_fields = HashMap::new();
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
            let (field_name, valid_ranges) = parse_ticket_field(line);
            ticket_fields.insert(field_name, valid_ranges);
        } else {
            let ticket: Ticket = line.trim().split(',').flat_map(str::parse).collect();
            if block == "ticket" {
                my_ticket = ticket;
            } else {
                nearby_tickets.push(ticket);
            }
        }
    }
    TicketNotes {
        ticket_fields,
        my_ticket,
        nearby_tickets,
    }
}

fn get_valid_tickets(notes: &TicketNotes) -> Vec<Ticket> {
    notes
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|field| {
                notes
                    .ticket_fields
                    .iter()
                    .any(|(_, ranges)| ranges.iter().any(|range| range.contains(field)))
            })
        })
        .cloned()
        .collect()
}

fn find_possible_fields(notes: &TicketNotes, valid_tickets: &Vec<Ticket>) -> Vec<HashSet<String>> {
    let mut possible_fields: Vec<HashSet<String>> =
        vec![notes.ticket_fields.keys().cloned().collect(); notes.ticket_fields.len()];

    for ticket in valid_tickets {
        for i in 0..possible_fields.len() {
            let value = ticket[i];
            possible_fields[i] = possible_fields[i]
                .iter()
                .filter(|field| {
                    notes
                        .ticket_fields
                        .get::<String>(field)
                        .unwrap()
                        .iter()
                        .any(|range| range.contains(&value))
                })
                .cloned()
                .collect();
        }
    }
    possible_fields
}

struct Solution;

impl Solution {
    fn part1(notes: &TicketNotes) -> usize {
        notes
            .nearby_tickets
            .iter()
            .flatten()
            .filter(|val| {
                !notes
                    .ticket_fields
                    .values()
                    .flatten()
                    .any(|vr| vr.contains(val))
            })
            .sum()
    }

    fn part2(notes: &TicketNotes) -> usize {
        let valid_tickets = get_valid_tickets(notes);
        let possible_fields = find_possible_fields(notes, &valid_tickets);

        let mut actual_fields = vec!["".to_owned(); possible_fields.len()];
        for i in 1..=possible_fields.len() {
            for (pos, field) in possible_fields
                .iter()
                .enumerate()
                .filter(|(_, row_set)| row_set.len() == i)
            {
                for f in field {
                    if !actual_fields.contains(f) {
                        actual_fields[pos] = f.to_owned()
                    }
                }
            }
        }

        actual_fields
            .iter()
            .enumerate()
            .filter(|(_, field)| field.starts_with("departure"))
            .map(|(i, _)| notes.my_ticket[i])
            .product()
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

    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&notes),
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
