mod vec;
mod tonemap;
mod ray;
mod intersection;
mod cube;

use image::ImageBuffer;
use vec::*;
use tonemap::*;
use ray::*;
use intersection::*;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

struct Light
{
    origin: Vec3,
    emission: Vec3
}

struct Scene
{
    lights: Vec<Light>,
    objects: Box<ObjectHierarchy>
}

fn main() {
    let w:f32 = 800.0;
    let h:f32 = 600.0;
    let mut img = ImageBuffer::new(w as u32, h as u32);

    let radius = 180.0;

    let spheres = vec![
         Sphere{radius, center: Vec3{x: 0.0, y: 0.0, z: 200.0}, albedo: Vec3{x: 1.0, y: 1.0, z: 1.0}, bsdf: BSDF::Diffuse},
         Sphere{radius, center: Vec3{x: -300.0, y: -300.0, z: 200.0}, albedo: Vec3{x: 0.0, y: 0.0, z: 1.0}, bsdf: BSDF::Diffuse},
         // Small sphere
         Sphere{radius: 40.0, center: Vec3{x: 0.0, y: 0.0, z: 50.0}, albedo: Vec3{x: 1.0, y: 1.0, z: 1.0}, bsdf: BSDF::Diffuse},

         // Background sphere
         Sphere{radius: 500.0, center: Vec3{x: 700.0, y: 350.0, z: 1500.0}, albedo: Vec3{x: 0.9, y: 0.9, z: 0.9}, bsdf: BSDF::Glass(1.5)},
         Sphere{radius: 200.0, center: Vec3{x: 700.0, y: 350.0, z: 3000.0}, albedo: Vec3{x: 1.0, y: 1.0, z: 0.0}, bsdf: BSDF::Diffuse},

         // Sol
         Sphere{radius: 50000.0, center: Vec3{x: 0.0, y: 50000.0 + 800.0, z: 0.0}, albedo: Vec3{x: 1.0, y: 1.0, z: 1.0}, bsdf: BSDF::Diffuse},
       ];

    let lights = vec![
          Light{origin: Vec3{x: 5000.0, y: 0.0, z: 0.0}, emission: Vec3{x: 400000.0, y:400000.0, z:400000.0}},
          Light{origin: Vec3{x: 1.0, y: -1000.0, z: 0.0}, emission: Vec3{x: 100000.0, y:0.0, z:0.0}},
          Light{origin: Vec3{x: -1000.0, y: 1000.0, z: 0.0}, emission: Vec3{x: 0.0, y:100000.0, z:0.0}}
         ];

    /*
    let mut spheres = vec![];
    let n = 100;
    let d = 300.0 / (n as f32);
    let radius = 80.0 / (n as f32);
    */

    /*
     * timings
     * n = 1: 115ms
     * n = 2: 164ms
     * n = 5: 2.29s
     * n = 10: 17s (8000 spheres)
     * n = 15: 71s (27000 spheres)
     */

    /*
    for i in -n..n
    {
        for j in -n..n
        {
            for k in -n..n
            {
                spheres.push(Sphere{radius, albedo: Vec3{x:1.0, y:1.0, z:1.0},
                            center: Vec3{x:(i as f32) * d, y: (j as f32) * d, z: 200.0 + (k as f32) * d}});

            }
        }
    }
    */

    println!("Nb spheres: {}", spheres.len());

    let scene = Scene{lights, objects: build_hierarchy(spheres)};

    println!("Rendering starts");

    let focal = 1000.0;

    for py in 0..(h as u32)
    {
        let y = py as f32;

        for px in 0..(w as u32)
        {
            let mut contrib_pixel = Vec3{x: 0.0, y: 0.0, z: 0.0};
            let nbSamples = 10;
            for sample in 0..nbSamples
            {
            let x = px as f32;

            let between = Uniform::new(0.0, 1.0);
            let mut rng = rand::thread_rng();

            let ux = between.sample(&mut rng);
            let uy = between.sample(&mut rng);

            // This is the pixel (note: that's one corner, we don't really care about that for now)
            let pixel = Vec3{y: y*2.0 -h + uy, x: x*2.0 - w + ux, z: 0.0};

            let focal_point = Vec3{x:0.0, y:0.0, z: -focal};
            let direction = pixel - focal_point;

            let ray = Ray {
                // We start from the screen and note the "focal point"
                origin: pixel,
                direction
            };

            let contrib = raytrace(&ray, &scene, 0);
            contrib_pixel = contrib_pixel + contrib;
            }

            let pixel = tonemap(&(contrib_pixel * (1.0 / (nbSamples as f32))), 2.0);
            img.put_pixel(px, py, pixel)
        }
    }

    img.save("result.png").unwrap();
}

