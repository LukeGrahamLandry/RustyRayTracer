# Rusty Ray Tracer

This is a real-time implementation of the ray tracer described in [The Ray Tracer Challenge by Jamis Buck](http://raytracerchallenge.com). 
The book presents a series of language agnostic test cases that guide you through writing your first 3d renderer. 
I'm using Metal, Apple's graphics API, so it only supports macOS.

## Architecture 

The actual ray tracing logic is written in MSL and runs in a fragment shader. MSL is a very close variant of 
c++. The only differences are no standard library, no dynamic memory allocation, limited recursion, and every 
pointer or reference needs an explicit address space qualifier. Since it's mostly a subset of c++, it can be 
compiled as normal code that runs on the CPU as well. The only changes needed to make it valid c++ is defining the 
address space keywords as macros that expand to an empty string and using my own linear algebra types instead of 
`metal_stdlib` when running on the CPU. This allows exactly the same code to run with the performance of shaders in production 
but debugged with any tools that work on c++. All the application logic that doesn't need to run for every pixel is written 
in rust and used when running on either the cpu or gpu. This makes it easy to check if bugs are caused by logic 
issues or mistakes in passing information to the GPU. CLion can set breakpoints in the c++ functions and inspect the 
entire call stack, back up into the rust code. 

This is a fairly naive port of the algorithm described in the book to a shader. It's probably not well optimised for 
the kinds of parallelism that GPUs are good at. So don't see this as any sort of Metal performance benchmark. 
It's still much faster running on the gpu than the cpu, so I'm satisfied for now. It also doesn't 
use any of Metal's fancy ray tracing acceleration structure stuff because it seems more interesting to do things myself. 

## Features

**From the book**

- Shapes: planes, spheres.
- Lighting, shadows, reflection, refraction.
- Patterns: stripes, gradients, rings, checkers.

**Additions**

- Moving camera. 

### Controls

WASD to move (space and LShift to go up and down). 
Number keys to switch between preset scenes. 
The window can be resized as normal. 

## Building

Install rust and the XCode Command Line Tools. Then just `cargo run` as usual. 
By default, it uses the gpu_runner. You can also `cargo run --release --bin cpu_runner` 
but it will be much slower (and complete trash when compiled in debug mode). 
`cargo test` will run the tests on the CPU (they can't run on the GPU).