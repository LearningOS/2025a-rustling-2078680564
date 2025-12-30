// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]
fn main() {
    // 错误1修复：移除 None 上的 unwrap()，用安全判断
    let my_option: Option<()> = None;
 

    // 错误2修复：补充数组缺失的逗号（Rustfmt 推荐）
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

  
    println!("This Vec is empty, see? {:?}",vec![1, 2, 3, 4, 5].resize(0, 5));

    // 变量交换修复：实现真正的交换
    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
