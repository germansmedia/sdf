use {
    crate::*,
    std::fmt::{
        Display,
        Debug,
        Formatter,
        Result,
    },
};

#[derive(Copy,Clone,Debug)]
pub struct Rect<T> {
    pub o: Vec2<T>,
    pub s: Vec2<T>,
}

macro_rules! rect_impl {
    ($($t:ty)+) => {
        $(
            impl Display for Rect<$t> {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{} {}x{})",self.o.x,self.o.y,self.s.x,self.s.y)
                }
            }
        )+
    }
}

rect_impl! { isize i8 i16 i32 i64 i128 f32 f64 }
