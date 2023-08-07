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

/// 2x2 matrix of numbers.
#[derive(Copy,Clone,Debug)]
pub struct Mat2x2<T: Sized + Unsigned + Signed> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

// implementations where $t: Sized + Zero + One
macro_rules! mat2x2_impl {
    ($($t:ty)+) => {
        $(
            impl Mat2x2<$t> {

                /// Transpose the matrix.
                pub fn transpose(self) -> Mat2x2<$t> {
                    Mat2x2 {
                        x: Vec2 {
                            x: self.x.x,
                            y: self.y.x,
                        },
                        y: Vec2 {
                            x: self.x.y,
                            y: self.y.y,
                        },
                    }
                }

                /// Calculate determinant of matrix.
                pub fn det(self) -> $t {
                    let a = self.x.x;
                    let b = self.y.x;
                    let c = self.x.y;
                    let d = self.y.y;
                    let aa = d;
                    let ab = c;
                    a * aa - b * ab
                }
            }

            impl Zero for Mat2x2<$t> {
                const ZERO: Mat2x2<$t> = Mat2x2 {
                    x: Vec2::ZERO,
                    y: Vec2::ZERO,
                };
            }

            impl One for Mat2x2<$t> {
                const ONE: Mat2x2<$t> = Mat2x2 {
                    x: Vec2::<$t>::UNIT_X,
                    y: Vec2::<$t>::UNIT_Y,
                };
            }

            impl PartialEq for Mat2x2<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.x == other.x) && (self.y == other.y)
                }
            }

            impl Display for Mat2x2<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"[{},{}]",self.x,self.y)
                }
            }

            /// Matrix + matrix.
            impl Add<Mat2x2<$t>> for Mat2x2<$t> {
                type Output = Self;
                fn add(self,other: Self) -> Self::Output {
                    Mat2x2 {
                        x: self.x + other.x,
                        y: self.y + other.y,
                    }
                }
            }

            /// Matrix += matrix.
            impl AddAssign<Mat2x2<$t>> for Mat2x2<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.x += other.x;
                    self.y += other.y;
                }
            }

            /// Matrix - matrix.
            impl Sub<Mat2x2<$t>> for Mat2x2<$t> {
                type Output = Self;
                fn sub(self,other: Self) -> Self::Output {
                    Mat2x2 {
                        x: self.x - other.x,
                        y: self.y - other.y,
                    }
                }
            }

            /// Matrix -= matrix.
            impl SubAssign<Mat2x2<$t>> for Mat2x2<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.x -= other.x;
                    self.y -= other.y;
                }
            }

            /// Scalar * matrix.
            impl Mul<Mat2x2<$t>> for $t {
                type Output = Mat2x2<$t>;
                fn mul(self,other: Mat2x2<$t>) -> Self::Output {
                    Mat2x2 {
                        x: self * other.x,
                        y: self * other.y,
                    }
                }
            }

            /// Matrix * scalar.
            impl Mul<$t> for Mat2x2<$t> {
                type Output = Mat2x2<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Mat2x2 {
                        x: self.x * other,
                        y: self.y * other,
                    }
                }
            }

            /// Matrix * vector.
            impl Mul<Vec2<$t>> for Mat2x2<$t> {
                type Output = Vec2<$t>;
                fn mul(self,other: Vec2<$t>) -> Self::Output {
                    Vec2 {
                        x: self.x.x * other.x + self.y.x * other.y,
                        y: self.x.y * other.x + self.y.y * other.y,
                    }
                }
            }

            // Vector * matrix is not defined.

            /// Matrix * matrix.
            impl Mul<Mat2x2<$t>> for Mat2x2<$t> {
                type Output = Mat2x2<$t>;
                fn mul(self,other: Mat2x2<$t>) -> Self::Output {
                    Mat2x2 {
                        x: self * other.x,
                        y: self * other.y,
                    }
                }
            }

            /// Matrix *= scalar.
            impl MulAssign<$t> for Mat2x2<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.x *= other;
                    self.y *= other;
                }
            }

            /// Matrix *= matrix.
            impl MulAssign<Mat2x2<$t>> for Mat2x2<$t> {
                fn mul_assign(&mut self,other: Mat2x2<$t>) {
                    let m = *self * other;
                    *self = m;
                }
            }

            /// Matrix / scalar.
            impl Div<$t> for Mat2x2<$t> {
                type Output = Mat2x2<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Mat2x2 {
                        x: self.x / other,
                        y: self.y / other,
                    }
                }
            }

            /// Matrix /= scalar.
            impl DivAssign<$t> for Mat2x2<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.x /= other;
                    self.y /= other;
                }
            }

            // scalar / matrix is only defined for matrices of real numbers

            /// -Matrix
            impl Neg for Mat2x2<$t> {
                type Output = Mat2x2<$t>;
                fn neg(self) -> Self::Output {
                    Mat2x2 {
                        x: -self.x,
                        y: -self.y,
                    }
                }
            }
        )+
    }
}

mat2x2_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

// implementations where $t: Real
macro_rules! mat2x2_real_impl {
    ($($t:ty)+) => {
        $(
            impl Mat2x2<$t> {

                /// Calculate inverse of matrix.
                pub fn inv(self) -> Self {
                    let a = self.x.x;
                    let b = self.y.x;
                    let c = self.x.y;
                    let d = self.y.y;
                    let aa = d;
                    let ab = c;
                    let det = a * aa - b * ab;
                    if det == 0.0 {
                        return self;
                    }
                    let ac = b;
                    let ad = a;
                    Mat2x2 {
                        x: Vec2 { x: aa,y: -ac, },
                        y: Vec2 { x: -ab,y: ad, },
                    } / det
                }
            }

            /// Scalar / matrix.
            impl Div<Mat2x2<$t>> for $t {
                type Output = Mat2x2<$t>;
                fn div(self,other: Mat2x2<$t>) -> Self::Output {
                    other.inv() * self
                }
            }

            /// Matrix / matrix.
            impl Div<Mat2x2<$t>> for Mat2x2<$t> {
                type Output = Mat2x2<$t>;
                fn div(self,other: Mat2x2<$t>) -> Self::Output {
                    self * other.inv()
                }
            }

            /// Matrix /= matrix.
            impl DivAssign<Mat2x2<$t>> for Mat2x2<$t> {
                fn div_assign(&mut self,other: Mat2x2<$t>) {
                    *self *= other.inv()
                }
            }
        )+
    }
}

mat2x2_real_impl! { f32 f64 }
