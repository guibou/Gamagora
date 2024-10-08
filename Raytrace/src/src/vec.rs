use std::ops::{Add, Sub, Mul};

// Vector logic
#[derive(Copy, Clone)]
pub struct Vec3
{
    pub x:f32, pub y:f32, pub z:f32
}

impl Vec3 {
  pub fn dot(&self, b: &Vec3) -> f32
  {
      self.x * b.x + self.y * b.y + self.z * b.z
  }
  
  pub fn length(&self) -> f32
  {
      self.dot(&self).sqrt()
  }
  
  // Special case, compute the squared length and saves a sqrt
  pub fn length_squared(&self) -> f32
  {
      self.dot(self)
  }
  
  pub fn normalize(&self) -> Vec3
  {
    let l = self.length();
    Vec3{x: self.x / l, y: self.y / l, z: self.z / l}
  }

  pub fn minv(&self, other: &Vec3) -> Vec3
  {
      Vec3{x:self.x.min(other.x)
          ,y:self.y.min(other.y)
          ,z:self.z.min(other.z)
      }
  }

  pub fn maxv(&self, other: &Vec3) -> Vec3
  {
      Vec3{x:self.x.max(other.x)
          ,y:self.y.max(other.y)
          ,z:self.z.max(other.z)
      }
  }
}

impl Add for Vec3
{
  type Output = Self;
  fn add(self, b: Self) -> Self
  {
      Vec3{x: self.x + b.x, y: self.y + b.y, z: self.z + b.z}
  }
}

impl Mul for Vec3
{
  type Output = Self;
  fn mul(self, b: Self) -> Self
  {
      Vec3{x: self.x * b.x, y: self.y * b.y, z: self.z * b.z}
  }
}

impl Sub for Vec3
{
  type Output = Self;
  fn sub(self, b: Self) -> Self
  {
      Vec3{x: self.x - b.x, y: self.y - b.y, z: self.z - b.z}
  }
}

impl Mul<f32> for Vec3
{
  type Output = Self;
  fn mul(self, f:f32) -> Vec3
  {
      Vec3{x: self.x * f, y: self.y * f, z: self.z * f}
  }
}

impl Mul<&Vec3> for f32
{
  type Output = Vec3;
  fn mul(self, v:&Vec3) -> Vec3
  {
      Vec3{x: self * v.x, y: self * v.y, z: self * v.z}
  }
}

impl Mul<Vec3> for f32
{
  type Output = Vec3;
  fn mul(self, v:Vec3) -> Vec3
  {
      Vec3{x: self * v.x, y: self * v.y, z: self * v.z}
  }
}
