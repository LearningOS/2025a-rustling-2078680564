//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
// I AM NOT DONE
fn main() {
    // In tests7, set the TEST_FOO environment variable
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // 设置环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, enable the "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}