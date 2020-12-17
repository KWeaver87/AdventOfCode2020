use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
struct Tickets {
    rules: Vec<TicketRule>,
    your_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

// A Vec of (low, high) ranges
type Ranges = Vec<(usize, usize)>;

#[derive(PartialEq, Debug, Clone)]
struct TicketRule {
    name: String,
    column: Option<u8>,
    ranges: Ranges,
}

/// Part1
fn ticket_error_rate(raw_tickets: String) -> usize {
    let tickets = parse_tickets(raw_tickets);

    let combined_ranges = combine_ranges(&tickets);

    let invalid_numbers: Vec<usize> = tickets
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|field| {
            !combined_ranges
                .iter()
                .any(|range| **field >= range.0 && **field <= range.1)
        })
        .map(|f| f.clone())
        .collect();

    invalid_numbers.iter().sum()
}

fn combine_ranges(tickets: &Tickets) -> Ranges {
    let mut all_ranges: Vec<&(usize, usize)> = tickets
        .rules
        .iter()
        .map(|rule| &rule.ranges)
        .flatten()
        .collect();
    all_ranges.sort();

    let first_range = vec![all_ranges[0].clone()];
    all_ranges
        .into_iter()
        .skip(1)
        .fold(first_range, |mut ranges, range| {
            let prev_range = ranges.pop().unwrap();
            if prev_range.1 + 1 >= range.0 {
                ranges.push((prev_range.0, range.1));
            } else {
                ranges.push(prev_range);
                ranges.push(range.clone());
            }

            ranges
        })
}

/// Returns a new Tickets, removing invalid tickets and mapping the rule.column fields
fn map_columns(tickets: &Tickets) -> Tickets {
    let valid_tickets = filter_valid_tickets(&tickets);
    let mut columns: Vec<(Vec<usize>, usize)> = vec![];

    for i in 0..tickets.your_ticket.len() {
        columns.push((valid_tickets.iter().map(|tic| tic[i]).collect(), i));
    }

    let mut mapped_rules: Vec<TicketRule> = vec![];
    let mut used_rules: HashSet<String> = HashSet::new();

    while columns.len() > 0 {
        let column = columns.pop().unwrap();
        let matching_rules: Vec<&TicketRule> = tickets
            .rules
            .iter()
            .filter(|rule| !used_rules.contains(&rule.name))
            .filter(|rule| {
                column.0.iter().all(|field| {
                    rule.ranges
                        .iter()
                        .any(|range| *field >= range.0 && *field <= range.1)
                })
            })
            .collect();

        if matching_rules.len() == 1 {
            let rule = matching_rules[0];
            used_rules.insert(rule.name.clone());
            mapped_rules.push(TicketRule {
                column: Some(column.1 as u8),
                ..rule.clone()
            });
        } else {
            columns.insert(0, column);
        }
    }

    let nearby_tickets = valid_tickets
        .into_iter()
        .filter(|tic| *tic != tickets.your_ticket)
        .collect();
    Tickets {
        rules: mapped_rules,
        nearby_tickets: nearby_tickets,
        ..tickets.clone()
    }
}

fn filter_valid_tickets(tickets: &Tickets) -> Vec<Vec<usize>> {
    let combined_ranges = combine_ranges(&tickets);

    let mut valid_tickets: Vec<Vec<usize>> = tickets
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|field| {
                combined_ranges
                    .iter()
                    .any(|range| *field >= range.0 && *field <= range.1)
            })
        })
        .map(|tic| tic.clone())
        .collect();

    valid_tickets.push(tickets.your_ticket.clone());
    valid_tickets
}

/// Part2
fn product_departure(raw_tickets: String) -> usize {
    let tickets = parse_tickets(raw_tickets);

    let tickets_with_columns = map_columns(&tickets);

    let departure_rules: Vec<TicketRule> = tickets_with_columns
        .rules
        .into_iter()
        .filter(|rule| rule.name.starts_with("departure"))
        .collect();

    let departure_fields: Vec<usize> = departure_rules
        .iter()
        .map(|rule| tickets.your_ticket[rule.column.unwrap() as usize])
        .collect();

    departure_fields.iter().product()
}

fn parse_tickets(raw_tickets: String) -> Tickets {
    let splits: Vec<&str> = raw_tickets.split("\n\n").collect();
    let raw_rules: Vec<&str> = splits[0].lines().collect();
    let raw_your_ticket: Vec<&str> = splits[1].lines().collect();
    let raw_nearby_tickets: Vec<&str> = splits[2].lines().collect();

    Tickets {
        rules: parse_rules(raw_rules),
        your_ticket: parse_your_ticket(raw_your_ticket),
        nearby_tickets: parse_nearby_tickets(raw_nearby_tickets),
    }
}

