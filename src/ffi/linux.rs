use super::*;

pub type size_t = ::std::os::raw::c_ulong;

pub const XCB_KEY_PRESS: u32 = 2;
pub const XCB_KEY_RELEASE: u32 = 3;
pub const XCB_BUTTON_PRESS: u32 = 4;
pub const XCB_BUTTON_RELEASE: u32 = 5;
pub const XCB_MOTION_NOTIFY: u32 = 6;
pub const XCB_EXPOSE: u32 = 12;
pub const XCB_CONFIGURE_NOTIFY: u32 = 22;
pub const XCB_CLIENT_MESSAGE: u32 = 33;

pub const XCB_COPY_FROM_PARENT: u32 = 0;

// X11

pub type Display = Opaque;

pub type xcb_connection_t = Opaque;

pub type xcb_window_t = u32;
pub type xcb_colormap_t = u32;
pub type xcb_visualid_t = u32;
pub type xcb_atom_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_generic_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub resource_id: u32,
    pub minor_code: u16,
    pub major_code: u8,
    pub pad0: u8,
    pub pad: [u32; 5usize],
    pub full_sequence: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths_len: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_intern_atom_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_intern_atom_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: xcb_atom_t,
}

pub const XCBOwnsEventQueue: XEventQueueOwner = 1;

pub type XEventQueueOwner = ::std::os::raw::c_uint;

pub type xcb_keycode_t = u8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_t {
    pub status: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_len: u16,
    pub maximum_request_length: u16,
    pub roots_len: u8,
    pub pixmap_formats_len: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screen_iterator_t {
    pub data: *mut xcb_screen_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_generic_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub pad: [u32; 7usize],
    pub full_sequence: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_expose_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
    pub pad1: [u8; 2usize],
}

pub type xcb_timestamp_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_key_press_event_t {
    pub response_type: u8,
    pub detail: xcb_keycode_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

pub type xcb_key_release_event_t = xcb_key_press_event_t;

pub type xcb_button_t = u8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_button_press_event_t {
    pub response_type: u8,
    pub detail: xcb_button_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

pub type xcb_button_release_event_t = xcb_button_press_event_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_motion_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_configure_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xcb_client_message_data_t {
    pub data8: [u8; 20usize],
    pub data16: [u16; 10usize],
    pub data32: [u32; 5usize],
    _bindgen_union_align: [u32; 5usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcb_client_message_event_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub type_: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}

pub type xcb_event_mask_t = ::std::os::raw::c_uint;
pub const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
pub const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
pub const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
pub const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
pub const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
pub const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;

pub type xcb_window_class_t = ::std::os::raw::c_uint;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;

pub type xcb_cw_t = ::std::os::raw::c_uint;
pub const XCB_CW_EVENT_MASK: xcb_cw_t = 2048;
pub const XCB_CW_COLORMAP: xcb_cw_t = 8192;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_void_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}

pub type xcb_prop_mode_t = ::std::os::raw::c_uint;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;

pub type xcb_atom_enum_t = ::std::os::raw::c_uint;
pub const XCB_ATOM_ATOM: xcb_atom_enum_t = 4;
pub const XCB_ATOM_STRING: xcb_atom_enum_t = 31;
pub const XCB_ATOM_WM_NAME: xcb_atom_enum_t = 39;

extern "C" { pub fn XOpenDisplay(arg1: *const ::std::os::raw::c_char) -> *mut Display; }
extern "C" { pub fn XCloseDisplay(arg1: *mut Display) -> ::std::os::raw::c_int; }
extern "C" { pub fn xcb_intern_atom(c: *mut xcb_connection_t,only_if_exists: u8,name_len: u16,name: *const ::std::os::raw::c_char) -> xcb_intern_atom_cookie_t; }
extern "C" { pub fn xcb_intern_atom_reply(c: *mut xcb_connection_t,cookie: xcb_intern_atom_cookie_t,e: *mut *mut xcb_generic_error_t) -> *mut xcb_intern_atom_reply_t; }
extern "C" { pub fn XGetXCBConnection(dpy: *mut Display) -> *mut xcb_connection_t; }
extern "C" { pub fn XSetEventQueueOwner(dpy: *mut Display, owner: XEventQueueOwner); }
extern "C" { pub fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t; }
extern "C" { pub fn xcb_setup_roots_iterator(R: *const xcb_setup_t) -> xcb_screen_iterator_t; }
extern "C" { pub fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t; }
extern "C" { pub fn xcb_generate_id(c: *mut xcb_connection_t) -> u32; }
extern "C" { pub fn xcb_create_window(c: *mut xcb_connection_t,depth: u8,wid: xcb_window_t,parent: xcb_window_t,x: i16,y: i16,width: u16,height: u16,border_width: u16,_class: u16,visual: xcb_visualid_t,value_mask: u32,value_list: *const ::std::os::raw::c_void) -> xcb_void_cookie_t; }
extern "C" { pub fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t; }
extern "C" { pub fn xcb_flush(c: *mut xcb_connection_t) -> ::std::os::raw::c_int; }
extern "C" { pub fn xcb_change_property(c: *mut xcb_connection_t,mode: u8,window: xcb_window_t,property: xcb_atom_t,type_: xcb_atom_t,format: u8,data_len: u32,data: *const ::std::os::raw::c_void) -> xcb_void_cookie_t; }
extern "C" { pub fn xcb_unmap_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t; }
extern "C" { pub fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t; }
