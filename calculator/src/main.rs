use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    println!("{:?}", args);

    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_num = first.parse::<f32>().unwrap();
    let second_num = second.parse::<f32>().unwrap();

    let sum = sum_of(operator, first_num, second_num);
    println!{"{} {} {} = {}", first_num, operator, second_num, sum.unwrap()};
}

fn sum_of(op: char, fi: f32, se: f32) -> Option<f32> {
    match op {
        '+' => Some(fi + se),
        '-' => Some(fi - se),
        '*' | 'X' | 'x' => Some(fi * se),
        '/' => Some(fi / se),
        _ => None
    }
}

