use spirv_std::glam::{Mat4, Vec4};

pub struct Ray {
    pub origin: Vec4,
    pub direction: Vec4,
}

#[derive(Copy, Clone)]
pub struct Intersection {
    pub t: f32,
    pub obj: u32,
}

pub const MAX_HITS: usize = 4;

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
            direction: (mat * self.direction).normalize(),
        }
    }
}

impl Intersections {
    #[allow(clippy::manual_swap)]
    pub fn add(&mut self, mut hit: Intersection) {
        if hit.t >= 0.0 {
            self.is_hit = true;
        }

        for i in 0..(self.count as usize) {
            if hit.t < self.hits[i].t {
                let temp = self.hits[i];
                self.hits[i] = hit;
                hit = temp;
            }
        }

        self.hits[self.count as usize] = hit;
        self.count += 1;
    }

    pub fn get_hit(&self) -> &Intersection {
        for i in 0..(self.count as usize) {
            if self.hits[i].t >= 0.0 {
                return &self.hits[i];
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

impl Default for Intersection {
    fn default() -> Self {
        Intersection { t: -1.0, obj: 0 }
    }
}
