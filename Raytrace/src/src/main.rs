use image::{ImageBuffer, Rgb};
use std::cmp;

// Vector logic

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
    center: Vec3
}

struct Rayon
{
    origin: Vec3,
    direction: Vec3
}

fn get_intersection_distance(ray: &Rayon, t:f32) -> f32
{
   t * length(&ray.direction)
}

fn get_intersection_point_t(ray: &Rayon, t:f32) -> Vec3
{
   add_vector(&ray.origin, &mul_scalar_vector(t, &ray.direction))
}

struct Intersection
{
    point: Vec3,
    normal: Vec3,
    distance: f32
}

fn intersect_sphere(ray: &Rayon, sphere: &Sphere) -> Option<Intersection>
{
    match intersect_sphere_t(&ray, &sphere)
    {
      None => None,
      Some(t) => {
          let p = get_intersection_point_t(ray, t);
          let n = normalize(&subtract_vector(&p, &sphere.center));

          Some(Intersection{point: p, normal: n, distance: t * length(&ray.direction)})
      }
    }
}

fn intersect_sphere_t(ray: &Rayon, sphere: &Sphere) -> Option<f32>
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
    let sphere = Sphere{radius, center: Vec3{x: 0.0, y: 0.0, z: 200.0}};
    let light = Light{origin: Vec3{x: 1000.0, y: 0.0, z: 200.0}, emission: Vec3{x: 100000.0, y:100000.0, z:100000.0}};

    let scene = Scene{lights: vec![light], spheres: vec![sphere]};

    let focal = 10000.0;

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

            let ray = Rayon {
                // We start from the screen and note the "focal point"
                origin: pixel,
                direction
            };

            let it_m = intersect_sphere(&ray, &scene.spheres[0]);

            let mut contrib = Vec3{x: 0.0, y: 0.0, z: 0.0};
            match it_m
            {
                // We have some intersection
                Some(it) =>
                {
                   // Compute the distance in "scene"-space
                   let albedo = Vec3{x: 1.0, y: 0.0, z: 0.0};

                   for light in &scene.lights
                   {
                     let to_light = subtract_vector(&light.origin, &it.point);
                     let light_distance = length(&to_light);
                     let cos = (dot(&normalize(&to_light), &it.normal)).clamp(0.0, 1.0);
                     println!("{}", cos);

                     let v = mul_vector(&mul_scalar_vector(cos / light_distance, &albedo), &light.emission);

                     contrib = add_vector(&contrib, &v);
                   }
                }
                None => 
                {
                }
            }

            let pixel = tonemap(&contrib, 1.0);
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
