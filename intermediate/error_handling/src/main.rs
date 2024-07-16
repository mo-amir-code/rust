use std::{fs::File, io::Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    let mut file_content = String::new();

    // greeting_file.read_to_string(&mut file_content).expect("Error occurred white reading file content");

    match greeting_file.read_to_string(&mut file_content) {
        Ok(_) => (),
        Err(err) => panic!("Error occurred while reading file content: {:?}", err),
    }

    println!("{greeting_file:?}");
    println!("{file_content}");
}

// fn main() {
//     let greeting_file_result = File::open("hello.txt").unwrap();

//     // println!("{greeting_file:?}");
// }

// fn main() {
//     let file_name = "hello.txt";
//     let greeting_file_result = File::open(&file_name).expect("{file_name} should be included in this project");
// }