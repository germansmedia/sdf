/// One trait. Defines associated constant `::ONE` for the type.
///
/// For numbers, this corresponds to the literal `1` or `1.0`. For vectors this is undefined. For matrices, this is the unit matrix. For complex numbers, this is `1 + 0i`. For quaternions, this is `1 + 0i + 0j + 0k`. For multivectors, this is undefined.
pub trait One {
    const ONE: Self;
}

impl One for u8 { const ONE: u8 = 1; }
impl One for u16 { const ONE: u16 = 1; }
impl One for u32 { const ONE: u32 = 1; }
impl One for u64 { const ONE: u64 = 1; }
impl One for u128 { const ONE: u128 = 1; }
impl One for usize { const ONE: usize = 1; }
impl One for i8 { const ONE: i8 = 1; }
impl One for i16 { const ONE: i16 = 1; }
impl One for i32 { const ONE: i32 = 1; }
impl One for i64 { const ONE: i64 = 1; }
impl One for i128 { const ONE: i128 = 1; }
impl One for isize { const ONE: isize = 1; }
impl One for f32 { const ONE: f32 = 1.0; }
impl One for f64 { const ONE: f64 = 1.0; }
