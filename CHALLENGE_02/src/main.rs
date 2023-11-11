#![allow(non_snake_case)]

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = String::from(&args[1]);
    let mut result = 0;
    let mut output = String::new();

    for symbol in code.chars() {
        interpreter(symbol, &mut result, &mut output);
    }

    println!("{}", output);
}

fn interpreter(symbol: char, result: &mut i32, output: &mut String) {
    match symbol {
        '#' => *result += 1,
        '@' => *result -= 1,
        '*' => *result = *result * 2,
        '&' => output.push_str(&result.to_string()),
        _ => {}
    }
}
