// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
fn main() {
    let mut res = 42;
    let option = Some(12);

    // 使用 Option 的 `unwrap_or_default` 简化代码
    res += option.unwrap_or_default();

    println!("{}", res); // 输出：54
}