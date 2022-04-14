extern crate chrono;
extern crate regex;
mod consts;
mod enums;
mod utils;

use chrono::prelude::*;
use consts::*;
pub use enums::{SSHDLogError, SSHDLogType};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use utils::{
    parse_date, parse_host_name, parse_ip_address, parse_log_id, parse_port, parse_username,
};

impl SSHDLog {
    /// Instantiate a new instance of an SSHDLog struct
    /// # Arguments
    /// * `input` - A string ending in a \n
    /// # Examples
    /// ```
    /// use parse_logs::SSHDLog;
    /// let input_str = "Apr 11 14:11:00 someserver sshd[2567574]: Received disconnect from 192.168.1.1 port 36614:11: Bye Bye [preauth]";
    /// let log = SSHDLog::new(input_str);
    /// assert_ne!(log, Err(SSHDLogError));
    /// ```
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
            log_type: SSHDLogType::ConnectionClosed,
        });
    }

    /// Gets the NaiveDateTime timestamp for this struct
    ///
    /// # Returns
    /// * `&chrono::NaiveDateTime`
    pub fn get_timestamp(&self) -> &NaiveDateTime {
        return &self.log_timestamp;
    }

    /// Gets the log ID for this struct
    ///
    /// # Returns
    /// * `&std::i64`
    pub fn get_id(&self) -> &i64 {
        return &self.id;
    }

    pub fn get_ip_addr(&self) -> Option<IpAddr> {
        return self.remote_address.to_owned();
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
/// A struct representation of an SSHD log line
pub struct SSHDLog {
    id: i64,
    log_timestamp: NaiveDateTime,
    host_name: String,
    username: Option<String>,
    remote_address: Option<IpAddr>,
    remote_port: Option<u16>,
    log_type: SSHDLogType,
}

#[cfg(test)]
mod sshd_tests {
    use super::*;

    const TEST_TIME: &'static str = "Apr 13 13:40:35";
    const TEST_HOST: &'static str = "devinserver94";
    const TEST_USERNAME: &'static str = "april_fools";
    const TEST_ID: i64 = 8675309i64;
    const TEST_IPV4_STR: &'static str = "192.168.1.1";
    const TEST_IPV6_STR: &'static str = "2601:600:c87f:8a67:42b0:76ff:fedd:44e";
    const TEST_PORT: &'static str = "8675";

    fn create_log(args: [&str; 6]) -> String {
        format!(
            "{} {} sshd[{}]: Invalid user {} from {} port {}\n",
            args[0], args[1], args[2], args[3], args[4], args[5]
        )
    }

    #[test]
    fn test_struct_instantiation() {
        let log = SSHDLog::new(&create_log([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
            TEST_IPV4_STR,
            TEST_PORT,
        ]));

        assert_ne!(log, Err(SSHDLogError::LogParseError));
        assert_ne!(log, Err(SSHDLogError::TimeParseError));
        assert_ne!(log, Err(SSHDLogError::Unknown));
        assert_ne!(log, Err(SSHDLogError::HostnameParseError));
        assert_ne!(log, Err(SSHDLogError::IdParseError));
        assert_ne!(log, Err(SSHDLogError::IpParseError));
        assert_ne!(log, Err(SSHDLogError::PortParseError));
    }

    #[test]
    fn test_hostname_populates_correctly() {
        let log = SSHDLog::new(&create_log([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
            TEST_IPV4_STR,
            TEST_PORT,
        ]))
        .unwrap();

        assert_eq!(log.host_name, TEST_HOST);
    }

    #[test]
    fn test_id_populates_correctly() {
        let log = SSHDLog::new(&create_log([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
            TEST_IPV4_STR,
            TEST_PORT,
        ]))
        .unwrap();

        assert_eq!(log.id, TEST_ID);
    }

    #[test]
    fn test_username_populates_correctly() {
        let log = SSHDLog::new(&create_log([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
            TEST_IPV4_STR,
            TEST_PORT,
        ]))
        .unwrap();

        assert_eq!(log.username, Some(TEST_USERNAME.to_owned()));
    }

    #[test]
    fn test_date_populates_correctly() {
        let log = SSHDLog::new(&create_log([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
            TEST_IPV4_STR,
            TEST_PORT,
        ]))
        .unwrap();
        let test_date_string = String::from(Utc::now().year().to_string()) + " " + TEST_TIME;

        assert_eq!(
            log.log_timestamp,
            NaiveDateTime::parse_from_str(&test_date_string, "%Y %b %d %X").unwrap()
        );
    }

    #[test]
    fn test_ip_addr_v4_populates_correctly() {
        let log = SSHDLog::new(&create_log([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
            TEST_IPV4_STR,
            TEST_PORT,
        ]))
        .unwrap();

        let addr = String::from(TEST_IPV4_STR).parse::<IpAddr>().unwrap();
        assert_eq!(Some(addr), log.remote_address);
    }

    #[test]
    fn test_ip_addr_v6_populates_correctly() {
        let addr = String::from(TEST_IPV6_STR)
            .parse::<IpAddr>()
            .expect("Can't parse IP");
        let log = SSHDLog::new(&create_log([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
            TEST_IPV6_STR,
            TEST_PORT,
        ]))
        .expect("Log instantiation failed");

        assert_eq!(Some(addr), log.remote_address);
    }

    #[test]
    fn test_port_populates_correctly() {
        let log = SSHDLog::new(&create_log([
            TEST_TIME,
            TEST_HOST,
            &TEST_ID.to_string(),
            TEST_USERNAME,
            TEST_IPV6_STR,
            TEST_PORT,
        ]))
        .expect("Log instantiation failed");

        let port_to_check = String::from(TEST_PORT).parse::<u16>().unwrap();
        assert_eq!(Some(port_to_check), log.remote_port);
    }
}
