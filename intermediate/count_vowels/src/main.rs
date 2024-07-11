use std::io::{self, BufRead};

fn main() {
    println!("Enter a string: ");

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input: String = String::new();
    let vowels = "aeiou";
    let mut count_vowels: usize = 0;

    handle.read_line(&mut input).expect("Failed to read line");

    // input.retain(|x| {
    //     if vowels.contains(x.to_ascii_lowercase()) {
    //         count_vowels = count_vowels + 1
    //     }
    //     false
    // });

    for c in input.chars() {
        if vowels.contains(c.to_ascii_lowercase()) {
            count_vowels = count_vowels + 1
        }
    }

    println!("{} vowels found", count_vowels);
}
