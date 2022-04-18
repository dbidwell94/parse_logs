use crate::ufw::ufw_status::{
    NetworkProtocol, UFWAction, UFWIpRuleSpecification, UFWPortRule, UFWPortRuleSpecification,
    UFWRuleDirection,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug)]
pub enum UFWParseError {
    PortParseError(bool),
    ActionParseError(bool),
    IpParseError(bool),
}

const PORT_RULE_REGEX_STR: &'static str =
    r"(?i)^(\d+|anywhere)(?::(\d+))?(?:/(tcp|udp))?(?:\s\((v6)\))?";
const ACTION_RULE_REGEX_STR: &'static str = r"(?i)(deny|allow)\s(in|out)";
const IP_RULE_REGEX_STR: &'static str =
    r"(?i)(?:(?:deny|allow|reject)\s(?:in|out))\s+(?:(anywhere)(?:\s?\((v6)\))?|((?:\w+|:|\.)+))";

lazy_static! {
    static ref PORT_RULE_REGEX: Regex = Regex::new(PORT_RULE_REGEX_STR).unwrap();
    static ref ACTION_RULE_REGEX: Regex = Regex::new(ACTION_RULE_REGEX_STR).unwrap();
    static ref IP_RULE_REGEX: Regex = Regex::new(IP_RULE_REGEX_STR).unwrap();
}

pub fn parse_port_rule(input: &str) -> Result<UFWPortRule, UFWParseError> {
    let mut is_v6 = false;

    if !PORT_RULE_REGEX.is_match(&input) {
        return Err(UFWParseError::PortParseError(false));
    }

    return match PORT_RULE_REGEX.captures(input) {
        None => Err(UFWParseError::PortParseError(true)),
        Some(cap) => {
            // port from
            let port = cap
                .get(1)
                .ok_or_else(|| UFWParseError::PortParseError(true))?
                .as_str();
            let port_num = match port.parse::<u16>() {
                Ok(v) => Ok(UFWPortRuleSpecification::Specific(v)),
                Err(_) => match port.to_lowercase().as_str() {
                    "anywhere" => Ok(UFWPortRuleSpecification::Anywhere),
                    _ => Err(UFWParseError::PortParseError(true)),
                },
            }?;
            // port to
            let port_to = match cap.get(2) {
                Some(value) => match value.as_str().parse::<u16>() {
                    Err(_) => return Err(UFWParseError::PortParseError(true)),
                    Ok(num) => Some(UFWPortRuleSpecification::Specific(num)),
                },
                None => None,
            };

            // protocol
            let protocol = match cap.get(3) {
                None => NetworkProtocol::Both,
                Some(v) => match v.as_str().to_lowercase().as_str() {
                    "tcp" => NetworkProtocol::Tcp,
                    "udp" => NetworkProtocol::Udp,
                    _ => NetworkProtocol::Both,
                },
            };
            // version
            match cap.get(4) {
                None => {}
                Some(v) => {
                    if v.as_str().to_lowercase().as_str() == "v6" {
                        is_v6 = true;
                    }
                }
            };

            Ok(UFWPortRule {
                port_from: port_num,
                port_to,
                is_v6,
                protocol,
            })
        }
    };
}

pub fn parse_ufw_action(input: &str) -> Result<UFWAction, UFWParseError> {
    if !ACTION_RULE_REGEX.is_match(input) {
        return Err(UFWParseError::ActionParseError(true));
    }

    return match ACTION_RULE_REGEX.captures(input) {
        None => Err(UFWParseError::ActionParseError(true)),
        Some(cap) => {
            let is_inbound_rule = cap
                .get(2)
                .map(|val| {
                    let val_str = val.as_str().to_lowercase();
                    return if val_str == "in" {
                        Ok(true)
                    } else if val_str == "out" {
                        Ok(false)
                    } else {
                        Err(UFWParseError::ActionParseError(true))
                    };
                })
                .ok_or(UFWParseError::ActionParseError(true))??;

            let allow_or_deny = cap
                .get(1)
                .map(|val| {
                    let val_str = val.as_str().to_lowercase();
                    return if val_str == "deny" {
                        if is_inbound_rule {
                            Ok(UFWAction::Deny(UFWRuleDirection::In))
                        } else {
                            Ok(UFWAction::Deny(UFWRuleDirection::Out))
                        }
                    } else if val_str == "allow" {
                        if is_inbound_rule {
                            Ok(UFWAction::Allow(UFWRuleDirection::In))
                        } else {
                            Ok(UFWAction::Allow(UFWRuleDirection::Out))
                        }
                    } else {
                        Err(UFWParseError::ActionParseError(true))
                    };
                })
                .ok_or(UFWParseError::ActionParseError(true))??;
            Ok(allow_or_deny)
        }
    };
}

