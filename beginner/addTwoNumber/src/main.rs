use std::io;

fn main() {
    let mut input1: String = String::new();
    let mut input2: String = String::new();

    println!("Enter any number: ");
    io::stdin().read_line(&mut input1).expect("Error while inputting data.");

    println!("Enter any number: ");
    io::stdin().read_line(&mut input2).expect("Error while inputting data.");

    let num1:i64 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: While parsing the input");
            return;
        },
    };

    let num2:i64 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: While parsing the input");
            return;
        },
    };

    let result: i64 = addTwoNumber(num1, num2);

    println!("Sum of two numbers: {}", result);

}

fn addTwoNumber(num1: i64, num2: i64) -> i64{
    num1 + num2
}
