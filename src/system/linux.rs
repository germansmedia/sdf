use {
    crate::*,
    std::{
        os::raw::c_void,
        ptr::null_mut,
        rc::Rc,
    },
};

fn intern_atom_cookie(xcb_connection: *mut ffi::xcb_connection_t,name: &str) -> ffi::xcb_intern_atom_cookie_t {
    let i8_name = unsafe { std::mem::transmute::<_,&[i8]>(name.as_bytes()) };
    unsafe { ffi::xcb_intern_atom(xcb_connection,0,name.len() as u16,i8_name.as_ptr()) }
}

fn resolve_atom_cookie(xcb_connection: *mut ffi::xcb_connection_t,cookie: ffi::xcb_intern_atom_cookie_t) -> u32 {
    unsafe { (*ffi::xcb_intern_atom_reply(xcb_connection,cookie,null_mut())).atom }
}

#[derive(Debug)]
pub struct System {
    pub(crate) xdisplay: *mut ffi::Display,
    pub(crate) xcb_connection: *mut ffi::xcb_connection_t,
    pub(crate) xcb_screen: *mut ffi::xcb_screen_t,
    pub(crate) wm_protocols: u32,
    pub(crate) wm_delete_window: u32,
}

impl System {

    pub fn open() -> Result<System,String> {

        let xdisplay = unsafe { ffi::XOpenDisplay(null_mut()) };
        if xdisplay == null_mut() {
            return Err("unable to connect to X server".to_string());
        }
        let xcb_connection = unsafe { ffi::XGetXCBConnection(xdisplay) };
        if xcb_connection == null_mut() {
            unsafe { ffi::XCloseDisplay(xdisplay) };
            return Err("unable to connect to X server".to_string());
        }
        unsafe { ffi::XSetEventQueueOwner(xdisplay,ffi::XCBOwnsEventQueue) };
        let xcb_setup = unsafe { ffi::xcb_get_setup(xcb_connection) };
        if xcb_setup == null_mut() {
            unsafe { ffi::XCloseDisplay(xdisplay) };
            return Err("unable to obtain X server setup".to_string());
        }

        let xcb_screen = unsafe { ffi::xcb_setup_roots_iterator(xcb_setup) }.data;

        let protocols_cookie = intern_atom_cookie(xcb_connection,"WM_PROTOCOLS");
        let delete_window_cookie = intern_atom_cookie(xcb_connection,"WM_DELETE_WINDOW");

        let wm_protocols = resolve_atom_cookie(xcb_connection,protocols_cookie);
        let wm_delete_window = resolve_atom_cookie(xcb_connection,delete_window_cookie);

        Ok(System {
            xdisplay,
            xcb_connection,
            xcb_screen,
            wm_protocols,
            wm_delete_window,
        })
    }

#[doc(hidden)]
    fn translate_xevent(&self,xcb_event: *mut ffi::xcb_generic_event_t) -> Option<(u32,Event)> {
        match (unsafe { *xcb_event }.response_type & 0x7F) as u32 {
            ffi::XCB_EXPOSE => {
                let expose = xcb_event as *const ffi::xcb_expose_event_t;
                //let expose = unsafe { std::mem::transmute::<_,xcb_expose_event_t>(xcb_event) };
                let r = Rect {
                    o: Vec2 {
                        x: unsafe { *expose }.x as i32,
                        y: unsafe { *expose }.y as i32,
                    },
                    s: Vec2 {
                        x: unsafe { *expose }.width as i32,
                        y: unsafe { *expose }.height as i32,
                    },
                };
                let xcb_window = unsafe { *expose }.window;
                return Some((xcb_window,Event::Expose(r)));
            },
            ffi::XCB_KEY_PRESS => {
                let key_press = xcb_event as *const ffi::xcb_key_press_event_t;
                let xcb_window = unsafe { *key_press }.event;
                return Some((xcb_window,Event::Key(KeyEvent::Press { code: unsafe { *key_press }.detail as u32, })));
            },
            ffi::XCB_KEY_RELEASE => {
                let key_release = xcb_event as *const ffi::xcb_key_release_event_t;
                let xcb_window = unsafe { *key_release }.event;
                return Some((xcb_window,Event::Key(KeyEvent::Release { code: unsafe { *key_release }.detail as u32, })));
            },
            ffi::XCB_BUTTON_PRESS => {
                let button_press = xcb_event as *const ffi::xcb_button_press_event_t;
                let p = unsafe { Vec2 {
                    x: (*button_press).event_x as f32,
                    y: (*button_press).event_y as f32,
                } };
                let xcb_window = unsafe { *button_press }.event;
                match unsafe { *button_press }.detail {
                    1 => { return Some((xcb_window,Event::Pointer(PointerEvent::Down { position: p,button: Button::Left, }))); },
                    2 => { return Some((xcb_window,Event::Pointer(PointerEvent::Down { position: p,button: Button::Middle, }))); },
                    3 => { return Some((xcb_window,Event::Pointer(PointerEvent::Down { position: p,button: Button::Right, }))); },
                    4 => { return Some((xcb_window,Event::Pointer(PointerEvent::Scroll { position: p,buttons: Vec::new(), delta: Vec2 { x: 0.0,y: -1.0, }, }))); },
                    5 => { return Some((xcb_window,Event::Pointer(PointerEvent::Scroll { position: p,buttons: Vec::new(), delta: Vec2 { x: 0.0,y: 1.0, }, }))); },
                    6 => { return Some((xcb_window,Event::Pointer(PointerEvent::Scroll { position: p,buttons: Vec::new(), delta: Vec2 { x: -1.0,y: 0.0, }, }))); },
                    7 => { return Some((xcb_window,Event::Pointer(PointerEvent::Scroll { position: p,buttons: Vec::new(), delta: Vec2 { x: 1.0,y: 0.0, }, }))); },
                    _ => { },
                }        
            },
            ffi::XCB_BUTTON_RELEASE => {
                let button_release = xcb_event as *const ffi::xcb_button_release_event_t;
                let p = unsafe { Vec2 {
                    x: (*button_release).event_x as f32,
                    y: (*button_release).event_y as f32,
                } };
                let xcb_window = unsafe { *button_release }.event;
                match unsafe { *button_release }.detail {
                    1 => { return Some((xcb_window,Event::Pointer(PointerEvent::Up { position: p,button: Button::Left, }))); },
                    2 => { return Some((xcb_window,Event::Pointer(PointerEvent::Up { position: p,button: Button::Middle, }))); },
                    3 => { return Some((xcb_window,Event::Pointer(PointerEvent::Up { position: p,button: Button::Right, }))); },
                    _ => { },
                }        
            },
            ffi::XCB_MOTION_NOTIFY => {
                let motion_notify = xcb_event as *const ffi::xcb_motion_notify_event_t;
                let p = Vec2 {
                    x: unsafe { *motion_notify }.event_x as f32,
                    y: unsafe { *motion_notify }.event_y as f32,
                };
                let xcb_window = unsafe { *motion_notify }.event;
                return Some((xcb_window,Event::Pointer(PointerEvent::Move { position: p,buttons: Vec::new(),hover: false, })));
            },
            ffi::XCB_CONFIGURE_NOTIFY => {
                let configure_notify = xcb_event as *const ffi::xcb_configure_notify_event_t;
                let r = Rect {
                    o: Vec2 {
                        x: unsafe { *configure_notify }.x as i32,
                        y: unsafe { *configure_notify }.y as i32,
                    },
                    s: Vec2 {
                        x: unsafe { *configure_notify }.width as i32,
                        y: unsafe { *configure_notify }.height as i32,
                    },
                };
                let xcb_window = unsafe { *configure_notify }.window;
                return Some((xcb_window,Event::Configure(r)));
            },
            ffi::XCB_CLIENT_MESSAGE => {
                let client_message = xcb_event as *const ffi::xcb_client_message_event_t;
                let atom = unsafe { (*client_message).data.data32[0] };
                if atom == self.wm_delete_window {
                    let xcb_window = unsafe { *client_message }.window;
                    return Some((xcb_window,Event::Close));
                }
            },
            _ => {
            },
        }
        None
    }

