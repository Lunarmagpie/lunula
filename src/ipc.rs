use std::{fs, cell, rc};
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;
use std::sync;

use crate::window_manager::WindowManager;

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{}", line.unwrap());
    }
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
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
