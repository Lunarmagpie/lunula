#![feature(get_mut_unchecked)]

mod bsp;
mod ipc;
mod utils;
mod virtual_desktop;
mod window;
mod window_manager;

use std::sync;
use std::thread;

mod config {
    use xcb::x;

    pub static MOD_KEY: x::ModMask = x::ModMask::N1; // Alt
                                                     // pub static MOD_KEY: x::ModMask = x::ModMask::N4; // Mod

    pub static DRAG_BUTTON: x::ButtonIndex = x::ButtonIndex::N1; // Left Mouse Button
    pub static DRAG_BUTTON_MASK: x::KeyButMask = x::KeyButMask::BUTTON1;

    pub static RESIZE_BUTTON: x::ButtonIndex = x::ButtonIndex::N3; // Right Mouse Button
    pub static RESIZE_BUTTON_MASK: x::KeyButMask = x::KeyButMask::BUTTON3;

    pub static BORDER_WIDTH: usize = 2;
}

use window_manager::WindowManager;

fn main() -> xcb::Result<()> {
    let wm = sync::Arc::new(sync::RwLock::new(WindowManager::new()?));

    let inner = wm.clone();
    thread::spawn(|| ipc::create_socket(inner));

    println!("Starting Lunula!");

    let k = wm.write();
    match k {
        Ok(mut wm_) => wm_.run(),
        Err(_) => panic!("Sussy!"),
    }
}
