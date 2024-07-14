fn main() {
    let s = String::from("hello I am a new worthy    ");
    // for (index, word) in s.split_whitespace().enumerate() {
    //     println!("Index:{}, Word:{}", index, word);
    // }
    let last_word = s.split_whitespace().last();
    match last_word {
        Some(word) => {
            println!("{}", word);
        },
        _ => {

        }
    }
}
