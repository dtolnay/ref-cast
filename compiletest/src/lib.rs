#![cfg(test)]

extern crate compiletest_rs as compiletest;

use std::env;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::Config::default();

    config.mode = mode.parse().expect("invalid mode");
    config.target_rustcflags = Some("-L ../target/debug/deps".to_owned());
    if let Ok(name) = env::var("TESTNAME") {
        config.filter = Some(name);
    }
    config.src_base = mode.into();

    compiletest::run_tests(&config);
}

#[test]
fn compile_fail() {
    run_mode("compile-fail");
}
