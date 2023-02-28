# Ray Tracer

## Features

**The Ray Tracer Challenge Book**

- Tuples: points, vectors, colours
- Canvas: draw pixels to image file (.ppm)
- Matrix Transformations: translate, scale, rotate about x/y/z axis, shear 

**Additions**

- Parser that generates c++ from the dialect of gherkin used for test specs in The Ray Tracer Challenge 
- Sphere caches transformation's inverse and transpose of inverse

## Stuff to fix

- limit .ppm line length
- duplication between tuple, colour and matrix

## Stuff to optimise

- gauss inverse might be faster
- cache inverses and determinants on matrix object. is it worth interning them?
- alternate front end that just does perspective transformation for faster but no reflections
  - should be able to reuse all the math and world/object stuff 

## Stuff to prove

- using w for point vs vector  
- cross and dot
- where do transformation matrices come from
  - what is shear transformation
- laplace expansion
    - 2x2 isn't the special case. det(1x1) = M.get(0, 0);
- line-sphere intersection
    - should be an easy substitution that becomes a quadratic
- world space vs object space
  - transforming the ray instead of the sphere means you don't have to have a complicated formula for intersection 
  - why does normal use transpose of inverse instead of just the raw transform
- reflect vector across normal
- why the phong reflection model works. where do the colour weightings come from 
- where does view transform come from
- camera
  - canvas 1 unit, just to make the tan math easy? rays are infinite so it doesn't matter?

Should make a nice website for all of these. Seems like a perfect application for experimenting with animating visual proofs.

## Stuff to extend

- compile the yml image specs from book 
- think about what nicer interfaces to use for the lye api
  - like the "fluent API" transformation chaining example from book
  - something nice for animating proofs. look at 3b1b/manim
- more language front ends
  - generate code to expose c++ class to lye / python 
    - then use that for the end of chapter things
  - lwjgl backend so i could use it in minecraft mods
  - js canvas backend so i could use it on website
  - if i'm tempted to rewrite it for all targets, then i should just do it in lye. only need native code for actually drawing the final pixels 
    - still going to start with c++
        - don't want to wait to finish lye and dev tools to play with rendering 
        - feels cooler to do it in something lower level even though it's not even meaningfully harder
  - but I worry about it being slow
    - subset that can compile to c++ by requiring explicit releasing memory 
      - this could extend to always compiling, complex enough to be worth overhead interop, pure functions when building an executable
- import minecraft structure nbt files 
- my own implementation of normal image formats
