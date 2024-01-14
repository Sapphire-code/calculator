use std::io;
use std::io::Write;
extern crate colored;
use colored::Colorize;

fn input(prompt: &str) -> String {
    print!("{}", prompt); // Print the prompt
    io::stdout().flush().expect("Failed to flush stdout"); // Force output

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    return input.trim().to_string() // Remove trailing newline and return
}

fn main() {
    let parameter_input = input("Enter the following inputs: \n1. add\n2. Subtract\n3. Multiply\n4. Divide\n\nEnter Parameter: ");

    match parameter_input.trim() {
        "add" => {
            let firstinputadd = input("Enter First Number: ");
            let secondinputadd = input("Enter Second Number: ");

            let first_input_add: i64 = firstinputadd.parse().unwrap();
            let second_input_add: i64 = secondinputadd.parse().unwrap();

            println!("Your Output is: {}", (first_input_add + second_input_add));

        }
        "subtract" => {
            let firstinputsub = input("Enter First Number: ");
            let secondinputsub = input("Enter Second Number: ");

            let first_input_sub: i64 = firstinputsub.parse().unwrap();
            let second_input_sub: i64 = secondinputsub.parse().unwrap();

            println!("Your Output is: {}", (first_input_sub - second_input_sub));
        }
        "multiply" => {
            let firstinputmul = input("Enter First Number: ");
            let secondinputmul = input("Enter Second Number");

            let first_input_mul: i64 = firstinputmul.parse().unwrap();
            let second_input_mul: i64 = secondinputmul.parse().unwrap();

            println!("Your Output is: {}", (first_input_mul * second_input_mul));
        }
        "divide" => {
            let firstinputdiv = input("Enter First Number: ");
            let secondinputdiv = input("Enter First Number: ");

            let first_input_div: i64 = firstinputdiv.parse().unwrap();
            let second_input_div: i64 = secondinputdiv.parse().unwrap();

            println!("Your Output is: {}", (first_input_div / second_input_div));
        }
        _ => {
            println!("\n{}" ,"Sorry Incorrect Input Was Provided".red());
            main()
        }
    }
}