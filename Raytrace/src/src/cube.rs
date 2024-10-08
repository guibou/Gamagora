use super::vec::Vec3;
use super::ray::*;

#[derive(Copy, Clone)]
pub struct AABB
{
    pub p_min: Vec3,
    pub p_max: Vec3
}

pub enum Axis { X, Y, Z }

impl AABB
{
    pub fn union(&self, other: &AABB) -> AABB
    {
        AABB{p_min: self.p_min.minv(&other.p_min),
             p_max: self.p_max.maxv(&other.p_max)}
    }

    pub fn largest_axis(&self) -> Axis
    {
        let axis = self.p_max - self.p_min;
        if axis.x >= axis.y && axis.x >= axis.z
        {
            Axis::X
        }
        else if axis.y >= axis.z
        {
            Axis::Y
        }
        else
        {
            Axis::Z
        }
    }
}

/*
-- Based on https://tavianator.com/fast-branchless-raybounding-box-intersections/
-- | Returns the entry and exit point of ray/box intersection. That is,
-- @rayIntersectAABBRange ray box@ returns @Just (tmin, tmax)@ meaning that the
-- ray enter the box at @tmin@ and exit it at @tmax@, which can be negative.
*/

pub fn intersect_cube(ray: &Ray, cube: &AABB) -> Option<f32>
{
/*
 *
rayIntersectAABBRange (Ray (P ox oy oz) (N dx dy dz)) (Box (P pminx pminy pminz) (P pmaxx pmaxy pmaxz))
  | tmax'' < tmin'' = Nothing
  | otherwise = Just (tmin'', tmax'')
  where
*/
    let rinvx = 1.0 / ray.direction.x;
    let rinvy = 1.0 / ray.direction.y;
    let rinvz = 1.0 / ray.direction.z;

    // X slab
    let tx1 = (cube.p_min.x - ray.origin.x) * rinvx;
    let tx2 = (cube.p_max.x - ray.origin.x) * rinvx;

    let tmin = min(tx1, tx2);
    let tmax = max(tx1, tx2);

    // Y slab
    let ty1 = (cube.p_min.y - ray.origin.y) * rinvy;
    let ty2 = (cube.p_max.y - ray.origin.y) * rinvy;

    let tminp = max(tmin, min(ty1, ty2));
    let tmaxp = min(tmax, max(ty1, ty2));

    // Z slab
    let tz1 = (cube.p_min.z - ray.origin.z) * rinvz;
    let tz2 = (cube.p_max.z - ray.origin.z) * rinvz;

    let tminpp = max(tminp, min(tz1, tz2));
    let tmaxpp = min(tmaxp, max(tz1, tz2));

    if tmaxpp < tminpp
    {
      None
    }
    else
    {
      Some(tminpp)
    }
}

fn min(a: f32, b: f32) -> f32
{
    a.min(b)
}

fn max(a: f32, b: f32) -> f32
{
    a.max(b)
}

