use crate::window_manager::WindowManager;
use xcb;

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

    match &**command_t {
        "focus-workspace" => {
            let id: i64 = command.iter().nth(1).unwrap().parse().unwrap();
            wm.desktops.focus(id, conn)
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
            _ => word.to_string(),
        })
        .collect()
}
