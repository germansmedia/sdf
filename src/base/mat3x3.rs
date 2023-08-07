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

#[derive(Copy,Clone,Debug)]
pub struct Mat3x3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

macro_rules! mat3x3_impl {
    ($($t:ty)+) => {
        $(
            impl Mat3x3<$t> {
                pub fn from_mv(m: Mat2x2<$t>,v: Vec2<$t>) -> Mat3x3<$t> {
                    Mat3x3 {
                        x: Vec3 {
                            x: m.x.x,
                            y: m.x.y,
                            z: <$t>::ZERO,
                        },
                        y: Vec3 {
                            x: m.y.x,
                            y: m.y.y,
                            z: <$t>::ZERO,
                        },
                        z: Vec3 {
                            x: v.x,
                            y: v.y,
                            z: <$t>::ONE,
                        },
                    }
                }

                pub fn transpose(self) -> Mat3x3<$t> {
                    Mat3x3 {
                        x: Vec3 {
                            x: self.x.x,
                            y: self.y.x,
                            z: self.z.x,
                        },
                        y: Vec3 {
                            x: self.x.y,
                            y: self.y.y,
                            z: self.z.y,
                        },
                        z: Vec3 {
                            x: self.x.z,
                            y: self.y.z,
                            z: self.z.z,
                        },
                    }
                }

                pub fn det(self) -> $t {
                    let a = self.x.x;
                    let d = self.x.y;
                    let g = self.x.z;
                    let b = self.y.x;
                    let e = self.y.y;
                    let h = self.y.z;
                    let c = self.z.x;
                    let f = self.z.y;
                    let i = self.z.z;
                    let aa = e * i - f * h;
                    let ab = d * i - f * g;
                    let ac = d * h - e * g;
                    a * aa - b * ab + c * ac
                }
            }

            impl Zero for Mat3x3<$t> {
                const ZERO: Mat3x3<$t> = Mat3x3 {
                    x: Vec3::ZERO,
                    y: Vec3::ZERO,
                    z: Vec3::ZERO,
                };
            }

            impl One for Mat3x3<$t> {
                const ONE: Mat3x3<$t> = Mat3x3 {
                    x: Vec3 {
                        x: <$t>::ONE,
                        y: <$t>::ZERO,
                        z: <$t>::ZERO,
                    },
                    y: Vec3 {
                        x: <$t>::ZERO,
                        y: <$t>::ONE,
                        z: <$t>::ZERO,
                    },
                    z: Vec3 {
                        x: <$t>::ZERO,
                        y: <$t>::ZERO,
                        z: <$t>::ONE,
                    },
                };
            }

            impl PartialEq for Mat3x3<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
                }
            }

            impl Display for Mat3x3<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"[{},{},{}]",self.x,self.y,self.z)
                }
            }

            // matrix + matrix
            impl Add<Mat3x3<$t>> for Mat3x3<$t> {
                type Output = Self;
                fn add(self,other: Self) -> Self::Output {
                    Mat3x3 {
                        x: self.x + other.x,
                        y: self.y + other.y,
                        z: self.z + other.z,
                    }
                }
            }

            // matrix += matrix
            impl AddAssign<Mat3x3<$t>> for Mat3x3<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.x += other.x;
                    self.y += other.y;
                    self.z += other.z;
                }
            }

            // matrix - matrix
            impl Sub<Mat3x3<$t>> for Mat3x3<$t> {
                type Output = Self;
                fn sub(self,other: Self) -> Self::Output {
                    Mat3x3 {
                        x: self.x - other.x,
                        y: self.y - other.y,
                        z: self.z - other.z,
                    }
                }
            }

            // matrix -= matrix
            impl SubAssign<Mat3x3<$t>> for Mat3x3<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.x -= other.x;
                    self.y -= other.y;
                    self.z -= other.z;
                }
            }

            // scalar * matrix
            impl Mul<Mat3x3<$t>> for $t {
                type Output = Mat3x3<$t>;
                fn mul(self,other: Mat3x3<$t>) -> Self::Output {
                    Mat3x3 {
                        x: self * other.x,
                        y: self * other.y,
                        z: self * other.z,
                    }
                }
            }

            // matrix * scalar
            impl Mul<$t> for Mat3x3<$t> {
                type Output = Mat3x3<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Mat3x3 {
                        x: self.x * other,
                        y: self.y * other,
                        z: self.z * other,
                    }
                }
            }

            // matrix * vector
            impl Mul<Vec3<$t>> for Mat3x3<$t> {
                type Output = Vec3<$t>;
                fn mul(self,other: Vec3<$t>) -> Self::Output {
                    Vec3 {
                        x: self.x.x * other.x + self.y.x * other.y + self.z.x * other.z,
                        y: self.x.y * other.x + self.y.y * other.y + self.z.y * other.z,
                        z: self.x.z * other.x + self.y.z * other.y + self.z.z * other.z,
                    }
                }
            }

            // matrix * matrix
            impl Mul<Mat3x3<$t>> for Mat3x3<$t> {
                type Output = Mat3x3<$t>;
                fn mul(self,other: Mat3x3<$t>) -> Self::Output {
                    Mat3x3 {
                        x: self * other.x,
                        y: self * other.y,
                        z: self * other.z,
                    }
                }
            }

            // matrix *= scalar
            impl MulAssign<$t> for Mat3x3<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.x *= other;
                    self.y *= other;
                    self.z *= other;
                }
            }

            // matrix *= matrix
            impl MulAssign<Mat3x3<$t>> for Mat3x3<$t> {
                fn mul_assign(&mut self,other: Mat3x3<$t>) {
                    let m = *self * other;
                    *self = m;
                }
            }

            // matrix / scalar
            impl Div<$t> for Mat3x3<$t> {
                type Output = Mat3x3<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Mat3x3 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                    }
                }
            }

            // matrix /= scalar
            impl DivAssign<$t> for Mat3x3<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                }
            }

            // -matrix
            impl Neg for Mat3x3<$t> {
                type Output = Mat3x3<$t>;
                fn neg(self) -> Self {
                    Mat3x3 {
                        x: -self.x,
                        y: -self.y,
                        z: -self.z,
                    }
                }
            }
        )+
    }
}

