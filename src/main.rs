// Lesson #55
use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first_number: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second_number: String = args.nth(0).unwrap();

    let first = first_number.parse::<f32>().unwrap();
    let second = second_number.parse::<f32>().unwrap();
    let result = operate(operator, first, second);

    println!("{}", output(first, operator, second, result));
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    )
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _ => panic!("Invalid operator used."),
    }
}
