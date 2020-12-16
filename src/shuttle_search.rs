#[derive(PartialEq, Clone)]
enum BusId {
    Id(u64),
    NoConstraintsId,
}

/// Part1
fn multiply_earliest_bus(schedule: String) -> u64 {
    let (depart_time, parsed_sch) = parse_schedule(schedule);
    let just_constrained_busses: Vec<u64> = parsed_sch
        .into_iter()
        .filter_map(|b| match b {
            BusId::NoConstraintsId => None,
            BusId::Id(id) => Some(id),
        })
        .collect();

    let (time, bus) = just_constrained_busses.into_iter().fold(
        (u64::MAX, 0 as u64),
        |(best_time, best_bus), bus| {
            let time_till_next = bus - (depart_time as u64 % bus);

            if time_till_next < best_time {
                (time_till_next, bus)
            } else {
                (best_time, best_bus)
            }
        },
    );

    time * bus
}

fn find_subsequent_time(schedule: String) -> u64 {
    let (_, parsed_sch) = parse_schedule(schedule);
    let max_bus = parsed_sch
        .iter()
        .map(|bus| match bus {
            BusId::Id(id) => id,
            BusId::NoConstraintsId => &0,
        })
        .enumerate()
        .max_by(|x, y| x.1.cmp(y.1))
        .unwrap();
    let mut time = 0;

    for n in 1..(u64::MAX / *max_bus.1) {
        time = (max_bus.1 * n) - max_bus.0 as u64;
        if time % 1_000_000 == 0 {
            println!("Checking: {}", time);
        }

        let x = parsed_sch
            .iter()
            .try_fold(time, |sub_time, bus| match bus {
                BusId::Id(id) => {
                    if sub_time % id == 0 {
                        Ok(sub_time + 1)
                    } else {
                        Err(())
                    }
                }
                BusId::NoConstraintsId => Ok(sub_time + 1),
            });

        if x.is_ok() {
            break;
        }
    }

    time
}

fn parse_schedule(schedule: String) -> (usize, Vec<BusId>) {
    let schedule_lines: Vec<&str> = schedule.lines().collect();
    let time = schedule_lines[0];
    let busses = schedule_lines[1];

    let bus_vec = busses
        .split(',')
        .map(|b| {
            if b == "x" {
                BusId::NoConstraintsId
            } else {
                BusId::Id(b.parse().unwrap())
            }
        })
        .collect();

    (time.parse().unwrap(), bus_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_string;
    use colored::Colorize;

    static EXAMPLE_SCHEDULE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn multiply_earliest_bus_example() {
        let expected = 295;
        let actual = multiply_earliest_bus(EXAMPLE_SCHEDULE.to_string());

        assert_eq!(actual, expected);
    }

    // Part1
    #[test]
    fn multiply_earliest_bus_from_input() {
        let expected = 246;

        let schedule = load_as_string("day13");
        let actual = multiply_earliest_bus(schedule);
        println!(
            "{}{}",
            "Product of earliest bus wait time and ID: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn find_subsequent_time_example() {
        let expected = 1068781;
        let actual = find_subsequent_time(EXAMPLE_SCHEDULE.to_string());

        assert_eq!(actual, expected);
    }

    #[test]
    fn find_subsequent_time_multi_examples() {
        let examples = vec![
            ("0\n17,x,13,19", 3417),
            ("0\n67,7,59,61", 754018),
            ("0\n67,x,7,59,61", 779210),
            ("0\n67,7,x,59,61", 1261476),
            ("0\n1789,37,47,1889", 1202161486),
        ];

        for ex in examples {
            let actual = find_subsequent_time(ex.0.to_string());

            assert_eq!(actual, ex.1);
            println!("Passed for {}", actual);
        }
    }

    // Part2 - This probably works, but is absurdly slow, taking a day or
    // longer to reach answer. So we don't want this running on `cargo test`.
    // #[test]
    fn find_subsequent_time_from_input() {
        let expected = 939490236001473;

        let schedule = load_as_string("day13");
        let actual = find_subsequent_time(schedule);
        println!(
            "{}{}",
            "Earliest time where busses arrive at each subsequent minute: "
                .green()
                .bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
