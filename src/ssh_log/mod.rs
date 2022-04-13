extern crate chrono;
extern crate regex;
use chrono::prelude::*;
use regex::Regex;
use std::net::IpAddr;
use lazy_static::lazy_static;

const REGEX_STR: &'static str =
    r"^\w{3}\s\d{1,2}\s\d{2}:\d{2}:\d{2}\s[\w\d]+\s[\w\-_]+\[\d+\]:\s.*\n?$";
const DATE_REGEX_STR: &'static str = r"^\w{3}\s\d{1,2}\s\d{2}:\d{2}:\d{2}";
const HOSTNAME_REGEX_STR: &'static str = r"^\w{3}\s\d{1,2}\s\d{2}:\d{2}:\d{2}\s([\w\d]+)";
const LOG_ID_REGEX_STR: &'static str = r"\w+\[(\d+)\]";
const USERNAME_REGEX_STR: &'static str = r"user ([\w\d]+)";

lazy_static! {
    static ref LOG_REGEX: Regex = Regex::new(REGEX_STR).unwrap();
    static ref DATE_REGEX: Regex = Regex::new(DATE_REGEX_STR).unwrap();
    static ref HOSTNAME_REGEX: Regex = Regex::new(HOSTNAME_REGEX_STR).unwrap();
    static ref LOG_ID_REGEX: Regex = Regex::new(LOG_ID_REGEX_STR).unwrap();
    static ref USERNAME_REGEX: Regex = Regex::new(USERNAME_REGEX_STR).unwrap();
}


#[derive(Debug, PartialEq)]
pub enum SSHDLogError {
    LogParseError,
    TimeParseError,
    HostnameParseError,
    UsernameParseError,
    IdParseError,
    IpAddressParseError,
    PortParseError,
    Unknown,
}

fn parse_date(input: &str) -> Result<NaiveDateTime, SSHDLogError> {
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

fn parse_host_name(input: &str) -> Result<String, SSHDLogError> {
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

fn parse_log_id(input: &str) -> Result<i64, SSHDLogError> {
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

fn parse_username(input: &str) -> Result<Option<String>, SSHDLogError> {
return match USERNAME_REGEX.captures(input) {
        Some(v) => match v.get(1) {
            Some(val) => Ok(Some(val.as_str().to_owned())),
            None => Ok(None),
        },
        None => Ok(None),
    };
}

fn parse_ip_address(input: &str) -> Result<Option<IpAddr>, SSHDLogError> {
    Ok(None)
}

fn parse_port(input: &str) -> Result<Option<u16>, SSHDLogError> {
    Ok(None)
}

#[derive(PartialEq, Debug)]
pub struct SSHDLog {
    id: i64,
    log_timestamp: NaiveDateTime,
    host_name: String,
    username: Option<String>,
    remote_address: Option<IpAddr>,
    remote_port: Option<u16>,
}

impl SSHDLog {
    pub fn new(input: &str) -> Result<SSHDLog, SSHDLogError> {
        if !LOG_REGEX.is_match(input) {
            return Err(SSHDLogError::LogParseError);
        };

        return Ok(SSHDLog {
            log_timestamp: parse_date(input)?,
            host_name: parse_host_name(input)?,
            username: parse_username(input)?,
            id: parse_log_id(input)?,
            remote_address: parse_ip_address(input)?,
            remote_port: parse_port(input)?,
        });
    }

    pub fn get_timestamp(&self) -> &NaiveDateTime {
        return &self.log_timestamp;
    }

    pub fn get_id(&self) -> &i64 {
        return &self.id;
    }
}

#[cfg(test)]
mod sshd_tests {
    use super::*;
    use chrono::RoundingError::TimestampExceedsLimit;

    const TEST_TIME: &'static str = "Apr 13 13:40:35";
    const TEST_HOST: &'static str = "devinserver94";
    const TEST_USERNAME: &'static str = "april_fools";
    const TEST_ID: i64 = 8675309i64;

    fn create_log_str_with_username(args: [&str; 4]) -> String {
        format!(
            "{} {} sshd[{}]: Invalid user {} from 143.198.68.239 port 56720\n",
            args[0], args[1], args[2], args[3]
        )
    }

    #[test]
    fn test_struct_instantiation() {
        let log = SSHDLog::new(&create_log_str_with_username([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
        ]));

        assert_ne!(log, Err(SSHDLogError::LogParseError));
        assert_ne!(log, Err(SSHDLogError::TimeParseError));
        assert_ne!(log, Err(SSHDLogError::Unknown));
        assert_ne!(log, Err(SSHDLogError::HostnameParseError));
        assert_ne!(log, Err(SSHDLogError::IdParseError));
        assert_ne!(log, Err(SSHDLogError::UsernameParseError));
        assert_ne!(log, Err(SSHDLogError::IpAddressParseError));
        assert_ne!(log, Err(SSHDLogError::PortParseError));
    }

    #[test]
    fn test_hostname_populates_correctly() {
        let log = SSHDLog::new(&create_log_str_with_username([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
        ]))
        .unwrap();

        assert_eq!(log.host_name, TEST_HOST);
    }

    #[test]
    fn test_id_populates_correctly() {
        let log = SSHDLog::new(&create_log_str_with_username([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
        ]))
        .unwrap();

        assert_eq!(log.id, TEST_ID);
    }

    #[test]
    fn test_username_populates_correctly() {
        let log = SSHDLog::new(&create_log_str_with_username([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
        ]))
        .unwrap();

        assert_eq!(log.username, Some(TEST_USERNAME.to_owned()));
    }

    #[test]
    fn test_date_populates_correctly() {
        let log = SSHDLog::new(&create_log_str_with_username([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
        ]))
        .unwrap();
        let test_date_string = String::from("70 ") + TEST_TIME;

        assert_eq!(
            log.log_timestamp,
            NaiveDateTime::parse_from_str(&test_date_string, "%-y %b %d %X").unwrap()
        );
    }
}
