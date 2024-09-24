use p2p_chat::config::Config;
use std::{env, process};

fn main() {
    let config = build_config_from_env_args().unwrap_or_else(|err| {
        println!("Invalid arguments: {err}");
        process::exit(1);
    });

    p2p_chat::run(&config);
}

fn build_config_from_env_args() -> Result<Config, &'static str> {
    let mut env_args = env::args();
    if env_args.len() < 3 {
        return Err("Not enough arguments");
    }
    // Consume the first argument, which is the program name.
    env_args.next();

    let Some(ip_arg) = env_args.next() else {
        return Err("IP address not specified");
    };
    let Some(port_arg) = env_args.next() else {
        return Err("Port not specified");
    };

    Config::build(&ip_arg, &port_arg)
}
