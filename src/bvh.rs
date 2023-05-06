use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::{surrounding_box, AABB},
    hittable::Hittable,
    hittable_list::HittableList,
    utils::random_i32_with_range,
};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb_box: AABB,
}

impl BvhNode {
    pub fn with_hittable_vec(
        src_objects: &Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f32,
        time1: f32,
    ) -> BvhNode {
        let mut objects = src_objects.clone();

        let axis = random_i32_with_range(0, 2) as usize;

        let box_compare = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize| {
            let mut box_a = AABB::default();
            let mut box_b = AABB::default();

            if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
                eprintln!("No bounding box in bvh construct.\n");
            }

            box_a.min()[axis].total_cmp(&box_b.min()[axis])
        };
        let box_x_compare = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| box_compare(a, b, 0);
        let box_y_compare = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| box_compare(a, b, 1);
        let box_z_compare = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| box_compare(a, b, 2);

        let comparator: Box<dyn Fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering> = match axis
        {
            0 => Box::new(box_x_compare),
            1 => Box::new(box_y_compare),
            2 => Box::new(box_z_compare),
            _ => Box::new(box_x_compare),
        };

        let object_span = end - start;

        let (left, right) = match object_span {
            1 => (Arc::clone(&objects[start]), Arc::clone(&objects[start])),
            2 => match comparator(&objects[start], &objects[start + 1]) {
                Ordering::Less => (Arc::clone(&objects[start]), Arc::clone(&objects[start + 1])),
                _ => (Arc::clone(&objects[start + 1]), Arc::clone(&objects[start])),
            },
            _ => {
                let slice = &mut objects[start..end];
                slice.sort_by(comparator);

                let mid = start + object_span / 2;

                let left: Arc<dyn Hittable> = Arc::new(BvhNode::with_hittable_vec(
                    &objects, start, mid, time0, time1,
                ));
                let right: Arc<dyn Hittable> =
                    Arc::new(BvhNode::with_hittable_vec(&objects, mid, end, time0, time1));
                (left, right)
            }
        };

        let mut box_left = AABB::default();
        let mut box_right = AABB::default();

        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in bvh construct.\n");
        }

        let aabb_box = surrounding_box(box_left, box_right);

        BvhNode {
            left,
            right,
            aabb_box,
        }
    }

    pub fn with_hittable_list(list: &HittableList, time0: f32, time1: f32) -> BvhNode {
        BvhNode::with_hittable_vec(&list.objects, 0, list.objects.len(), time0, time1)
    }
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        if !self.aabb_box.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);
        return hit_left || hit_right;
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        std::mem::swap(output_box, &mut self.aabb_box.clone());
        true
    }
}
