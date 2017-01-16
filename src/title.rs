// use x11::xlib;
// use std::mem::{zeroed};
// use std::ptr::{null, null_mut};
// use libc::{c_int, c_uint, c_long, c_double};
// use std::time::Duration;
// use std::thread::sleep;
// use std::cmp;

// use window;

// fn mkcolor(display: *mut xlib::Display, color: &mut window::Color, name: char) {
//     let mut tc: xlib::XColor = unsafe{zeroed()};
//     let mut sc: xlib::XColor = unsafe{zeroed()};


//     unsafe {
//         let screen = xlib::XDefaultScreen(display);

//         xlib::XAllocNamedColor(display, xlib::XDefaultColormap(display, screen),
// 	    name, &mut sc, &mut tc);
//         color.normal = sc.pixel as i64;

//         scalecolor(&mut sc, &mut tc, -0.07);
//         xlib::XAllocColor(display, xlib::XDefaultColormap(display, screen), &mut sc);
//         color.shadow1 = sc.pixel as i64;

//         scalecolor(&mut sc, &mut tc, -0.25);
//         xlib::XAllocColor(display, xlib::XDefaultColormap(display, screen), &mut sc);
//         color.shadow2 = sc.pixel as i64;

//         scalecolor(&mut sc, &mut tc, 0.07);
//         xlib::XAllocColor(display, xlib::XDefaultColormap(display, screen), &mut sc);
//         color.bright1 = sc.pixel as i64;

//         scalecolor(&mut sc, &mut tc, 0.25);
//         xlib::XAllocColor(display, xlib::XDefaultColormap(display, screen), &mut sc);
// 	      color.bright2 = sc.pixel as i64;
//     }
// }

// fn scalecolor(rp: &mut xlib::XColor, cp: &mut xlib::XColor, d: c_double) {
//     rp.red = scalepixel(cp.red, d);
//     rp.green = scalepixel(cp.green, d);
//     rp.blue = scalepixel(cp.blue, d);
// }

// fn scalepixel(c: u16, d: c_double) -> u16 {
//     let r: c_double;
//     r = (c as f64) + 65535.0 * d;
//     r = r.min(65535.0);
//     r = r.max(0.0);
//     r as u16
// }
