// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!


#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let mut vec1 = fill_vec(vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}



















// When running this exercise for the first time, you'll notice an error about
// "borrow of moved value". In Rust, when an argument is passed to a function and
// it's not explicitly returned, you can't use the original variable anymore.
// We call this "moving" a variable. When we pass `vec0` into `fill_vec`, it's being
// "moved" into `vec1`, meaning we can't access `vec0` anymore after the fact.
// Rust provides a couple of different ways to mitigate this issue, feel free to try them all:
// 1. You could make another, separate version of the data that's in `vec0` and pass that
//    to `fill_vec` instead.
// 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
//    and then copy the data within the function (`vec.clone()`) in order to return an owned
//    `Vec<i32>`.