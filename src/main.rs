use std::io;

fn calculator(operator: &str) -> Result<f64, String> {
    println!("Please enter the two numbers seperated by space: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // split the input into seperate variables
    let mut tokens = input.split_whitespace();

    let x: f64 = match tokens.next() {
        Some(s) => s.parse().expect("Failed to parse number"),
        None => return Err("No value provided for x".to_string()),
    };

    let y: f64 = match tokens.next() {
        Some(s) => s.parse().expect("Failed to parse number"),
        None => return Err("No value provided for y".to_string()),
    };

    let result = match operator {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => panic!("Invalid inputs. please check the number."),
    };

    Ok(result)
}

fn main() {
    println!("Calculator project \nPlease enter an operator (+, -, *, /)\n");

    let mut operator_input = String::new();
    io::stdin()
        .read_line(&mut operator_input)
        .expect("Failed to read line");

    let result = match operator_input.as_str().trim() {
        "+" => calculator("+"),
        "-" => calculator("-"),
        "*" => calculator("*"),
        "/" => calculator("/"),
        _ => panic!("Invalid operator"),
    }
    .expect("Failed to calculate");

    // // print the result
    println!("Result: {}", result);
}
