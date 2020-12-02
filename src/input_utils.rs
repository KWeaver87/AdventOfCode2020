use std::str::FromStr;
use std::{
    env,
    fs::{self},
};

// inputName should be formatted as "dayX-X"
pub fn load_as_vec_usize(input_name: &str) -> Vec<usize> {
    let err_msg = "failure at ";
    let inputs: Vec<&str> = input_name.split_terminator("-").collect();
    let mut path = env::current_dir().expect(&[err_msg, "current_dir"].concat());
    path.push("inputs");
    path.push(inputs[0]);
    path.push(inputs[1]);
    let contents = fs::read_to_string(&path).expect(path.to_str().unwrap_or_default());
    let content_lines = contents.lines();

    content_lines
        .map(|l| usize::from_str(l).expect(l))
        .collect()
}
