use std::net::IpAddr;

pub struct Config {
    pub ip: IpAddr,
    pub port: u16,
}

impl Config {
    pub fn build(ip: &str, port: &str) -> Result<Self, &'static str> {
        Ok(Self {
            ip: Self::parse_ip(ip)?,
            port: Self::parse_port(port)?,
        })
    }

    pub fn parse_ip(ip: &str) -> Result<IpAddr, &'static str> {
        Ok(IpAddr::from(
            match ip.chars().filter(|char| *char == '.').count() {
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
            },
        ))
    }

    pub fn parse_port(port: &str) -> Result<u16, &'static str> {
        port.parse::<u16>().map_err(|_| "Invalid port")
    }
}
