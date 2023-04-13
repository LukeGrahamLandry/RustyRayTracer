use std::fs;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=shaders/src");
    assert_macos();
    for_gpu_runner();
    for_cpu_runner();
}

fn assert_macos() {  // TODO: cpu_runner would work if it didn't use Apple's simd library for matrices.
    let is_macos = std::env::var("TARGET").unwrap().contains("-apple-darwin");
    if !is_macos {
        panic!("This project only supports MacOS.");
    }
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

/// Compile the metal code as c++ so rust code can call it on the cpu for debugging.
fn for_cpu_runner(){
    let paths = fs::read_dir("shaders/src").unwrap();
    let paths = paths
        .into_iter()
        .map(|p| { p.unwrap().path() })
        .filter(|p| !p.ends_with("shaders.metal"));

    cc::Build::new()
        .define("NOT_BUILDING_AS_MSL", None)
        .cpp(true)
        .flag("-std=c++11")
        .files(paths)
        .archiver()
        .compile("shaders");
}
