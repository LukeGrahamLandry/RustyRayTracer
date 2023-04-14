use std::fs;
use std::path::PathBuf;
use std::process::Command;

const SHADERS_SRC: &str = "shaders/src";

// This feels insane. But also cc-rs refuses to create .o files from anything that ends in .metal
// and Xcode refuses to link against metal_stdlib for anything that doesn't end in .metal so here we are.
fn main() {
    assert_targeting_macos();
    cargo_watch_changes();
    change_file_extensions("cc", "metal");
    build_as_msl();
    change_file_extensions("metal", "cc");
    build_as_cpp();
}

/// Compiles the shaders xcode project into a .metallib file.
fn build_as_msl(){
    let shaders = Command::new("xcodebuild").arg("build").current_dir("shaders").status();
    match shaders {
        Ok(s) => if !s.success() {
            panic!("xcodebuild failed with {s}.");
        }
        Err(e) => {
            panic!("Failed to run xcodebuild: {e}. Install the Xcode Command Line Tools.");
        }
    }

    // Move it into the src directory so I can use include_bytes.
    fs::copy("shaders/build/Release/shaders.metallib", "src/bin/shaders.metallib").unwrap();
}

/// Compile the metal code as c++ so rust code can call it on the cpu for debugging.
fn build_as_cpp(){
    cc::Build::new()
        .define("NOT_BUILDING_AS_MSL", None)
        .cpp(true)
        .flag("-std=c++14")
        .files(cc_src_files())
        .compile("shaders");
}

fn cargo_watch_changes(){
    fs::read_dir(SHADERS_SRC)
        .unwrap()
        .map(|p| { p.unwrap().path() })
        .for_each(|p| {
            println!("cargo:rerun-if-changed={}", p.to_str().unwrap());
        });
}

fn cc_src_files() -> impl Iterator<Item=PathBuf> {
    fs::read_dir(SHADERS_SRC)
        .unwrap()
        .into_iter()
        .map(|p| { p.unwrap().path() })
        .filter(|p| p.extension().unwrap() != "h")
}

fn change_file_extensions(from: &str, to: &str) {
    fs::read_dir(SHADERS_SRC)
        .unwrap()
        .map(|p| { p.unwrap().path() })
        .filter(|p| p.extension().unwrap() == from)
        .for_each(|p| {
            let new_path = p.with_extension(to);
            fs::rename(p, new_path).unwrap();
        });
}

fn assert_targeting_macos() {  // TODO: cpu_runner would work if it didn't use Apple's simd library for matrices.
    let is_macos = std::env::var("TARGET").unwrap().contains("-apple-darwin");
    if !is_macos {
        panic!("This project only supports MacOS.");
    }
}
