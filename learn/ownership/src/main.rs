fn main() {
    let s: String = String::from("Hello World!");
    take_ownership(&s);
    println!("{}", s);
    let s1 = s;
}

fn take_ownership(take_ownership: &String) {
    println!("{}", take_ownership);
}


