use x11::xlib;
use std::mem::{zeroed};
use std::ptr::{null, null_mut};
use libc::{c_int, c_uint, c_long};
use std::time::Duration;
use std::thread::sleep;

use parent;
use window;
use title;

pub struct Altmove {
    pub moving: bool,
    pub xoff: i32,
    pub yoff: i32,
}

pub struct Color {
    pub bright2: c_long,
    pub bright1: c_long,
    pub normal: c_long,
    pub shadow1: c_long,
    pub shadow2: c_long,
}

pub struct Title {
    pub gc: xlib::GC,
    pub xoff: i32,
    pub yoff: i32,
    pub fg: Color,
    pub bg: Color,
    pub pixmap: xlib::Pixmap,
    pub pixmapwidth: c_int,
    pub pixmapheight: c_int,
    pub xwindow: xlib::Window,
}

#[derive(Clone, Debug)]
pub struct Dim {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
pub struct Window {
    pub title: Title,
    pub client: xlib::Window,
    pub parent: parent::Parent,
    pub altmove: Altmove,
    pub active: bool,
    pub maximized: bool,
    pub odim: Dim,
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

pub fn manage(display: *mut xlib::Display, root: u64, client: xlib::Window, wmstart: i32, windows: &mut Vec<Window>) {
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
    win.altmove.moving = false;
    win.altmove.xoff = 0;
    win.altmove.yoff = 0;

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

        // Setup title
        let mut gcval: xlib::XGCValues = unsafe{zeroed()};
        let screen = xlib::XDefaultScreen(display);
        let mut tp: Title = unsafe{zeroed()};
        let mut title_attr: xlib::XSetWindowAttributes = unsafe{zeroed()};
        tp.xwindow = xlib::XCreateWindow(display, win.parent.xwindow, 4, 4,
                                         (attr.width - 2 * 4) as u32, 12, 0,
                                         xlib::CopyFromParent as c_int,
                                         xlib::InputOutput as u32, null_mut(),
                                         xlib::CWOverrideRedirect, &mut title_attr);
        tp.pixmapwidth = attr.width;
        tp.pixmapheight = attr.height;
        tp.pixmap = xlib::XCreatePixmap(display, win.parent.xwindow, tp.pixmapwidth as u32, tp.pixmapheight as u32, xlib::XDefaultDepth(display, screen) as u32);
        gcval.graphics_exposures = false as c_int;
        tp.gc = xlib::XCreateGC(display, win.parent.xwindow, xlib::GCGraphicsExposures as u64, &mut gcval);
        xlib::XSelectInput(display, win.parent.xwindow, xlib::ButtonPressMask | xlib::ButtonMotionMask | xlib::ButtonReleaseMask | xlib::ExposureMask);
        xlib::XMapWindow(display, tp.xwindow);
        win.title = tp;

        // Reparent window
        xlib::XSelectInput(display, win.parent.xwindow, xlib::ExposureMask |
                           xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask);

        xlib::XAddToSaveSet(display, client);
        xlib::XSetWindowBorderWidth(display, client, 0);
        xlib::XReparentWindow(display, client, win.parent.xwindow, 4, 12);
        xlib::XLowerWindow(display, client);
        xlib::XSelectInput(display, client, xlib::PropertyChangeMask);
        xlib::XMapWindow(display, client);
        xlib::XMapWindow(display, win.parent.xwindow);
        xlib::XSetInputFocus(display, client, xlib::RevertToPointerRoot, xlib::CurrentTime);
        xlib::XSync(display, 0);
    }

    // paint title
    let gc: xlib::GC = win.title.gc;
    

    // paint etc
    // unsafe {
    //     // draw raised
    //     xlib::XSetForeground(display, gc, 24);
    //     xlib::XDrawLine(display, win.parent.xwindow, gc, 0, 0, 3, 3);
        
    //     xlib::XSetForeground(display, gc, 24);
    //     xlib::XFillRectangle(display, win.parent.xwindow, gc, 1, 1, (attr.width - 3) as u32, 1);
    //     xlib::XFillRectangle(display, win.parent.xwindow, gc, 1, attr.height - 3, (attr.width - 3) as u32, 1);
    //     xlib::XFillRectangle(display, win.parent.xwindow, gc, 1, 2, 1, (attr.height - 2 * 7) as u32);
    //     xlib::XFillRectangle(display, win.parent.xwindow, gc, (attr.width - 5), 2, 1, (attr.height - 2 * 7) as u32);
    // }
    
    for win in windows.iter_mut() {
        win.active = false;
    }
    win.active = true;
    windows.push(win);
}

pub fn windowevent(display: *mut xlib::Display, e: xlib::XEvent, win: &mut Window) {
    match e.get_type() {
        xlib::ButtonPress => {
            unsafe {
                xlib::XRaiseWindow(display, win.parent.xwindow);
                xlib::XSetInputFocus(display, win.client, xlib::RevertToPointerRoot, xlib::CurrentTime);
                let xbutton: xlib::XButtonEvent = From::from(e);
                if xbutton.state & xlib::Mod1Mask != 0 {
                    win.altmove.xoff = xbutton.x;
                    win.altmove.yoff = xbutton.y;
                    win.altmove.moving = true;
                }
            }
        },
        xlib::MotionNotify => {
            if win.altmove.moving {
                unsafe {
                    let xmotion: xlib::XMotionEvent = From::from(e);
                    win.parent.dim.x = xmotion.x_root;
                    win.parent.dim.y = xmotion.y_root;
                    xlib::XMoveWindow(display, win.parent.xwindow,
                                      xmotion.x_root - win.altmove.xoff,
                                      xmotion.y_root - win.altmove.yoff);
                }
            }
        },
        xlib::ButtonRelease => {
            unsafe {
                win.altmove.moving = false;
            }
        }
        _ => ()
    }
}

pub fn maximize(display: *mut xlib::Display, win: &mut Window) {
    if win.maximized {
        unsafe {
            xlib::XMoveResizeWindow(display, win.parent.xwindow, win.odim.x as i32, win.odim.y as i32, win.odim.width as u32, win.odim.height as u32);
            xlib::XMoveResizeWindow(display, win.client, win.odim.x as i32, win.odim.y as i32, win.odim.width as u32, win.odim.height as u32);
        }
        win.maximized = false;
    } else {
        win.odim = win.parent.dim.clone();
        unsafe {
            let screen = xlib::XDefaultScreen(display);
            xlib::XMoveResizeWindow(display, win.parent.xwindow, 0, 0, xlib::XDisplayWidth(display, screen) as u32, xlib::XDisplayHeight(display, screen) as u32);
            xlib::XMoveResizeWindow(display, win.client, 0, 0, xlib::XDisplayWidth(display, screen) as u32, xlib::XDisplayHeight(display, screen) as u32);
        }
        win.maximized = true;
    }
}
