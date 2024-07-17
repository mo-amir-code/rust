mod library;
mod operations;

use library::Library;
use operations::*;

fn main() {
    let mut library = Library::new();

    println!("\t\t <------------ Welcome to Library ------------> \n\n");

    loop {
        println!("\n\n\nChoose any option: ");
        println!("1. Register yourself in the library: ");
        println!("2. Borrow book: ");
        println!("3. Return book: ");
        println!("4. List of all books: ");
        println!("5. List of all available books: ");
        println!("6. List of all unavailable books: ");
        println!("7. List of all patrons: ");
        println!("8. Add book: ");
        println!("9. Exit: \n\n");

        
        println!("Enter any number between 1 to 8:  ");
        let selection:Box<i32> = take_input_to_int();

        match *selection {
            1 => {   
                register_patron(&mut library);
            },
            2 => {   
                handle_borrow_book(&mut library);
            },
            3 => {   
                handle_return_book(&mut library);
            },
            4 => {   
                library.book_list();
            },
            5 => {   
                library.available_book_list();
            },
            6 => {   
                library.patrons_borrowed_book_list();
            },
            7 => {   
                library.patrons_list();
            },
            8 => {   
                handle_add_book(&mut library);
            },
            9 => {   
                println!("\t\t <------------- Thank you for using library ------------->");
                break;
            },
            _=>{
                println!("Please enter valid number between 1 to 9");
            }
        }


    }
    
}