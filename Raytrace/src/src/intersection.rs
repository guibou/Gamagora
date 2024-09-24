use super::vec::*;
use super::ray::*;

// primitives
pub struct Sphere
{
    pub radius: f32,
    pub center: Vec3,
    pub albedo: Vec3
}

pub fn intersect_sphere(ray: &Ray, sphere: &Sphere) -> Option<f32>
{
    let oc = subtract_vector(&ray.origin, &sphere.center);

    // Note: we can simplify the value of a if ray direction is normalized
    let a = length_squared(&ray.direction);
    let b = 2.0 * dot(&oc, &ray.direction);
    let c = length_squared(&oc) - sq(sphere.radius);

    let delta = sq(b) - 4.0 * a * c;

    // Is intersection (we don't care about positive or first)
    if delta >= 0.0
    {
        let t1 = (-b - delta.sqrt()) / (2.0 * a);
        let t2 = (-b + delta.sqrt()) / (2.0 * a);

        if t1 >= 0.0
        {
            Some(t1)
        } else if t2 >= 0.0
        {
            Some(t2)
        }
        else
        {
            None
        }
    }
    else
    {
        None
    }
}

// Intersection logic

pub struct Intersection
{
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub albedo: Vec3
}

pub trait Intersectable
{
  fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

impl Intersectable for Sphere
{
fn intersect(&self, ray: &Ray) -> Option<Intersection>
{
    match intersect_sphere(&ray, &self)
    {
      None => None,
      Some(t) => {
          let p = get_intersection_point_t(ray, t);
          let n = normalize(&subtract_vector(&p, &self.center));

          Some(Intersection{point: p, normal: n, distance: t * length(&ray.direction), albedo: self.albedo})
      }
    }
}
}

impl Intersectable for Vec<Sphere>
{
fn intersect(&self, ray: &Ray) -> Option<Intersection>
{
    let mut currentItM = None;

    for obj in self
    {
        match obj.intersect(ray)
        {
            None => {}
            Some(newIt) => match currentItM
            {
                None => currentItM = Some(newIt),
                Some(currentIt) => if newIt.distance < currentIt.distance
                {
                    currentItM = Some(newIt)
                }
                else
                {
                    currentItM = Some(currentIt)
                }
            }
        }
    }

    currentItM
}
}

pub fn sq(x:f32) -> f32
{
    x * x
}


