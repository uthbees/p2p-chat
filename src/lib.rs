#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

pub mod config;

use config::Config;
use std::net::{SocketAddr, TcpListener};

pub fn run(config: &Config) {
    let listener =
        TcpListener::bind(SocketAddr::new(config.ip, config.port)).expect("failed to bind to port");

    for stream in listener.incoming() {
        match stream {
            Ok(_) => println!("Connection established!"),
            Err(e) => println!("Connection failed: {e}"),
        }
    }
}
