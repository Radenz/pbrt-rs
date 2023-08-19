use std::ops::{Add, AddAssign, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, SubAssign};

pub struct Vector<const N: usize, T>([T; N]);

impl<T, const N: usize> Default for Vector<N, T>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self([T::default(); N])
    }
}

impl<T, const N: usize> Vector<N, T> {
    pub fn new(values: [T; N]) -> Self {
        Self(values)
    }

    pub fn set(&mut self, values: [T; N]) {
        self.0 = values;
    }

    pub fn take_inner(self) -> [T; N] {
        self.0
    }
}

impl<T, const N: usize> Index<usize> for Vector<N, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < N);
        &self.0[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < N);
        &mut self.0[index]
    }
}

impl<T, const N: usize> AddAssign for Vector<N, T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, mut rhs: Self) {
        for i in 0..N {
            self[i] += rhs[i];
        }
    }
}

impl<T, const N: usize> SubAssign for Vector<N, T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] -= rhs[i];
        }
    }
}

impl<T, const N: usize> PartialEq for Vector<N, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self[i] != other[i] {
                return false;
            };
        }
        true
    }
}

impl<T, const N: usize, U> MulAssign<U> for Vector<N, T>
where
    T: MulAssign<U>,
    U: Copy,
{
    fn mul_assign(&mut self, rhs: U) {
        for i in 0..N {
            self[i] *= rhs;
        }
    }
}

impl<T, const N: usize, U> DivAssign<U> for Vector<N, T>
where
    T: DivAssign<U>,
    U: Copy,
{
    fn div_assign(&mut self, rhs: U) {
        for i in 0..N {
            self[i] /= rhs;
        }
    }
}

impl<T, const N: usize> Neg for Vector<N, T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for i in 0..N {
            self[i] = -self[i];
        }
        self
    }
}

pub type Vector2<T> = Vector<2, T>;
pub type Vector3<T> = Vector<3, T>;

pub type Vector2i = Vector2<i32>;
pub type Vector2f = Vector2<f64>;

pub type Vector3i = Vector3<i32>;
pub type Vector3f = Vector3<f64>;

impl<T> Vector2<T> {
    pub fn x(&self) -> &T {
        &self[0]
    }

    pub fn y(&self) -> &T {
        &self[1]
    }
}

impl<T> Vector3<T> {
    pub fn x(&self) -> &T {
        &self[0]
    }

    pub fn y(&self) -> &T {
        &self[1]
    }

    pub fn z(&self) -> &T {
        &self[2]
    }
}

impl<T> Vector3<T>
where
    T: Copy,
{
    pub fn permute(&self, x: usize, y: usize, z: usize) -> Self {
        Self([self[x], self[y], self[z]])
    }
}

macro_rules! num_vec_ops_impl {
    ($type:ty) => {
        impl<const N: usize> Vector<N, $type> {
            pub fn zero() -> Self {
                Self([0 as $type; N])
            }

            pub fn dot(&self, other: &Self) -> $type {
                let mut result = 0 as $type;
                for i in 0..N {
                    result += self[i] * other[i];
                }
                result
            }

            pub fn abs_dot(&self, other: &Self) -> $type {
                self.dot(other).abs()
            }

            pub fn length_squared(&self) -> f64 {
                let mut squared_sum = 0.;
                for i in 0..N {
                    squared_sum += (self[i] * self[i]) as f64;
                }
                squared_sum
            }

            pub fn length(&self) -> f64 {
                self.length_squared().sqrt()
            }

            pub fn normalize(&self) -> Vector<N, f64> {
                let mut result = Vector::<N, f64>::zero();
                for i in 0..N {
                    result[i] = self[i] as f64;
                }
                result /= self.length();
                result
            }

            pub fn min_component(&self) -> $type {
                *self
                    .0
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
            }

            pub fn max_component(&self) -> $type {
                *self
                    .0
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
            }

            pub fn max_dimension(&self) -> usize {
                self.0
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .map(|(index, _)| index)
                    .unwrap()
            }
        }

        impl Vector3<$type> {
            pub fn cross(&self, other: &Self) -> Self {
                let x = self.y() * other.z() - self.z() * other.y();
                let y = self.z() * other.x() - self.x() * other.z();
                let z = self.x() * other.y() - self.y() * other.x();
                Self([x, y, z])
            }
        }
    };
}

num_vec_ops_impl!(i32);
num_vec_ops_impl!(f64);

pub fn coordinate_system(v1: &Vector3<f64>, v2: &mut Vector3<f64>, v3: &mut Vector3<f64>) {
    if v1.x().abs() > v1.y().abs() {
        v2.set([-v1.z(), 0., *v1.x()]);
        *v2 /= (v1.x() * v1.x() + v1.z() * v1.z()).sqrt();
    } else {
        v2.set([0., *v1.z(), -v1.y()]);
        *v2 /= (v1.y() * v1.y() + v1.z() * v1.z()).sqrt();
    }

    *v3 = v1.cross(v2);
}
