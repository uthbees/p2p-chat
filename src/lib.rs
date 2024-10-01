#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

pub mod config;
pub mod utils;

use crate::utils::non_blocking_read_line;
use config::Config;
use std::io::ErrorKind::WouldBlock;
use std::io::{prelude::*, BufReader};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub fn run(config: &Config) {
    let listener =
        TcpListener::bind(SocketAddr::new(config.ip, config.port)).expect("failed to bind to port");
    listener
        .set_nonblocking(true)
        .expect("failed to set listener as non-blocking");

    notify(NotificationType::WaitingForConnections(config));

    // Wait either for someone else to connect to us, or for us to connect to someone else.
    loop {
        let poll_result = listener.accept();

        match poll_result {
            Err(e) => {
                if e.kind() != WouldBlock {
                    println!("Error: {e:#?}");
                }
            }
            Ok(result) => handle_connection_as_host(result.0, config),
        }

        if let Some(line) = non_blocking_read_line() {
            if line == "exit" {
                // End the program.
                break;
            }

            match Config::parse_port(&line) {
                Err(e) => {
                    println!("Failed to parse port: {e}");
                    println!("Try again.");
                }
                Ok(port) => {
                    if let Ok(stream) = TcpStream::connect(SocketAddr::new(config.ip, port)) {
                        handle_connection_as_client(stream, config);
                    } else {
                        println!("Couldn't connect to port {port}.");
                        println!("Try again.");
                    }
                }
            }
        }
    }
}

fn handle_connection_as_host(stream: TcpStream, config: &Config) {
    notify(NotificationType::ConnectedAsHost);
    handle_connection(stream, config);
}

fn handle_connection_as_client(stream: TcpStream, config: &Config) {
    notify(NotificationType::ConnectedAsClient);
    handle_connection(stream, config);
}

fn handle_connection(mut stream: TcpStream, config: &Config) {
    // We set the listener as non-blocking earlier, but the streams need to be set as non-blocking separately.
    stream
        .set_nonblocking(true)
        .expect("failed to set stream as non-blocking");
    // Get another reference to the stream so we can both read and write.
    let stream_clone = stream
        .try_clone()
        .expect("should be able to clone stream reference");
    let mut stream_buffer_reader = BufReader::new(&stream_clone);
    let mut stream_buffer = [0; 8192];

    // Alternate between checking for received text and user input.
    loop {
        match stream_buffer_reader.read(&mut stream_buffer) {
            Err(err) => {
                // WouldBlock errors are expected. Other errors are not.
                if err.kind() != WouldBlock {
                    println!("Connection error: {err}");
                    send_disconnect_signal(&stream);
                    break;
                }
            }
            Ok(read_amount) if read_amount > 0 => {
                let message = core::str::from_utf8(&stream_buffer[..read_amount])
                    .expect("TCP message should be valid UTF-8");

                if message.contains('\0') {
                    // Disconnect.
                    println!("Peer disconnected.");
                    break;
                }

                println!("Received message: {message}");
            }
            _ => {}
        }

        if let Some(line) = non_blocking_read_line() {
            if line == "exit" {
                // Disconnect.
                send_disconnect_signal(&stream);
                break;
            }

            println!("Got input: {line}");

            if let Err(err) = stream.write_all(line.as_bytes()) {
                println!("Connection error: {err}");
                send_disconnect_signal(&stream);
                break;
            }
        }
    }
    notify(NotificationType::Disconnected(config));
}

/// Attempt to tell the peer that we're disconnecting by sending the null character. If that fails,
/// ignore the error since there's nothing we can do.
fn send_disconnect_signal(mut stream: &TcpStream) {
    const NULL_CHARACTER_IN_ARRAY: [u8; 1] = [b'\0'];
    let _ = stream.write_all(&NULL_CHARACTER_IN_ARRAY);
}

enum NotificationType<'a> {
    WaitingForConnections(&'a Config),
    ConnectedAsHost,
    ConnectedAsClient,
    Disconnected(&'a Config),
}

#[allow(clippy::needless_pass_by_value)]
fn notify(notification_type: NotificationType) {
    match notification_type {
        NotificationType::WaitingForConnections(config) => {
            println!(
                "Listening for connections on {}:{}...",
                config.ip, config.port
            );
            println!("To connect to another peer on localhost, enter their port.");
            println!("(External connections are not currently supported.)");
            println!("To exit the program, type \"exit\".");
        }
        NotificationType::ConnectedAsHost => {
            println!("Connected to peer as host!");
            println!("(No longer accepting connections - multiple simultaneous connections are not currently supported.)");
            println!("Type to send messages.");
            println!("To disconnect, type \"exit\".");
        }
        NotificationType::ConnectedAsClient => {
            println!("Connected to peer as client!");
            println!("(No longer accepting connections - multiple simultaneous connections are not currently supported.)");
            println!("Type to send messages.");
            println!("To disconnect, type \"exit\".");
        }
        NotificationType::Disconnected(config) => {
            println!("Disconnected.");
            notify(NotificationType::WaitingForConnections(config));
        }
    }
}
