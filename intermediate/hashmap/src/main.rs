use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert("name", "Aamir");
//     println!("{scores:?}");
// }


// // Accessing values in hashmap
// fn main() {
//     let mut scores = HashMap::new();

//     let blue = String::from("blue");

//     scores.insert(&blue, 20);
    
//     let color_value = scores.get(&blue).copied().unwrap_or(0);
//     println!("{}", color_value);
// }

// // Accessing values in hashmap with second method
// fn main() {
//     let mut scores = HashMap::new();

//     let blue = String::from("blue");

//     scores.insert(&blue, 20);
    
//     let color_value = scores.get(&blue);

//     match color_value {
//         Some(v) => {
//             println!("{} value is {}", blue, v);
//         },
//         None => {
//             println!("{} value not found in the {:?} hashmap", blue, scores);
//         }
//     }
// }

// Count word in string linteral
fn main() {
    let text: &str = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}