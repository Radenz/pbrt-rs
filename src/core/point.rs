use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::vec::{Vector2, Vector3};

#[derive(Clone, Copy)]
pub struct Point2<T>([T; 2]);
#[derive(Clone, Copy)]
pub struct Point3<T>([T; 3]);

pub type Point2i = Point2<i32>;
pub type Point2f = Point2<f64>;
pub type Point3i = Point3<i32>;
pub type Point3f = Point3<f64>;

impl<T> Display for Point2<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self[0], self[1])
    }
}

impl<T> Display for Point3<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self[0], self[1], self[2])
    }
}

impl<T> Point2<T> {
    pub fn new(values: [T; 2]) -> Self {
        Self(values)
    }

    pub fn take_inner(self) -> [T; 2] {
        self.0
    }
}

impl<T> Point3<T> {
    pub fn new(values: [T; 3]) -> Self {
        Self(values)
    }

    pub fn take_inner(self) -> [T; 3] {
        self.0
    }
}

impl<T> Index<usize> for Point2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Point2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> Index<usize> for Point3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Point3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> Point3<T>
where
    T: Copy,
{
    pub fn permute(&self, x: usize, y: usize, z: usize) -> Self {
        Self([self[x], self[y], self[z]])
    }
}

impl<T> From<Point3<T>> for Point2<T> {
    fn from(value: Point3<T>) -> Self {
        let [x, y, _] = value.0;
        Self([x, y])
    }
}

impl<T> From<Vector2<T>> for Point2<T> {
    fn from(vec: Vector2<T>) -> Self {
        Self(vec.take_inner())
    }
}

impl<T> From<Vector3<T>> for Point3<T> {
    fn from(vec: Vector3<T>) -> Self {
        Self(vec.take_inner())
    }
}

macro_rules! num_point2_impl {
    ($type:ty) => {
        impl Point2<$type> {
            pub fn zero() -> Self {
                Self([0 as $type, 0 as $type])
            }

            pub fn set(&mut self, value: &Self) {
                self[0] = value[0];
                self[1] = value[1];
            }
        }

        impl Add<Vector2<$type>> for Point2<$type> {
            type Output = Self;
            fn add(mut self, rhs: Vector2<$type>) -> Self::Output {
                self[0] += rhs[0];
                self[1] += rhs[1];
                self
            }
        }

        impl AddAssign<Vector2<$type>> for Point2<$type> {
            fn add_assign(&mut self, rhs: Vector2<$type>) {
                self[0] += rhs[0];
                self[1] += rhs[1];
            }
        }

        impl Add<Point2<$type>> for Point2<$type> {
            type Output = Self;
            fn add(mut self, rhs: Point2<$type>) -> Self::Output {
                self[0] += rhs[0];
                self[1] += rhs[1];
                self
            }
        }

        impl AddAssign<Point2<$type>> for Point2<$type> {
            fn add_assign(&mut self, rhs: Point2<$type>) {
                self[0] += rhs[0];
                self[1] += rhs[1];
            }
        }

        impl Sub<Vector2<$type>> for Point2<$type> {
            type Output = Self;
            fn sub(mut self, rhs: Vector2<$type>) -> Self::Output {
                self[0] -= rhs[0];
                self[1] -= rhs[1];
                self
            }
        }

        impl SubAssign<Vector2<$type>> for Point2<$type> {
            fn sub_assign(&mut self, rhs: Vector2<$type>) {
                self[0] -= rhs[0];
                self[1] -= rhs[1];
            }
        }

        impl Sub<Point2<$type>> for Point2<$type> {
            type Output = Vector2<$type>;
            fn sub(mut self, rhs: Point2<$type>) -> Self::Output {
                self[0] -= rhs[0];
                self[1] -= rhs[1];
                self.into()
            }
        }

        impl Neg for Point2<$type> {
            type Output = Self;
            fn neg(mut self) -> Self::Output {
                self[0] *= -1 as $type;
                self[1] *= -1 as $type;
                self
            }
        }

        impl<T, U> Mul<T> for Point2<$type>
        where
            $type: Mul<T, Output = U>,
            T: Copy,
        {
            type Output = Point2<U>;
            fn mul(self, rhs: T) -> Self::Output {
                Point2([self[0] * rhs, self[1] * rhs])
            }
        }

        impl<T> MulAssign<T> for Point2<$type>
        where
            $type: MulAssign<T>,
            T: Copy,
        {
            fn mul_assign(&mut self, rhs: T) {
                self[0] *= rhs;
                self[1] *= rhs;
            }
        }

        impl<T, U> Div<T> for Point2<$type>
        where
            $type: Div<T, Output = U>,
            T: Copy,
        {
            type Output = Point2<U>;
            fn div(self, rhs: T) -> Self::Output {
                Point2([self[0] / rhs, self[1] / rhs])
            }
        }

        impl<T> DivAssign<T> for Point2<$type>
        where
            $type: DivAssign<T>,
            T: Copy,
        {
            fn div_assign(&mut self, rhs: T) {
                self[0] /= rhs;
                self[1] /= rhs;
            }
        }

        impl PartialEq<Point2<$type>> for Point2<$type> {
            fn eq(&self, other: &Self) -> bool {
                self[0] == other[0] && self[1] == other[1]
            }
        }
    };
}

macro_rules! num_point2_cast_impl {
    ($from:ty => $to:ty) => {
        impl From<Point2<$from>> for Point2<$to> {
            fn from(value: Point2<$from>) -> Self {
                let [x, y] = value.0;
                Self([x as $to, y as $to])
            }
        }
    };
}

