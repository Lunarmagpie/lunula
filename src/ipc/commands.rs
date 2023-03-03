use crate::window_manager::WindowManager;
use xcb;
use xcb::{x, Xid};

pub fn handle_command(
    command: Vec<&str>,
    wm: &mut WindowManager,
    conn: &xcb::Connection,
) -> Result<(), String> {
    let command = replace_vars(command, wm);

    let command_t = match command.iter().nth(0) {
        Some(s) => s,
        None => return Err("Could not find argument type.".to_string()),
    };

    match command_t.as_str() {
        "focus-workspace" => {
            let id: i64 = match command.iter().nth(1) {
                Some(number) => match number.parse() {
                    Ok(n) => {
                        wm.unfocus(conn).or(Err("Window could not be unfocused".to_string()))?;
                        n
                    }
                    Err(e) => return Err(e.to_string()),
                },
                None => return Err("Not enough arguments provided".to_string()),
            };
            wm.desktops.focus(id, conn)
        }
        "kill-window" => {
            if let Some(window) = wm.focused_window {
                if window.resource_id() == wm.root.resource_id() {
                    return Err("Could not close window because no window is selected.".to_string());
                }

                let cookie = conn.send_request_checked(&x::DestroyWindow { window });
                wm.unfocus(conn).or(Err("Window could not be unfocused".to_string()))?;
                conn.check_request(cookie).or(Err("Window could not be destroyed".to_string()))?;
            }
        }
        "query" => {
            println!("{}", wm.desktops.focused_desktop_id)
        }

        _ => return Err(format!("{} is not a command", command_t)),
    }

    Ok(())
}

pub fn replace_vars(command: Vec<&str>, wm: &mut WindowManager) -> Vec<String> {
    command
        .into_iter()
        .map(|word| match word {
            "&workspace-left" => format!("{}", wm.desktops.prev(wm.desktops.focused_desktop_id)),
            "&workspace-right" => format!("{}", wm.desktops.next(wm.desktops.focused_desktop_id)),
            "&selected-window" => format!("{}", wm.focused_window.map_or(0, |w| w.resource_id())),
            _ => word.to_string(),
        })
        .collect()
}
