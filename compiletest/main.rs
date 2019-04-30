#![cfg(test)]

#[test]
fn compile_fail() {
    let mut config = compiletest::Config {
        mode: compiletest::common::Mode::Ui,
        src_base: std::path::PathBuf::from("ui"),
        target_rustcflags: Some(String::from(
            "\
             --edition=2018 \
             -Z unstable-options \
             --extern ref_cast \
             ",
        )),
        build_base: std::path::PathBuf::from("../target/ui"),
        ..Default::default()
    };

    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}
