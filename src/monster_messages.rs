fn count_messages_match(raw_messages: String) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
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
}
