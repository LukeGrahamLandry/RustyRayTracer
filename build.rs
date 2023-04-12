use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=shaders/src");
    for_gpu_runner();
    // TODO: preprocess the metal code into valid c++ so I can debug on the cpu
}

/// Compiles the shaders xcode project into a .metallib file.
fn for_gpu_runner(){
    let shaders = Command::new("xcodebuild").arg("build").current_dir("shaders").status();
    match shaders {
        Ok(s) => if !s.success() {
            panic!("xcodebuild failed with {s}.");
        }
        Err(e) => {
            panic!("Failed to run xcodebuild: {e}. Install the Xcode Command Line Tools.");
        }
    }
}
