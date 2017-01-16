use x11::xlib;
use std::mem::{zeroed};
use std::ptr::{null, null_mut};
use libc::{c_int, c_uint};

use window;

pub struct Parent {
    pub dim: window::Dim,
    pub xwindow: xlib::Window,
}

pub fn create_parent(display: *mut xlib::Display, window: &mut window::Window,
                     xparent: xlib::Window, class: i32, x: i32,
                     y: i32, width: i32, height: i32) {

    let mut attr: xlib::XSetWindowAttributes = unsafe{zeroed()};

    window.parent.dim.x = x;
    window.parent.dim.y = y;
    window.parent.dim.width = width;
    window.parent.dim.height = height;

    unsafe {
        window.parent.xwindow = xlib::XCreateWindow(display, xparent,
                                                    x, y, (width + 4) as u32, (height + 12) as u32,
                                                    0, xlib::CopyFromParent as c_int,
                                                    class as u32, null_mut(),
                                                    xlib::CWOverrideRedirect, &mut attr);
    }
}
