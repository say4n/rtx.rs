use crate::physics::Ray;

pub trait Hittable {
    fn hit(&self, r: &Ray) -> bool;
}
