fn ticket_error_rate(tickets: Vec<String>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

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
    fn ticket_error_rate_example() {
        let expected = 71;
        let tickets = EXAMPLE_TICKET.lines().map(|l| l.to_string()).collect();
        let actual = ticket_error_rate(tickets);

        assert_eq!(actual, expected);
    }
}
