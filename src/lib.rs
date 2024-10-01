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

    let stdin = stdin();
    let mut user_input_buffer = String::new();

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

        // TODO: Fix read_line blocking with crossterm's poll and read
        let read_bytes = stdin
            .read_line(&mut user_input_buffer)
            .expect("encountered error reading user input");
        if read_bytes > 0 {
            let trimmed_input = user_input_buffer.trim();

            if trimmed_input == "exit" {
                // End the program.
                break;
            }

            match Config::parse_port(user_input_buffer.trim()) {
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
            user_input_buffer.clear();
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
    let mut stream_buffer_reader = BufReader::new(&mut stream);
    let stream_buffer = &mut vec![];
    let mut user_input_buffer = String::new();
    let stdin = stdin();

    // Alternate checking for received text or user input.
    loop {
        if let Err(err) = stream_buffer_reader.read_to_end(stream_buffer) {
            // WouldBlock errors are expected. Other errors are not.
            assert_eq!(
                err.kind(),
                WouldBlock,
                "encountered error reading TCP stream: {err}"
            );
        }
        println!("Received : {stream_buffer:#?}");
        // TODO: Break out of the loop once the other side has disconnected.

        let read_bytes = stdin
            .read_line(&mut user_input_buffer)
            .expect("encountered error reading user input");
        if read_bytes > 0 {
            let trimmed_input = user_input_buffer.trim();

            if trimmed_input == "exit" {
                // Disconnect.
                break;
            }

            println!("Got input: {trimmed_input}");
            // TODO: Send text.
        }
    }
    notify(NotificationType::Disconnected(config));
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
            println!("(Multiple simultaneous connections are not currently supported.)");
            println!("Type to send messages.");
            println!("To disconnect, type \"exit\".");
        }
        NotificationType::ConnectedAsClient => {
            println!("Connected to peer as client!");
            println!("(Multiple simultaneous connections are not currently supported.)");
            println!("Type to send messages.");
            println!("To disconnect, type \"exit\".");
        }
        NotificationType::Disconnected(config) => {
            println!("Disconnected.");
            notify(NotificationType::WaitingForConnections(config));
        }
    }
}
