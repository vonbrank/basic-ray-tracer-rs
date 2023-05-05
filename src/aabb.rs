use crate::{ray::Ray, vec3::Point3};
#[derive(Clone, Copy)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn default() -> AABB {
        AABB {
            minimum: Point3::new(-1.0, -1.0, -1.0),
            maximum: Point3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn new(a: &Point3, b: &Point3) -> AABB {
        AABB {
            minimum: a.clone(),
            maximum: b.clone(),
        }
    }

    fn min(&self) -> Point3 {
        self.minimum
    }

    fn max(&self) -> Point3 {
        self.maximum
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_min >= t_max {
                return false;
            }
        }

        true
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Point3::new(
        f32::min(box0.min().x(), box1.min().x()),
        f32::min(box0.min().y(), box1.min().y()),
        f32::min(box0.min().z(), box1.min().z()),
    );
    let big = Point3::new(
        f32::max(box0.max().x(), box1.max().x()),
        f32::max(box0.max().y(), box1.max().y()),
        f32::max(box0.max().z(), box1.max().z()),
    );
    AABB {
        minimum: small,
        maximum: big,
    }
}
