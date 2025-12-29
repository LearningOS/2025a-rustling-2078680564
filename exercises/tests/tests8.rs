// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::time::SystemTime;

fn main() {
    // 满足 tests7.rs 的要求：设置环境变量 TEST_FOO
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get system time")
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 满足 tests8.rs 的要求：启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}