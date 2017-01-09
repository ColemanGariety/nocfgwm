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

const TITLE: &'static str = "XInput Demo";
const DEFAULT_WIDTH: c_uint = 640;
const DEFAULT_HEIGHT: c_uint = 480;

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


    let mut demo_window = Window::new(display, root, screen, TITLE, DEFAULT_WIDTH, DEFAULT_HEIGHT);

    // query XInput support
    let mut opcode: c_int = 0;
    let mut event: c_int = 0;
    let mut error: c_int = 0;
    let xinput_str = CString::new("XInputExtension").unwrap();

    let mut xinput_major_ver = xinput2::XI_2_Major;
    let mut xinput_minor_ver = xinput2::XI_2_Minor;
    if unsafe{xinput2::XIQueryVersion(demo_window.display,
      &mut xinput_major_ver, &mut xinput_minor_ver)} != xlib::Success as c_int {
        panic!("XInput2 not available");
    }
    println!("XI version available {}.{}", xinput_major_ver, xinput_minor_ver);

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

    match unsafe{xinput2::XISelectEvents(demo_window.display,
      demo_window.window, &mut input_event_mask, 1)} {
        status if status as u8 == xlib::Success => (),
        err => panic!("Failed to select events {:?}", err)
    }

    // Show window
    demo_window.show();

    let mut prev_state = InputState{
        cursor_pos: (0.0, 0.0),
    };

    let mut event: xlib::XEvent = unsafe{zeroed()};
    let mut start_x: f64 = 0.0;
    let mut start_y: f64 = 0.0;

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
                        let event_data: &xinput2::XIDeviceEvent = unsafe{transmute(cookie.data)};
                        start_x = event_data.event_x;
                        start_y = event_data.event_y;
                    },
                    xinput2::XI_Motion => {
                        let event_data: &xinput2::XIDeviceEvent = unsafe{transmute(cookie.data)};
                        let mut xdiff = 0.0;
                        let mut ydiff = 0.0;

                        let mask = unsafe{from_raw_parts(event_data.buttons.mask, event_data.buttons.mask_len as usize)};
                        if xinput2::XIMaskIsSet(&mask, 1) {
                            xdiff = event_data.event_x - start_x;
                            ydiff = event_data.event_y - start_y;
                        }

                        unsafe {
                            xlib::XGetWindowAttributes(display, event_data.event, &mut attr);
                            xlib::XMoveResizeWindow(
                                display,
                                event_data.event,
                                attr.x + (xdiff as i32),
                                attr.y + (ydiff as i32),
                                attr.width as u32,
                                attr.height as u32
                            );
                        };
                    },
                    _ => ()
                }
                unsafe{xlib::XFreeEventData(display, &mut cookie)};
            },
            _ => ()
        }
    }
}
