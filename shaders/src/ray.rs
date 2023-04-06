use core::ops::{Index, IndexMut};
use spirv_std::arch::IndexUnchecked;
use spirv_std::glam::{Affine3A, Mat4, Vec4};

pub struct Ray {
    pub origin: Vec4,
    pub direction: Vec4,
}

#[derive(Copy, Clone)]
pub struct Intersection {
    pub t: f32,
    pub obj: u32,
}

#[derive(Copy, Clone)]
pub struct Comps {
    pub t: f32,
    pub obj: u32,
    pub point: Vec4,
    pub eyev: Vec4,
    pub normalv: Vec4,
    pub is_inside: bool,
    pub over_point: Vec4,
}

pub const MAX_HITS: usize = 25;

pub struct Intersections {
    hits: [Intersection; MAX_HITS],
    count: u32,
    is_hit: bool,
}

impl Ray {
    pub fn position(&self, t: f32) -> Vec4 {
        self.origin + (self.direction * t)
    }

    pub(crate) fn transform(&self, mat: Mat4) -> Ray {
        Ray {
            origin: mat * self.origin,
            direction: mat * self.direction, // don't normalize here! it messes up the t calculated inside a transform
        }
    }
}

impl Intersections {
    #[allow(clippy::manual_swap)]
    pub fn add(&mut self, mut hit: Intersection) {
        debug_assert!(self.count < MAX_HITS as u32);
        if hit.t >= 0.0 {
            self.is_hit = true;
        }

        for i in 0..(self.count as usize) {
            if hit.t < self[i].t {
                let temp = self[i];
                self[i] = hit;
                hit = temp;
            }
        }

        let count = self.count;
        self[count as usize] = hit;
        self.count += 1;
    }

    pub fn get_hit(&self) -> &Intersection {
        for i in 0..(self.count as usize) {
            if self[i].t >= 0.0 {
                return &self[i];
            }
        }

        panic!("No hit");
    }

    pub fn has_hit(&self) -> bool {
        self.is_hit
    }
}

impl Default for Intersections {
    fn default() -> Self {
        Intersections {
            hits: [Intersection::default(); MAX_HITS],
            count: 0,
            is_hit: false,
        }
    }
}


// Prevents rust gpu from generating bounds checks everywhere (the normal get_unchecked doesnt work for spirv targets)
// Can see that this works by looking at the msl generated by spirv-cross, it removes 14 comparisons with MAX_HITS (even in release mode)
// Takes it from ~25 to ~40 fps on the chapter7 scene.
// The debug_assert doesn't seem to add the checks back to the msl even without --release but using assert does.
// With debug_assert its checked on cpu_runner without --release.
impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.hits.len());
        unsafe {
            self.hits.index_unchecked(index)
        }
    }
}

impl IndexMut<usize> for Intersections {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.hits.len());
        unsafe {
            self.hits.index_unchecked_mut(index)
        }
    }
}

impl Default for Intersection {
    fn default() -> Self {
        Intersection { t: -1.0, obj: 0 }
    }
}
