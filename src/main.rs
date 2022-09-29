mod bsp;
mod utils;
mod window;
mod window_manager;

mod config {
    use xcb::x;

    pub static MOD_KEY: x::ModMask = x::ModMask::N1; // Alt

    pub static DRAG_BUTTON: x::ButtonIndex = x::ButtonIndex::N1; // Left Mouse Button
    pub static DRAG_BUTTON_MASK: x::KeyButMask = x::KeyButMask::BUTTON1;

    pub static RESIZE_BUTTON: x::ButtonIndex = x::ButtonIndex::N3; // Right Mouse Button
    pub static RESIZE_BUTTON_MASK: x::KeyButMask = x::KeyButMask::BUTTON3;

}

use window_manager::WindowManager;

// Many xcb functions return a `xcb::Result` or compatible result.
fn main() -> xcb::Result<()> {
    let mut wm = WindowManager::new()?;

    println!("Starting Lunula!");

    wm.run()
}
