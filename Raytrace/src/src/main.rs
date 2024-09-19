use image::{ImageBuffer, Rgb};
use std::cmp;

// Vector logic
#[derive(Copy, Clone)]
struct Vec3
{
    x:f32,y:f32,z:f32
}

fn subtract_vector(a: &Vec3, b: &Vec3) -> Vec3
{
    Vec3{x: a.x - b.x, y: a.y - b.y, z: a.z - b.z}
}

fn add_vector(a: &Vec3, b: &Vec3) -> Vec3
{
    Vec3{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z}
}

fn mul_vector(a: &Vec3, b: &Vec3) -> Vec3
{
    Vec3{x: a.x * b.x, y: a.y * b.y, z: a.z * b.z}
}

fn mul_scalar_vector(a: f32, b: &Vec3) -> Vec3
{
    Vec3{x: a * b.x, y: a * b.y, z: a * b.z}
}

fn dot(a: &Vec3, b: &Vec3) -> f32
{
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn length(a: &Vec3) -> f32
{
    dot(a, a).sqrt()
}

// Special case, compute the squared length and saves a sqrt
fn length_squared(a: &Vec3) -> f32
{
    dot(a, a)
}

fn normalize(a: &Vec3) -> Vec3
{
  let l = length(a);
  Vec3{x: a.x / l, y: a.y / l, z: a.z / l}

}

struct Sphere
{
    radius: f32,
    center: Vec3,
    albedo: Vec3
}

struct Ray
{
    origin: Vec3,
    direction: Vec3
}

fn get_intersection_distance(ray: &Ray, t:f32) -> f32
{
   t * length(&ray.direction)
}

fn get_intersection_point_t(ray: &Ray, t:f32) -> Vec3
{
   add_vector(&ray.origin, &mul_scalar_vector(t, &ray.direction))
}

struct Intersection
{
    point: Vec3,
    normal: Vec3,
    distance: f32,
    albedo: Vec3
}

fn intersect_sphere(ray: &Ray, sphere: &Sphere) -> Option<Intersection>
{
    match intersect_sphere_t(&ray, &sphere)
    {
      None => None,
      Some(t) => {
          let p = get_intersection_point_t(ray, t);
          let n = normalize(&subtract_vector(&p, &sphere.center));

          Some(Intersection{point: p, normal: n, distance: t * length(&ray.direction), albedo: sphere.albedo})
      }
    }
}

fn intersect_spheres(ray: &Ray, spheres: &Vec<Sphere>) -> Option<Intersection>
{
    let mut currentItM = None;

    for sphere in spheres
    {
        match intersect_sphere(ray, sphere)
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

fn intersect_sphere_t(ray: &Ray, sphere: &Sphere) -> Option<f32>
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


fn sq(x:f32) -> f32
{
    x * x
}

struct Light
{
    origin: Vec3,
    emission: Vec3
}

struct Scene
{
    lights: Vec<Light>,
    spheres: Vec<Sphere>
}

fn main() {
    let w:f32 = 800.0;
    let h:f32 = 600.0;
    let mut img = ImageBuffer::new(w as u32, h as u32);

    let radius = 180.0;
    let spheres = vec![
         Sphere{radius, center: Vec3{x: 0.0, y: 0.0, z: 200.0}, albedo: Vec3{x: 1.0, y: 1.0, z: 1.0}},
         Sphere{radius, center: Vec3{x: -300.0, y: -300.0, z: 200.0}, albedo: Vec3{x: 0.0, y: 0.0, z: 1.0}},
         // Small sphere
         Sphere{radius: 40.0, center: Vec3{x: 0.0, y: 0.0, z: 50.0}, albedo: Vec3{x: 1.0, y: 1.0, z: 1.0}},

         // Sol
         Sphere{radius: 50000.0, center: Vec3{x: 0.0, y: 50000.0 + 800.0, z: 0.0}, albedo: Vec3{x: 1.0, y: 1.0, z: 1.0}},
       ];
    let lights = vec![
          Light{origin: Vec3{x: 5000.0, y: 0.0, z: 0.0}, emission: Vec3{x: 400000.0, y:400000.0, z:400000.0}},
          Light{origin: Vec3{x: 1.0, y: -1000.0, z: 0.0}, emission: Vec3{x: 100000.0, y:0.0, z:0.0}},
          Light{origin: Vec3{x: -1000.0, y: 1000.0, z: 0.0}, emission: Vec3{x: 0.0, y:100000.0, z:0.0}}
         ];

    let scene = Scene{lights, spheres};

    let focal = 1000.0;

    for py in 0..(h as u32)
    {
        let y = py as f32;

        for px in 0..(w as u32)
        {
            let x = px as f32;

            // This is the pixel (note: that's one corner, we don't really care about that for now)
            let pixel = Vec3{y: y*2.0 -h, x: x*2.0 - w, z: 0.0};

            let focal_point = Vec3{x:0.0, y:0.0, z: -focal};
            let direction = subtract_vector(&pixel, &focal_point);

            let ray = Ray {
                // We start from the screen and note the "focal point"
                origin: pixel,
                direction
            };

            let it_m = intersect_spheres(&ray, &scene.spheres);

            let mut contrib = Vec3{x: 0.0, y: 0.0, z: 0.0};
            match it_m
            {
                // We have some intersection
                Some(it) =>
                {
                   for light in &scene.lights
                   {
                     let to_light = subtract_vector(&light.origin, &it.point);
                     let light_distance = length(&to_light);
                     let cos = (dot(&normalize(&to_light), &it.normal)).clamp(0.0, 1.0);

                     // This is the amount of light, but not taking into account the visibilty
                     let v = mul_vector(&mul_scalar_vector(cos / light_distance, &it.albedo), &light.emission);

                     // Visibility
                     // We shoot a ray toward the light
                     let shadow_direction = normalize(&to_light);
                     // Let's add a small offset to the origin point of the ray, so it won't
                     // intersect the surface we are on.
                     let origin_with_delta = add_vector(&it.point, &mul_scalar_vector(0.1, &shadow_direction));
                     let shadow_ray = Ray{origin: origin_with_delta, direction: shadow_direction};
            
                     // compute the intersection
                     // NOTE: this could be optimised because we don't care about the FIRST
                     // intersection. Instead we want to find any intersection between the point
                     // and the light. So we can implement an early exit. We also don't care about
                     // material / normal / ... at intersection point.
                     let it_shadow = intersect_spheres(&shadow_ray, &scene.spheres);

                     let visibility = match it_shadow
                        {
                            // No intersection: full visibility
                            None => 1.0,
                            // Intersection, we check that the intersection is between us and the
                            // light
                            Some(it) => if it.distance > light_distance
                                { 1.0 } else { 0.0 }
                        };

                     contrib = add_vector(&mul_scalar_vector(visibility, &v), &contrib);
                   }
                }
                None => 
                {
                }
            }

            let pixel = tonemap(&contrib, 2.0);
            img.put_pixel(px, py, pixel)
        }
    }

    img.save("result.png").unwrap();
}

fn tonemap(v: &Vec3, scale: f32) -> Rgb<u8>
{
  let r = v.x * scale;
  let g = v.y * scale;
  let b = v.z * scale;

  // According to
  // https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions,
  // the cast using "as" is a "saturating" cast, it will be clamped to the biggest
  // (or smallest) value of the destination type.
  Rgb([r as u8, g as u8, b as u8])
}
