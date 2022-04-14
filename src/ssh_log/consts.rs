use lazy_static::lazy_static;
use regex::Regex;

pub const REGEX_STR: &'static str =
    r"^\w{3}\s\d{1,2}\s\d{2}:\d{2}:\d{2}\s[\w\d]+\s[\w\-_]+\[\d+\]:\s.*\n?$";
pub const DATE_REGEX_STR: &'static str = r"^\w{3}\s\d{1,2}\s\d{2}:\d{2}:\d{2}";
pub const HOSTNAME_REGEX_STR: &'static str = r"^\w{3}\s\d{1,2}\s\d{2}:\d{2}:\d{2}\s([\w\d]+)";
pub const LOG_ID_REGEX_STR: &'static str = r"\w+\[(\d+)\]";
pub const USERNAME_REGEX_STR: &'static str = r"user ([\w\d]+)";
pub const IPV4_REGEX_STR: &'static str = r"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}(\\\d{1,2})?)";
pub const IPV6_REGEX_STR: &'static str = r"(((([0-9A-Fa-f]{1,4}:){7}([0-9A-Fa-f]{1,4}|:))|(([0-9A-Fa-f]{1,4}:){6}(:[0-9A-Fa-f]{1,4}|((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])){3})|:))|(([0-9A-Fa-f]{1,4}:){5}(((:[0-9A-Fa-f]{1,4}){1,2})|:((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])){3})|:))|(([0-9A-Fa-f]{1,4}:){4}(((:[0-9A-Fa-f]{1,4}){1,3})|((:[0-9A-Fa-f]{1,4})?:((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])){3}))|:))|(([0-9A-Fa-f]{1,4}:){3}(((:[0-9A-Fa-f]{1,4}){1,4})|((:[0-9A-Fa-f]{1,4}){0,2}:((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])){3}))|:))|(([0-9A-Fa-f]{1,4}:){2}(((:[0-9A-Fa-f]{1,4}){1,5})|((:[0-9A-Fa-f]{1,4}){0,3}:((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])){3}))|:))|(([0-9A-Fa-f]{1,4}:){1}(((:[0-9A-Fa-f]{1,4}){1,6})|((:[0-9A-Fa-f]{1,4}){0,4}:((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])){3}))|:))|(:(((:[0-9A-Fa-f]{1,4}){1,7})|((:[0-9A-Fa-f]{1,4}){0,5}:((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])){3}))|:)))(%.+)?)";
pub const PORT_REGEX_STR: &'static str = r"(?i)port (\d+)";
pub const INVALID_PASSWORD_REGEX_STR: &'static str =
    r"(?i)(failed password|authentication failure)";
pub const INVALID_USER_REGEX_STR: &'static str = r"(?i)(invalid user)";

lazy_static! {
    pub static ref LOG_REGEX: Regex = Regex::new(REGEX_STR).unwrap();
    pub static ref DATE_REGEX: Regex = Regex::new(DATE_REGEX_STR).unwrap();
    pub static ref HOSTNAME_REGEX: Regex = Regex::new(HOSTNAME_REGEX_STR).unwrap();
    pub static ref LOG_ID_REGEX: Regex = Regex::new(LOG_ID_REGEX_STR).unwrap();
    pub static ref USERNAME_REGEX: Regex = Regex::new(USERNAME_REGEX_STR).unwrap();
    pub static ref IPV4_REGEX: Regex = Regex::new(IPV4_REGEX_STR).unwrap();
    pub static ref IPV6_REGEX: Regex = Regex::new(IPV6_REGEX_STR).unwrap();
    pub static ref PORT_REGEX: Regex = Regex::new(PORT_REGEX_STR).unwrap();
    pub static ref INVALID_PASSWORD_REGEX: Regex = Regex::new(INVALID_PASSWORD_REGEX_STR).unwrap();
    pub static ref INVALID_USER_REGEX: Regex = Regex::new(INVALID_USER_REGEX_STR).unwrap();
}
