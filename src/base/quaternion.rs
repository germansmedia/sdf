use {
    crate::*,
    std::{
        cmp::PartialEq,
        fmt::{
            Display,
            Debug,
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

/// Quaternion of real numbers.
/// 
/// A quaternion is a way to represent 3D orientation and allow for correct rotations without gimbal lock. The concept is
/// similar to [`Complex`], where imaginary numbers are combined with scalars. The [`Quaternion`] adds three separate
/// imaginary numbers, allowing rotations around 3 orthogonal axes.
#[derive(Copy,Clone,Debug)]
pub struct Quaternion<T: Real> {
    pub r: T,
    pub i: T,
    pub j: T,
    pub k: T,
}

macro_rules! quaternion_impl {
    ($($t:ty)+) => {
        $(
            impl Quaternion<$t> {

                /// Convert Euler angles to quaternion.
                pub fn from_euler(r: $t,p: $t,y: $t) -> Self {
                    let hr = 0.5 * r;
                    let hp = 0.5 * p;
                    let hy = 0.5 * y;
                    let cr = hr.cos();
                    let sr = hr.sin();
                    let cp = hp.cos();
                    let sp = hp.sin();
                    let cy = hy.cos();
                    let sy = hy.sin();
                    Quaternion {
                        r: cr * cp * cy + sr * sp * sy,
                        i: sr * cp * cy - cr * sp * sy,
                        j: cr * sp * cy + sr * cp * sy,
                        k: cr * cp * sy - sr * sp * cy,
                    }
                }

                /// Calculate quaternion conjugate.
                pub fn conj(&self) -> Self {
                    Quaternion {
                        r: self.r,
                        i: -self.i,
                        j: -self.j,
                        k: -self.k,
                    }
                }

                /// Invert the quaternion.
                pub fn inv(&self) -> Self {
                    let f = self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k;
                    Quaternion {
                        r: self.r / f,
                        i: -self.i / f,
                        j: -self.j / f,
                        k: -self.k / f,
                    }
                }
            }

            impl Zero for Quaternion<$t> {
                const ZERO: Self = Quaternion { r: <$t>::ZERO,i: <$t>::ZERO,j: <$t>::ZERO,k: <$t>::ZERO, };
            }

            impl One for Quaternion<$t> {
                const ONE: Self = Quaternion { r: <$t>::ONE,i: <$t>::ZERO,j: <$t>::ZERO,k: <$t>::ZERO, };
            }

            impl Display for Quaternion<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    let si = if self.i < <$t>::ZERO {
                        format!("{}i",self.i)
                    } else {
                        format!("+{}i",self.i)
                    };
                    let sj = if self.j < <$t>::ZERO {
                        format!("{}j",self.j)
                    } else {
                        format!("+{}j",self.j)
                    };
                    let sk = if self.k < <$t>::ZERO {
                        format!("{}k",self.k)
                    } else {
                        format!("+{}k",self.k)
                    };
                    write!(f,"{}{}{}{}",self.r,si,sj,sk)        
                }            
            }

            impl PartialEq<Quaternion<$t>> for Quaternion<$t> {
                fn eq(&self,other: &Quaternion<$t>) -> bool {
                    (self.r == other.r) && (self.i == other.i) && (self.j == other.j) && (self.k == other.k)
                }
            }

            /// Scalar + quaternion.
            impl Add<Quaternion<$t>> for $t {
                type Output = Quaternion<$t>;
                fn add(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self + other.r,
                        i: other.i,
                        j: other.j,
                        k: other.k,
                    }
                }
            }

            /// Quaternion + scalar.
            impl Add<$t> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn add(self,other: $t) -> Self::Output {
                    Quaternion {
                        r: self.r + other,
                        i: self.i,
                        j: self.j,
                        k: self.k,
                    }
                }
            }

            /// Quaternion += scalar.
            impl AddAssign<$t> for Quaternion<$t> {
                fn add_assign(&mut self,other: $t) {
                    self.r += other;
                }
            }

            /// Scalar - quaternion.
            impl Sub<Quaternion<$t>> for $t {
                type Output = Quaternion<$t>;
                fn sub(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self - other.r,
                        i: -other.i,
                        j: -other.j,
                        k: -other.k,
                    }
                }
            }

            /// Quaternion - scalar.
            impl Sub<$t> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn sub(self,other: $t) -> Self::Output {
                    Quaternion {
                        r: self.r - other,
                        i: self.i,
                        j: self.j,
                        k: self.k,
                    }
                }
            }

            /// Quaternion -= scalar.
            impl SubAssign<$t> for Quaternion<$t> {
                fn sub_assign(&mut self,other: $t) {
                    self.r -= other;
                }
            }

            /// Scalar * quaternion.
            impl Mul<Quaternion<$t>> for $t {
                type Output = Quaternion<$t>;
                fn mul(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self * other.r,
                        i: self * other.i,
                        j: self * other.j,
                        k: self * other.k,
                    }
                }
            }

            /// Quaternion * scalar.
            impl Mul<$t> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn mul(self,other: $t) -> Self::Output {
                    Quaternion {
                        r: self.r * other,
                        i: self.i * other,
                        j: self.j * other,
                        k: self.k * other,
                    }
                }
            }

            /// Quaternion * quaternion.
            impl Mul<Quaternion<$t>> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn mul(self,other: Quaternion<$t>) -> Self::Output {
                    Quaternion {
                        r: self.r * other.r - self.i * other.i - self.j * other.j - self.k * other.k,
                        i: self.r * other.i + self.i * other.r + self.j * other.k - self.k * other.j,
                        j: self.r * other.j - self.i * other.k + self.j * other.r + self.k * other.i,
                        k: self.r * other.k + self.i * other.j - self.j * other.i + self.k * other.r,
                    }
                }
            }

            /// Quaternion * vector.
            impl Mul<Vec3<$t>> for Quaternion<$t> {
                type Output = Vec3<$t>;
                fn mul(self,other: Vec3<$t>) -> Self::Output {
                    let rr = self.r * self.r;
                    let ri = self.r * self.i;
                    let rj = self.r * self.j;
                    let rk = self.r * self.k;
                    let ii = self.i * self.i;
                    let ij = self.i * self.j;
                    let ik = self.i * self.k;
                    let jj = self.j * self.j;
                    let jk = self.j * self.k;
                    let kk = self.k * self.k;
                    let ijprk = ij + rk;
                    let ijprk2 = ijprk + ijprk;
                    let ijmrk = ij - rk;
                    let ijmrk2 = ijmrk + ijmrk;
                    let jkpri = jk + ri;
                    let jkpri2 = jkpri + jkpri;
                    let jkmri = jk - ri;
                    let jkmri2 = jkmri + jkmri;
                    let ikprj = ik + rj;
                    let ikprj2 = ikprj + ikprj;
                    let ikmrj = ik - rj;
                    let ikmrj2 = ikmrj + ikmrj;
                    Vec3 {
                        x: (rr + ii - jj - kk) * other.x + ijmrk2 * other.y + ikprj2 * other.z,
                        y: (rr - ii + jj - kk) * other.y + jkmri2 * other.z + ijprk2 * other.x,
                        z: (rr - ii - jj + kk) * other.z + ikmrj2 * other.x + jkpri2 * other.y,
                    }            
                }
            }

            /// Quaternion *= scalar.
            impl MulAssign<$t> for Quaternion<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.r *= other;
                    self.i *= other;
                    self.j *= other;
                    self.k *= other;
                }
            }

            /// Quaternion *= quaternion.
            impl MulAssign<Quaternion<$t>> for Quaternion<$t> {
                fn mul_assign(&mut self,other: Quaternion<$t>) {
                    let r = self.r * other.r - self.i * other.i - self.j * other.j - self.k * other.k;
                    let i = self.r * other.i + self.i * other.r + self.j * other.k - self.k * other.j;
                    let j = self.r * other.j - self.i * other.k + self.j * other.r + self.k * other.i;
                    let k = self.r * other.k + self.i * other.j - self.j * other.i + self.k * other.r;
                    self.r = r;
                    self.i = i;
                    self.j = j;
                    self.k = k;            
                }
            }

            /// Scalar / quaternion.
            impl Div<Quaternion<$t>> for $t {
                type Output = Quaternion<$t>;
                fn div(self,other: Quaternion<$t>) -> Self::Output {
                    let f = other.r * other.r + other.i * other.i + other.j * other.j + other.k * other.k;
                    Quaternion {
                        r: (self * other.r) / f,
                        i: (-self * other.i) / f,
                        j: (-self * other.j) / f,
                        k: (-self * other.k) / f,
                    }
                }
            }

            /// Quaternion / scalar.
            impl Div<$t> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn div(self,other: $t) -> Self::Output {
                    Quaternion {
                        r: self.r / other,
                        i: self.i / other,
                        j: self.j / other,
                        k: self.k / other,
                    }
                }
            }

            /// Quaternion / quaternion.
            impl Div<Quaternion<$t>> for Quaternion<$t> {
                type Output = Quaternion<$t>;
                fn div(self,other: Quaternion<$t>) -> Self::Output {
                    let f = other.r * other.r + other.i * other.i + other.j * other.j + other.k * other.k;
                    Quaternion {
                        r: (self.r * other.r + self.i * other.i + self.j * other.j + self.k * other.k) / f,
                        i: (self.i * other.r - self.j * other.k + self.k * other.j - self.r * other.i) / f,
                        j: (self.j * other.r - self.k * other.i - self.r * other.j + self.i * other.k) / f,
                        k: (self.k * other.r - self.r * other.k - self.i * other.j + self.j * other.i) / f,
                    }
                }
            }

            /// Quaternion /= scalar.
            impl DivAssign<$t> for Quaternion<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.r *= other;
                    self.i *= other;
                    self.j *= other;
                    self.k *= other;
                }
            }

            /// Quaternion /= quaternion.
            impl DivAssign<Quaternion<$t>> for Quaternion<$t> {
                fn div_assign(&mut self,other: Quaternion<$t>) {
                    let f = other.r * other.r + other.i * other.i + other.j * other.j + other.k * other.k;
                    let r = (self.r * other.r + self.i * other.i + self.j * other.j + self.k * other.k) / f;
                    let i = (self.i * other.r - self.j * other.k + self.k * other.j - self.r * other.i) / f;
                    let j = (self.j * other.r - self.k * other.i - self.r * other.j + self.i * other.k) / f;
                    let k = (self.k * other.r - self.r * other.k - self.i * other.j + self.j * other.i) / f;
                    self.r = r;
                    self.i = i;
                    self.j = j;
                    self.k = k;            
                }
            }

            /// -Quaternion.
            impl Neg for Quaternion<$t> {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    Quaternion {
                        r: -self.r,
                        i: -self.i,
                        j: -self.j,
                        k: -self.k,
                    }
                }
            }
        )+
    }
}

quaternion_impl! { f32 f64 }
