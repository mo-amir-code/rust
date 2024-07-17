use std::io;

use error::LibraryError;

use crate::library::*;

pub enum Input {
    Int(i32),
    Str(String)
}

pub fn register_patron(library:&mut Library){
    println!("Enter your name: ");
    let name = take_input_to_string();

    function_status(library.add_patron(&name), "You are registered");
}

pub fn handle_borrow_book(library:&mut Library){
    println!("Enter your name: ");
    let name = take_input_to_string();
    println!("Enter book ISBN number: ");
    let isbn = take_input_to_string();

    function_status(library.borrow_book(&isbn, &name), "Book borrowed on your name");
}

pub fn handle_return_book(library:&mut Library){
    println!("Enter your name: ");
    let name = take_input_to_string();
    println!("Enter book ISBN number: ");
    let isbn = take_input_to_string();

    function_status(library.return_book(&isbn, &name), "Book returned");
}

pub fn handle_add_book(library:&mut Library){
    println!("Enter book title: ");
    let title = take_input_to_string();
    println!("Enter book author name: ");
    let author = take_input_to_string();
    println!("Enter book ISBN number: ");
    let isbn = take_input_to_string();

    function_status(library.add_book(&title, &author, &isbn), "Book added successfully");
}

pub fn take_input(is_number: bool) -> Input {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error Occurred While Taking Input");

    if is_number {
        match input.trim().parse() {
            Ok(num) => return Input::Int(num),
            Err(_) => {
                println!("Please enter a valid number.");
                return take_input(true); // Retry input if parsing fails
            }
        }
    }

    Input::Str(input)
}

pub fn take_input_to_string() -> Box<String> {
    match take_input(false) {
        Input::Str(s) => Box::new(s),
        Input::Int(_) => {
            panic!("Expected a string input, got an integer."); 
        }
    }
}

pub fn take_input_to_int() -> Box<i32> {
    match take_input(true) {
        Input::Int(n) => Box::new(n),
        Input::Str(_) => {
            panic!("Expected a string input, got an integer."); 
        }
    }
}

pub fn function_status(result: Result<(), LibraryError>, msg:&str) {
    match result {
        Ok(()) => {
            println!("{msg}");
        },
        Err(err) => {
            println!("ERROR: {:?}", err);
        }
    }
}