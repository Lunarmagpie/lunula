#![feature(get_mut_unchecked)]
#![feature(drain_filter)]

mod bsp;
mod ipc;
mod utils;
mod virtual_desktop;
mod window;
mod window_manager;

use expanduser;
use log_panics;
use simple_logging;
use std::process;
use std::sync;
use std::thread;

mod config {
    use xcb::x;

    // pub static MOD_KEY: x::ModMask = x::ModMask::N1; // Alt
    pub static MOD_KEY: x::ModMask = x::ModMask::N4; // Mod
    pub static MOD_KEY_BUT: x::KeyButMask = x::KeyButMask::MOD4;

    pub static DRAG_BUTTON: x::ButtonIndex = x::ButtonIndex::N1; // Left Mouse Button
    pub static DRAG_BUTTON_MASK: x::KeyButMask = x::KeyButMask::BUTTON1;

    pub static SELECT_BUTTON: x::ButtonIndex = x::ButtonIndex::N1; // Left Mouse Button

    pub static RESIZE_BUTTON: x::ButtonIndex = x::ButtonIndex::N3; // Right Mouse Button
    pub static RESIZE_BUTTON_MASK: x::KeyButMask = x::KeyButMask::BUTTON3;

    pub static BORDER_WIDTH: usize = 2;

    pub static BORDER_COLOR: u32 = 0xcccccc;
    pub static BORDER_COLOR_FOCUS: u32 = 0x00ccff;
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

    process::Command::new(
        expanduser::expanduser("~/.config/lunularc").expect("Could not find user home directory."),
    )
    .spawn()
    .expect("failed to execute process");

    log_panics::init();
    simple_logging::log_to(std::io::stdout(), log::LevelFilter::max());

    loop {
        match window_manager::run(wm.clone(), &*conn) {
            Ok(()) => (),
            Err(err) => log::error!("{}", err.to_string()),
        };
    }
}
