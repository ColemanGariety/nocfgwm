#![cfg_attr(not(feature = "xlib"), allow(dead_code))]
#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

extern crate libc;
extern crate x11;

use std::ffi::CString;
use std::mem::{zeroed};
use std::ptr::{null, null_mut};
use libc::{c_int, c_uint};
use x11::{xlib, keysym};

mod parent;
mod window;

fn max(a : c_int, b : c_int) -> c_uint { if a > b { a as c_uint } else { b as c_uint } }

fn main() {
    let display: *mut xlib::Display = unsafe{xlib::XOpenDisplay(null())};
    if display.is_null() {
        panic!("Could not grab display");
    }

    let screen;
    let root;

    unsafe {
        screen = xlib::XDefaultScreen(display);
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
                window::manage(display, root, xwindow, 0);
            },
            xlib::ButtonPress => {
                // not implemented
            },
            xlib::KeyPress | xlib::KeyRelease => {
                // not implemented
            },
            _ => ()
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
