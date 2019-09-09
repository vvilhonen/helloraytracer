use core::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let rand_vector = Vec3(
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>(),
            );
            let candidate = 2.0 * rand_vector - Vec3(1.0, 1.0, 1.0);
            if candidate.squared_length() < 1.0 {
                return candidate;
            }
        }
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        let Vec3(x, y, z) = self;
        (x * x + y * y + z * z)
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        let Vec3(ax, ay, az) = self;
        let Vec3(bx, by, bz) = other;
        Vec3(ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
    }

    pub fn rotate_around_y(&self, angle_rad: f32) -> Self {
        let Vec3(x, y, z) = self;
        Vec3(
            angle_rad.cos() * x - angle_rad.sin() * z,
            *y,
            angle_rad.sin() * x + angle_rad.cos() * z,
        )
    }

    pub fn mix(&self, other: &Self, t: f32) -> Self {
        *self * t + *other * (1.0 - t)
    }
}

impl ::core::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ::core::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.x(), -self.y(), -self.z())
    }
}

impl ::core::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl ::core::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ::core::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ::core::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ::core::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), ::core::fmt::Error> {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)?;
        Ok(())
    }
}
