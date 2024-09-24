mod vec;
mod tonemap;
mod ray;
mod intersection;

use image::ImageBuffer;
use vec::*;
use tonemap::*;
use ray::*;
use intersection::*;

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

            let it_m = scene.spheres.intersect(&ray);

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
                     let it_shadow = scene.spheres.intersect(&shadow_ray);

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
