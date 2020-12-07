fn count_bags_hold_shiny_gold(rules: String) -> usize {

    0
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
    // use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

    #[test]
    fn calc_seat_example_1() {
        let expected = 4;
        let actual = count_bags_hold_shiny_gold(EXAMPLE_RULES.to_string());

        assert_eq!(actual, expected);
    }
}
