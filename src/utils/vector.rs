use glam::DVec3;

pub type Color = DVec3;
pub type Point = DVec3;

pub fn unit_vector(vector: DVec3) -> DVec3 {
    vector / vector.length()
}
