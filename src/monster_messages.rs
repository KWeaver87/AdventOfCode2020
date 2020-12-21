use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Eq, Debug)]
struct RuleOr {
    left: Vec<usize>,
    right: Vec<usize>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Rule {
    Simple(Vec<usize>),
    RuleOr(RuleOr),
    SingleChar(char),
}

type Rules = HashMap<usize, Rule>;

/// Part1
fn count_messages_match(raw_messages: String) -> usize {
    let (rules, messages) = parse_messages(raw_messages);

    let max_size = messages.iter().map(|m| m.len()).max().unwrap();
    let valid_messages: HashSet<String> = build_valid_messages(&rules, 0, max_size)
        .into_iter()
        .collect();

    messages
        .into_iter()
        .filter(|m| valid_messages.contains(m))
        .count()
}

fn build_valid_messages(rules: &Rules, i: usize, size: usize) -> Vec<String> {
    let rule = rules.get(&i).unwrap().clone();

    match rule {
        Rule::SingleChar(c) => vec![c.to_string()],
        Rule::Simple(subrules) => map_subrules(&rules, subrules, size),
        Rule::RuleOr(rule_or) => {
            let left = map_subrules(rules, rule_or.left, size);
            let right = map_subrules(rules, rule_or.right, size);

            left.iter().chain(right.iter()).map(|s| s.clone()).collect()
        }
    }
}

fn map_subrules(rules: &Rules, subrules: Vec<usize>, size: usize) -> Vec<String> {
    subrules.iter().fold(vec![], |acc, j| {
        let cur_rule = build_valid_messages(&rules, *j, size);

        if acc.len() == 0 {
            cur_rule
        } else {
            acc.iter()
                .filter(|a| a.len() <= size)
                .flat_map(|a| {
                    cur_rule
                        .iter()
                        .filter(|b| b.len() <= size)
                        .map(move |b| format!("{}{}", a, b))
                })
                .collect()
        }
    })
}

/// Part2
fn count_messages_match_new_rules(raw_messages: String) -> usize {
    let (rules, messages) = parse_messages(raw_messages);
    let mut new_rules = rules.clone();
    new_rules.insert(
        8,
        Rule::RuleOr(RuleOr {
            left: vec![42],
            right: vec![42, 8],
        }),
    );
    new_rules.insert(
        11,
        Rule::RuleOr(RuleOr {
            left: vec![42, 31],
            right: vec![42, 11, 31],
        }),
    );

    let max_size = messages.iter().map(|m| m.len()).max().unwrap();

    let valid_messages: HashSet<String> = build_valid_messages(&new_rules, 0, max_size)
        .into_iter()
        .collect();

    messages
        .into_iter()
        .filter(|m| valid_messages.contains(m))
        .count()
}

fn parse_messages(raw_messages: String) -> (Rules, Vec<String>) {
    let (raw_rules, raw_msgs) = raw_messages.split_at(raw_messages.find("\n\n").unwrap());
    let rules = parse_rules(raw_rules);
    let messages: Vec<String> = raw_msgs
        .lines()
        // Filter out blank lines
        .filter(|l| l.len() > 1)
        .map(|l| l.to_string())
        .collect();

    (rules, messages)
}

fn parse_rules(raw_rules: &str) -> Rules {
    raw_rules
        .lines()
        .map(|l| {
            let (key, rule) = l.split_at(l.find(":").unwrap() + 2);

            let key_out: usize = key.replace(":", "").trim().parse().unwrap();

            let rule_out = if let Some(i) = rule.find('|') {
                let (left, right) = rule.split_at(i);

                Rule::RuleOr(RuleOr {
                    left: parse_digit_list(left),
                    right: parse_digit_list(right),
                })
            } else if rule.contains(char::is_numeric) {
                Rule::Simple(parse_digit_list(rule))
            } else {
                // Grab the char without the "s
                Rule::SingleChar(rule.trim().chars().nth(1).unwrap())
            };

            (key_out, rule_out)
        })
        .collect()
}

fn parse_digit_list(digit_list: &str) -> Vec<usize> {
    digit_list
        .split(char::is_whitespace)
        .filter_map(|ch| ch.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use colored::Colorize;

    use crate::input_utils::load_as_string;

    use super::*;

    static EXAMPLE_RECEIVED: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    #[test]
    fn count_messages_match_example() {
        let expected = 2;
        let actual = count_messages_match(EXAMPLE_RECEIVED.to_string());

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_valid_messages_single_char_test() {
        let expected = vec!["a".to_string()];
        let mut rules: Rules = HashMap::new();
        rules.insert(0, Rule::SingleChar('a'));
        let actual = build_valid_messages(&rules, 0, 5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_valid_messages_simple_test() {
        let expected = vec!["aa".to_string()];
        let mut rules: Rules = HashMap::new();
        rules.insert(0, Rule::Simple(vec![1, 1]));
        rules.insert(1, Rule::SingleChar('a'));
        let actual = build_valid_messages(&rules, 0, 5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_valid_messages_rule_or_test() {
        let expected = vec!["ab".to_string(), "ba".to_string()];
        let mut rules: Rules = HashMap::new();
        rules.insert(
            0,
            Rule::RuleOr(RuleOr {
                left: vec![1, 2],
                right: vec![2, 1],
            }),
        );
        rules.insert(1, Rule::SingleChar('a'));
        rules.insert(2, Rule::SingleChar('b'));
        let actual = build_valid_messages(&rules, 0, 5);

        assert_eq!(actual, expected);
    }

    #[test]
    // Part1
    fn count_messages_match_from_input() {
        let expected = 210;

        let messages = load_as_string("day19");
        let actual = count_messages_match(messages);
        println!(
            "{}{}",
            "Number of messages that match rule 0: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    // #[test]
    // Part2 - Attempt at making work, but new rules cause a stack overflow
    fn count_messages_match_new_rules_from_input() {
        let expected = 0;

        let messages = load_as_string("day19");
        let actual = count_messages_match_new_rules(messages);
        println!(
            "{}{}",
            "Number of messages that match rule 0 (with new rules): ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
