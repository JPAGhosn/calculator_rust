use std::str::FromStr;

use strum_macros::EnumString;
use crate::Operation;

#[derive(Debug)]
pub struct CLIArgs {
    operation: Operation,
    first_number: f64,
    second_number: f64
}

impl CLIArgs {
    pub fn new<'a>(operation: &'a str, first_number: f64, second_number: f64) -> Self {

        let op = Operation::from_str(operation).expect("Cannot convert the input to an opeation.");


        Self { 
            operation: Operation::from_str(operation).expect("Cannot convert the input to an opeation."), 
            first_number: first_number,
            second_number: second_number
        }
    }
}

impl CLIArgs {
    pub fn operation(&self) -> &Operation{
        &(self.operation)
    }

    pub fn first_number(&self) -> f64 {
        self.first_number
    }

    pub fn second_number(&self) -> f64 {
        self.second_number
    }
}