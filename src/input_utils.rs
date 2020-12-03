use std::str::{FromStr};
use std::{
    env,
    fs::{self},
};

/// Loads an input file as a `Vec<usize>`
/// Formatted as `dayX`, e.g. `day1`
#[allow(dead_code)]
pub fn load_as_vec_usize(input_name: &str) -> Vec<usize> {
    load_as_vec_string(input_name)
        .iter()
        .map(|line| usize::from_str(line).expect(line))
        .collect()
}

/// Loads an input file as a `Vec<String>`
/// Formatted as `dayX`, e.g. `day1`
#[allow(dead_code)]
pub fn load_as_vec_string(input_name: &str) -> Vec<String> {
    let mut path = env::current_dir().expect("failure at current_dir");
    path.push("inputs");
    path.push(input_name);
    let contents = fs::read_to_string(&path).expect(&path.to_str().unwrap());

    contents.lines().map(|s| s.into()).collect()
}