macro_rules! num_point3_impl {
    ($type:ty) => {
        impl Point3<$type> {
            pub fn zero() -> Self {
                Self([0 as $type, 0 as $type, 0 as $type])
            }

            pub fn set(&mut self, value: &Self) {
                self[0] = value[0];
                self[1] = value[1];
                self[2] = value[2];
            }

            pub fn distance(&self, other: &Self) -> f64 {
                (self.clone() - other.clone()).length()
            }

            pub fn distance_squared(&self, other: &Self) -> f64 {
                (self.clone() - other.clone()).length_squared()
            }

            pub fn lerp(ratio: f64, point1: &Self, point2: &Self) -> Point3<f64> {
                let point1: Point3<f64> = (*point1).into();
                let point2: Point3<f64> = (*point2).into();
                point1 * (1. - ratio) + point2 * ratio
            }

            pub fn min(&self, other: &Self) -> Self {
                Self([
                    self[0].min(other[0]),
                    self[1].min(other[1]),
                    self[2].min(other[2]),
                ])
            }

            pub fn max(&self, other: &Self) -> Self {
                Self([
                    self[0].max(other[0]),
                    self[1].max(other[1]),
                    self[2].max(other[2]),
                ])
            }

            pub fn abs(&self) -> Self {
                Self([self[0].abs(), self[1].abs(), self[2].abs()])
            }
        }

        impl Add<Vector3<$type>> for Point3<$type> {
            type Output = Self;
            fn add(mut self, rhs: Vector3<$type>) -> Self::Output {
                self[0] += rhs[0];
                self[1] += rhs[1];
                self[2] += rhs[2];
                self
            }
        }

        impl AddAssign<Vector3<$type>> for Point3<$type> {
            fn add_assign(&mut self, rhs: Vector3<$type>) {
                self[0] += rhs[0];
                self[1] += rhs[1];
                self[2] += rhs[2];
            }
        }

        impl Add<Point3<$type>> for Point3<$type> {
            type Output = Self;
            fn add(mut self, rhs: Point3<$type>) -> Self::Output {
                self[0] += rhs[0];
                self[1] += rhs[1];
                self[2] += rhs[2];
                self
            }
        }

        impl AddAssign<Point3<$type>> for Point3<$type> {
            fn add_assign(&mut self, rhs: Point3<$type>) {
                self[0] += rhs[0];
                self[1] += rhs[1];
                self[2] += rhs[2];
            }
        }

        impl Sub<Vector3<$type>> for Point3<$type> {
            type Output = Self;
            fn sub(mut self, rhs: Vector3<$type>) -> Self::Output {
                self[0] -= rhs[0];
                self[1] -= rhs[1];
                self[2] -= rhs[2];
                self
            }
        }

        impl SubAssign<Vector3<$type>> for Point3<$type> {
            fn sub_assign(&mut self, rhs: Vector3<$type>) {
                self[0] -= rhs[0];
                self[1] -= rhs[1];
                self[2] -= rhs[2];
            }
        }

        impl Sub<Point3<$type>> for Point3<$type> {
            type Output = Vector3<$type>;
            fn sub(mut self, rhs: Point3<$type>) -> Self::Output {
                self[0] -= rhs[0];
                self[1] -= rhs[1];
                self[2] -= rhs[2];
                self.into()
            }
        }

        impl Neg for Point3<$type> {
            type Output = Self;
            fn neg(mut self) -> Self::Output {
                self[0] *= -1 as $type;
                self[1] *= -1 as $type;
                self[2] *= -1 as $type;
                self
            }
        }

        impl<T, U> Mul<T> for Point3<$type>
        where
            $type: Mul<T, Output = U>,
            T: Copy,
        {
            type Output = Point3<U>;
            fn mul(self, rhs: T) -> Self::Output {
                Point3([self[0] * rhs, self[1] * rhs, self[2] * rhs])
            }
        }

        impl<T> MulAssign<T> for Point3<$type>
        where
            $type: MulAssign<T>,
            T: Copy,
        {
            fn mul_assign(&mut self, rhs: T) {
                self[0] *= rhs;
                self[1] *= rhs;
                self[2] *= rhs;
            }
        }

        impl<T, U> Div<T> for Point3<$type>
        where
            $type: Div<T, Output = U>,
            T: Copy,
        {
            type Output = Point3<U>;
            fn div(self, rhs: T) -> Self::Output {
                Point3([self[0] / rhs, self[1] / rhs, self[2] / rhs])
            }
        }

        impl<T> DivAssign<T> for Point3<$type>
        where
            $type: DivAssign<T>,
            T: Copy,
        {
            fn div_assign(&mut self, rhs: T) {
                self[0] /= rhs;
                self[1] /= rhs;
                self[2] /= rhs;
            }
        }

        impl PartialEq<Point3<$type>> for Point3<$type> {
            fn eq(&self, other: &Self) -> bool {
                self[0] == other[0] && self[1] == other[1] && self[2] == other[2]
            }
        }
    };
}

impl Point3<f64> {
    pub fn floor(&self) -> Self {
        Self([self[0].floor(), self[1].floor(), self[2].floor()])
    }

    pub fn ceil(&self) -> Self {
        Self([self[0].ceil(), self[1].ceil(), self[2].ceil()])
    }
}

macro_rules! num_point3_cast_impl {
    ($from:ty => $to:ty) => {
        impl From<Point3<$from>> for Point3<$to> {
            fn from(value: Point3<$from>) -> Self {
                let [x, y, z] = value.0;
                Self([x as $to, y as $to, z as $to])
            }
        }
    };
}

num_point2_impl!(i32);
num_point2_cast_impl!(i32 => f64);
num_point2_impl!(f64);
num_point2_cast_impl!(f64 => i32);

num_point3_impl!(i32);
num_point3_cast_impl!(i32 => f64);
num_point3_impl!(f64);
num_point3_cast_impl!(f64 => i32);
