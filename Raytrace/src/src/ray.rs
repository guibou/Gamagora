use super::vec::*;

pub struct Ray
{
    pub origin: Vec3,
    pub direction: Vec3
}

pub fn get_intersection_point_t(ray: &Ray, t:f32) -> Vec3
{
   ray.origin + t * ray.direction
}

