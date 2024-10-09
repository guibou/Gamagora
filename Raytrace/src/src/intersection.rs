use super::vec::*;
use super::ray::*;
use super::cube::*;

// primitives
#[derive(Copy, Clone)]
pub struct Sphere
{
    pub radius: f32,
    pub center: Vec3,
    pub albedo: Vec3,
    pub bsdf: BSDF
}

#[derive(Copy, Clone)]
pub enum BSDF {
    Diffuse,
    Mirror,
    Glass(f32)
}


pub fn intersect_sphere(ray: &Ray, sphere: &Sphere) -> Option<f32>
{
    let oc = ray.origin - sphere.center;

    // Note: we can simplify the value of a if ray direction is normalized
    let a = ray.direction.length_squared();
    let b = 2.0 * &ray.direction.dot(&oc);
    let c = oc.length_squared() - sq(sphere.radius);

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
    pub albedo: Vec3,
    pub bsdf: BSDF
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
      Some(t) => {
          let p = get_intersection_point_t(ray, t);
          let n = (p - self.center).normalize();

          Some(Intersection{point: p, normal: n, distance: t * ray.direction.length(), albedo: self.albedo, bsdf: self.bsdf})
      }
      None => None,
    }
}
}

impl Intersectable for Vec<Sphere>
{
fn intersect(&self, ray: &Ray) -> Option<Intersection>
{
    let mut current_it_m = None;

    for obj in self
    {
        match obj.intersect(ray)
        {
            None => {}
            Some(new_it) => match current_it_m
            {
                None => current_it_m = Some(new_it),
                Some(current_it) => if new_it.distance < current_it.distance
                {
                    current_it_m = Some(new_it)
                }
                else
                {
                    current_it_m = Some(current_it)
                }
            }
        }
    }

    current_it_m
}
}

pub fn sq(x:f32) -> f32
{
    x * x
}

// Intersection

#[derive(Clone)]
pub enum ObjectHierarchy
{
    Leaf { spheres: Vec<Sphere>, bounding_box: AABB },
    Node { bounding_box: AABB, left_tree: Box<ObjectHierarchy>, right_tree: Box< ObjectHierarchy>},
}

// Build a hierarchy by splitting on the largest axis
pub fn build_hierarchy (mut spheres: Vec<Sphere>) -> Box<ObjectHierarchy>
{
   let mut aabb = sphere_to_aabb(&spheres[0]);
   for sphere in &spheres
   {
       aabb = aabb.union(&sphere_to_aabb(sphere));
   }

   if spheres.len() < 10
   {
      Box::new(ObjectHierarchy::Leaf{spheres, bounding_box: aabb})
   }
   else
   {
       match aabb.largest_axis()
       {
           Axis::X => spheres.sort_by(|a, b| a.center.x.partial_cmp(&b.center.x).unwrap()),
           Axis::Y => spheres.sort_by(|a, b| a.center.y.partial_cmp(&b.center.y).unwrap()),
           Axis::Z => spheres.sort_by(|a, b| a.center.z.partial_cmp(&b.center.z).unwrap()),
       }

       let mut left_spheres = vec![];
       let mut right_spheres = vec![];
       let cut = spheres.len() / 2;
       let len = spheres.len();

       for i in 0..cut
       {
           left_spheres.push(spheres[i]);
       }

       for i in cut..len
       {
           right_spheres.push(spheres[i]);
       }

       let left_tree = build_hierarchy(left_spheres);
       let right_tree = build_hierarchy(right_spheres);

       Box::new(ObjectHierarchy::Node{bounding_box: aabb,
                             left_tree, right_tree})
   }
}

impl<'a> Intersectable for ObjectHierarchy
{
   fn intersect(&self, ray: &Ray) -> Option<Intersection>
   {
       match self
       {
           ObjectHierarchy::Leaf{bounding_box, spheres} =>
             match intersect_cube(&ray, &bounding_box)
             {
                 None => None,
                 Some(_) => spheres.intersect(&ray)
             }
           ObjectHierarchy::Node{bounding_box, left_tree, right_tree} => 
             match intersect_cube(&ray, &bounding_box)
             {
                 None => None,
                 Some(_) => {
                     let it_left = left_tree.intersect(&ray);
                     let it_right = right_tree.intersect(&ray);

                     match it_left
                     {
                        None => it_right,
                        Some(it) => match it_right
                        {
                            None => Some(it),
                            Some(it2) => if it.distance < it2.distance
                            {
                                Some(it)
                            }
                            else
                            {
                                Some(it2)
                            }
                        }
                    }
                 }
             }
        }
   }
}

fn sphere_to_aabb(sphere: &Sphere) -> AABB
{
    let r = Vec3{x: sphere.radius, y: sphere.radius, z: sphere.radius};
    AABB{p_min: sphere.center - r, p_max: sphere.center + r}
}
