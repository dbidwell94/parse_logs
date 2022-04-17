use crate::ufw::utils::{parse_port_rule, parse_ufw_action, UFWParseError};
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;

#[derive(Debug)]
pub enum UFWStatusError {
    ErrorParsingStatus,
    ErrorParsingAction,
}

#[derive(Debug)]
pub struct UFWStatus {
    rules: HashSet<UFWRule>,
}

impl UFWStatus {
    pub fn new(stdout: Vec<u8>) -> Result<Self, UFWStatusError> {
        Self::parse_stdout(stdout)
    }

    fn parse_stdout(stdout: Vec<u8>) -> Result<Self, UFWStatusError> {
        let status_str =
            String::from_utf8(stdout).or_else(|_| Err(UFWStatusError::ErrorParsingStatus))?;

        for status_line in status_str.split('\n') {
            let port_rule = match parse_port_rule(status_line) {
                Ok(status) => status,
                Err(e) => {
                    if let UFWParseError::PortParseError(true) = e {
                        return Err(UFWStatusError::ErrorParsingStatus);
                    } else {
                        continue;
                    }
                }
            };

            let action = match parse_ufw_action(status_line) {
                Ok(act) => act,
                Err(e) => {
                    if let UFWParseError::ActionParseError(true) = e {
                        return Err(UFWStatusError::ErrorParsingAction);
                    } else {
                        continue;
                    }
                }
            };

            println!("{:?} {:?}", port_rule, action);
        }

        // let status = parse_port_rule(status_str.as_str())
        //     .or_else(|_| Err(UFWStatusError::ErrorParsingStatus))?;

        todo!()
    }
}

#[derive(Debug)]
pub enum UFWPortRuleSpecification {
    Anywhere,
    Specific(u16),
}

#[derive(Debug)]
pub enum UFWIpRuleSpecification {
    /// If true, rule is for IPV6.
    Anywhere(bool),
    Specific(IpAddr),
}

#[derive(Debug)]
pub enum NetworkProtocol {
    Tcp,
    Udp,
    Both,
}

#[derive(Debug)]
pub enum UFWRuleDirection {
    In,
    Out,
}

#[derive(Debug)]
pub enum UFWAction {
    Allow(UFWRuleDirection),
    Deny(UFWRuleDirection),
}

#[derive(Debug)]
pub struct UFWPortRule {
    pub(crate) protocol: NetworkProtocol,
    pub(crate) port_from: UFWPortRuleSpecification,
    pub(crate) port_to: Option<UFWPortRuleSpecification>,
    pub(crate) is_v6: bool,
}

#[derive(Debug)]
pub struct UFWRule {
    port: UFWPortRule,
    action: UFWAction,
    ip_address: UFWIpRuleSpecification,
}
