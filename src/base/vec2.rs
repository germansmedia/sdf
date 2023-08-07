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

/// 2D vector of numbers.
#[derive(Copy,Clone,Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

// implementations where $t: Sized + Zero + One
macro_rules! vec2_impl {
    ($($t:ty)+) => {
        $(
            impl Vec2<$t> {

                /// Unit vector in positive X-direction.
                pub const UNIT_X: Self = Vec2 { x: <$t>::ONE,y: <$t>::ZERO, };

                /// Unit vector in positive Y-direction.
                pub const UNIT_Y: Self = Vec2 { x: <$t>::ZERO,y: <$t>::ONE, };

                /// Calculate dot product.
                pub fn dot(self,other: &Vec2<$t>) -> $t {
                    self.x * other.x + self.y * other.y
                }
            }

            impl Display for Vec2<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{})",self.x,self.y)
                }
            }

            impl PartialEq for Vec2<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.x == other.x) && (self.y == other.y)
                }
            }

            impl Zero for Vec2<$t> {
                const ZERO: Vec2<$t> = Vec2 { x: <$t>::ZERO,y: <$t>::ZERO, };
            }

            /// Vector + vector.
            impl Add<Vec2<$t>> for Vec2<$t> {
                type Output = Vec2<$t>;
                fn add(self,other: Self) -> Self {
                    Vec2 {
                        x: self.x + other.x,
                        y: self.y + other.y,
                    }
                }
            }

            /// Vector += vector.
            impl AddAssign<Vec2<$t>> for Vec2<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.x += other.x;
                    self.y += other.y;
                }
            }

            /// Vector - vector.
            impl Sub<Vec2<$t>> for Vec2<$t> {
                type Output = Vec2<$t>;
                fn sub(self,other: Self) -> Self {
                    Vec2 {
                        x: self.x - other.x,
                        y: self.y - other.y,
                    }
                }
            }

            /// Vector -= vector.
            impl SubAssign<Vec2<$t>> for Vec2<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.x -= other.x;
                    self.y -= other.y;
                }
            }

            /// Scalar * vector.
            impl Mul<Vec2<$t>> for $t {
                type Output = Vec2<$t>;
                fn mul(self,other: Vec2<$t>) -> Self::Output {
                    Vec2 {
                        x: self * other.x,
                        y: self * other.y,
                    }
                }
            }
            
            /// Vector * scalar.
            impl Mul<$t> for Vec2<$t> {
                type Output = Vec2<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Vec2 {
                        x: self.x * other,
                        y: self.y * other,
                    }
                }
            }

            /// Component-wise vector * vector.
            impl Mul<Vec2<$t>> for Vec2<$t> {
                type Output = Vec2<$t>;
                fn mul(self,other: Vec2<$t>) -> Self::Output {
                    Vec2 {
                        x: self.x * other.x,
                        y: self.y * other.y,
                    }
                }
            } 

            /// Vector *= scalar.
            impl MulAssign<$t> for Vec2<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.x *= other;
                    self.y *= other;
                }
            }

            /// Component-wise vector *= vector.
            impl MulAssign<Vec2<$t>> for Vec2<$t> {
                fn mul_assign(&mut self,other: Vec2<$t>) {
                    self.x *= other.x;
                    self.y *= other.y;
                }
            }

            /// Vector / scalar.
            impl Div<$t> for Vec2<$t> {
                type Output = Vec2<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Vec2 {
                        x: self.x / other,
                        y: self.y / other,
                    }
                }
            }

            /// Vector /= scalar.
            impl DivAssign<$t> for Vec2<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.x /= other;
                    self.y /= other;
                }
            }

            /// -Vector.
            impl Neg for Vec2<$t> {
                type Output = Vec2<$t>;
                fn neg(self) -> Self::Output {
                    Vec2 {
                        x: -self.x,
                        y: -self.y,
                    }
                }
            }
        )+
    }
}

vec2_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

// implementations where $t: Real
macro_rules! vec2_real_impl {
    ($($t:ty)+) => {
        $(
            impl Vec2<$t> {

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

vec2_real_impl! { f32 f64 }