fn parse_rules(raw_rules: Vec<&str>) -> Vec<TicketRule> {
    raw_rules
        .into_iter()
        .map(|rule| {
            let colon_i = rule.find(":").unwrap();
            let (name, rest) = rule.split_at(colon_i);
            let raw_ranges: Vec<&str> = rest.trim().split(" or ").collect();
            let ranges = raw_ranges
                .into_iter()
                .map(|range| {
                    let dash_i = range.find("-").unwrap();
                    let (raw_low, raw_high) = range.split_at(dash_i);
                    let filter_chars: &[_] = &[':', '-'];
                    let low = raw_low.trim_matches(filter_chars).trim();
                    let high = raw_high.trim_matches(filter_chars).trim();

                    (low.parse().unwrap(), high.parse().unwrap())
                })
                .collect();

            TicketRule {
                name: name.to_string(),
                column: None,
                ranges: ranges,
            }
        })
        .collect()
}

fn parse_ticket(raw_ticket: &str) -> Vec<usize> {
    raw_ticket
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn parse_your_ticket(raw_your_ticket: Vec<&str>) -> Vec<usize> {
    parse_ticket(raw_your_ticket[1])
}

fn parse_nearby_tickets(raw_nearby_tickets: Vec<&str>) -> Vec<Vec<usize>> {
    raw_nearby_tickets
        .into_iter()
        .skip(1)
        .map(|t| parse_ticket(t))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_string;
    use colored::Colorize;

    static EXAMPLE_TICKET: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn combine_ranges_test() {
        let expected = vec![(1, 3), (5, 11), (13, 50)];

        let tickets = Tickets {
            rules: vec![
                TicketRule {
                    name: "class".to_string(),
                    column: None,
                    ranges: vec![(1, 3), (5, 7)],
                },
                TicketRule {
                    name: "row".to_string(),
                    column: None,
                    ranges: vec![(6, 11), (33, 44)],
                },
                TicketRule {
                    name: "seat".to_string(),
                    column: None,
                    ranges: vec![(13, 40), (45, 50)],
                },
            ],
            your_ticket: vec![],
            nearby_tickets: vec![],
        };
        let actual = combine_ranges(&tickets);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_tickets_test() {
        let expected = Tickets {
            rules: vec![
                TicketRule {
                    name: "class".to_string(),
                    column: None,
                    ranges: vec![(1, 3), (5, 7)],
                },
                TicketRule {
                    name: "row".to_string(),
                    column: None,
                    ranges: vec![(6, 11), (33, 44)],
                },
                TicketRule {
                    name: "seat".to_string(),
                    column: None,
                    ranges: vec![(13, 40), (45, 50)],
                },
            ],
            your_ticket: vec![7, 1, 14],
            nearby_tickets: vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ],
        };

        let tickets = EXAMPLE_TICKET.to_string();
        let actual = parse_tickets(tickets);

        assert_eq!(actual, expected);
    }

    #[test]
    fn ticket_error_rate_example() {
        let expected = 71;
        let tickets = EXAMPLE_TICKET.to_string();
        let actual = ticket_error_rate(tickets);

        assert_eq!(actual, expected);
    }

    // Part1
    #[test]
    fn ticket_error_rate_from_input() {
        let expected = 28884;

        let tickets = load_as_string("day16");
        let actual = ticket_error_rate(tickets);
        println!("{}{}", "Sum of invalid numbers: ".green().bold(), actual);

        assert_eq!(actual, expected);
    }

    #[test]
    fn map_columns_test() {
        let tickets = Tickets {
            rules: vec![
                TicketRule {
                    name: "class".to_string(),
                    column: None,
                    ranges: vec![(0, 1), (4, 19)],
                },
                TicketRule {
                    name: "row".to_string(),
                    column: None,
                    ranges: vec![(0, 5), (8, 19)],
                },
                TicketRule {
                    name: "seat".to_string(),
                    column: None,
                    ranges: vec![(0, 13), (16, 19)],
                },
            ],
            your_ticket: vec![11, 12, 13],
            nearby_tickets: vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]],
        };
        let mut expected = tickets.clone();
        let actual = map_columns(&tickets);
        expected.rules[0].column = Some(1);
        expected.rules[1].column = Some(0);
        expected.rules[2].column = Some(2);

        expected
            .rules
            .sort_by(|a, b| a.column.unwrap().cmp(&b.column.unwrap()));

        assert_eq!(actual, expected);
    }

    #[test]
    fn filter_valid_tickets_test() {
        let expected: Vec<Vec<usize>> = vec![vec![7, 3, 47], vec![7, 1, 14]];
        let tickets = parse_tickets(EXAMPLE_TICKET.to_string());
        let actual = filter_valid_tickets(&tickets);

        assert_eq!(actual, expected);
    }

    // Part2
    #[test]
    fn product_departure_from_input() {
        let expected = 1001849322119;

        let tickets = load_as_string("day16");
        let actual = product_departure(tickets);
        println!(
            "{}{}",
            "Product of your ticket fields that start with \"departure\": "
                .green()
                .bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
