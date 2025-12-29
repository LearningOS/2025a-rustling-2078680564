// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
fn main() {
    let mut res = 42;
    let option = Some(12);

    // 明确匹配Some变体，提取包裹值x
    if let Some(x) = option {
        res += x;
    }

    println!("{}", res); // 输出：54
}