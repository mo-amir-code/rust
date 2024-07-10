fn main() {
    let mut duplicates_numbers = vec![2, 3, 3, 5, 6, 7, 7];
    let unique_numbers: Vec<u64> = find_unique_numbers(&mut duplicates_numbers);
    println!("{:?} {:?}", unique_numbers, duplicates_numbers);
}

fn find_unique_numbers(arr: &mut Vec<u64>) -> Vec<u64> {
    let mut new_arr: Vec<u64> = Vec::new();
    arr.retain(|x| {
        if new_arr.contains(x){
            false
        }else{
            new_arr.push(*x);
            true
        }
    });
    new_arr
}