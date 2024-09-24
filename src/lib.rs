#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

pub mod config;
pub mod utils;

use config::Config;
use std::io::ErrorKind::WouldBlock;
use std::io::{prelude::*, stdin, BufReader};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub fn run(config: &Config) {
    let listener =
        TcpListener::bind(SocketAddr::new(config.ip, config.port)).expect("failed to bind to port");
    listener
        .set_nonblocking(true)
        .expect("failed to set listener as non-blocking");

    println!(
        "Listening for connections on {}:{}...",
        config.ip, config.port
    );
    println!("To connect to another peer on localhost, enter their port.");
    println!("(External connections are not currently supported.)");

    let stdin = stdin();
    let mut user_input_buffer = String::new();

    // Wait either for someone else to connect to us, or for us to connect to someone else.
    loop {
        let poll_result = listener.accept();

        match poll_result {
            Err(e) => {
                if e.kind() != WouldBlock {
                    println!("Error: {e:#?}");
                }
            }
            Ok(result) => handle_connection(result.0),
        }

        let read_bytes = stdin
            .read_line(&mut user_input_buffer)
            .expect("encountered error reading user input");
        if read_bytes > 0 {
            match Config::parse_port(user_input_buffer.trim()) {
                Err(e) => {
                    println!("Failed to parse port: {e}");
                }
                Ok(port) => {
                    // TODO: Attempt to set up a connection on that port.
                    // Make sure to notify that we won't be listening for connections anymore, and
                    // make sure to return to listening (and notify of that) when this connection ends.
                }
            }
            user_input_buffer.clear();
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connected to peer as host!");

    let mut buf_reader = BufReader::new(&mut stream);
    let buf = &mut vec![];
    buf_reader
        .read_to_end(buf)
        .expect("encountered error reading TCP stream");
    println!("Received : {buf:#?}");
    // TODO: Allow sending text. (This also needs to be done when connected as client.)
}
