// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // 支持 tests7.rs：设置环境变量 TEST_FOO
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 支持 tests8.rs：启用 "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}