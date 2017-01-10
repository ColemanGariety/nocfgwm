#![cfg_attr(not(feature = "xlib"), allow(dead_code))]
#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

extern crate libc;
extern crate x11;

use std::ffi::CString;
use std::mem::{zeroed};
use std::ptr::{null, null_mut};
use libc::{c_int, c_uint};
use x11::xlib;

fn max(a : c_int, b : c_int) -> c_uint { if a > b { a as c_uint } else { b as c_uint } }

fn main() {
    let display: *mut xlib::Display = unsafe{xlib::XOpenDisplay(null())};

    let mut attr: xlib::XWindowAttributes = unsafe { zeroed() };
    let mut start: xlib::XButtonEvent = unsafe { zeroed() };

    if display.is_null() {
        std::process::exit(1);
    }

    let mut event: xlib::XEvent = unsafe { zeroed() };

    let f1 = CString::new("F1").unwrap();
    unsafe {
        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, xlib::XStringToKeysym(f1.as_ptr())) as c_int, xlib::Mod1Mask,
                       xlib::XDefaultRootWindow(display), true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);

        xlib::XGrabButton(display, 1, xlib::AnyModifier, xlib::XDefaultRootWindow(display), true as c_int,
                          (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync,
                          0, 0);
        xlib::XGrabButton(display, 3, xlib::Mod1Mask, xlib::XDefaultRootWindow(display), true as c_int,
                          (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync,
                          0, 0);
    };

    start.subwindow = 0;

    loop {
        unsafe {
            xlib::XNextEvent(display, &mut event);

            match event.get_type() {
                xlib::MapRequest => {
                    let xmaprequest: xlib::XMapRequestEvent = From::from(event);
                    xlib::XMapWindow(display, xmaprequest.window);
                    xlib::XSetInputFocus(display, xmaprequest.window, xlib::RevertToPointerRoot, xlib::CurrentTime);
                },
                xlib::ButtonPress => {
                    let xbutton: xlib::XButtonEvent = From::from(event);
                    xlib::XSetInputFocus(display, xbutton.subwindow, xlib::RevertToPointerRoot, xlib::CurrentTime);
                    xlib::XRaiseWindow(display, xbutton.subwindow);

                    if xbutton.subwindow != 0 {
                        xlib::XGetWindowAttributes(display, xbutton.subwindow, &mut attr);
                        start = xbutton;
                    }
                },
                xlib::ButtonRelease => {
                    start.subwindow = 0;
                },
                xlib::MotionNotify => {
                    if start.subwindow != 0 {
                        let xbutton: xlib::XButtonEvent = From::from(event);
                        let xdiff : c_int = xbutton.x_root - start.x_root;
                        let ydiff : c_int = xbutton.y_root - start.y_root;
                        xlib::XMoveResizeWindow(display, start.subwindow,
                                                attr.x + (if start.button==1 { xdiff } else { 0 }),
                                                attr.y + (if start.button==1 { ydiff } else { 0 }),
                                                max(1, attr.width + (if start.button==3 { xdiff } else { 0 })),
                                                max(1, attr.height + (if start.button==3 { ydiff } else { 0 })));
                    }
                },
                _ => {}
            }
        }
    }
}
