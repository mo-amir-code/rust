fn main() {
    let vec_1: Vec<i64> = vec![1, 3, 5, 7];
    let vec_2: Vec<i64> = vec![2, 4, 6, 8];
    let final: Vec<i64> = merge_sorted_vector(vec_1, vec_2);
}

fn merge_sorted_vector(vec_1: Vec<i64>, vec_2:Vec<i64>) -> Vec<i64> {
    let mut merged: Vec<i64> = vec_1.into_iter().chain(vec_2.into_iter()).collect();
    merged.sort();
    merged
}