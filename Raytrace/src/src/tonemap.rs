use image::Rgb;
use super::vec::Vec3;

pub fn tonemap(v: &Vec3, scale: f32) -> Rgb<u8>
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
