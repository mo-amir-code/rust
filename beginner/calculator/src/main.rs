use std::io;

fn main() {
    let mut input_one: String = String::new();
    let mut input_two: String = String::new();
    let mut op: String = String::new();
    let mut result:i32 = 0;

    println!("Enter any number value: ");
    io::stdin()
        .read_line(&mut input_one)
        .expect("Failed to read line");

    println!("Enter any number value: ");
    io::stdin()
        .read_line(&mut input_two)
        .expect("Failed to read line");

    let num1: i32 = match input_one.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Inavalid Input!");
            return;
        }
    };

    let num2: i32 = match input_two.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Inavalid Input!");
            return;
        }
    };

    println!("List of the operators");
    println!("(1) Add");
    println!("(2) Substract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("Select the number to perform the desired operation: ");
    io::stdin().read_line(&mut op).expect("Failed to read line");

    let operation: i32 = match op.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Inavalid Input!");
            return;
        }
    };

    match operation {
        1 => result = num1 + num2,
        2 => result = num1 - num2,
        3 => result = num1 * num2,
        4 => result = num1 / num2,
        _ => {
            println!("Invalid Input!");
            return;
        }
    }


    println!("Answer --> {}", result);
}
