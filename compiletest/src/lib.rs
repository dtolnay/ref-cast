#![cfg(test)]

extern crate compiletest_rs as compiletest;

#[test]
fn compile_fail() {
    let mut config = compiletest::Config {
        mode: compiletest::common::Mode::Ui,
        src_base: std::path::PathBuf::from("ui"),
        ..Default::default()
    };

    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}
