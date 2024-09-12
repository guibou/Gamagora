use image::{ImageBuffer, Rgb};
use std::vec;

struct Vec3
{
    x:f32,y:f32,z:f32
}

fn substractVector(a: &Vec3, b: &Vec3) -> Vec3
{
    Vec3{x: a.x - b.x, y: a.y - b.y, z: a.z - b.z}
}

fn dot(a: &Vec3, b: &Vec3) -> f32
{
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn length(a: &Vec3) -> f32
{
    dot(a, a).sqrt()
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

fn intersectSphere(rayon: &Rayon, sphere: &Sphere) -> bool
{
    let oc = substractVector(&rayon.origin, &sphere.center);
    let a = sq(length(&rayon.direction));
    let b = 2.0 * dot(&oc, &rayon.direction);
    let c = sq(length(&oc)) - sq(sphere.radius);

    let delta = sq(b) - 4.0 * a * c;

    // Is intersection (we don't care about positive or first)
    if delta >= 0.0
    {
        let t1 = (-b - delta.sqrt()) / (2.0 * a);
        let t2 = (-b + delta.sqrt()) / (2.0 * a);

        if t1 >= 0.0
        {
            true
        } else if t2 >= 0.0
        {
            true
        }
        else
        {
            false
        }
    }
    else
    {
        false
    }
}


fn sq(x:f32) -> f32
{
    x * x
}

fn main() {
    let w:f32 = 800.0;
    let h:f32 = 600.0;
    let mut img = ImageBuffer::new(w as u32, h as u32);

    let cx = w / 2.0;
    let cy = h / 2.0;

    let radius = 180.0;
    let sphere = Sphere{radius: radius, center: Vec3{x: 0.0, y: 0.0, z: 200.0}};

    let focal = 10000.0;

    for py in 0..(h as u32)
    {
        let y = py as f32;

        for px in 0..(w as u32)
        {
            let x = px as f32;

            let pixel = Vec3{y: y*2.0 -h, x: x*2.0 - w, z: 0.0};
            let origin = Vec3{x:0.0, y:0.0, z: -focal};
            let direction = substractVector(&pixel, &origin);

            let ray = Rayon {
                origin: pixel,
                direction: direction
            };

            if intersectSphere(&ray, &sphere)
            {
               img.put_pixel(px, py, Rgb([255 as u8, 255, 255]));
            } else
            {
               img.put_pixel(px, py, Rgb([0 as u8, 100, 50]));
            }
        }

    }

    img.save("result.png").unwrap();
}
