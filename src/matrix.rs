use std::cmp;
use std::ops::{Add, Mul};

pub struct Matrix4x4<T>(T, T, T, T,
                        T, T, T, T,
                        T, T, T, T,
                        T, T, T, T);

impl<'a, 'b, T> Mul<&'a Vec4<T>> for &'b Matrix4x4<T>
    where T: Mul<T, Output=T> + Add<T, Output=T> + Copy + Num {
    type Output = Vec4<T>;

    fn mul(self, rhs: &Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.get(0, 0) * rhs.x + self.get(0, 1) * rhs.y + self.get(0, 2) * rhs.z + self.get(0, 3) * rhs.w,
            y: self.get(1, 0) * rhs.x + self.get(1, 1) * rhs.y + self.get(1, 2) * rhs.z + self.get(1, 3) * rhs.w,
            z: self.get(2, 0) * rhs.x + self.get(2, 1) * rhs.y + self.get(2, 2) * rhs.z + self.get(2, 3) * rhs.w,
            w: self.get(3, 0) * rhs.x + self.get(3, 1) * rhs.y + self.get(3, 2) * rhs.z + self.get(3, 3) * rhs.w,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output=T> + Copy> Mul for Matrix4x4<T> {
    type Output = Vec4<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec4 {
            x: self.0 * rhs.x + self.1 * rhs.y + self.2 * rhs.z + self.3 * rhs.w,
            y: self.4 * rhs.x + self.5 * rhs.y + self.6 * rhs.z + self.7 * rhs.w,
            z: self.8 * rhs.x + self.9 * rhs.y + self.10 * rhs.z + self.11 * rhs.w,
            w: self.12 * rhs.x + self.13 * rhs.y + self.14 * rhs.z + self.15 * rhs.w,
        }
    }
}