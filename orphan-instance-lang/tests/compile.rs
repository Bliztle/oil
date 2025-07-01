use compiletest_rs::{run_tests, Config};
use std::path::PathBuf;

fn run_mode(mode: &'static str) {
    let mut config = Config {
        mode: mode.parse().expect("Invalid mode"),
        src_base: PathBuf::from(format!("tests/{mode}")),
        ..Default::default()
    };
    config.link_deps();
    config.clean_rmeta();

    run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail");
}
