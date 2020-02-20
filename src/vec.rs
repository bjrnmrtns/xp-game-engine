use std::cmp;
use std::ops::{Sub, Add, Mul, Div};
use num::traits::{One};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Copy> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }
    pub fn xy(&self) -> Vec2<T> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl<T: Copy + One> Vec3<T> {
    pub fn to_vec4(&self) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: T::one(),
        }
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,

        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Copy + Mul<Output = T> + Add<Output = T>> Vec3<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y + rhs.y + self.z * rhs.z
    }
}

impl<T: Copy + Mul<Output = T> + Sub<Output = T>> Vec3<T> {
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Vec3<f32> {
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(self) -> Vec3<f32> {
        let length = self.length();
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Div<Output = T>> Div for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Copy + Mul<Output = T> + Add<Output = T>> Vec2<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y + rhs.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_vec2() {
        assert_eq!(Vec2::<i32> { x: 1, y: 1 } + Vec2::<i32> { x: 2, y: 2 }, Vec2::<i32> { x: 3, y: 3 });
    }
    #[test]
    fn sub_vec2() {
        assert_eq!(Vec2::<i32> { x: 1, y: 1 } - Vec2::<i32> { x: 2, y: 2 }, Vec2::<i32> { x: -1, y: -1 });
    }
    #[test]
    fn mul_vec2() {
        assert_eq!(Vec2::<i32> { x: 1, y: 2 } * Vec2::<i32> { x: 2, y: 2 }, Vec2::<i32> { x: 2, y: 4 });
    }
    #[test]
    fn div_vec2() {
        assert_eq!(Vec2::<i32> { x: 4, y: 2 } / Vec2::<i32> { x: 2, y: 2 }, Vec2::<i32> { x: 2, y: 1 });
    }
}
