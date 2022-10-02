use xcb;
use xcb::{x, Xid};

use crate::utils::Vec2D;
use crate::virtual_desktop::DesktopManager;
use crate::window;
use std::sync;

pub struct WindowManager {
    pub desktops: DesktopManager,
    root: x::Window,

    drag_start_pos: Vec2D,
    drag_start_frame_pos: Vec2D,
}

impl WindowManager {
    pub fn new() -> xcb::Result<(Self, xcb::Connection)> {
        // Connect to the X server.
        let (conn, screen_num) = xcb::Connection::connect(None)?;

        let setup = conn.get_setup();
        let root = setup.roots().nth(screen_num as usize).unwrap().root();

        let cookie = conn.send_request_checked(&x::ChangeWindowAttributes {
            window: root,

            value_list: &[
                x::Cw::EventMask(
                    x::EventMask::SUBSTRUCTURE_NOTIFY | x::EventMask::SUBSTRUCTURE_REDIRECT,
                ),
                x::Cw::Cursor(Xid::none()),
            ],
        });

        conn.check_request(cookie)
            .expect("Could not start lunula window manager! Is one already running?");

        let mut desktops = DesktopManager::new();
        desktops.create_virtual_desktop(0);
        desktops.create_virtual_desktop(1);

        Ok((
            WindowManager {
                root,
                desktops: desktops,
                drag_start_pos: Vec2D::new(0, 0),
                drag_start_frame_pos: Vec2D::new(0, 0),
            },
            conn,
        ))
    }
}

pub fn run(wm: sync::Arc<sync::RwLock<WindowManager>>, conn: &xcb::Connection) -> xcb::Result<()> {
    loop {
        let event = conn.wait_for_event()?;
        let mut wm = wm.write().unwrap();
        match event {
            xcb::Event::X(x::Event::ButtonPress(ev)) => {
                let cookie = conn.send_request(&x::GetGeometry {
                    drawable: x::Drawable::Window(ev.event()),
                });

                let resp = conn.wait_for_reply(cookie)?;

                wm.drag_start_pos = Vec2D::new(ev.root_x(), ev.root_y());
                wm.drag_start_frame_pos = Vec2D::new(resp.x(), resp.y());
            }
            xcb::Event::X(x::Event::ConfigureRequest(ev)) => {
                let cookie = conn.send_request_checked(&x::ConfigureWindow {
                    window: ev.window(),
                    value_list: &[
                        x::ConfigWindow::X(ev.x() as i32),
                        x::ConfigWindow::Y(ev.y() as i32),
                        x::ConfigWindow::Width(ev.width() as u32),
                        x::ConfigWindow::Height(ev.height() as u32),
                        x::ConfigWindow::BorderWidth(crate::config::BORDER_WIDTH as u32),
                        x::ConfigWindow::StackMode(ev.stack_mode()),
                    ],
                });
                conn.check_request(cookie)?;
            }
            xcb::Event::X(x::Event::MotionNotify(ev)) => {
                let mouse_pos = Vec2D::new(ev.root_x(), ev.root_y());

                if ev.state().contains(crate::config::DRAG_BUTTON_MASK) {
                    let window_pos = wm.drag_start_frame_pos + mouse_pos - wm.drag_start_pos;

                    let cookie = conn.send_request_checked(&x::ConfigureWindow {
                        window: ev.event(),
                        value_list: &[
                            x::ConfigWindow::X(window_pos.x as i32),
                            x::ConfigWindow::Y(window_pos.y as i32),
                        ],
                    });
                    conn.check_request(cookie)?;
                } else if ev.state().contains(crate::config::RESIZE_BUTTON_MASK) {
                    let size = (mouse_pos - wm.drag_start_frame_pos).max(Vec2D::new(32, 32));

                    let cookie = conn.send_request_checked(&x::ConfigureWindow {
                        window: ev.event(),
                        value_list: &[
                            x::ConfigWindow::Width(size.x as u32),
                            x::ConfigWindow::Height(size.y as u32),
                        ],
                    });
                    conn.check_request(cookie)?;
                }
            }
            xcb::Event::X(x::Event::MapRequest(ev)) => {
                let w = window::Window::new(ev.window());
                w.map(&conn, wm.root)?;
                w.to_floating(&conn)?;
                wm.desktops.add_window(w);
            }
            xcb::Event::X(x::Event::DestroyNotify(ev)) => wm.desktops.kill(ev.window()),
            _ => {}
        }
    }
}