mat3x3_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! mat3x3_real_impl {
    ($($t:ty)+) => {
        $(
            impl Mat3x3<$t> {

                pub fn from_quaternion(q: Quaternion<$t>) -> Mat3x3<$t> {
                    Mat3x3 {
                        x: Vec3 {
                            x: 1.0 - 2.0 * q.j * q.j - 2.0 * q.k * q.k,
                            y: 2.0 * q.i * q.j + 2.0 * q.k * q.r,
                            z: 2.0 * q.i * q.k - 2.0 * q.j * q.r,
                        },
                        y: Vec3 {
                            x: 2.0 * q.i * q.j - 2.0 * q.k * q.r,
                            y: 1.0 - 2.0 * q.i * q.i - 2.0 * q.k * q.k,
                            z: 2.0 * q.j * q.k + 2.0 * q.i * q.r,
                        },
                        z: Vec3 {
                            x: 2.0 * q.i * q.k + 2.0 * q.j * q.r,
                            y: 2.0 * q.j * q.k - 2.0 * q.i * q.r,
                            z: 1.0 - 2.0 * q.i * q.i - 2.0 * q.j * q.j,
                        },
                    }
                }

                pub fn inv(self) -> Self {
                    let a = self.x.x;
                    let d = self.x.y;
                    let g = self.x.z;
                    let b = self.y.x;
                    let e = self.y.y;
                    let h = self.y.z;
                    let c = self.z.x;
                    let f = self.z.y;
                    let i = self.z.z;
                    let aa = e * i - f * h;
                    let ab = d * i - f * g;
                    let ac = d * h - e * g;
                    let det = a * aa - b * ab + c * ac;
                    if det == 0.0 {
                        return self;
                    }
                    let ad = b * i - c * h;
                    let ae = a * i - c * g;
                    let af = a * h - b * g;
                    let ag = b * f - c * e;
                    let ah = a * f - c * d;
                    let ai = a * e - b * d; 
                    Mat3x3 {
                        x: Vec3 { x: aa,y: -ad,z: ag, },
                        y: Vec3 { x: -ab,y: ae,z: -ah, },
                        z: Vec3 { x: ac,y: -af,z: ai, },
                    } / det
                }
            }

            // scalar / matrix
            impl Div<Mat3x3<$t>> for $t {
                type Output = Mat3x3<$t>;
                fn div(self,other: Mat3x3<$t>) -> Self::Output {
                    self * other.inv()
                }
            }

            // matrix / matrix
            impl Div<Mat3x3<$t>> for Mat3x3<$t> {
                type Output = Mat3x3<$t>;
                fn div(self,other: Mat3x3<$t>) -> Self::Output {
                    self * other.inv()
                }
            }

            // matrix /= matrix
            impl DivAssign<Mat3x3<$t>> for Mat3x3<$t> {
                fn div_assign(&mut self,other: Mat3x3<$t>) {
                    *self *= other.inv()
                }
            }

            impl From<Quaternion<$t>> for Mat3x3<$t> {
                fn from(value: Quaternion<$t>) -> Self {
                    let x2 = value.i + value.i;
                    let y2 = value.j + value.j;
                    let z2 = value.k + value.k;
                    let xx2 = value.i * x2;
                    let yy2 = value.j * y2;
                    let zz2 = value.k * z2;
                    let yz2 = value.j * z2;
                    let wx2 = value.r * x2;
                    let xy2 = value.i * y2;
                    let wz2 = value.r * z2;
                    let xz2 = value.i * z2;
                    let wy2 = value.r * y2;
                    Mat3x3 {
                        x: Vec3 {
                            x: <$t>::ONE - yy2 - zz2,
                            y: xy2 + wz2,
                            z: xz2 - wy2,
                        },
                        y: Vec3 {
                            x: xy2 - wz2,
                            y: <$t>::ONE - xx2 - zz2,
                            z: yz2 + wx2,
                        },
                        z: Vec3 {
                            x: xz2 + wy2,
                            y: yz2 - wx2,
                            z: <$t>::ONE - xx2 - yy2,
                        },
                    }
                }
            }
        )+
    }
}

mat3x3_real_impl! { f32 f64 }
