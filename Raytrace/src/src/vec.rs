// Vector logic
#[derive(Copy, Clone)]
pub struct Vec3
{
    pub x:f32, pub y:f32, pub z:f32
}

pub fn subtract_vector(a: &Vec3, b: &Vec3) -> Vec3
{
    Vec3{x: a.x - b.x, y: a.y - b.y, z: a.z - b.z}
}

pub fn add_vector(a: &Vec3, b: &Vec3) -> Vec3
{
    Vec3{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z}
}

pub fn mul_vector(a: &Vec3, b: &Vec3) -> Vec3
{
    Vec3{x: a.x * b.x, y: a.y * b.y, z: a.z * b.z}
}

pub fn mul_scalar_vector(a: f32, b: &Vec3) -> Vec3
{
    Vec3{x: a * b.x, y: a * b.y, z: a * b.z}
}

pub fn dot(a: &Vec3, b: &Vec3) -> f32
{
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn length(a: &Vec3) -> f32
{
    dot(a, a).sqrt()
}

// Special case, compute the squared length and saves a sqrt
pub fn length_squared(a: &Vec3) -> f32
{
    dot(a, a)
}

pub fn normalize(a: &Vec3) -> Vec3
{
  let l = length(a);
  Vec3{x: a.x / l, y: a.y / l, z: a.z / l}

}
