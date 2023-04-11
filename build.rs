use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=shaders/src");
    Command::new("xcodebuild").arg("build").current_dir("shaders").status().unwrap();

    // TODO: preprocess the metal code into valid c++ so I can debug on the cpu
}