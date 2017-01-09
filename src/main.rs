// XInput2 example for x11-rs
//
// This is a basic example showing how to use XInput2 to read
// keyboard, mouse and other input events in X11.
//
// See Pete Hutterer's "XI2 Recipes" blog series,
// starting at http://who-t.blogspot.co.uk/2009/05/xi2-recipes-part-1.html
// for a guide.

#![cfg_attr(not(feature = "xlib"), allow(dead_code))]
#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

extern crate x11;
extern crate libc;

use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::*;
use std::slice::{from_raw_parts};
use x11::{xlib, xinput2};
use window::Window;
use std::mem::zeroed;
use std::ptr::{null, null_mut};

mod window;

struct InputState {
    cursor_pos: (f64, f64),
}

/// Given an input motion event for an axis and the previous
/// state of the axises, return the horizontal/vertical
/// scroll deltas
#[cfg(not(all(feature = "xlib", feature = "xinput")))]
fn main () {
    panic!("this example requires `--features 'xlib xinput'`");
}

#[cfg(all(feature = "xlib", feature = "xinput"))]
fn main () {
    // Open display
    let display = unsafe{xlib::XOpenDisplay(null())};
    if display == null_mut() {
        panic!("can't open display");
    }

    let screen = unsafe{xlib::XDefaultScreen(display)};
    let root = unsafe{xlib::XRootWindow(display, screen)};


    let mut demo_window = Window::new(display, root, screen, "A", 600, 400);
    let mut other_window = Window::new(display, root, screen, "B", 300, 200);

    // query XInput support
    let mut opcode: c_int = 0;
    let mut event: c_int = 0;
    let xinput_str = CString::new("XInputExtension").unwrap();

    // init XInput events
    let mut mask: [c_uchar; 1] = [0];
    let mut input_event_mask = xinput2::XIEventMask {
        deviceid: xinput2::XIAllMasterDevices,
        mask_len: mask.len() as i32,
        mask: mask.as_mut_ptr()
    };
    let events = &[
        xinput2::XI_ButtonPress,
        xinput2::XI_ButtonRelease,
        xinput2::XI_KeyPress,
        xinput2::XI_KeyRelease,
        xinput2::XI_Motion
    ];
    for &event in events {
        xinput2::XISetMask(&mut mask, event);
    }

    match unsafe{xinput2::XISelectEvents(display,
      root, &mut input_event_mask, 1)} {
        status if status as u8 == xlib::Success => (),
        err => panic!("Failed to select events {:?}", err)
    }

    // Show window
    other_window.show();
    demo_window.show();

    let mut event: xlib::XEvent = unsafe{zeroed()};
    let mut start: &xinput2::XIDeviceEvent = unsafe{zeroed()};

    loop {
        unsafe{xlib::XNextEvent(display, &mut event)};
        match event.get_type() {
            xlib::GenericEvent => {
                let mut attr: xlib::XWindowAttributes = unsafe{zeroed()};
                let mut cookie: xlib::XGenericEventCookie = From::from(event);
                if unsafe{xlib::XGetEventData(display, &mut cookie)} != xlib::True {
                    println!("Failed to retrieve event data");
                    return;
                }

                match cookie.evtype {
                    xinput2::XI_ButtonPress => {
                        start = unsafe{transmute(cookie.data)};
                    },
                    xinput2::XI_ButtonPress => {
                        start = unsafe{zeroed()};
                    },
                    xinput2::XI_Motion => {
                        let event_data: &xinput2::XIDeviceEvent = unsafe{transmute(cookie.data)};

                        if event_data.child != 0 && event_data.mods.base == 8 {
                            unsafe {
                                xlib::XGetWindowAttributes(display, event_data.child, &mut attr);
                                xlib::XMoveResizeWindow(
                                    display,
                                    event_data.child,
                                    event_data.event_x as i32,
                                    event_data.event_y as i32,
                                    attr.width as u32,
                                    attr.height as u32
                                );
                            };
                        }
                    },
                    _ => ()
                }
                unsafe{xlib::XFreeEventData(display, &mut cookie)};
            },
            _ => ()
        }
    }
}
