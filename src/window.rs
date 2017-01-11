use x11::xlib;
use std::mem::{zeroed};
use std::ptr::{null, null_mut};
use libc::{c_int, c_uint};

use parent;

pub struct Title;
pub struct Dim {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
pub struct Window {
    title: Title,
    client: xlib::Window,
    odim: Dim,
    pub parent: parent::Parent,
}

pub fn configure(display: *mut xlib::Display, conf: &xlib::XConfigureRequestEvent) {
    let mut wc: xlib::XWindowChanges = unsafe{zeroed()};

    wc.x = conf.x;
    wc.y = conf.y;
    wc.width = conf.width;
    wc.height = conf.height;
    wc.border_width = conf.border_width;
    wc.sibling = conf.above;
    wc.stack_mode = conf.detail;

    unsafe{xlib::XConfigureWindow(display, conf.window, conf.value_mask as u32, &mut wc)};
}

pub fn manage(display: *mut xlib::Display, root: u64, client: xlib::Window, wmstart: i32) {
    let mut win: Window = unsafe{zeroed()};
    let mut attr: xlib::XWindowAttributes = unsafe{zeroed()};
    let mut sz: xlib::XSizeHints = unsafe{zeroed()};
    let wmhints: xlib::XWMHints;
    let state: i64;
    let mut dummy: i64 = unsafe{zeroed()};

    unsafe{
        xlib::XGetWindowAttributes(display, client, &mut attr);
        xlib::XGetWMNormalHints(display, client, &mut sz, &mut dummy);
    }

    win.client = client;
    win.title = unsafe{zeroed()};
    win.parent = unsafe{zeroed()};
    win.odim.x = attr.x;
    win.odim.y = attr.y;
    win.odim.width = attr.width;
    win.odim.height = attr.height;

    parent::create_parent(display, &mut win, root, xlib::InputOutput,
                          attr.x, attr.y, attr.width, attr.height);

    unsafe {
        xlib::XGrabButton(display, xlib::Button1, xlib::Mod1Mask,
                          win.parent.xwindow, false as c_int, xlib::ButtonPressMask as u32 |
                          xlib::ButtonReleaseMask as u32 | xlib::ButtonMotionMask as u32,
                          xlib::GrabModeAsync, xlib::GrabModeAsync,
                          0, xlib::XCreateFontCursor(display, 1));

        xlib::XGrabButton(display, xlib::Button1, xlib::Mod1Mask | xlib::ControlMask,
                          win.parent.xwindow, false as c_int, xlib::ButtonPressMask as u32 |
                          xlib::ButtonReleaseMask as u32 | xlib::ButtonMotionMask as u32,
                          xlib::GrabModeAsync, xlib::GrabModeAsync,
                          0, xlib::XCreateFontCursor(display, 1));

        xlib::XSelectInput(display, win.parent.xwindow, xlib::ExposureMask |
                           xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask);

        xlib::XSetWindowBorderWidth(display, client, 10);
        xlib::XLowerWindow(display, client);
        xlib::XSelectInput(display, client, xlib::PropertyChangeMask);

        xlib::XGrabButton(display, xlib::AnyButton as u32, 0, client, true as i32,
                          xlib::ButtonPressMask as u32, xlib::GrabModeSync, xlib::GrabModeSync,
                          0, 0);

        xlib::XMapWindow(display, client);
        xlib::XRaiseWindow(display, client);
        xlib::XSetInputFocus(display, client, xlib::RevertToPointerRoot, xlib::CurrentTime);
        xlib::XSync(display, 0);
    }
}
