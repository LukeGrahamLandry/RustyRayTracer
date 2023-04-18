pub mod demo;
pub mod shader_types;
pub mod window;
mod controller;

// These are autogenerated bindings to the c++ shaders types.
// Tradition would be to put it in the OUT_DIR and include the file
// but that really confused my IDE's indexing for some reason.
mod bindings;

#[cfg(test)]
mod rtc_tests;
