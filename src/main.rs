#![cfg_attr(not(feature = "xlib"), allow(dead_code))]
#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

extern crate libc;
extern crate x11;

use std::mem::{zeroed};
use std::ptr::{null, null_mut};
use libc::{c_int, c_uint};
use x11::{xlib, keysym};

mod parent;
mod window;
mod title;

fn max(a : c_int, b : c_int) -> c_uint { if a > b { a as c_uint } else { b as c_uint } }

fn main() {
    let display: *mut xlib::Display = unsafe{xlib::XOpenDisplay(null())};
    if display.is_null() {
        panic!("Could not grab display");
    }

    let root;

    unsafe {
        root = xlib::XDefaultRootWindow(display);

        xlib::XSelectInput(display, root, xlib::ButtonPressMask |
                           xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask |
                           xlib::KeyPressMask | xlib::KeyReleaseMask);

        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, keysym::XK_Tab as u64) as c_int,
                       xlib::Mod1Mask, root, true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);
        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, keysym::XK_Tab as u64) as c_int,
                       xlib::ShiftMask | xlib::Mod1Mask, root, true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);
        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, keysym::XK_space as u64) as c_int,
                       xlib::Mod1Mask, root, true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);
        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, keysym::XK_Escape as u64) as c_int,
                       xlib::Mod1Mask, root, true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);
        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, keysym::XK_BackSpace as u64) as c_int,
                       xlib::Mod1Mask, root, true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);
        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, keysym::XK_BackSpace as u64) as c_int,
                       xlib::ShiftMask | xlib::Mod1Mask, root, true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);
    }

    let mut e: xlib::XEvent = unsafe{zeroed()};
    let mut xwindow: xlib::Window;
    let mut windows: Vec<window::Window> = Vec::new();

    loop {
        unsafe {
            xlib::XFlush(display);
            xlib::XNextEvent(display, &mut e);
        }

        xwindow = xeventwindow(e);

        match e.get_type() {
            xlib::ConfigureRequest => {
                let xconfigurerequest: xlib::XConfigureRequestEvent = From::from(e);
                window::configure(display, &xconfigurerequest);
            },
            xlib::MapRequest => {
                window::manage(display, root, xwindow, 0, &mut windows);
            },
            // xlib::ButtonPress => {
            //     // not implemented
            // },
            xlib::KeyPress | xlib::KeyRelease => {
                let xkey: xlib::XKeyEvent = From::from(e);
                if  xkey.window == root {
                    handlekey(display, e, xkey, &mut windows);
                }
            },
            xlib::ClientMessage |
            xlib::CreateNotify |
            xlib::DestroyNotify |
            xlib::ConfigureNotify |
            xlib::ReparentNotify |
            xlib::MapNotify |
            xlib::UnmapNotify => (),
            _ => {
                for win in windows.iter_mut() {
                    if win.parent.xwindow == xwindow {
                        win.active = true;
                        window::windowevent(display, e, win);
                    } else {
                        win.active = false;
                    }
                }
            }
        }
    }
}

fn xeventwindow(e: xlib::XEvent) -> xlib::Window {
    match e.get_type() {
        xlib::ConfigureRequest => {
            let xconfigurerequest: xlib::XConfigureRequestEvent = From::from(e);
            return xconfigurerequest.window;
        },
        xlib::MapRequest => {
            let xmaprequest: xlib::XMapRequestEvent = From::from(e);
            return xmaprequest.window;
        },
        _ => {
            let xany: xlib::XAnyEvent = From::from(e);
            return xany.window;
        }
    }
}

fn handlekey(display: *mut xlib::Display, e: xlib::XEvent, ep: xlib::XKeyEvent, windows: &mut Vec<window::Window>) {
    match unsafe{xlib::XKeycodeToKeysym(display, ep.keycode as u8, 0) as u32} {
        keysym::XK_Meta_L |
        keysym::XK_Meta_R |
        keysym::XK_Alt_L |
        keysym::XK_Alt_R |
        keysym::XK_Super_L |
        keysym::XK_Super_R => {
            // implement window cycling
        },
        keysym::XK_Tab => {
            // implement window switching
        },
        keysym::XK_space => {
            if e.get_type() == xlib::KeyPress {
                match windows.iter_mut().find(|ref win| win.active) {
                    Some(mut win) => {
                        window::maximize(display, win);
                    },
                    None => ()
                }
            }
        },
        keysym::XK_Escape => {
            // implement hide window
        },
        keysym::XK_BackSpace => {
            if e.get_type() == xlib::KeyPress {
                let mut revert: c_int = unsafe{zeroed()};
                unsafe {
                    match windows.iter().find(|&win| win.active) {
                        Some(win) => {
                            xlib::XGrabServer(display);
                            xlib::XSetErrorHandler(Some(xerr_ignore));
                            xlib::XDestroyWindow(display, win.parent.xwindow);
                            xlib::XSync(display, false as c_int);
                            xlib::XUngrabServer(display);
                        },
                        None => ()
                    }
                    for win in windows.iter_mut() {
                        win.active = false;
                    }
                }
            }
        },
        _ => ()
    }
}

#[allow(unused_variables)]
pub extern "C" fn xerr_ignore(display: *mut xlib::Display,
                               event: *mut xlib::XErrorEvent)
                               -> c_int {
    let e: xlib::XErrorEvent = unsafe { *event };
    0
}
