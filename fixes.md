### build shaders as c++ with cc-rs in buildscript

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
