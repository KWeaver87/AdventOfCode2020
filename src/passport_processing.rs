use std::collections::HashMap;

use regex::Regex;

struct ValidatedField<'a> {
    id: &'a str,
    validation: fn(&str) -> bool,
}

static REQ_VAL_FIELDS: &[ValidatedField] = &[
    ValidatedField {
        id: "byr",
        validation: |v| v >= "1920" && v <= "2002",
    },
    ValidatedField {
        id: "iyr",
        validation: |v| v >= "2010" && v <= "2020",
    },
    ValidatedField {
        id: "eyr",
        validation: |v| v >= "2020" && v <= "2030",
    },
    ValidatedField {
        id: "hgt",
        validation: |v| validate_height(v),
    },
    ValidatedField {
        id: "hcl",
        validation: |v| validate_hex_color(v),
    },
    ValidatedField {
        id: "ecl",
        validation: |v| "amb blu brn gry grn hzl oth".contains(v),
    },
    ValidatedField {
        id: "pid",
        validation: |v| v.len() == 9 && v >= "000000001" && v <= "999999999",
    },
];

fn validate_height(value: &str) -> bool {
    // Get last 2 characters
    match &value[(value.len() - 2)..] {
        "cm" => value >= "150cm" && value <= "193cm",
        "in" => value >= "59in" && value <= "76in",
        _ => false,
    }
}

fn validate_hex_color(value: &str) -> bool {
    let re = Regex::new(r"#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap();
    re.is_match(value)
}

/// Gives a count of passwords that contain all required fields.
fn passports_with_req_fields(batch: &str) -> usize {
    split_passports_from_batch(batch)
        .iter()
        .filter(|pp| {
            // bind here so that it can be dereferenced
            let passport = **pp;
            REQ_VAL_FIELDS
                .iter()
                .all(|field| passport.contains(format!("{}:", field.id).as_str()))
        })
        .count()
}

/// Gives a count of passwords that contain all required fields and validates their contents
fn passports_with_valid_fields(batch: &str) -> usize {
    split_passports_from_batch(batch)
        .iter()
        .map(|pp| map_pp_to_hashmap(*pp))
        .filter(|pp_map| {
            REQ_VAL_FIELDS
                .iter()
                .all(|field| pp_map.contains_key(field.id) && (field.validation)(pp_map[field.id]))
        })
        .count()
}

fn map_pp_to_hashmap(passport: &str) -> HashMap<&str, &str> {
    let ids_fields = passport.split_whitespace().collect::<Vec<&str>>();

    ids_fields
        .iter()
        .map(|f| {
            let k_v = f.split(':').collect::<Vec<&str>>();
            (k_v[0], k_v[1])
        })
        .collect()
}

fn split_passports_from_batch(batch: &str) -> Vec<&str> {
    batch.split("\n\n").collect()
}

static TEST_PASSPORT_BATCH: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_string;
    use colored::Colorize;

    #[test]
    fn count_passports_with_req_fields_example() {
        let expected = 2;
        let actual = passports_with_req_fields(TEST_PASSPORT_BATCH);

        assert_eq!(actual, expected);
    }

    #[test]
    fn count_passports_with_req_fields_from_input() {
        let expected = 245;

        let passport_batch = load_as_string("day4");
        let actual = passports_with_req_fields(passport_batch.as_str());
        println!(
            "{}{}",
            "Number of passports with all required fields: "
                .green()
                .bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn count_passports_with_valid_fields_example_1() {
        let expected = 0;
        let actual = passports_with_valid_fields(
            "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn count_passports_with_valid_fields_example_2() {
        let expected = 4;
        let actual = passports_with_valid_fields(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn count_passports_with_valid_fields_from_input() {
        let expected = 133;

        let passport_batch = load_as_string("day4");
        let actual = passports_with_valid_fields(passport_batch.as_str());
        println!("{}{}", "Number of valid passports: ".green().bold(), actual);

        assert_eq!(actual, expected);
    }

    #[test]
    fn splitting_from_batch() {
        let expected = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let actual = split_passports_from_batch(TEST_PASSPORT_BATCH);

        assert_eq!(actual[0], expected);
    }
}
