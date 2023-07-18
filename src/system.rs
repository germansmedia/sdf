use {
    crate::*,
    std::{
        fmt,
        os::raw::{
            c_void,
            c_int,
        },
        ptr::null_mut,
        rc::Rc,
        mem::MaybeUninit,
    },
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

/// The system structure (linux).
#[derive(Debug)]
pub struct System {
    pub(crate) xdisplay: *mut ffi::Display,
    pub(crate) xcb_connection: *mut ffi::xcb_connection_t,
    pub(crate) xcb_screen: *mut ffi::xcb_screen_t,
    //pub(crate) epfd: c_int,
    pub(crate) wm_protocols: u32,
    pub(crate) wm_delete_window: u32,
    //pub(crate) wm_motif_hints: u32,
#[allow(dead_code)]
    pub(crate) wm_transient_for: u32,
#[allow(dead_code)]
    pub(crate) wm_net_type: u32,
#[allow(dead_code)]
    pub(crate) wm_net_type_utility: u32,
#[allow(dead_code)]
    pub(crate) wm_net_type_dropdown_menu: u32,
    //pub(crate) wm_net_state: u32,
    //pub(crate) wm_net_state_above: u32,
}

fn intern_atom_cookie(xcb_connection: *mut ffi::xcb_connection_t,name: &str) -> ffi::xcb_intern_atom_cookie_t {
    let i8_name = unsafe { std::mem::transmute::<_,&[i8]>(name.as_bytes()) };
    unsafe { ffi::xcb_intern_atom(xcb_connection,0,name.len() as u16,i8_name.as_ptr()) }
}

fn resolve_atom_cookie(xcb_connection: *mut ffi::xcb_connection_t,cookie: ffi::xcb_intern_atom_cookie_t) -> u32 {
    unsafe { (*ffi::xcb_intern_atom_reply(xcb_connection,cookie,null_mut())).atom }
}

impl System {

    /// Open the system interface.
    pub fn open() -> Result<System,String> {

        // open X connection and get first screen
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

        // start by assuming the root depth and visual
        let xcb_screen = unsafe { ffi::xcb_setup_roots_iterator(xcb_setup) }.data;

        // create epoll descriptor to be able to wait for UI events on a system level
        let fd = unsafe { ffi::xcb_get_file_descriptor(xcb_connection) };
        let epfd = unsafe { ffi::epoll_create1(0) };
        let mut epe = [ffi::epoll_event { events: ffi::EPOLLIN as u32,data: ffi::epoll_data_t { u64_: 0, }, }];
        unsafe { ffi::epoll_ctl(epfd,ffi::EPOLL_CTL_ADD as c_int,fd,epe.as_mut_ptr()) };

        // get the atoms
        let protocols_cookie = intern_atom_cookie(xcb_connection,"WM_PROTOCOLS");
        let delete_window_cookie = intern_atom_cookie(xcb_connection,"WM_DELETE_WINDOW");
        let transient_for_cookie = intern_atom_cookie(xcb_connection,"WM_TRANSIENT_FOR");
        let net_type_cookie = intern_atom_cookie(xcb_connection,"_NET_WM_TYPE");
        let net_type_utility_cookie = intern_atom_cookie(xcb_connection,"_NET_WM_TYPE_UTILITY");
        let net_type_dropdown_menu_cookie = intern_atom_cookie(xcb_connection,"_NET_WM_TYPE_DROPDOWN_MENU");

        let wm_protocols = resolve_atom_cookie(xcb_connection,protocols_cookie);
        let wm_delete_window = resolve_atom_cookie(xcb_connection,delete_window_cookie);
        let wm_transient_for = resolve_atom_cookie(xcb_connection,transient_for_cookie);
        let wm_net_type = resolve_atom_cookie(xcb_connection,net_type_cookie);
        let wm_net_type_utility = resolve_atom_cookie(xcb_connection,net_type_utility_cookie);
        let wm_net_type_dropdown_menu = resolve_atom_cookie(xcb_connection,net_type_dropdown_menu_cookie);

        Ok(System {
            xdisplay,
            xcb_connection,
            xcb_screen,
            //epfd,
            wm_protocols,
            wm_delete_window,
            //wm_motif_hints,
            wm_transient_for,
            wm_net_type,
            wm_net_type_utility,
            wm_net_type_dropdown_menu,
            //wm_net_state,
            //wm_net_state_above,
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

    /// Get all OS window events that have gathered.

    // TODO: this should be combined with a regular async handler/loop
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

        // create window
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

    pub fn create_gpu(self: &Rc<System>) -> Result<Rc<Gpu>,String> {

        // create instance
        let extension_names = [
            ffi::VK_KHR_SURFACE_EXTENSION_NAME.as_ptr(),
            ffi::VK_KHR_XCB_SURFACE_EXTENSION_NAME.as_ptr(),
        ];
        let info = ffi::VkInstanceCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            pApplicationInfo: &ffi::VkApplicationInfo {
                sType: ffi::VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: null_mut(),
                pApplicationName: b"System\0".as_ptr() as *const i8,
                applicationVersion: (1 << 22) as u32,
                pEngineName: b"VulkanGpu\0".as_ptr() as *const i8,
                engineVersion: (1 << 22) as u32,
                apiVersion: ((1 << 22) | (2 << 11)) as u32,
            },
            enabledExtensionCount: extension_names.len() as u32,
            ppEnabledExtensionNames: extension_names.as_ptr() as *const *const i8,
            enabledLayerCount: 0,
            ppEnabledLayerNames: null_mut(),
        };
        let mut vk_instance = MaybeUninit::<ffi::VkInstance>::uninit();
        match unsafe { ffi::vkCreateInstance(&info,null_mut(),vk_instance.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => return Err(format!("System::create_gpu: unable to create VkInstance ({})",vk_code_to_string(code))),
        }
        let vk_instance = unsafe { vk_instance.assume_init() };

        // enumerate physical devices
        let mut count = MaybeUninit::<u32>::uninit();
        unsafe { ffi::vkEnumeratePhysicalDevices(vk_instance,count.as_mut_ptr(),null_mut()) };
        let count = unsafe { count.assume_init() };
        if count == 0 {
            unsafe { ffi::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("System::create_gpu: unable to enumerate physical devices".to_string());
        }
        let mut vk_physical_devices = vec![null_mut(); count as usize];
        unsafe { ffi::vkEnumeratePhysicalDevices(vk_instance,&count as *const u32 as *mut u32,vk_physical_devices.as_mut_ptr()) };

        println!("System::create_gpu: physical devices:");
        vk_physical_devices.iter().for_each(|vk_physical_device| {
            let mut properties = MaybeUninit::<ffi::VkPhysicalDeviceProperties>::uninit();
            unsafe { ffi::vkGetPhysicalDeviceProperties(*vk_physical_device,properties.as_mut_ptr()) };
            let properties = unsafe { properties.assume_init() };
            let slice: &[u8] = unsafe { &*(&properties.deviceName as *const [i8] as *const [u8]) };
            println!("System::create_gpu:     {}",std::str::from_utf8(slice).unwrap());
        });

        // get first physical device
        println!("System::create_gpu: choosing first device");
        let vk_physical_device = vk_physical_devices[0];
        
        // get supported queue families
        let mut count = 0u32;
        unsafe { ffi::vkGetPhysicalDeviceQueueFamilyProperties(vk_physical_device,&mut count as *mut u32,null_mut()) };
        if count == 0 {
            unsafe { ffi::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("System::create_gpu: no queue families supported on this GPU".to_string());
        }
        let mut vk_queue_families = vec![MaybeUninit::<ffi::VkQueueFamilyProperties>::uninit(); count as usize];
        unsafe { ffi::vkGetPhysicalDeviceQueueFamilyProperties(
            vk_physical_device,
            &count as *const u32 as *mut u32,
            vk_queue_families.as_mut_ptr() as *mut ffi::VkQueueFamilyProperties,
        ) };
        let vk_queue_families = unsafe { std::mem::transmute::<_,Vec<ffi::VkQueueFamilyProperties>>(vk_queue_families) };

        // DEBUG: display the number of queues and capabilities
        println!("System::create_gpu: supported queue families:");
        vk_queue_families.iter().for_each(|vk_queue_family| {
            let mut capabilities = String::new();
            if vk_queue_family.queueFlags & ffi::VK_QUEUE_GRAPHICS_BIT != 0 {
                capabilities.push_str("graphics ");
            }
            if vk_queue_family.queueFlags & ffi::VK_QUEUE_TRANSFER_BIT != 0 {
                capabilities.push_str("transfer ");
            }
            if vk_queue_family.queueFlags & ffi::VK_QUEUE_COMPUTE_BIT != 0 {
                capabilities.push_str("compute ");
            }
            if vk_queue_family.queueFlags & ffi::VK_QUEUE_SPARSE_BINDING_BIT != 0 {
                capabilities.push_str("sparse ");
            }
            println!("System::create_gpu:     - {} queues, capable of: {}",vk_queue_family.queueCount,capabilities);
        });

        // assume the first queue family is the one we want for all queues
        println!("System::create_gpu: choosing first family");
        let vk_queue_family = vk_queue_families[0];
        let mask = ffi::VK_QUEUE_GRAPHICS_BIT | ffi::VK_QUEUE_TRANSFER_BIT | ffi::VK_QUEUE_COMPUTE_BIT;
        if (vk_queue_family.queueFlags & mask) != mask {
            unsafe { ffi::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("System::create_gpu: first queue family does not support graphics, transfer and compute operations".to_string());
        }

        // assume that presentation is done on the same family as graphics and create logical device with one queue of queue family 0
        let mut queue_create_infos = Vec::<ffi::VkDeviceQueueCreateInfo>::new();
        let priority = 1f32;
        queue_create_infos.push(ffi::VkDeviceQueueCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            queueFamilyIndex: 0,
            queueCount: 1,
            pQueuePriorities: &priority as *const f32,
        });
        let extension_names = [
            ffi::VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr(),
        ];
        let info = ffi::VkDeviceCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            queueCreateInfoCount: queue_create_infos.len() as u32,
            pQueueCreateInfos: queue_create_infos.as_ptr(),
            enabledLayerCount: 0,
            ppEnabledLayerNames: null_mut(),
            enabledExtensionCount: extension_names.len() as u32,
            ppEnabledExtensionNames: extension_names.as_ptr() as *const *const i8,
            pEnabledFeatures: &ffi::VkPhysicalDeviceFeatures {
                robustBufferAccess: 0,
                fullDrawIndexUint32: 0,
                imageCubeArray: 0,
                independentBlend: 0,
                geometryShader: 0,
                tessellationShader: 0,
                sampleRateShading: 0,
                dualSrcBlend: 0,
                logicOp: 0,
                multiDrawIndirect: 0,
                drawIndirectFirstInstance: 0,
                depthClamp: 0,
                depthBiasClamp: 0,
                fillModeNonSolid: 0,
                depthBounds: 0,
                wideLines: 0,
                largePoints: 0,
                alphaToOne: 0,
                multiViewport: 0,
                samplerAnisotropy: 0,
                textureCompressionETC2: 0,
                textureCompressionASTC_LDR: 0,
                textureCompressionBC: 0,
                occlusionQueryPrecise: 0,
                pipelineStatisticsQuery: 0,
                vertexPipelineStoresAndAtomics: 0,
                fragmentStoresAndAtomics: 0,
                shaderTessellationAndGeometryPointSize: 0,
                shaderImageGatherExtended: 0,
                shaderStorageImageExtendedFormats: 0,
                shaderStorageImageMultisample: 0,
                shaderStorageImageReadWithoutFormat: 0,
                shaderStorageImageWriteWithoutFormat: 1,
                shaderUniformBufferArrayDynamicIndexing: 0,
                shaderSampledImageArrayDynamicIndexing: 0,
                shaderStorageBufferArrayDynamicIndexing: 0,
                shaderStorageImageArrayDynamicIndexing: 0,
                shaderClipDistance: 0,
                shaderCullDistance: 0,
                shaderFloat64: 0,
                shaderInt64: 0,
                shaderInt16: 0,
                shaderResourceResidency: 0,
                shaderResourceMinLod: 0,
                sparseBinding: 0,
                sparseResidencyBuffer: 0,
                sparseResidencyImage2D: 0,
                sparseResidencyImage3D: 0,
                sparseResidency2Samples: 0,
                sparseResidency4Samples: 0,
                sparseResidency8Samples: 0,
                sparseResidency16Samples: 0,
                sparseResidencyAliased: 0,
                variableMultisampleRate: 0,
                inheritedQueries: 0,
            },
        };
        let mut vk_device = MaybeUninit::<ffi::VkDevice>::uninit();
        match unsafe { ffi::vkCreateDevice(vk_physical_device,&info,null_mut(),vk_device.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => { 
                unsafe { ffi::vkDestroyInstance(vk_instance,null_mut()) };
                return Err(format!("System::create_gpu: unable to create VkDevice ({})",super::vk_code_to_string(code)));
            },
        }
        let vk_device = unsafe { vk_device.assume_init() };

        // obtain the queue from queue family 0
        let mut vk_queue = MaybeUninit::<ffi::VkQueue>::uninit();
        unsafe { ffi::vkGetDeviceQueue(vk_device,0,0,vk_queue.as_mut_ptr()) };
        let vk_queue = unsafe { vk_queue.assume_init() };

        // create command pool for this queue
        let info = ffi::VkCommandPoolCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: null_mut(),
            flags: ffi::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
            queueFamilyIndex: 0,
        };
        let mut vk_command_pool = MaybeUninit::<ffi::VkCommandPool>::uninit();
        match unsafe { ffi::vkCreateCommandPool(vk_device,&info,null_mut(),vk_command_pool.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => {
                unsafe {
                    ffi::vkDestroyDevice(vk_device,null_mut());
                    ffi::vkDestroyInstance(vk_instance,null_mut());
                }
                return Err(format!("System::create_gpu: unable to create command pool ({})",super::vk_code_to_string(code)));
            },
        }
        let vk_command_pool = unsafe { vk_command_pool.assume_init() };

        // create descriptor pool
        let info = ffi::VkDescriptorPoolCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            maxSets: 5u32,
            poolSizeCount: 1,
            pPoolSizes: &ffi::VkDescriptorPoolSize {
                type_: ffi::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
                descriptorCount: 5u32,
            },
        };
        let mut vk_descriptor_pool = MaybeUninit::uninit();
        match unsafe { ffi::vkCreateDescriptorPool(vk_device,&info,null_mut(),vk_descriptor_pool.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => {
                unsafe {
                    ffi::vkDestroyDevice(vk_device,null_mut());
                    ffi::vkDestroyInstance(vk_instance,null_mut());
                }
                return Err(format!("System::create_gpu: unable to create descriptor pool ({})",super::vk_code_to_string(code)));
            }
        }
        let vk_descriptor_pool = unsafe { vk_descriptor_pool.assume_init() };

        // get memory properties
        let mut vk_memory_properties = MaybeUninit::<ffi::VkPhysicalDeviceMemoryProperties>::uninit();
        unsafe { ffi::vkGetPhysicalDeviceMemoryProperties(vk_physical_device,vk_memory_properties.as_mut_ptr()) };
        let vk_memory_properties = unsafe { vk_memory_properties.assume_init() };

        // DEBUG: show the entire memory description
        println!("System::create_gpu: device memory properties:");
        println!("System::create_gpu:     memory heaps:");
        for i in 0..vk_memory_properties.memoryHeapCount as usize {
            println!("System::create_gpu:         {}: size {} MiB, {:X}",i,vk_memory_properties.memoryHeaps[i].size / (1024 * 1024),vk_memory_properties.memoryHeaps[i].flags);
        }
        println!("System::create_gpu:     memory types:");
        for i in 0..vk_memory_properties.memoryTypeCount as usize {
            let mut flags = String::new();
            let vk_memory_type = &vk_memory_properties.memoryTypes[i];
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT) != 0 {
                flags += "device_local ";
            }
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT) != 0 {
                flags += "host_visible ";
            }
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT) != 0 {
                flags += "host_coherent ";
            }
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_HOST_CACHED_BIT) != 0 {
                flags += "host_cached ";
            }
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT) != 0 {
                flags += "lazily_allocated ";
            }
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_PROTECTED_BIT) != 0 {
                flags += "protected ";
            }
            println!("System::create_gpu:         - on heap {}, {}",vk_memory_type.heapIndex,flags);
        }

        // find shared memory heap and type (later also find device-only index)
        let mask = ffi::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT | ffi::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | ffi::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;
        let valid_types: Vec<(usize,&ffi::VkMemoryType)> = vk_memory_properties.memoryTypes.iter().enumerate().filter(|vk_memory_type| (vk_memory_type.1.propertyFlags & mask) == mask).collect();
        if valid_types.is_empty() {
            return Err("System::create_gpu: no valid memory types found".to_string());
        }
        let shared_index = valid_types[0].0;

        Ok(Rc::new(Gpu {
            system: Rc::clone(&self),
            vk_instance,
            vk_physical_device,
            vk_device,
            vk_queue,
            vk_command_pool,
            vk_descriptor_pool,
            shared_index,
        }))
    }
}

#[derive(Debug)]
pub struct Window {
    pub system: Rc<System>,
#[doc(hidden)]
    pub(crate) xcb_window: ffi::xcb_window_t,
}

impl Drop for System {

    /// Drop the system interface.
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
