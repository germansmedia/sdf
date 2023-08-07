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

/// 4D vector of numbers.
#[derive(Copy,Clone,Debug)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

// implementations where $t: Sized + Zero + One
macro_rules! vec4_impl {
    ($($t:ty)+) => {
        $(
            impl Vec4<$t> {

                /// Unit vector in positive X-direction.
                pub const UNIT_X: Self = Vec4 { x: <$t>::ONE,y: <$t>::ZERO,z: <$t>::ZERO,w: <$t>::ZERO, };

                /// Unit vector in positive Y-direction.
                pub const UNIT_Y: Self = Vec4 { x: <$t>::ZERO,y: <$t>::ONE,z: <$t>::ZERO,w: <$t>::ZERO, };

                /// Unit vector in positive Z-direction.
                pub const UNIT_Z: Self = Vec4 { x: <$t>::ZERO,y: <$t>::ZERO,z: <$t>::ONE,w: <$t>::ZERO, };

                /// Unit vector in positive W-direction.
                pub const UNIT_W: Self = Vec4 { x: <$t>::ZERO,y: <$t>::ZERO,z: <$t>::ZERO,w: <$t>::ONE, };

                /// Calculate dot product.
                pub fn dot(self,other: &Vec4<$t>) -> $t {
                    self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
                }
            }

            impl Display for Vec4<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{},{},{})",self.x,self.y,self.z,self.w)
                }
            }

            impl PartialEq for Vec4<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.x == other.x) && (self.y == other.y) && (self.z == other.z) && (self.w == other.w)
                }
            }

            impl Zero for Vec4<$t> {
                const ZERO: Vec4<$t> = Vec4 { x: <$t>::ZERO,y: <$t>::ZERO,z: <$t>::ZERO,w: <$t>::ZERO, };
            }

            /// Vector + vector.
            impl Add<Vec4<$t>> for Vec4<$t> {
                type Output = Vec4<$t>;
                fn add(self,other: Self) -> Self {
                    Vec4 {
                        x: self.x + other.x,
                        y: self.y + other.y,
                        z: self.z + other.z,
                        w: self.w + other.w,
                    }
                }
            }

            /// Vector += vector.
            impl AddAssign<Vec4<$t>> for Vec4<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.x += other.x;
                    self.y += other.y;
                    self.z += other.z;
                    self.w += other.w;
                }
            }

            /// Vector - vector.
            impl Sub<Vec4<$t>> for Vec4<$t> {
                type Output = Vec4<$t>;
                fn sub(self,other: Self) -> Self {
                    Vec4 {
                        x: self.x - other.x,
                        y: self.y - other.y,
                        z: self.z - other.z,
                        w: self.w - other.w,
                    }
                }
            }

            /// Vector -= vector.
            impl SubAssign<Vec4<$t>> for Vec4<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.x -= other.x;
                    self.y -= other.y;
                    self.z -= other.z;
                    self.w -= other.w;
                }
            }

            /// Scalar * vector.
            impl Mul<Vec4<$t>> for $t {
                type Output = Vec4<$t>;
                fn mul(self,other: Vec4<$t>) -> Self::Output {
                    Vec4 {
                        x: self * other.x,
                        y: self * other.y,
                        z: self * other.z,
                        w: self * other.w,
                    }
                }
            }
            
            /// Vector * scalar.
            impl Mul<$t> for Vec4<$t> {
                type Output = Vec4<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Vec4 {
                        x: self.x * other,
                        y: self.y * other,
                        z: self.z * other,
                        w: self.w * other,
                    }
                }
            }

            /// Component-wise vector * vector.
            impl Mul<Vec4<$t>> for Vec4<$t> {
                type Output = Vec4<$t>;
                fn mul(self,other: Vec4<$t>) -> Self::Output {
                    Vec4 {
                        x: self.x * other.x,
                        y: self.y * other.y,
                        z: self.z * other.z,
                        w: self.w * other.w,
                    }
                }
            }

            /// Vector *= scalar.
            impl MulAssign<$t> for Vec4<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.x *= other;
                    self.y *= other;
                    self.z *= other;
                    self.w *= other;
                }
            }

            /// Component-wise vector *= vector.
            impl MulAssign<Vec4<$t>> for Vec4<$t> {
                fn mul_assign(&mut self,other: Vec4<$t>) {
                    self.x *= other.x;
                    self.y *= other.y;
                    self.z *= other.z;
                    self.w *= other.w;
                }
            }

            /// Vector / scalar.
            impl Div<$t> for Vec4<$t> {
                type Output = Vec4<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Vec4 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                        w: self.w / other,
                    }
                }
            }

            /// Vector /= scalar.
            impl DivAssign<$t> for Vec4<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                    self.w /= other;
                }
            }

            /// -Vector.
            impl Neg for Vec4<$t> {
                type Output = Vec4<$t>;
                fn neg(self) -> Self::Output {
                    Vec4 {
                        x: -self.x,
                        y: -self.y,
                        z: -self.z,
                        w: -self.w,
                    }
                }
            }
        )+
    }
}

vec4_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

// implementations where $t: Real
macro_rules! vec4_real_impl {
    ($($t:ty)+) => {
        $(
            impl Vec4<$t> {

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

vec4_real_impl! { f32 f64 }
