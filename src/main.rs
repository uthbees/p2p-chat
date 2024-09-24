use p2p_chat::config::Config;
use p2p_chat::utils::print;
use std::{io, process};

fn main() {
    print("Enter port to listen on: ");

    let mut untrimmed_port = String::new();
    io::stdin()
        .read_line(&mut untrimmed_port)
        .expect("should have been able to read line");

    let config = Config::build("127.0.0.1", untrimmed_port.trim()).unwrap_or_else(|err| {
        println!("Invalid arguments: {err}");
        process::exit(1);
    });

    p2p_chat::run(&config);
}
