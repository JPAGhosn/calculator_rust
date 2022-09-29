use calculator_rust::{execute, Operation};
use std::{io, str::FromStr};
use strum_macros::EnumString;
use calculator_rust::cli_args;

fn main() {
    let operation = std::env::args().nth(1).expect("No operation given");
    let first_number = std::env::args().nth(2).expect("First number not set");
    let first_number = f64::from_str(first_number.trim()).expect("Cannot convert string to f64");
    let second_number = std::env::args().nth(3).expect("Second number not set");
    let second_number = f64::from_str(&second_number.trim()).expect("Cannot convert string to f64");

    let cli = cli_args::CLIArgs::new(&operation, first_number, second_number);

    let result = execute(cli.operation(), cli.first_number(), cli.second_number()).expect("Something went wrong while executing the command.");

    println!("{:?}", cli);
    println!("The result is {}", result);
}
