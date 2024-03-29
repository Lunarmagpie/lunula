use crate::bsp::Bsp;
use crate::window::Window;
use std::collections::HashMap;
use std::sync;
use xcb::x;
use xcb::{self, ProtocolResult};

pub struct DesktopManager {
    pub focused_desktop_id: i64,
    pub desktops: HashMap<i64, VirtualDesktop>,
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

    pub fn add_window(&mut self, window: sync::Arc<Window>) -> Result<(), String> {
        let desktop = &mut self
            .desktops
            .get_mut(&self.focused_desktop_id)
            .ok_or("No virtual desktop is focused.")?;
        desktop.windows.push(window.clone());
        desktop.layout.insert(window);
        Ok(())
    }

    pub fn kill(&mut self, window: x::Window) -> Result<(), String> {
        let desktop = self
            .desktops
            .get_mut(&self.focused_desktop_id)
            .ok_or("No virtual desktop is focused.")?;
        desktop.windows = desktop
            .windows
            .drain_filter(|x| x.window != window)
            .collect();
        Ok(())
    }

    pub fn next(&self, focused: i64) -> i64 {
        let mut is_next = false;
        for desktop in self.desktops.keys() {
            if is_next {
                return *desktop;
            } else {
                is_next = &focused == desktop;
            }
        }
        *self
            .desktops
            .keys()
            .nth(0)
            .expect("There are no virtual desktops.")
    }

    pub fn prev(&self, focused: i64) -> i64 {
        let mut last = self
            .desktops
            .keys()
            .last()
            .expect("There are no virtual desktops.");
        for desktop in self.desktops.keys() {
            if desktop == &focused {
                return *last;
            } else {
                last = desktop;
            }
        }
        *self
            .desktops
            .keys()
            .last()
            .expect("There are no virtual desktops.")
    }

    pub fn focus(&mut self, id: i64, conn: &xcb::Connection) {
        self.desktops[&self.focused_desktop_id].hide(conn);
        self.desktops[&id].show(conn);
        self.focused_desktop_id = id
    }
}

pub struct VirtualDesktop {
    windows: Vec<sync::Arc<Window>>,
    pub layout: Bsp,
}

impl VirtualDesktop {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            layout: Bsp::new(),
        }
    }

    pub fn hide(&self, conn: &xcb::Connection) -> ProtocolResult<()> {
        let mut cookies = Vec::new();
        for window in &self.windows {
            cookies.push(conn.send_request_checked(&x::UnmapWindow {
                window: window.window,
            }));
        }

        cookies
            .into_iter()
            .map(|cookie| conn.check_request(cookie))
            .collect()
    }

    pub fn show(&self, conn: &xcb::Connection) -> ProtocolResult<()> {
        let mut cookies = Vec::new();
        for window in &self.windows {
            cookies.push(conn.send_request_checked(&x::MapWindow {
                window: window.window,
            }));
        }
        cookies
            .into_iter()
            .map(|cookie| conn.check_request(cookie))
            .collect()
    }
}
