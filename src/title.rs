use x11::xlib;
use std::mem::{zeroed};
use std::ptr::{null, null_mut};
use libc::{c_int, c_uint, c_long, c_double};
use std::time::Duration;
use std::thread::sleep;

use window;

fn mkcolor(display: *mut xlib::Display, color: &mut window::Color) {
    let mut tc: xlib::XColor = unsafe{zeroed()};
    let mut sc: xlib::XColor = unsafe{zeroed()};

    color.normal = sc.pixel as i64;

    unsafe {
        let screen = xlib::XDefaultScreen(display);

        scalecolor(&mut sc, &mut tc, -0.07);
        xlib::XAllocColor(display, xlib::XDefaultColormap(display, screen), &mut sc);
        color.shadow1 = sc.pixel as i64;

        scalecolor(&mut sc, &mut tc, -0.25);
        xlib::XAllocColor(display, xlib::XDefaultColormap(display, screen), &mut sc);
        color.shadow2 = sc.pixel as i64;

        scalecolor(&mut sc, &mut tc, 0.07);
        xlib::XAllocColor(display, xlib::XDefaultColormap(display, screen), &mut sc);
        color.bright1 = sc.pixel as i64;

        scalecolor(&mut sc, &mut tc, 0.25);
        xlib::XAllocColor(display, xlib::XDefaultColormap(display, screen), &mut sc);
	      color.bright2 = sc.pixel as i64;
    }
}

fn scalecolor(rp: *mut xlib::XColor, cp: *mut xlib::XColor, d: c_double) {
    
}