    pub fn flush(&self) -> Vec<(u32,Event)> {
        let mut events = Vec::<(u32,Event)>::new();
        loop {
            let event = unsafe { ffi::xcb_poll_for_event(self.xcb_connection) };
            if event != null_mut() {
                if let Some((window_id,event)) = self.translate_xevent(event) {
                    events.push((window_id,event));
                }
            }
            else {
                break;
            }
        }
        events
    }

    pub fn create_frame(self: &Rc<System>,r: Rect<i32>,title: &str) -> Result<Window,String> {

        let xcb_window = unsafe { ffi::xcb_generate_id(self.xcb_connection) };
        let values = [
            ffi::XCB_EVENT_MASK_EXPOSURE
            | ffi::XCB_EVENT_MASK_KEY_PRESS
            | ffi::XCB_EVENT_MASK_KEY_RELEASE
            | ffi::XCB_EVENT_MASK_BUTTON_PRESS
            | ffi::XCB_EVENT_MASK_BUTTON_RELEASE
            | ffi::XCB_EVENT_MASK_POINTER_MOTION
            | ffi::XCB_EVENT_MASK_STRUCTURE_NOTIFY,
            ffi::XCB_COPY_FROM_PARENT,
        ];
        let protocol_set = [self.wm_delete_window];
        let protocol_set_void = protocol_set.as_ptr() as *const std::os::raw::c_void;
        unsafe {
            ffi::xcb_create_window(
                self.xcb_connection,
                (*self.xcb_screen).root_depth,
                xcb_window as u32,
                (*self.xcb_screen).root,
                r.o.x as i16,
                r.o.y as i16,
                r.s.x as u16,
                r.s.y as u16,
                0,
                ffi::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
                (*self.xcb_screen).root_visual,
                ffi::XCB_CW_EVENT_MASK | ffi::XCB_CW_COLORMAP,
                &values as *const u32 as *const c_void
            );
            ffi::xcb_map_window(self.xcb_connection,xcb_window as u32);
            ffi::xcb_flush(self.xcb_connection);
            ffi::xcb_change_property(
                self.xcb_connection,
                ffi::XCB_PROP_MODE_REPLACE as u8,
                xcb_window as u32,
                self.wm_protocols,
                ffi::XCB_ATOM_ATOM,
                32,
                1,
                protocol_set_void
            );
            ffi::xcb_change_property(
                self.xcb_connection,
                ffi::XCB_PROP_MODE_REPLACE as u8,
                xcb_window as u32,
                ffi::XCB_ATOM_WM_NAME,
                ffi::XCB_ATOM_STRING,
                8,
                title.len() as u32,
                title.as_bytes().as_ptr() as *const std::os::raw::c_void
            );
            ffi::xcb_flush(self.xcb_connection);
        };

        Ok(Window {
            system: Rc::clone(self),
            xcb_window,
        })
    }
}

#[derive(Debug)]
pub struct Window {
    pub system: Rc<System>,
#[doc(hidden)]
    pub(crate) xcb_window: ffi::xcb_window_t,
}

impl Drop for System {

    fn drop(&mut self) {
        unsafe { ffi::XCloseDisplay(self.xdisplay) };
    }
}

impl Drop for Window {

    fn drop(&mut self) {
        unsafe {
            ffi::xcb_unmap_window(self.system.xcb_connection,self.xcb_window as u32);
            ffi::xcb_destroy_window(self.system.xcb_connection,self.xcb_window as u32);
        }
    }
}
