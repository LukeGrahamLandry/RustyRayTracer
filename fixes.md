> Trying to take notes on bugs I encounter, so I know what to do if I make the same mistake in other projects.  
> These are in reverse chronological order. 

### rust bindgen to call c from rust

Block it from trying to generate bindings for the vector types and insert lines that use `type` to alias to glam ones. 
But that stops it from deriving Copy and Clone because it doesn't know those are just data blobs now. 
I can just write them myself, but it's annoying cause that's pretty much the same amount of code as defining 
the structs twice. Takes away the chance of forgetting to change both tho which is good and makes calling c from 
rust for tests easier than my previous manual bindings for calling rust from c. 

Trying to test the `ray.position` function and the vector returned is wrong but setting a break point in 
the c code it's right before returning. Had this problem before where the calling convention for simd vectors must be 
different. Was hoping it might just be an alignment thing since the bindgen tests also failed. 
Using `alignas(16)` on my fake struct definitions fixes their tests since that's the real alignment of the glam types. 
But returning a float4 still doesn't work. Seems strange since passing it in on the struct works.  
But I guess a big array of shape structs can never make the mistake of being passed in a special wide register 
because it's just a pointer. Which would mean they do have the same representation in memory when used as a field. 

Writing my own LA structs on the c side and using those in the cpu runner instead of simd.h fixed it. 
And it looks right. Also needed to be careful about glam using row major order.  
Made a mistake making my float3 have a padding field, Vec3A just forces 16 alignment which is a different thing. 
As a bonus, cpu_runner can be cross-platform now. Should figure out how to turn off glam simd on x86 
since that will probably mess it up similarly to Apple's simd. 

### calling rust functions from c tests

Using a WorldView as an output parameter, the values in the arrays are all wrong with a lot of zeros. 
The ShaderInputs passed by value work tho. The array pointers are the same but data is garbage. 
Because backing world is created on the stack in the default_world function, I can't have pointers to it after it drops. 
Putting the world in a box and returning that pointer as well works. 

### trying to build MSL with .cc files

Can change the file type in Xcode dropdown separately from file extension to get msl autocomplete in the editor. 
Build script gave `error: Multiple commands produce '[...]/Release/shaders.metallib'`.
Fixed by making sure all files were set to Metal Shader Source instead of some that and some c.

Now its giving `fatal error: 'metal_stdlib' file not found` so it's not in fact compiling it as metal? 
Tried setting the header files to MSL as well but didn't work. 
It really just wants them to have the .metal extension. 

I can have the build script change all the file extensions from .cc to .metal before building as MSL 
and then back before building as c++. And then CLion can index well. This relies on the Xcode project 
referencing the filenames as .metal, so you can never use xcode as an editor now because it won't find the files. 

But now cargo reruns the build script everytime because that name swapping counts as a change to the directory. 
Fixed by printing `cargo:rerun-if-changed=` for each file instead of the whole directory. 

### cpu_runner renders wrong

Then the cpu runner had one side all green and one all yellow.
Now I can look in the debugger to see where the alignment problem is.
Camera was missing a field but that didn't fix it, I guess the alignment is bigger than a float, so it skips forward.
First two shapes match on both sides, and so does the light. But the returned vector is wrong.
(0.31, 0.28, 0.28, 1) in the shader comes back to rust as (0.31, 1, 1, 0.31).
But since the data passed in got there, surely they have the same repr.
Using an output pointer argument instead of returning makes the number go through correctly.
Interestingly that's what rust-gpu was doing too, maybe there's some deeper weirdness in calling conventions?

It's still two solid squares of colour, but now they're the same colours as the back corner of the scene.  
Maybe I just have the camera wrong somehow? Looks similar to only having the back walls, 
also shifted over but that was fixed by that was fixed by dividing by the scale factor since it's 
using physical instead of logical pixels. 

### build MSL shaders as c++ with cc-rs in buildscript

Using macros to replace address qualifiers and make it valid c++ but keeping .metal as the file extension, 
it says `ld: in [...]/libraytracer-a92a5def6bdcf84f.rlib(material.o), archive member 'material.o' with length 9459416 is not mach-o or llvm bitcode file '[...]/libraytracer-a92a5def6bdcf84f.rlib' for architecture arm64`
but disassembling with `objdump -d [...].rlib`, the only (*.o) I see is lib.o from lib.cc

Using .cc instead of .metal for everything, changes the complaint to being about shapes.o 
but now disassembling shows the other .o files (with `file format mach-o arm64`) except shapes.o still isn't there. 

Looking more at the assembly, I can see some of my mangled function names but nothing from shapes.cc. 
The intersection loop in world is all unwrapped so maybe its just inlined the whole shapes thing somehow? 
I tried `-O0` and `extern "C"`, `__attribute__((noinline))` on normal_at, and it still didnt show up.
shapes.o also not in disassembly of libshaders.a. 

Since it complains about common.o when not ignoring common.h,
tried ignoring shapes.h and that compiled... but im not ignoring the other header files so don't understand. 
But ignoring all header files also works and makes sense so that's fine.  
Still doesn't work with .metal, just doesn't produce .o file for those. 
