use std::collections::BTreeSet;

fn product_jolt_differences(adapters: Vec<usize>) -> usize {
    let adapts: BTreeSet<usize> = adapters.into_iter().collect();

    let joltages = adapts
        .iter()
        .fold((0, 0, 0), |(prev, j1, j3), &cur| match cur - prev {
            1 => (cur, j1 + 1, j3),
            3 => (cur, j1, j3 + 1),
            _ => (cur, j1, j3),
        });

    joltages.1 * (joltages.2 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_vec_usize;
    use colored::Colorize;

    #[test]
    fn product_jolt_differences_example_1() {
        let expected = 35;
        let example_adapters = "16
10
15
5
1
11
7
19
6
12
4"
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
        let actual = product_jolt_differences(example_adapters);

        assert_eq!(actual, expected);
    }

    #[test]
    fn product_jolt_differences_example_2() {
        let expected = 220;
        let example_adapters = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
        let actual = product_jolt_differences(example_adapters);

        assert_eq!(actual, expected);
    }

    #[test]
    fn product_jolt_differences_from_input() {
        let expected = 2277;

        let adapters = load_as_vec_usize("day10");
        let actual = product_jolt_differences(adapters);
        println!(
            "{}{}",
            "Product of 1 jolt and 3 jolt differences: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
