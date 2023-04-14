use std::fs;
use std::process::Command;

const SHADERS_SRC: &str = "shaders/src";

fn main() {
    fs::read_dir(SHADERS_SRC)
        .unwrap()
        .map(|p| { p.unwrap().path() })
        .for_each(|p| {
            println!("cargo:rerun-if-changed={}", p.to_str().unwrap());
        });

    assert_macos();

    // This feels insane. But also cc-rs refuses to create .o files from anything that ends in .metal
    // and Xcode refuses to link against metal_stdlib for anything that doesn't end in .metal so here we are.
    swap_file_extensions(SHADERS_SRC, "cc", "metal");
    for_gpu_runner();
    swap_file_extensions(SHADERS_SRC, "metal", "cc");
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
    let paths = fs::read_dir(SHADERS_SRC).unwrap();
    let paths = paths
        .into_iter()
        .map(|p| { p.unwrap().path() })
        .filter(|p| p.extension().unwrap() != "h");

    cc::Build::new()
        .define("NOT_BUILDING_AS_MSL", None)
        .cpp(true)
        .flag("-std=c++14")
        .files(paths)
        .compile("shaders");
}

fn swap_file_extensions(dir: &str, from: &str, to: &str) {
    fs::read_dir(dir)
        .expect("Cannot swap_file_extensions, directory not found.")
        .map(|p| { p.unwrap().path() })
        .filter(|p| p.extension().unwrap() == from)
        .for_each(|p| {
            let new_path = p.with_extension(to);
            fs::rename(p, new_path).expect("Cannot swap_file_extensions, failed to rename.");
        });
}