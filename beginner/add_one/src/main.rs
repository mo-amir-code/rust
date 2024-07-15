
fn main() {
    let mut vec = vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
    add_one(&mut vec);
    println!("{vec:?}");
}

fn add_one(digits: &mut Vec<i32>) {
    let n = digits.len();
    for (index, i) in digits.iter_mut().rev().enumerate() {
        if *i > 8 {
            if n != (index+1) {
                *i = 0;
            }else{
                *i = 1;
                digits.push(0);
                return;
            }
        } else {
            *i = *i + 1;
            return;
        }
    }
}
