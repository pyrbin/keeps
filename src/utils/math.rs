use crate::prelude::*;

/// Calculate the intersection point of a vector and a plane defined as a point and normal vector
/// where `pv` is the vector point, `dv` is the vector direction, `pp` is the plane point
/// and `np` is the planes' normal vector
pub fn plane_intersection(pv: Vec3, dv: Vec3, pp: Vec3, np: Vec3) -> Vec3 {
    let d = dv.dot(np);
    let t = (pp.dot(np) - pv.dot(np)) / d;
    pv + dv * t
}
