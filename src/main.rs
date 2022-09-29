mod bsp;
mod window;
mod window_manager;
mod utils;

use window_manager::WindowManager;

// Many xcb functions return a `xcb::Result` or compatible result.
fn main() -> xcb::Result<()> {
    let mut wm = WindowManager::new()?;

    println!("Starting Lunula!");

    wm.run()
}