fn direct_light(scene: &Scene, it: &Intersection) -> Vec3
{
                   let between = Uniform::new(0, scene.lights.len());
                   let mut rng = rand::thread_rng();
                   let i = between.sample(&mut rng);
                   let light = &scene.lights[i];
                   let light_probability = 1.0 / (scene.lights.len() as f32);

                   let mut contrib = Vec3{x: 1.0, y:1.0, z:1.0};

                   // for light in &scene.lights
                   {
                     let to_light = light.origin - it.point;
                     let light_distance = to_light.length();
                     let cos = (to_light.normalize().dot(&it.normal)).clamp(0.0, 1.0);

                     // This is the amount of light, but not taking into account the visibilty
                     let v = cos / light_distance * light.emission;

                     // Visibility
                     // We shoot a ray toward the light
                     let shadow_direction = to_light.normalize();
                     // Let's add a small offset to the origin point of the ray, so it won't
                     // intersect the surface we are on.
                     let origin_with_delta = it.point + 0.1 * shadow_direction;
                     let shadow_ray = Ray{origin: origin_with_delta, direction: shadow_direction};
            
                     // compute the intersection
                     // NOTE: this could be optimised because we don't care about the FIRST
                     // intersection. Instead we want to find any intersection between the point
                     // and the light. So we can implement an early exit. We also don't care about
                     // material / normal / ... at intersection point.
                     let it_shadow = scene.objects.intersect(&shadow_ray);

                     let visibility = match it_shadow
                        {
                            // No intersection: full visibility
                            None => 1.0,
                            // Intersection, we check that the intersection is between us and the
                            // light
                            Some(it) => if it.distance > light_distance
                                { 1.0 } else { 0.0 }
                        };

                     contrib = contrib + (visibility / light_probability * v);
                  }

                contrib

}

fn raytrace(ray: &Ray, scene: &Scene, depth: u32) -> Vec3
{
    if depth > 10
    {
        Vec3{x: 0.0, y: 0.0, z: 0.0}
    }
    else
    {
            let it_m = scene.objects.intersect(&ray);

            match it_m
            {
                // We have some intersection
                Some(it) =>
                {
                    match it.bsdf
                    {
                        BSDF::Diffuse => direct_light(&scene, &it) * it.albedo,
                        BSDF::Mirror => {
                            contrib_mirror(ray, &it, scene, depth) * it.albedo
                        },
                        BSDF::Glass(eta) => {
                            let between = Uniform::new(0.0, 1.0);
                            let mut rng = rand::thread_rng();

                            let u = between.sample(&mut rng);

                            let r = schlick(1.0, eta, ray.direction.normalize().dot(&it.normal).abs());

                            let threshold = 1.0 - r;


                            let (n, eta2) = if ray.direction.dot(&it.normal) < 0.0 {
                                  (it.normal, eta)
                                 } else {
                                        (-1.0 * it.normal, 1.0 / eta)
                                 };
                            let refract_directionM = refract(&ray.direction.normalize(), &n, eta2);

                            let c_refract = match refract_directionM
                            {
                                Some(refract_direction) => {
                                    if(u < threshold)
                                    {
                                      let origin_with_delta = it.point + 0.01 * refract_direction;
                                      let ray_refract = Ray{
                                          origin: origin_with_delta,
                                          direction: refract_direction
                                      };
                                      let contrib_refract = raytrace(&ray_refract, &scene, depth + 1);
                                      contrib_refract * it.albedo * (1.0 / threshold)
                                    }
                                    else
                                    {
                                        Vec3{x: 0.0, y: 0.0, z: 0.0}
                                    }
                                    // Vec3{x: 2000.0, y: 0.0, z: 2000.0}

                                },
                                None => 
                                {
                                     // Vec3{x: 0.0, y: 2000.0, z: 0.0}
                                     Vec3{x: 0.0, y: 000.0, z: 0.0}
                                }
                            };

                            let c_miror = 
                                if(u > threshold)
                                {
                                   contrib_mirror(ray, &it, scene, depth) * it.albedo * (1.0 / (1.0 - threshold))
                                }
                               else
                                {
                                    Vec3{x: 0.0, y: 0.0, z: 0.0}
                                };


                            (1.0 - r) * c_refract + r * c_miror
                        }
                    }
                }
                None => 
                {
                    // No intersection, let's put a stupid atmospherse
                    // Vertical is (x = 0, y = -1, z = 0)
                    let vertical_offset = ray.direction.normalize().dot(&Vec3{x: 0.0, y: -1.0, z: 0.0}).abs();
                    let scale = 100.0;

                   Vec3{x: 0.5 * scale, y: (0.5 + vertical_offset / 2.0) * scale, z: 0.5 * scale}
                }
            }
      }
}

fn reflect(i: &Vec3, n: &Vec3) -> Vec3
{
    *i * 2.0 * (- i.dot(n) * n)
}

fn refract(I: &Vec3, N: &Vec3, eta: f32) -> Option<Vec3>
{
  let k = 1.0 - eta * eta * (1.0 - N.dot(I) * N.dot(I));
    if (k < 0.0)
    {
        None
}
    else
    {
        Some(eta * I - (eta * N.dot(I) + k.sqrt()) * N)
    }
}

fn contrib_mirror(ray: &Ray, it: &Intersection, scene: &Scene, depth: u32) -> Vec3
{

                            let mirror_direction = reflect(&ray.direction.normalize(), &it.normal);

                            let origin_with_delta = it.point + 0.1 * mirror_direction;
                            let ray_mirror = Ray{
                                origin: origin_with_delta,
                                direction: mirror_direction
                            };
                            let contrib_mirror = raytrace(&ray_mirror, &scene, depth + 1);
                            contrib_mirror * it.albedo


}

fn schlick(n1:f32, n2: f32, cos_theta: f32) -> f32
{
    let r0 = ((n1 - n2) / (n1 + n2)).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
}


/*
 *
 *
 *  mesure = x
 *  probabilité: p
 *
 *  contribution = x / p
 *
 *
 *
 *  0.1 / 0.1 => 1
 *  0.9 / 0.9 => 1
 *
 *
 *
 */

