use xcb::x;

// Window that is tracked by the window manager
pub struct Window {
    pub window: x::Window,
}

impl Window {
    pub fn new<'a>(window: x::Window) -> Self {
        Window { window }
    }

    pub fn map(&self, conn: &mut xcb::Connection, root: x::Window) -> xcb::Result<()> {
        let attr_cookie = conn.send_request_checked(&x::ChangeWindowAttributes {
            window: self.window,
            value_list: &[
                x::Cw::BorderPixel(0x0000ff),
                x::Cw::EventMask(
                    x::EventMask::SUBSTRUCTURE_NOTIFY | x::EventMask::SUBSTRUCTURE_REDIRECT,
                ),
            ],
        });

        let save_set_cookie = conn.send_request_checked(&x::ChangeSaveSet {
            mode: x::SetMode::Insert,
            window: self.window,
        });

        let reparent_cookie = conn.send_request_checked(&x::ReparentWindow {
            window: self.window,
            parent: root,
            x: 0,
            y: 0,
        });

        let map_cookie = conn.send_request_checked(&x::MapWindow {
            window: self.window,
        });

        conn.check_request(attr_cookie)?;
        conn.check_request(save_set_cookie)?;
        conn.check_request(reparent_cookie)?;
        conn.check_request(map_cookie)?;
        Ok(())
    }

    pub fn to_floating(&self, conn: &mut xcb::Connection) -> xcb::Result<()> {
        // Drag windows
        let drag_cookie = conn.send_request_checked(&x::GrabButton {
            owner_events: false,
            grab_window: self.window,
            event_mask: x::EventMask::BUTTON_PRESS
                | x::EventMask::BUTTON_RELEASE
                | x::EventMask::BUTTON_MOTION,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
            confine_to: xcb::Xid::none(),
            cursor: xcb::Xid::none(),
            button: crate::config::DRAG_BUTTON,
            modifiers: crate::config::MOD_KEY,
        });

        // Drag windows
        let resize_cookie = conn.send_request_checked(&x::GrabButton {
            owner_events: false,
            grab_window: self.window,
            event_mask: x::EventMask::BUTTON_PRESS
                | x::EventMask::BUTTON_RELEASE
                | x::EventMask::BUTTON_MOTION,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
            confine_to: xcb::Xid::none(),
            cursor: xcb::Xid::none(),
            button: crate::config::RESIZE_BUTTON,
            modifiers: crate::config::MOD_KEY,
        });

        conn.check_request(drag_cookie)?;
        conn.check_request(resize_cookie)?;

        Ok(())
    }

    pub fn to_tiled(&self, conn: xcb::Connection) -> xcb::Result<()> {
        let cookie = conn.send_request_checked(&x::UngrabButton {
            grab_window: self.window,
            button: x::ButtonIndex::N1,
            modifiers: x::ModMask::N1,
        });
        conn.check_request(cookie)?;
        Ok(())
    }
}
