fn main() {
    let a = String::from("100");
    let b = String::from("01");

    let s = isize::from_str_radix(&a, 2).expect("Invalid binary number");
    let s2 = isize::from_str_radix(&b, 2).expect("Invalid binary number");

    
    
}