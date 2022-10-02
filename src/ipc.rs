use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync;
use std::thread;

use crate::window_manager::WindowManager;

fn handle_client(wm: sync::Arc<sync::RwLock<WindowManager>>, stream: UnixStream) {
    let stream = BufReader::new(stream);

    for line in stream.lines() {
        println!("{}", line.unwrap());
    }

    println!("here1");
    let wm = &mut *wm.write().unwrap();
    println!("here2");

    let desktops = &mut wm.desktops;
    let conn = &wm.conn;
    println!("here3");

    let next = desktops.next(0).expect("Could not unwrap.");
    println!("here4");
    desktops.focus(next, conn);
}

pub fn create_socket(wm: sync::Arc<sync::RwLock<WindowManager>>) {
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
                thread::spawn(move || handle_client(clone, stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
