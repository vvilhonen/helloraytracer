use crate::util::vec3::Vec3;
use std::ops::Mul;

#[derive(Debug, PartialEq)]
struct Mat4 {
    data: [f32; 16],
}

impl Mat4 {
    pub fn new(data: [f32; 16]) -> Self {
        Mat4 { data }
    }

    #[rustfmt::skip]
    pub fn identity() -> Self {
        Mat4 {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    #[rustfmt::skip]
    pub fn rotate_y(angle_rad: f32) -> Self {
        Mat4 {
            data: [
                angle_rad.cos(), 0.0, angle_rad.sin(), 0.0,
                0.0, 1.0, 0.0, 0.0,
                -angle_rad.sin(), 0.0, angle_rad.cos(), 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    #[rustfmt::skip]
    pub fn translate(how: &Vec3) -> Self {
        let mut id = Self::identity();
        id.data[0*4+3] = how.x();
        id.data[1*4+3] = how.y();
        id.data[2*4+3] = how.z();
        id
    }
}

impl Mul<Vec3> for Mat4 {
    type Output = Vec3;

    #[rustfmt::skip]
    fn mul(self, rhs: Vec3) -> Self::Output {
        macro_rules! mulrow {
            ($row:expr, $x:expr, $y:expr, $z:expr, $w:expr) => {{
                self.data[$row*4+0] * $x +
                self.data[$row*4+1] * $y +
                self.data[$row*4+2] * $z +
                self.data[$row*4+3] * $w
            }};
        }

        let Vec3(x, y, z) = rhs;
        let w = mulrow!(3, x, y, z, 1.0);
        Vec3(
            mulrow!(0, x, y, z, 1.0),
            mulrow!(1, x, y, z, 1.0),
            mulrow!(2, x, y, z, 1.0),
        ) / w
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut data = [0.0f32; 16];
        for x in 0..4 {
            for y in 0..4 {
                data[y * 4 + x] = self.data[y * 4 + x] * rhs.data[x * 4 + y];
            }
        }
        Mat4 { data }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32::consts::PI;

    const TESTVEC: Vec3 = Vec3(1.0, 2.0, 3.0);

    #[test]
    fn identity() {
        let multiplied = Mat4::identity() * TESTVEC;
        assert_eq!(multiplied, TESTVEC, "identity multiplication works");
    }

    #[test]
    fn translate() {
        let translated = Mat4::translate(&Vec3(3.0, 2.0, 1.0)) * TESTVEC;
        assert_eq!(translated, Vec3(4.0, 4.0, 4.0));
    }

    #[test]
    fn rotation() {
        let rotated = Mat4::identity() * Mat4::rotate_y(PI * 2.0);
        assert_eq!(rotated, Mat4::identity());
    }
}
