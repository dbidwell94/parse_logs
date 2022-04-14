use super::consts::*;
use super::enums::SSHDLogError;
use chrono::prelude::*;
use regex::Captures;
use std::net::IpAddr;

pub fn parse_date(input: &str) -> Result<NaiveDateTime, SSHDLogError> {
    let date_match = match DATE_REGEX.find(input) {
        Some(v) => v.as_str(),
        None => return Err(SSHDLogError::TimeParseError),
    };

    let date_with_fake_year = String::from("70 ") + date_match;

    match NaiveDateTime::parse_from_str(&date_with_fake_year, "%-y %b %d %X") {
        Ok(v) => Ok(v),
        Err(_) => Err(SSHDLogError::TimeParseError),
    }
}

pub fn parse_host_name(input: &str) -> Result<String, SSHDLogError> {
    Ok(match HOSTNAME_REGEX.captures(input) {
        Some(v) => match v.get(1) {
            Some(val) => val.as_str(),
            None => {
                return Err(SSHDLogError::HostnameParseError);
            }
        },
        None => {
            return Err(SSHDLogError::HostnameParseError);
        }
    }
    .to_string())
}

pub fn parse_log_id(input: &str) -> Result<i64, SSHDLogError> {
    let res = match LOG_ID_REGEX.captures(input) {
        Some(v) => match v.get(1) {
            Some(val) => match val.as_str().parse::<i64>() {
                Ok(res) => res,
                Err(_) => return Err(SSHDLogError::IdParseError),
            },
            None => {
                return Err(SSHDLogError::IdParseError);
            }
        },
        None => {
            return Err(SSHDLogError::IdParseError);
        }
    };
    return Ok(res);
}

pub fn parse_username(input: &str) -> Result<Option<String>, SSHDLogError> {
    return match USERNAME_REGEX.captures(input) {
        Some(v) => match v.get(1) {
            Some(val) => Ok(Some(val.as_str().to_owned())),
            None => Ok(None),
        },
        None => Ok(None),
    };
}

pub fn parse_ip_address(input: &str) -> Result<Option<IpAddr>, SSHDLogError> {
    fn get_capture(cap: Option<Captures>) -> Result<Option<IpAddr>, SSHDLogError> {
        return match cap {
            Some(v) => {
                let ip_str = v.get(1).ok_or_else(|| SSHDLogError::IpParseError)?.as_str();
                let addr = ip_str
                    .parse::<IpAddr>()
                    .or_else(|_| Err(SSHDLogError::IpParseError))?;
                Ok(Some(addr))
            }
            _ => Ok(None),
        };
    }

    let mut cap = get_capture(IPV4_REGEX.captures(input))?;
    if cap.is_none() {
        cap = get_capture(IPV6_REGEX.captures(input))?;
    }

    Ok(cap)
}

pub fn parse_port(input: &str) -> Result<Option<u16>, SSHDLogError> {
    return match PORT_REGEX.captures(input) {
        Some(v) => match v.get(1) {
            Some(port) => {
                let port_str = port.as_str();
                let port_u16 = port_str
                    .parse::<u16>()
                    .or_else(|_| Err(SSHDLogError::PortParseError))?;
                Ok(Some(port_u16))
            }
            None => Err(SSHDLogError::PortParseError),
        },
        None => Ok(None),
    };
}
