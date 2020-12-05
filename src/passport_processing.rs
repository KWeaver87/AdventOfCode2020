static REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn count_valid_passports(batch: &str) -> usize {
    split_passports_from_batch(batch)
        .iter()
        .filter(|pp| {
            // bind locally so that it can be properly dereferenced
            let passport = **pp;
            REQUIRED_FIELDS
                .iter()
                .all(|field| passport.contains(format!("{}:", field).as_str()))
        })
        .count()
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
    // use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

    use super::*;

    #[test]
    fn count_valid_passwords_example() {
        let actual = count_valid_passports(TEST_PASSPORT_BATCH);
        let expected = 2;

        assert_eq!(actual, expected);
    }

    #[test]
    fn splitting_from_batch() {
        let actual = split_passports_from_batch(TEST_PASSPORT_BATCH);
        let expected = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";

        assert_eq!(actual[0], expected);
    }
}
