use std::collections::BTreeSet;

use crate::window::Window;

pub struct Bsp {
    map: BTreeSet<Window>,
}

impl Bsp {
    pub fn new() -> Self {
        Self {
            map: BTreeSet::new(),
        }
    }
    pub fn insert(&mut self, window: Window) {
        
    }
}
