use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter any number: ");
    io::stdin().read_line(&mut input1).expect("Error while inputting data.");

    println!("Enter any number: ");
    io::stdin().read_line(&mut input2).expect("Error while inputting data.");

    let num1: Box<i32> = match input1.trim().parse() {   // Used the Box method to store the value in the heap location reason being to clear the concept the of the dynamic location while parsing the input .
        Ok(num) => Box::new(num),
        Err(_) => {
            println!("Error: While parsing the input");
            return;
        },
    };

    let num2: Box<i32> = match input2.trim().parse() {
        Ok(num) => Box::new(num),
        Err(_) => {
            println!("Error: While parsing the input");
            return;
        },
    };

    let result: i32 = add_two_numbers(&num1, &num2);

    println!("Sum of two numbers: {}", result);
}

fn add_two_numbers(num1: &Box<i32>, num2: &Box<i32>) -> i32 {
    **num1 + **num2
}
// Here are changes made :- The required value is dynamically allocated which make our code away from the default location . 
