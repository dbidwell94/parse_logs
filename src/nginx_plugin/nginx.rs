use anyhow::anyhow;
use chrono::{DateTime, Utc};
use http::{Method, StatusCode, Version};
use lazy_static::lazy_static;
use regex::Regex;
use std::net::IpAddr;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    pub static ref NGINX_LOG_REGEX: Regex = Regex::new(r#"^([\w\.:]+)\s-\s([-\w\.]+)\s\[(.*)\]\s"(\w+)\s([\w\./-]+)\s([\w/\.]+)"\s(\d+)\s(\d+)\s"(.*)"\s"(.*)"$"#).unwrap();
}

#[derive(Error, Debug)]
enum NginxError {
    #[error("Unable to parse log")]
    LogParseError,
}

#[derive(Debug)]
pub struct NginxLog {
    ip_address: IpAddr,
    date_time: DateTime<Utc>,
    path: String,
    status: StatusCode,
    http_method: Method,
    bytes_sent: usize,
    user_agent: String,
    http_version: String,
}

impl NginxLog {
    pub fn new(log: &str) -> anyhow::Result<Self> {
        if !NGINX_LOG_REGEX.is_match(log) {
            return Err(anyhow!(NginxError::LogParseError));
        }
        // 1
        let ip_address: IpAddr;
        // 3
        let date_time: DateTime<Utc>;
        // 4
        let http_method: Method;
        // 5
        let path: String;
        // 6
        let http_version: String;
        // 7
        let status: StatusCode;
        // 8
        let bytes_sent: usize;
        // 10
        let user_agent: String;

        if let Some(captures) = NGINX_LOG_REGEX.captures(log) {
            // 1
            {
                let addr_str = captures.get(1).unwrap().as_str();
                ip_address = IpAddr::from_str(addr_str).unwrap();
            }
            // 3
            {
                let date_str = captures.get(3).unwrap().as_str();

                date_time = DateTime::from(
                    DateTime::parse_from_str(date_str, "%d/%b/%Y:%H:%M:%S%.3f %z").unwrap(),
                );
            }
            // 4
            {
                let method_str = captures.get(4).unwrap().as_str();
                http_method = Method::from_str(method_str)?;
            }
            // 5
            path = captures.get(5).unwrap().as_str().to_string();
            // 6
            http_version = captures.get(6).unwrap().as_str().to_string();
            // 7
            {
                let status_str = captures.get(7).unwrap().as_str();
                status = StatusCode::from_str(status_str)?;
            }
            // 8
            {
                let bytes_str = captures.get(8).unwrap().as_str();
                bytes_sent = usize::from_str(bytes_str)?;
            }
            // 10
            user_agent = captures.get(10).unwrap().as_str().to_string();
        } else {
            return Err(anyhow!(NginxError::LogParseError));
        }
        Ok(Self {
            bytes_sent,
            status,
            http_version,
            http_method,
            date_time,
            ip_address,
            path,
            user_agent,
        })
    }
}
