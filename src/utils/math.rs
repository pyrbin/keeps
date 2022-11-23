use crate::prelude::*;

/// Calculate the intersection point of a vector and a plane defined as a point and normal vector
/// where `pv` is the vector point, `dv` is the vector direction, `pp` is the plane point
/// and `np` is the planes' normal vector
pub fn plane_intersection(pv: Vec3, dv: Vec3, pp: Vec3, np: Vec3) -> Vec3 {
    let d = dv.dot(np);
    let t = (pp.dot(np) - pv.dot(np)) / d;
    pv + dv * t
}

pub trait Pos2d {
    fn pos_2d(&self) -> Vec2;
}

pub trait Pos3d {
    fn pos_3d(&self) -> Vec3;
}

impl Pos2d for Vec3 {
    fn pos_2d(&self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }
}

impl Pos3d for Vec2 {
    fn pos_3d(&self) -> Vec3 {
        Vec3::new(self.x, 0.0, self.y)
    }
}
