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

/// Complex number.
///
/// A complex number is a linear combination of a real number `r` and an imaginary number `i`.
#[derive(Copy,Clone,Debug)]
pub struct Complex<T: Real> {
    pub r: T,
    pub i: T,
}

macro_rules! complex_impl {
    ($($t:ty)+) => {
        $(
            impl Complex<$t> {

                /// Complex conjugate.
                pub fn conj(&self) -> Self {
                    Complex {
                        r: self.r,
                        i: -self.i,
                    }
                }

                /// Complex argument.
                pub fn arg(&self) -> $t {
                    self.r.atan2(self.i)
                }
            }

            impl Zero for Complex<$t> {
                const ZERO: Self = Complex { r: <$t>::ZERO,i: <$t>::ZERO, };
            }
            
            impl One for Complex<$t> {
                const ONE: Self = Complex { r: <$t>::ONE,i: <$t>::ZERO, };
            }

            impl Display for Complex<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    let si = if self.i < <$t>::ZERO {
                        format!("{}i",self.i)
                    }
                    else {
                        format!("+{}i",self.i)
                    };
                    write!(f,"{}{}",self.r,si)
                }
            }
            
            impl PartialEq<Complex<$t>> for Complex<$t> {
                fn eq(&self,other: &Complex<$t>) -> bool {
                    (self.r == other.r) && (self.i == other.i)
                }
            }

            /// Real + complex.
            impl Add<Complex<$t>> for $t {
                type Output = Complex<$t>;
                fn add(self,other: Complex<$t>) -> Self::Output {
                    Complex {
                        r: self + other.r,
                        i: other.i,
                    }
                }
            }

            /// Complex + real.
            impl Add<$t> for Complex<$t> {
                type Output = Self;
                fn add(self,other: $t) -> Self::Output {
                    Complex {
                        r: self.r + other,
                        i: self.i,
                    }
                }
            }

            /// Complex + complex.
            impl Add<Complex<$t>> for Complex<$t> {
                type Output = Self;
                fn add(self,other: Self) -> Self::Output {
                    Complex {
                        r: self.r + other.r,
                        i: self.i + other.i,
                    }
                }
            }

            /// Complex += real.
            impl AddAssign<$t> for Complex<$t> {
                fn add_assign(&mut self,other: $t) {
                    self.r += other;
                }
            }

            /// Complex += complex.
            impl AddAssign<Complex<$t>> for Complex<$t> {
                fn add_assign(&mut self,other: Self) {
                    self.r += other.r;
                    self.i += other.i;
                }
            }

            /// Real - complex.
            impl Sub<Complex<$t>> for $t {
                type Output = Complex<$t>;
                fn sub(self,other: Complex<$t>) -> Self::Output {
                    Complex {
                        r: self - other.r,
                        i: -other.i,
                    }
                }
            }

            /// Complex - real.
            impl Sub<$t> for Complex<$t> {
                type Output = Self;
                fn sub(self,other: $t) -> Self::Output {
                    Complex {
                        r: self.r - other,
                        i: self.i,
                    }
                }
            }

            /// Complex - complex.
            impl Sub<Complex<$t>> for Complex<$t> {
                type Output = Self;
                fn sub(self,other: Self) -> Self::Output {
                    Complex {
                        r: self.r - other.r,
                        i: self.i - other.i,
                    }
                }
            }

            /// Complex -= real.
            impl SubAssign<$t> for Complex<$t> {
                fn sub_assign(&mut self,other: $t) {
                    self.r -= other;
                }
            }

            /// Complex -= complex.
            impl SubAssign<Complex<$t>> for Complex<$t> {
                fn sub_assign(&mut self,other: Self) {
                    self.r -= other.r;
                    self.i -= other.i;
                }
            }

            /// Real * complex.
            impl Mul<Complex<$t>> for $t {
                type Output = Complex<$t>;
                fn mul(self,other: Complex<$t>) -> Self::Output {
                    Complex {
                        r: self * other.r,
                        i: self * other.i,
                    }
                }
            }

            /// Complex * real.
            impl Mul<$t> for Complex<$t> {
                type Output = Self;
                fn mul(self,other: $t) -> Self::Output {
                    Complex {
                        r: self.r * other,
                        i: self.i * other,
                    }
                }
            }

            /// Complex * complex.
            impl Mul<Complex<$t>> for Complex<$t> {
                type Output = Self;
                fn mul(self,other: Self) -> Self::Output {
                    Complex {
                        r: self.r * other.r - self.i * other.i,
                        i: self.r * other.i + self.i * other.r,
                    }
                }
            }

            /// Complex *= real.
            impl MulAssign<$t> for Complex<$t> {
                fn mul_assign(&mut self,other: $t) {
                    self.r *= other;
                    self.i *= other;
                }
            }

            /// Complex *= complex.
            impl MulAssign<Complex<$t>> for Complex<$t> {
                fn mul_assign(&mut self,other: Self) {
                    let r = self.r * other.r - self.i * other.i;
                    let i = self.r * other.i + self.i * other.r;
                    self.r = r;
                    self.i = i;
                }
            }

            /// Real / complex.
            impl Div<Complex<$t>> for $t {
                type Output = Complex<$t>;
                fn div(self,other: Complex<$t>) -> Self::Output {
                    let f = other.r * other.r + other.i * other.i;
                    Complex {
                        r: self * other.r / f,
                        i: -self * other.i / f,
                    }
                }
            }

            /// Complex / real.
            impl Div<$t> for Complex<$t> {
                type Output = Self;
                fn div(self,other: $t) -> Self::Output {
                    Complex {
                        r: self.r / other,
                        i: self.i / other,
                    }
                }
            }

            /// Complex / complex.
            impl Div<Complex<$t>> for Complex<$t> {
                type Output = Self;
                fn div(self,other: Self) -> Self::Output {
                    let f = other.r * other.r + other.i * other.i;
                    Complex {
                        r: (self.r * other.r + self.i * other.i) / f,
                        i: (self.i * other.r - self.r * other.i) / f,
                    }
                }
            }

            /// Complex /= real.
            impl DivAssign<$t> for Complex<$t> {
                fn div_assign(&mut self,other: $t) {
                    self.r /= other;
                    self.i /= other;
                }
            }

            /// Complex /= complex.
            impl DivAssign<Complex<$t>> for Complex<$t> {
                fn div_assign(&mut self,other: Self) {
                    let f = other.r * other.r + other.i * other.i;
                    let r = (self.r * other.r + self.i * other.i) / f; 
                    let i = (self.i * other.r - self.r * other.i) / f;
                    self.r = r;
                    self.i = i;
                }
            }

            /// -Complex.
            impl Neg for Complex<$t> {
                type Output = Self;
                fn neg(self) -> Self {
                    Complex {
                        r: -self.r,
                        i: -self.i,
                    }
                }
            }
        )+
    }
}

complex_impl! { f32 f64 }

/// Complex number built from `f32`s.
#[allow(non_camel_case_types)]
pub type c32 = Complex<f32>;

/// Complex number built from `f64`s.
#[allow(non_camel_case_types)]
pub type c64 = Complex<f64>;
