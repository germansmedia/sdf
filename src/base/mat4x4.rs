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

/// 4x4 matrix of numbers.
#[derive(Copy,Clone,Debug)]
pub struct Mat4x4<T: Sized + Zero + One> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

// implementations where $t: Sized + Zero + One
macro_rules! mat4x4_impl {
    ($($t:ty)+) => {
        $(
            impl Mat4x4<$t> {

                /// Compose matrix from 3x3 matrix and vector.
                pub fn from_mv(m: Mat3x3<$t>,v: Vec3<$t>) -> Mat4x4<$t> {
                    Mat4x4 {
                        x: Vec4 {
                            x: m.x.x,
                            y: m.x.y,
                            z: m.x.z,
                            w: <$t>::ZERO,
                        },
                        y: Vec4 {
                            x: m.y.x,
                            y: m.y.y,
                            z: m.y.z,
                            w: <$t>::ZERO,
                        },
                        z: Vec4 {
                            x: m.z.x,
                            y: m.z.y,
                            z: m.z.z,
                            w: <$t>::ZERO,
                        },
                        w: Vec4 {
                            x: v.x,
                            y: v.y,
                            z: v.z,
                            w: <$t>::ONE,
                        },
                    }
                }

                /// Transpose the matrix.
                pub fn transpose(self) -> Mat4x4<$t> {
                    Mat4x4 {
                        x: Vec4 {
                            x: self.x.x,
                            y: self.y.x,
                            z: self.z.x,
                            w: self.w.x,
                        },
                        y: Vec4 {
                            x: self.x.y,
                            y: self.y.y,
                            z: self.z.y,
                            w: self.w.y,
                        },
                        z: Vec4 {
                            x: self.x.z,
                            y: self.y.z,
                            z: self.z.z,
                            w: self.w.z,
                        },
                        w: Vec4 {
                            x: self.x.w,
                            y: self.y.w,
                            z: self.z.w,
                            w: self.w.w,
                        },
                    }
                }

                /// Calculate determinant of matrix.
                pub fn det(self) -> $t {
                    let a = self.x.x;
                    let e = self.x.y;
                    let i = self.x.z;
                    let m = self.x.w;
                    let b = self.y.x;
                    let f = self.y.y;
                    let j = self.y.z;
                    let n = self.y.w;
                    let c = self.z.x;
                    let g = self.z.y;
                    let k = self.z.z;
                    let o = self.z.w;
                    let d = self.w.x;
                    let h = self.w.y;
                    let l = self.w.z;
                    let p = self.w.w;
                    let kplo = k * p - l * o;
                    let jpln = j * p - l * n;
                    let jokn = j * o - k * n;
                    let iplm = i * p - l * m;
                    let iokm = i * o - k * m;
                    let injm = i * n - j * m;
                    let aa = f * kplo - g * jpln + h * jokn;
                    let ab = e * kplo - g * iplm + h * iokm;
                    let ac = e * jpln - f * iplm + h * injm;
                    let ad = e * jokn - f * iokm + g * injm;
                    a * aa - b * ab + c * ac - d * ad
                }
            }

            impl Zero for Mat4x4<$t> {
                const ZERO: Mat4x4<$t> = Mat4x4 {
                    x: Vec4::ZERO,
                    y: Vec4::ZERO,
                    z: Vec4::ZERO,
                    w: Vec4::ZERO,
                };
            }

            impl One for Mat4x4<$t> {
                const ONE: Mat4x4<$t> = Mat4x4 {
                    x: Vec4::<$t>::UNIT_X,
                    y: Vec4::<$t>::UNIT_Y,
                    z: Vec4::<$t>::UNIT_Z,
                    w: Vec4::<$t>::UNIT_W,
                };
            }

            impl PartialEq for Mat4x4<$t> {
                fn eq(&self,other: &Self) -> bool {
                    (self.x == other.x) && (self.y == other.y) && (self.z == other.z) && (self.w == other.w)
                }
            }

            impl Display for Mat4x4<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"[{},{},{},{}]",self.x,self.y,self.z,self.w)
                }
            }

            /// Matrix + matrix.
            impl Add<Mat4x4<$t>> for Mat4x4<$t> {
                type Output = Self;
                fn add(self,other: Self) -> Self::Output {
                    Mat4x4 {
                        x: self.x + other.x,
                        y: self.y + other.y,
                        z: self.z + other.z,
                        w: self.w + other.w,
                    }
                }
            }

            /// Matrix += matrix.
            impl AddAssign<Mat4x4<$t>> for Mat4x4<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.x += other.x;
                    self.y += other.y;
                    self.z += other.z;
                    self.w += other.w;
                }
            }

            /// Matrix - matrix.
            impl Sub<Mat4x4<$t>> for Mat4x4<$t> {
                type Output = Self;
                fn sub(self,other: Self) -> Self::Output {
                    Mat4x4 {
                        x: self.x - other.x,
                        y: self.y - other.y,
                        z: self.z - other.z,
                        w: self.w - other.w,
                    }
                }
            }

            /// Matrix -= matrix.
            impl SubAssign<Mat4x4<$t>> for Mat4x4<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.x -= other.x;
                    self.y -= other.y;
                    self.z -= other.z;
                    self.w -= other.w;
                }
            }

            /// Scalar * matrix.
            impl Mul<Mat4x4<$t>> for $t {
                type Output = Mat4x4<$t>;
                fn mul(self,other: Mat4x4<$t>) -> Self::Output {
                    Mat4x4 {
                        x: self * other.x,
                        y: self * other.y,
                        z: self * other.z,
                        w: self * other.w,
                    }
                }
            }

            /// Matrix * scalar.
            impl Mul<$t> for Mat4x4<$t> {
                type Output = Mat4x4<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Mat4x4 {
                        x: self.x * other,
                        y: self.y * other,
                        z: self.z * other,
                        w: self.w * other,
                    }
                }
            }

            /// Matrix * vector.
            impl Mul<Vec4<$t>> for Mat4x4<$t> {
                type Output = Vec4<$t>;
                fn mul(self,other: Vec4<$t>) -> Self::Output {
                    Vec4 {
                        x: self.x.x * other.x + self.y.x * other.y + self.z.x * other.z + self.w.x * other.w,
                        y: self.x.y * other.x + self.y.y * other.y + self.z.y * other.z + self.w.y * other.w,
                        z: self.x.z * other.x + self.y.z * other.y + self.z.z * other.z + self.w.z * other.w,
                        w: self.x.w * other.x + self.y.w * other.y + self.z.w * other.z + self.w.w * other.w,
                    }
                }
            }

            // Vector * matrix is not implemented.

            /// Matrix * matrix.
            impl Mul<Mat4x4<$t>> for Mat4x4<$t> {
                type Output = Mat4x4<$t>;
                fn mul(self,other: Mat4x4<$t>) -> Self::Output {
                    Mat4x4 {
                        x: self * other.x,
                        y: self * other.y,
                        z: self * other.z,
                        w: self * other.w,
                    }
                }
            }

            /// Matrix *= scalar.
            impl MulAssign<$t> for Mat4x4<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.x *= other;
                    self.y *= other;
                    self.z *= other;
                    self.w *= other;
                }
            }

            /// Matrix *= matrix.
            impl MulAssign<Mat4x4<$t>> for Mat4x4<$t> {
                fn mul_assign(&mut self,other: Mat4x4<$t>) {
                    let m = *self * other;
                    *self = m;
                }
            }
            
            /// Matrix / scalar.
            impl Div<$t> for Mat4x4<$t> {
                type Output = Mat4x4<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Mat4x4 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                        w: self.w / other,
                    }
                }
            }

            // Scalar / matrix is only defined for matrices of real numbers.

            /// Matrix /= scalar.
            impl DivAssign<$t> for Mat4x4<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                    self.w /= other;
                }
            }

            /// -Matrix.
            impl Neg for Mat4x4<$t> {
                type Output = Mat4x4<$t>;
                fn neg(self) -> Self::Output {
                    Mat4x4 {
                        x: -self.x,
                        y: -self.y,
                        z: -self.z,
                        w: -self.w,
                    }
                }
            }

            /// Convert vector to translation matrix.
            impl From<Vec3<$t>> for Mat4x4<$t> {
                fn from(value: Vec3<$t>) -> Self {
                    Mat4x4 {
                        x: Vec4::<$t>::UNIT_X,
                        y: Vec4::<$t>::UNIT_Y,
                        z: Vec4::<$t>::UNIT_Z,
                        w: Vec4 {
                            x: value.x,
                            y: value.y,
                            z: value.z,
                            w: <$t>::ONE,
                        },
                    }
                }
            }

            /// Convert 3x3 matrix to 4x4 matrix.
            impl From<Mat3x3<$t>> for Mat4x4<$t> {
                fn from(value: Mat3x3<$t>) -> Self {
                    Mat4x4 {
                        x: Vec4 {
                            x: value.x.x,
                            y: value.x.y,
                            z: value.x.z,
                            w: <$t>::ZERO,
                        },
                        y: Vec4 {
                            x: value.y.x,
                            y: value.y.y,
                            z: value.y.z,
                            w: <$t>::ZERO,
                        },
                        z: Vec4 {
                            x: value.z.x,
                            y: value.z.y,
                            z: value.z.z,
                            w: <$t>::ZERO,
                        },
                        w: Vec4::<$t>::UNIT_W,
                    }
                }
            }
        )+
    }
}

