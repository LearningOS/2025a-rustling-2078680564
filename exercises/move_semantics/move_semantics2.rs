// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // Initialize vec0 with the filled vector from fill_vec
    let vec0 = fill_vec(Vec::new());

    // Clone vec0 to create vec1 (mutable for modification)
    let mut vec1 = vec0.clone();

    // Print vec0 (now filled with [22, 44, 66])
    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    // Modify vec1 by adding 88
    vec1.push(88);

    // Print vec1 (now [22, 44, 66, 88])
    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}