pub fn parse_ufw_ip(input: &str) -> Result<UFWIpRuleSpecification, UFWParseError> {
    if !IP_RULE_REGEX.is_match(input) {
        return Err(UFWParseError::IpParseError(true));
    }

    return match IP_RULE_REGEX.captures(input) {
        None => Err(UFWParseError::IpParseError(true)),
        Some(cap) => {
            if let Some(value) = cap.get(3) {
                let ip_addr_str = value.as_str();
                Ok(UFWIpRuleSpecification::Specific(
                    IpAddr::from_str(ip_addr_str)
                        .or_else(|_| Err(UFWParseError::IpParseError(true)))?,
                ))
            } else {
                let anywhere_str = cap
                    .get(1)
                    .ok_or_else(|| UFWParseError::IpParseError(true))?
                    .as_str();
                if anywhere_str.to_lowercase() != "anywhere" {
                    Err(UFWParseError::IpParseError(true))
                } else {
                    match cap.get(2) {
                        None => Ok(UFWIpRuleSpecification::Anywhere(false)),
                        Some(value) => {
                            let v6_str = value.as_str().to_lowercase();
                            if v6_str != "v6" {
                                Err(UFWParseError::IpParseError(true))
                            } else {
                                Ok(UFWIpRuleSpecification::Anywhere(true))
                            }
                        }
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::ufw::ufw_status::{
        NetworkProtocol, UFWAction, UFWIpRuleSpecification, UFWPortRuleSpecification,
        UFWRuleDirection,
    };
    use crate::ufw::utils::{parse_port_rule, parse_ufw_action, parse_ufw_ip};
    use std::net::IpAddr;
    use std::str::FromStr;

    #[test]
    fn test_parse_port_rule_range() {
        let port_rule_str = "2456:2457/udp              ALLOW       Anywhere";
        let port_rule_result = parse_port_rule(port_rule_str);
        assert!(port_rule_result.is_ok());
        let port_rule = port_rule_result.unwrap();
        assert_eq!(port_rule.is_v6, false);
        assert_eq!(
            port_rule.port_from,
            UFWPortRuleSpecification::Specific(2456u16)
        );
        assert_eq!(
            port_rule.port_to,
            Some(UFWPortRuleSpecification::Specific(2457u16))
        );
        assert_eq!(port_rule.protocol, NetworkProtocol::Udp);
    }

    #[test]
    fn test_parse_port_rule_non_range() {
        let port_rule_str = "25/tcp                     ALLOW       Anywhere";
        let port_rule_result = parse_port_rule(port_rule_str);
        assert!(port_rule_result.is_ok());
        let port_rule = port_rule_result.unwrap();
        assert_eq!(
            port_rule.port_from,
            UFWPortRuleSpecification::Specific(25u16)
        );
        assert_eq!(port_rule.port_to, None);
        assert_eq!(port_rule.protocol, NetworkProtocol::Tcp);
        assert_eq!(port_rule.is_v6, false);
    }

    #[test]
    fn test_parse_port_rule_non_range_v6() {
        let port_rule_str = "8675 (v6)                  ALLOW       Anywhere (v6)";
        let port_rule_result = parse_port_rule(port_rule_str);
        assert!(port_rule_result.is_ok());
        let port_rule = port_rule_result.unwrap();
        assert_eq!(port_rule.is_v6, true);
        assert_eq!(port_rule.protocol, NetworkProtocol::Both);
        assert_eq!(
            port_rule.port_from,
            UFWPortRuleSpecification::Specific(8675u16)
        );
        assert_eq!(port_rule.port_to, None);
    }

    #[test]
    fn test_parse_ufw_action_allow_inbound() {
        let action_str = "8675 (v6)                  ALLOW IN    Anywhere (v6)";
        let action_result = parse_ufw_action(action_str);
        assert!(action_result.is_ok());
        let action = action_result.unwrap();
        assert_eq!(action, UFWAction::Allow(UFWRuleDirection::In));
    }

    #[test]
    fn test_parse_ufw_action_allow_outbound() {
        let action_str = "8675 (v6)                  ALLOW OUT    Anywhere (v6)";
        let action_result = parse_ufw_action(action_str);
        assert!(action_result.is_ok());
        let action = action_result.unwrap();
        assert_eq!(action, UFWAction::Allow(UFWRuleDirection::Out));
    }

    #[test]
    fn test_parse_ufw_action_deny_in() {
        let action_str = "8675 (v6)                  DENY IN    Anywhere (v6)";
        let action_result = parse_ufw_action(action_str);
        assert!(action_result.is_ok());
        let action = action_result.unwrap();
        assert_eq!(action, UFWAction::Deny(UFWRuleDirection::In));
    }

    #[test]
    fn test_parse_ufw_action_deny_outbound() {
        let action_str = "8675 (v6)                  DENY OUT    Anywhere (v6)";
        let action_result = parse_ufw_action(action_str);
        assert!(action_result.is_ok());
        let action = action_result.unwrap();
        assert_eq!(action, UFWAction::Deny(UFWRuleDirection::Out));
    }

    #[test]
    fn test_parse_ufw_ip_anywhere_v6() {
        let ip_str = "8675 (v6)                  DENY OUT    Anywhere (v6)";
        let ip_result = parse_ufw_ip(ip_str);
        assert!(ip_result.is_ok());
        let ip = ip_result.unwrap();
        assert_eq!(ip, UFWIpRuleSpecification::Anywhere(true));
    }

    #[test]
    fn test_parse_ufw_ip_anywhere() {
        let ip_str = "8675 (v6)                  DENY IN    Anywhere";
        let ip_result = parse_ufw_ip(ip_str);
        assert!(ip_result.is_ok());
        let ip = ip_result.unwrap();
        assert_eq!(ip, UFWIpRuleSpecification::Anywhere(false));
    }

    #[test]
    fn test_parse_ufw_ip_specific_v6() {
        let ip_str = "Anywhere (v6)              DENY IN        2001:4860:4860::8844";
        let ip_result = parse_ufw_ip(ip_str);
        assert!(ip_result.is_ok());
        let ip = ip_result.unwrap();
        assert_eq!(
            ip,
            UFWIpRuleSpecification::Specific(IpAddr::from_str("2001:4860:4860::8844").unwrap())
        );
    }

    #[test]
    fn test_parse_ufw_ip_specific() {
        let ip_str = "Anywhere              DENY IN       10.0.0.1";
        let ip_result = parse_ufw_ip(ip_str);
        assert!(ip_result.is_ok());
        let ip = ip_result.unwrap();
        assert_eq!(
            ip,
            UFWIpRuleSpecification::Specific(IpAddr::from_str("10.0.0.1").unwrap())
        );
    }
}
