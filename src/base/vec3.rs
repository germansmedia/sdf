use {
    crate::*,
    std::{
        cmp::PartialEq,
        fmt::{
            Display,
            Formatter,
            Result,
        },
        ops::{
            Add,
            Sub,
            Mul,
            Div,
            AddAssign,
            SubAssign,
            MulAssign,
            DivAssign,
            Neg,
        },
    },
};

/// 3D vector of numbers.
#[derive(Copy,Clone,Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// implementations where $t: Sized + Zero + One
macro_rules! vec3_impl {
    ($($t:ty)+) => {
        $(
            impl Vec3<$t> {

                /// Unit vector in positive X-direction.
                pub const UNIT_X: Self = Vec3 { x: <$t>::ONE,y: <$t>::ZERO,z: <$t>::ZERO, };

                /// Unit vector in positive Y-direction.
                pub const UNIT_Y: Self = Vec3 { x: <$t>::ZERO,y: <$t>::ONE,z: <$t>::ZERO, };

                /// Unit vector in positive Z-direction.
                pub const UNIT_Z: Self = Vec3 { x: <$t>::ZERO,y: <$t>::ZERO,z: <$t>::ONE, };

                /// Calculate dot product.
                pub fn dot(self,other: &Vec3<$t>) -> $t {
                    self.x * other.x + self.y * other.y + self.z * other.z
                }

                /// Calculate cross product.
                pub fn cross(self,other: &Vec3<$t>) -> Vec3<$t> {
                    Vec3 {
                        x: self.y * other.z - self.z * other.y,
                        y: self.z * other.x - self.x * other.z,
                        z: self.x * other.y - self.y * other.x,
                    }
                }
            }

            impl Display for Vec3<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{},{})",self.x,self.y,self.z)
                }
            }

            impl PartialEq for Vec3<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
                }
            }

            impl Zero for Vec3<$t> {
                const ZERO: Vec3<$t> = Vec3 { x: <$t>::ZERO,y: <$t>::ZERO,z: <$t>::ZERO, };
            }

            /// Vector + vector.
            impl Add<Vec3<$t>> for Vec3<$t> {
                type Output = Vec3<$t>;
                fn add(self,other: Self) -> Self {
                    Vec3 {
                        x: self.x + other.x,
                        y: self.y + other.y,
                        z: self.z + other.z,
                    }
                }
            }

            /// Vector += vector.
            impl AddAssign<Vec3<$t>> for Vec3<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.x += other.x;
                    self.y += other.y;
                    self.z += other.z;
                }
            }

            /// Vector - vector.
            impl Sub<Vec3<$t>> for Vec3<$t> {
                type Output = Vec3<$t>;
                fn sub(self,other: Self) -> Self {
                    Vec3 {
                        x: self.x - other.x,
                        y: self.y - other.y,
                        z: self.z - other.z,
                    }
                }
            }

            /// Vector -= vector.
            impl SubAssign<Vec3<$t>> for Vec3<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.x -= other.x;
                    self.y -= other.y;
                    self.z -= other.z;
                }
            }

            /// Scalar * vector.
            impl Mul<Vec3<$t>> for $t {
                type Output = Vec3<$t>;
                fn mul(self,other: Vec3<$t>) -> Self::Output {
                    Vec3 {
                        x: self * other.x,
                        y: self * other.y,
                        z: self * other.z,
                    }
                }
            }
            
            /// Vector * scalar.
            impl Mul<$t> for Vec3<$t> {
                type Output = Vec3<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Vec3 {
                        x: self.x * other,
                        y: self.y * other,
                        z: self.z * other,
                    }
                }
            }

            /// Component-wise vector * vector.
            impl Mul<Vec3<$t>> for Vec3<$t> {
                type Output = Vec3<$t>;
                fn mul(self,other: Vec3<$t>) -> Self::Output {
                    Vec3 {
                        x: self.x * other.x,
                        y: self.y * other.y,
                        z: self.z * other.z,
                    }
                }
            }

            /// Vector *= scalar.
            impl MulAssign<$t> for Vec3<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.x *= other;
                    self.y *= other;
                    self.z *= other;
                }
            }

            /// Component-wise vector *= vector.
            impl MulAssign<Vec3<$t>> for Vec3<$t> {
                fn mul_assign(&mut self,other: Vec3<$t>) {
                    self.x *= other.x;
                    self.y *= other.y;
                    self.z *= other.z;
                }
            }

            /// Vector / scalar.
            impl Div<$t> for Vec3<$t> {
                type Output = Vec3<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Vec3 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                    }
                }
            }

            /// Vector /= scalar.
            impl DivAssign<$t> for Vec3<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                }
            }

            /// -Vector.
            impl Neg for Vec3<$t> {
                type Output = Vec3<$t>;
                fn neg(self) -> Self::Output {
                    Vec3 {
                        x: -self.x,
                        y: -self.y,
                        z: -self.z,
                    }
                }
            }
        )+
    }
}

vec3_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

// implementations where $t: Real
macro_rules! vec3_real_impl {
    ($($t:ty)+) => {
        $(
            impl Vec3<$t> {

                /// Calculate vector length.
                pub fn length(&self) -> $t {
                    self.dot(&self).sqrt()
                }

                /// Normalize vector.
                pub fn normalize(&mut self) {
                    let d = self.length();
                    if d != <$t>::ZERO {
                        *self /= d;
                    }
                }
            }
        )+
    }
}

vec3_real_impl! { f32 f64 }