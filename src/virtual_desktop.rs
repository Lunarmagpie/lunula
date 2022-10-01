use std::collections::HashMap;

use crate::bsp::Bsp;
use crate::window::Window;

pub struct DesktopManager {
    desktops: HashMap<i64, VirtualDesktop>,
}

impl DesktopManager {
    pub fn new() -> Self {
        Self {
            desktops: HashMap::new(),
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
                is_next = true;
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
}
