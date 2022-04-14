#[derive(Debug, PartialEq)]
pub enum SSHDLogError {
    LogParseError,
    TimeParseError,
    HostnameParseError,
    IdParseError,
    IpParseError,
    PortParseError,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum SSHDLogType {
    InvalidPassword,
    InvalidUser,
    ConnectionClosed,
}
