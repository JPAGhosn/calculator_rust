use strum_macros::EnumString;
pub mod cli_args;

#[derive(EnumString, Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}


pub fn execute(op: &Operation, first_number: f64, second_number: f64) -> Option<f64> {
    match op {
        Operation::Add => Some(first_number + second_number),
        Operation::Subtract => Some(first_number - second_number),
        Operation::Multiply => Some(first_number * second_number),
        Operation::Divide => Some(first_number / second_number),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let x = 5.0;
        let y = 6.0;
        let result  = execute(Operation::Add, x, y).expect("Something went worng in the add test.");
        assert_eq!(result, 11.0)
    }

    #[test]
    #[should_panic]
    fn add_should_not_work() {
        let x = 5.0;
        let y = 6.0;
        let result  = execute(Operation::Add, x, y).expect("Something went worng in the add test.");
        assert_ne!(result, 11.0);
    }
}

