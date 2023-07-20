use {
    crate::*,
    std::fmt,
};

#[derive(Copy,Clone,Debug)]
pub enum KeyEvent {
    Press { code: u32, },
    Release { code: u32, },
}

impl fmt::Display for KeyEvent {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyEvent::Press { code, } => write!(f,"Press {{ code: {}, }}",code),
            KeyEvent::Release { code, } => write!(f,"Release {{ code: {}, }}",code),
        }
    }
}

#[derive(Clone,Debug)]
pub enum Button {
    Left,
    Right,
    Middle,
}

impl fmt::Display for Button {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Button::Left => write!(f,"Button::Left"),
            Button::Right => write!(f,"Button::Right"),
            Button::Middle => write!(f,"Button::Middle"),
        }
    }
}

#[derive(Clone,Debug)]
pub enum PointerEvent {
    Down { position: Vec2<f32>, button: Button, },
    Up { position: Vec2<f32>, button: Button, },
    Move { position: Vec2<f32>, buttons: Vec<Button>, hover: bool, },
    Scroll { position: Vec2<f32>, buttons: Vec<Button>, delta: Vec2<f32>, },
}

impl fmt::Display for PointerEvent {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PointerEvent::Down { position, button, } => write!(f,"Down {{ position: {},buttons: {}, }}",position,button),
            PointerEvent::Up { position, button, } => write!(f,"Up {{ position: {},buttons: {}, }}",position,button),
            PointerEvent::Move { position, hover, .. } => write!(f,"Move {{ position: {},buttons: TODO,hover: {}, }}",position,hover),
            PointerEvent::Scroll { position, delta, .. } => write!(f,"Scroll {{ position: {}, buttons: TODO, delta: {}, }}",position,delta),
        }
    }
}

#[derive(Clone,Debug)]
pub enum Event {
    Key(KeyEvent),
    Pointer(PointerEvent),
    Configure(Rect<i32>),
    Expose(Rect<i32>),
    Close,
}

impl fmt::Display for Event {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::Key(event) => write!(f,"{}",event),
            Event::Pointer(event) => write!(f,"{}",event),
            Event::Configure(rect) => write!(f,"Configure({})",rect),
            Event::Expose(rect) => write!(f,"Expose({})",rect),
            Event::Close => write!(f,"Close"),
        }
    }
}

#[cfg(system="linux")]
mod linux;
#[cfg(system="linux")]
pub use linux::*;

#[cfg(system="windows")]
mod windows;
#[cfg(system="windows")]
pub use windows::*;

#[cfg(system="android")]
mod android;
#[cfg(system="android")]
pub use android::*;
