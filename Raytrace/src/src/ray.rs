use super::vec::*;

pub struct Ray
{
    pub origin: Vec3,
    pub direction: Vec3
}

pub fn get_intersection_distance(ray: &Ray, t:f32) -> f32
{
   t * length(&ray.direction)
}

pub fn get_intersection_point_t(ray: &Ray, t:f32) -> Vec3
{
   add_vector(&ray.origin, &mul_scalar_vector(t, &ray.direction))
}

