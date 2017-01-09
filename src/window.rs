use libc::c_uint;
use x11::{xlib, xinput2};
use std::mem::zeroed;
use std::ffi::CString;
use std::ptr::{null, null_mut};

#[derive(Debug)]
pub struct Window {
    pub display: *mut xlib::Display,
    pub window: xlib::Window,

    wm_protocols: xlib::Atom,
    wm_delete_window: xlib::Atom
}

impl Window {
    /// Create a new window with a given title and size
    pub fn new(display: *mut xlib::Display, root: u64, screen: i32, title: &str, width: u32, height: u32) -> Window {
        unsafe {
            // Load atoms
            let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();
            let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();

            let wm_delete_window = xlib::XInternAtom(display, wm_delete_window_str.as_ptr(), xlib::False);
            let wm_protocols = xlib::XInternAtom(display, wm_protocols_str.as_ptr(), xlib::False);

            if wm_delete_window == 0 || wm_protocols == 0 {
                panic!("can't load atoms");
            }

            // Create window
            let white_pixel = xlib::XWhitePixel(display, screen);
            let mut attributes: xlib::XSetWindowAttributes = zeroed();
            attributes.background_pixel = white_pixel;

            let window = xlib::XCreateWindow(display, root, 0, 0, width as c_uint, height as c_uint, 0, 0,
                                             xlib::InputOutput as c_uint, null_mut(),
                                             xlib::CWBackPixel, &mut attributes);
            // Set window title
            let title_str = CString::new(title).unwrap();
            xlib::XStoreName(display, window, title_str.as_ptr() as *mut _);

            // Subscribe to delete (close) events
            let mut protocols = [wm_delete_window];

            if xlib::XSetWMProtocols(display, window, &mut protocols[0] as *mut xlib::Atom, 1) == xlib::False {
                panic!("can't set WM protocols");
            }

            Window{
                display: display,
                window: window,
                wm_protocols: wm_protocols,
                wm_delete_window: wm_delete_window
            }
        }
    }

    /// Display the window
    pub fn show(&mut self) {
        unsafe {
            xlib::XMapWindow(self.display, self.window);
        }
    }
}

impl Drop for Window {
    /// Destroys the window and disconnects from the display
    fn drop(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display, self.window);
            xlib::XCloseDisplay(self.display);
        }
    }
}
