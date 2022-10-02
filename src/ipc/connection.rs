use std::fs;
use std::io::{BufReader, Read};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync;
use std::thread;

use crate::ipc::commands::handle_command;

use crate::window_manager::WindowManager;

fn handle_client(
    wm: sync::Arc<sync::RwLock<WindowManager>>,
    conn: &xcb::Connection,
    stream: UnixStream,
) {
    let stream = BufReader::new(stream);
    let wm = &mut *wm.write().unwrap();

    let mut buf = BufReader::new(stream);
    let mut command = String::new();
    match buf.read_to_string(&mut command) {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    };
    
    match handle_command(command.split_whitespace().collect(), wm, conn) {
        Ok(_) => (),
        Err(s) => println!("{}", s)
    };
}

pub fn create_socket(wm: sync::Arc<sync::RwLock<WindowManager>>, conn: sync::Arc<xcb::Connection>) {
    match fs::remove_file("/tmp/lunula-socket.socket") {
        _ => (), // I dont care if this fails.
    }

    let listener = match UnixListener::bind("/tmp/lunula-socket.socket") {
        Ok(sock) => sock,
        Err(e) => {
            println!("Couldn't connect: {e:?}");
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clone = wm.clone();
                let clone_conn = conn.clone();
                thread::spawn(move || handle_client(clone, &*clone_conn, stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
