fn multiply_earliest_bus(schedule: String) -> usize {
    let (depart_time, parsed_sch) = parse_schedule(schedule);

    let (time, bus) =
        parsed_sch
            .into_iter()
            .fold((usize::MAX, 0 as usize), |(best_time, best_bus), bus| {
                let time_till_next = bus - (depart_time % bus);

                if time_till_next < best_time {
                    (time_till_next, bus)
                } else {
                    (best_time, best_bus)
                }
            });

    time * bus
}

fn parse_schedule(schedule: String) -> (usize, Vec<usize>) {
    let schedule_lines: Vec<&str> = schedule.lines().collect();
    let time = schedule_lines[0];
    let busses = schedule_lines[1];


    let bus_vec = busses
        .split(',')
        .filter_map(|b| {
            if b == "x" {
                None
            } else {
                Some(b.parse().unwrap())
            }
        })
        .collect();

    (time.parse().unwrap(), bus_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::{load_as_string};
    use colored::Colorize;

    static EXAMPLE_SCHEDULE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn multiply_earliest_bus_example() {
        let expected = 295;
        let actual = multiply_earliest_bus(EXAMPLE_SCHEDULE.to_string());

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_earliest_bus_from_input() {
        let expected = 246;

        let schedule = load_as_string("day13");
        let actual = multiply_earliest_bus(schedule);
        println!(
            "{}{}",
            "Product of earliest bus wait time and ID: "
                .green()
                .bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
