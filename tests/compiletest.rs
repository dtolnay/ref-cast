extern crate compiletest_rs as compiletest;

#[test]
fn compile_fail() {
    let mut config = compiletest::Config {
        mode: compiletest::common::Mode::CompileFail,
        src_base: std::path::PathBuf::from("tests/compile-fail"),
        ..Default::default()
    };

    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}
