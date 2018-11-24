extern crate askama;

use std::process::Command;

fn main() {
    askama::rerun_if_templates_changed();

    println!(
        "cargo:rustc-env=PUG_VERSION={}",
        format!(
            "{}({})",
            String::from_utf8(
                Command::new("git")
                    .args(&["rev-parse", "--short", "HEAD"])
                    .output()
                    .unwrap()
                    .stdout,
            ).unwrap(),
            String::from_utf8(Command::new("date").args(&["-R"]).output().unwrap().stdout).unwrap()
        )
    );
}
