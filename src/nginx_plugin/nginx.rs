use lazy_static::lazy_static;
use regex::Regex;
use std::net::IpAddr;

lazy_static! {
    pub static ref NGINX_LOG_REGEX: Regex = Regex::new(r#"^([\w\.:]+)\s-\s([-\w\.]+)\s\[(.*)\]\s"(\w+)\s([\w\./-]+)\s([\w/\.]+)"\s(\d+)\s(\d+)\s"(.*)"\s"(.*)"$"#).unwrap();
}

struct NginxLog {
    ip_address: IpAddr,
    time: String,
    path: String,
    status: u8,
    bytes_sent: usize,
    user_agent: String,
}
