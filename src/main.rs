#![feature(get_mut_unchecked)]
#![feature(drain_filter)]

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
    let (wm, conn) = WindowManager::new()?;

    let wm = sync::Arc::new(sync::RwLock::new(wm));
    let conn = sync::Arc::new(conn);

    let inner = wm.clone();
    let clone_conn = conn.clone();
    thread::spawn(|| ipc::create_socket(inner, clone_conn));

    println!("Starting Lunula!");

    window_manager::run(wm, &*conn)
}
