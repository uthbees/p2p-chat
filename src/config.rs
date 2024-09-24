use std::env;
use std::net::IpAddr;

pub struct Config {
    pub ip: IpAddr,
    pub port: u16,
}

impl Config {
    pub fn build(ip: &str, port: &str) -> Result<Config, &'static str> {
        let parsed_ip = IpAddr::from(match ip.chars().filter(|char| *char == '.').count() {
            3 => {
                let mut split_ip = ip.split('.');
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
                return match ip.chars().filter(|char| *char == ':').count() {
                    7 => Err("IPv6 is not supported"),
                    _ => Err("Invalid IP address"),
                }
            }
        });

        let Ok(parsed_port) = port.parse::<u16>() else {
            return Err("Invalid port");
        };

        Ok(Config {
            ip: parsed_ip,
            port: parsed_port,
        })
    }
}
