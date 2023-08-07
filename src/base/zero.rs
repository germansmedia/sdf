/// Zero trait. Defines associated constant `::ZERO` for the type.
///
/// For numbers, this corresponds to the literal `0` or `0.0`. For vectors this is the origin. For matrices, this is a completely empty matrix. For complex numbers, this is `0 + 0i`. For quaternions, this is `0 + 0i + 0j + 0k`. For multivectors, this has all components 0.

pub trait Zero {
    const ZERO: Self;
}

impl Zero for u8 { const ZERO: u8 = 0; }
impl Zero for u16 { const ZERO: u16 = 0; }
impl Zero for u32 { const ZERO: u32 = 0; }
impl Zero for u64 { const ZERO: u64 = 0; }
impl Zero for u128 { const ZERO: u128 = 0; }
impl Zero for usize { const ZERO: usize = 0; }
impl Zero for i8 { const ZERO: i8 = 0; }
impl Zero for i16 { const ZERO: i16 = 0; }
impl Zero for i32 { const ZERO: i32 = 0; }
impl Zero for i64 { const ZERO: i64 = 0; }
impl Zero for i128 { const ZERO: i128 = 0; }
impl Zero for isize { const ZERO: isize = 0; }
impl Zero for f32 { const ZERO: f32 = 0.0; }
impl Zero for f64 { const ZERO: f64 = 0.0; }
