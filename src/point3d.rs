use std::ops;

#[derive(Debug, PartialEq)]
pub struct Point3D(i32, i32, i32);

impl ops::Add<Point3D> for Point3D {
    type Output = Self;

    fn add(self, rhs: Point3D) -> Self::Output {
        Self(rhs.0 + self.0, rhs.1 + self.1, rhs.2 + self.2)
    }
}

impl ops::Sub<Point3D> for Point3D {
    type Output = Self;

    fn sub(self, rhs: Point3D) -> Self::Output {
	Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Neg for Point3D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl ops::Mul<f32> for Point3D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(
            (self.0 as f32 * rhs) as i32,
            (self.1 as f32 * rhs) as i32,
            (self.2 as f32 * rhs) as i32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point3d_add_test() {
        assert_eq!(Point3D(1, 2, 3) + Point3D(1, 2, 3), Point3D(2, 4, 6));
    }

    #[test]
    fn point3d_sub_test() {
        assert_eq!(Point3D(2, 4, 6) - Point3D(1, 2, 3), Point3D(1, 2, 3));
    }

    #[test]
    fn point3d_neg_test() {
        assert_eq!(-Point3D(1, 2, 3), Point3D(-1, -2, -3));
    }

    #[test]
    fn point3d_mul_test() {
        assert_eq!(Point3D(2, 4, 6) * 0.5, Point3D(1, 2, 3));
    }
}
