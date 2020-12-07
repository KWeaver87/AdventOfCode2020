use std::collections::HashMap;

fn count_bags_hold_shiny_gold(rules: Vec<String>) -> usize {
    let parsed_rules = parse_rules(rules);

    count_bags_that_hold("shiny gold".to_string(), &parsed_rules)
}

fn count_bags_that_hold(
    desired_bag: String,
    rules: &HashMap<String, HashMap<String, usize>>,
) -> usize {
    let bags_containing: Vec<String> = rules
        .iter()
        .filter(|(_, v)| v.contains_key(&desired_bag))
        .map(|(k, _)| k.to_owned())
        .collect();

    bags_containing.len()
        + bags_containing
            .iter()
            .map(|b| count_bags_that_hold(b.to_owned(), &rules))
            .count()
}

fn parse_rules(rules: Vec<String>) -> HashMap<String, HashMap<String, usize>> {
    rules
        .iter()
        .map(|l| parse_single_rule(l.to_string()))
        .collect()
}

fn parse_single_rule(rule: String) -> (String, HashMap<String, usize>) {
    let (big_bag, contains) = rule.split_at(rule.find(" bag").unwrap());
    let contents = contains.split("contain ").nth(1).unwrap().to_string();

    (big_bag.to_string(), parse_bag_contents(contents))
}

fn parse_bag_contents(contents: String) -> HashMap<String, usize> {
    contents
        .split(',')
        .filter_map(|bag| {
            if bag == "no other bags." {
                None
            } else {
                let words: Vec<&str> = bag.split_ascii_whitespace().collect();
                Some((
                    words
                        .iter()
                        .skip(1)
                        .filter(|w| !w.contains("bag"))
                        .map(|w| w.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                    words[0].parse::<usize>().unwrap(),
                ))
            }
        })
        .collect()
}

static EXAMPLE_RULES: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_vec_string;
    use colored::Colorize;

    #[test]
    fn count_bags_example() {
        let expected = 4;
        let rules = EXAMPLE_RULES.lines().map(|l| l.to_string()).collect();
        let actual = count_bags_hold_shiny_gold(rules);

        assert_eq!(actual, expected);
    }

    #[test]
    fn count_bags_from_input() {
        let expected = 0;

        let rules = load_as_vec_string("day7");
        let actual = count_bags_hold_shiny_gold(rules);
        println!(
            "{}{}",
            "Number of bags that can hold a shiny gold bag: "
                .green()
                .bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
