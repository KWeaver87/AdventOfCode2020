#[allow(dead_code)]
fn multiply_2020(report: Vec<usize>) -> usize {
    let length = report.len() - 1;
    let mut x = std::usize::MIN;
    for a_index in 0..=length {
        let a = report[a_index];
        for b_index in a_index + 1..=length {
            let b = report[b_index];
            if a + b == 2020 {
                x = a * b;
                break;
            }
        }
        if x > 0 {
            break;
        }
    }

    x
}

#[cfg(test)]
mod tests {
    use crate::input_utils::load_as_vec_usize;

    use super::*;

    #[test]
    fn given_example() {
        let test_report = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(multiply_2020(test_report), 514579);
    }

    #[test]
    fn run_input() {
        let report = load_as_vec_usize("day1-1");
        println!("Product of 2020 entries: {}", multiply_2020(report));

        assert!(true);
    }
}
