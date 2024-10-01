use p2p_chat::config::Config;
use p2p_chat::utils::{print, read_line};
use std::process;

fn main() {
    print("Enter port to listen on: ");

    let port = read_line();

    let config = Config::build("127.0.0.1", &port).unwrap_or_else(|err| {
        println!("Invalid arguments: {err}");
        process::exit(1);
    });

    p2p_chat::run(&config);
}
