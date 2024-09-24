use std::env;
use std::net::IpAddr;

pub struct Config {
    pub ip: IpAddr,
    pub port: u16,
}

impl Config {
    pub fn build_from_env_args() -> Result<Config, &'static str> {
        let mut env_args = env::args();
        if env_args.len() < 3 {
            return Err("Not enough arguments");
        }
        // Consume the first argument, which is the program name.
        env_args.next();

        let Some(ip_arg) = env_args.next() else {
            return Err("IP address not specified");
        };

        let parsed_ip = IpAddr::from(match ip_arg.chars().filter(|char| *char == '.').count() {
            3 => {
                let mut split_ip = ip_arg.split('.');
                let mut ip_segments = [0u8; 4];
                for elem in &mut ip_segments {
                    *elem = match split_ip
                        .next()
                        .expect("ip should have four segments")
                        .parse()
                    {
                        Ok(num) => num,
                        Err(_) => return Err("Invalid IP address"),
                    }
                }
                ip_segments
            }
            _ => {
                return match ip_arg.chars().filter(|char| *char == ':').count() {
                    7 => Err("IPv6 is not supported"),
                    _ => Err("Invalid IP address"),
                }
            }
        });

        let Some(port_arg) = env_args.next() else {
            return Err("Port not specified");
        };

        let Ok(parsed_port) = port_arg.parse::<u16>() else {
            return Err("Invalid port");
        };

        Ok(Config {
            ip: parsed_ip,
            port: parsed_port,
        })
    }
}
