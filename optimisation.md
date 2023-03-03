
## Do less work

- cache matrix inverse

## Do more work

- multithreading 

## Never make a copy

- memo matrix return const reference instead of copying 
  - same for matrix getCol
- shape local intersection functions take reference to intersections list instead of allocating a new one and returning a copy
  - save for world intersect function 
- intersections `get` and `hit` return const reference instead of copying 
- representing matrices as `double[16]` instead of `Tuple[4]` and using memcpy in the copy constructor 
    - somehow like 20% faster even though i imagine an array of objects being continuous 
    - tried switching to memcpy in tuple but no difference 
- have the intersections constructor reserve(5), turns out vector defaults to 0 starting capacity 
  - makes Intersections constructor slower but still net is faster. 
    - constructor: ~100 to ~500 but add: ~1900 to ~500
    - actual runtime difference is basically nothing tho
  - would want a different magic starting number depending on the scene 