mat4x4_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

// implementations where $t: Real
macro_rules! mat4x4_real_impl {
    ($($t:ty)+) => {
        $(
            impl Mat4x4<$t> {
    
                /// Calculate inverse of matrix.
                pub fn inv(self) -> Self {
                    let a = self.x.x;
                    let e = self.x.y;
                    let i = self.x.z;
                    let m = self.x.w;
                    let b = self.y.x;
                    let f = self.y.y;
                    let j = self.y.z;
                    let n = self.y.w;
                    let c = self.z.x;
                    let g = self.z.y;
                    let k = self.z.z;
                    let o = self.z.w;
                    let d = self.w.x;
                    let h = self.w.y;
                    let l = self.w.z;
                    let p = self.w.w;
                    let kplo = k * p - l * o;
                    let jpln = j * p - l * n;
                    let jokn = j * o - k * n;
                    let iplm = i * p - l * m;
                    let iokm = i * o - k * m;
                    let injm = i * n - j * m;
                    let aa = f * kplo - g * jpln + k * iokm;
                    let ab = e * kplo - g * iplm + h * iokm;
                    let ac = e * jpln - f * iplm + h * injm;
                    let ad = e * jokn - f * iokm + g * injm;
                    let det = a * aa - b * ab + c * ac - d * ad;
                    if det == 0.0 {
                        return self;
                    }
                    let ae = e * kplo - c * jpln + d * jokn;
                    let af = a * kplo - c * iplm + d * iokm;
                    let ag = a * jpln - b * iplm + d * injm;
                    let ah = a * jokn - b * iokm + c * injm;
                    let chdg = c * h - d * g;
                    let bhdf = b * h - d * f;
                    let bgcf = b * g - c * f;
                    let ahde = a * h - d * e;
                    let agce = a * g - c * e;
                    let afbe = a * f - b * e;
                    let ai = n * chdg - o * bhdf + p * bgcf;
                    let aj = m * chdg - o * ahde + p * agce;
                    let ak = m * bhdf - n * ahde + p * afbe;
                    let al = m * bgcf - n * agce + o * afbe;
                    let am = j * chdg - k * bhdf + l * bgcf;
                    let an = i * chdg - k * ahde + l * agce;
                    let ao = i * bhdf - j * ahde + l * afbe;
                    let ap = i * bgcf - j * agce + k * afbe;
                    Mat4x4 {
                        x: Vec4 { x: aa,y: -ae,z: ai,w: -am, },
                        y: Vec4 { x: -ab,y: af,z: -aj,w: an, },
                        z: Vec4 { x: ac,y: -ag,z: ak,w: -ao, },
                        w: Vec4 { x: -ad,y: ah,z: -al,w: ap, },
                    } / det
                }

                /// Create perspective projection matrix.
                pub fn perspective(fovy: $t,aspect: $t,n: $t,f: $t) -> Mat4x4<$t> {
                    let t = (0.5 * fovy).tan();
                    Mat4x4 {
                        x: Vec4 {
                            x: aspect / t,
                            y: 0.0,
                            z: 0.0,
                            w: 0.0,
                        },
                        y: Vec4 {
                            x: 0.0,
                            y: 1.0 / t,
                            z: 0.0,
                            w: 0.0,
                        },
                        z: Vec4 {
                            x: 0.0,
                            y: 0.0,
                            z: -(f + n) / (n - f),
                            w: -1.0,
                        },
                        w: Vec4 {
                            x: 0.0,
                            y: 0.0,
                            z: -2.0 * f * n / (n - f),
                            w: 0.0,
                        },
                    }
                }
    
                /// Create frustum projection matrix.
                pub fn frustum(l: $t,r: $t,t: $t,b: $t,n: $t,f: $t) -> Mat4x4<$t> {
                    Mat4x4 {
                        x: Vec4 {
                            x: (n + n) / (r - l),
                            y: 0.0,
                            z: 0.0,
                            w: 0.0,
                        },
                        y: Vec4 {
                            x: 0.0,
                            y: (n + n) / (t - b),
                            z: 0.0,
                            w: 0.0,
                        },
                        z: Vec4 {
                            x: (r + l) / (r - l),
                            y: (t + b) / (t - b),
                            z: -(f + n) / (f - n),
                            w: -1.0,
                        },
                        w: Vec4 {
                            x: 0.0,
                            y: 0.0,
                            z: -2.0 * f * n / (f - n),
                            w: 0.0,
                        }
                    }
                }
            }

            // Scalar / matrix.
            impl Div<Mat4x4<$t>> for $t {
                type Output = Mat4x4<$t>;
                fn div(self,other: Mat4x4<$t>) -> Self::Output {
                    self * other.inv()
                }
            }

            // Matrix / matrix.
            impl Div<Mat4x4<$t>> for Mat4x4<$t> {
                type Output = Mat4x4<$t>;
                fn div(self,other: Mat4x4<$t>) -> Self::Output {
                    self * other.inv()
                }
            }

            // Matrix /= matrix.
            impl DivAssign<Mat4x4<$t>> for Mat4x4<$t> {
                fn div_assign(&mut self,other: Mat4x4<$t>) {
                    *self *= other.inv()
                }
            }

            /// Convert quaternion to rotation matrix.
            impl From<Quaternion<$t>> for Mat4x4<$t> {
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
                    Mat4x4 {
                        x: Vec4 {
                            x: <$t>::ONE - yy2 - zz2,
                            y: xy2 + wz2,
                            z: xz2 - wy2,
                            w: <$t>::ZERO,
                        },
                        y: Vec4 {
                            x: xy2 - wz2,
                            y: <$t>::ONE - xx2 - zz2,
                            z: yz2 + wx2,
                            w: <$t>::ZERO,
                        },
                        z: Vec4 {
                            x: xz2 + wy2,
                            y: yz2 - wx2,
                            z: <$t>::ONE - xx2 - yy2,
                            w: <$t>::ZERO,
                        },
                        w: Vec4::<$t>::UNIT_W,
                    }
                }
            }
        )+
    }
}

mat4x4_real_impl! { f32 f64 }
