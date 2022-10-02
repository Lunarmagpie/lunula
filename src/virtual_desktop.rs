use std::collections::HashMap;
use xcb;
use xcb::x;

use crate::bsp::Bsp;
use crate::window::Window;

pub struct DesktopManager {
    desktops: HashMap<i64, VirtualDesktop>,
    focused_desktop_id: i64,
}

impl DesktopManager {
    pub fn new() -> Self {
        Self {
            desktops: HashMap::new(),
            focused_desktop_id: 0,
        }
    }
    pub fn create_virtual_desktop(&mut self, id: i64) {
        self.desktops.insert(id, VirtualDesktop::new());
    }

    pub fn next(&self, focused: i64) -> Option<i64> {
        let mut is_next = false;
        for desktop in self.desktops.keys() {
            if is_next {
                return Some(*desktop);
            } else {
                is_next = &focused == desktop;
            }
        }
        return None;
    }

    pub fn prev(&self, focused: i64) -> Option<i64> {
        let mut last = self.desktops.keys().last()?;
        for desktop in self.desktops.keys() {
            if desktop == &focused {
                return Some(*last);
            } else {
                last = desktop;
            }
        }
        return None;
    }

    pub fn focus(&mut self, id: i64, conn: &xcb::Connection) {
        println!("{}", id);
        self.desktops[&id].hide(conn);
        self.focused_desktop_id = id
    }
}

pub struct VirtualDesktop {
    windows: Vec<Window>,
    layout: Bsp,
}

impl VirtualDesktop {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            layout: Bsp::new(),
        }
    }

    pub fn hide(&self, conn: &xcb::Connection) {
        println!("here");
        let mut cookies = Vec::new();
        for window in &self.windows {
            cookies.push(conn.send_request_checked(&x::UnmapWindow {
                window: window.window,
            }));
        }
        for cookie in cookies {
            conn.check_request(cookie).unwrap();
        }
    }

    pub fn show(&self, conn: &xcb::Connection) {
        let mut cookies = Vec::new();
        for window in &self.windows {
            cookies.push(conn.send_request_checked(&x::MapWindow {
                window: window.window,
            }));
        }
        for cookie in cookies {
            conn.check_request(cookie).unwrap();
        }
    }
}
