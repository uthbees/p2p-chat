use p2p_chat::config::Config;
use std::process;

fn main() {
    let config = Config::build_from_env_args().unwrap_or_else(|err| {
        println!("Invalid arguments: {err}");
        process::exit(1);
    });

    p2p_chat::run(&config);
}
