#[derive(PartialEq, Debug)]
struct Tickets {
    rules: Vec<TicketRule>,
    your_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

// A Vec of (low, high) ranges
type Ranges = Vec<(usize, usize)>;

#[derive(PartialEq, Debug)]
struct TicketRule {
    name: String,
    column: Option<u8>,
    ranges: Ranges,
}

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

    #[test]
    fn ticket_error_rate_from_input() {
        let expected = 28884;

        let tickets = load_as_string("day16");
        let actual = ticket_error_rate(tickets);
        println!("{}{}", "Sum of invalid numbers: ".green().bold(), actual);

        assert_eq!(actual, expected);
    }
}
