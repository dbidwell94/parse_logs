use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
/// Returned when there was an error parsing the SSHD log
pub enum SSHDLogError {
    LogParseError,
    TimeParseError,
    HostnameParseError,
    IdParseError,
    IpParseError,
    PortParseError,
    Unknown,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// Denotes what kind of log this SSHD log is
pub enum SSHDLogType {
    InvalidPassword,
    InvalidUser,
    ConnectionClosed,
    ConnectionSuccessful(String),
}
