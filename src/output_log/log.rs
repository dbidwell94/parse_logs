use crate::{SSHDLog, SSHDLogError};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct StructuredLog {
    updated_time: NaiveDateTime,
    remote_endpoints: Vec<RemoteEndpoint>,
    #[serde(skip)]
    remote_endpoints_map: HashMap<IpAddr, RemoteEndpoint>,
}

impl StructuredLog {
    pub fn empty() -> Self {
        Self {
            remote_endpoints: Vec::new(),
            updated_time: Utc::now().naive_local(),
            remote_endpoints_map: HashMap::new(),
        }
    }

    pub fn init(input: &str) -> Option<StructuredLog> {
        return match serde_json::from_str::<StructuredLog>(input) {
            Ok(mut log) => {
                log.remote_endpoints_map = HashMap::new();
                for remote_endpoint in &log.remote_endpoints {
                    log.remote_endpoints_map.insert(
                        remote_endpoint.address.to_owned(),
                        remote_endpoint.to_owned(),
                    );
                }

                return Some(log);
            }
            Err(_) => None,
        };
    }

    pub fn sort_endpoints(&mut self) {
        let mut vec: Vec<RemoteEndpoint> = Vec::new();

        for ep in self.remote_endpoints_map.iter().map(|(_, v)| return v) {
            vec.push(ep.clone());
        }

        vec.sort_by(|v1, v2| v2.log_count.cmp(&v1.log_count));
        self.remote_endpoints = vec;
    }

    pub fn add_ip_log(&mut self, log: &SSHDLog) -> Result<(), SSHDLogError> {
        let now = Utc::now();
        self.updated_time = NaiveDateTime::from(now.naive_local());

        let addr = log
            .get_ip_addr()
            .ok_or_else(|| return SSHDLogError::IpParseError)?;

        if self.remote_endpoints_map.contains_key(&addr) {
            let remote_endpoint = self.remote_endpoints_map.get_mut(&addr).unwrap();

            remote_endpoint.log_count += 1;
            self.sort_endpoints();
            return Ok(());
        }

        self.remote_endpoints_map.insert(
            addr.to_owned(),
            RemoteEndpoint {
                address: addr,
                log_count: 1,
            },
        );
        self.sort_endpoints();
        return Ok(());
    }

    pub fn count_of_addresses(&self) -> usize {
        self.remote_endpoints.len()
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct RemoteEndpoint {
    address: IpAddr,
    log_count: usize,
}

#[cfg(test)]
mod structured_log_test {
    use crate::output_log::log::StructuredLog;
    use crate::SSHDLog;
    use std::io::stdin;

    const FIRST_LOG: &'static str = "Apr 11 14:11:05 devinserver sshd[2567619]: Connection closed by invalid user debian 190.1.202.12 port 52218 [preauth]";

    #[test]
    fn test_structured_log_empty() {
        let structured_log = StructuredLog::empty();
        assert_eq!(structured_log.remote_endpoints_map.len(), 0);
        assert_eq!(structured_log.remote_endpoints.len(), 0);
    }

    #[test]
    fn test_structured_log_insertion() {
        let mut structured_log = StructuredLog::empty();
        assert_eq!(structured_log.remote_endpoints_map.len(), 0);

        let log_to_insert = SSHDLog::new(&FIRST_LOG).unwrap();
        assert_ne!(structured_log.add_ip_log(&log_to_insert).is_err(), true);

        assert_eq!(structured_log.remote_endpoints_map.len(), 1);
        assert_eq!(structured_log.remote_endpoints.len(), 1);
    }

    #[test]
    fn test_structured_log_with_same_ip_address() {
        let mut structured_log = StructuredLog::empty();
        assert_eq!(structured_log.remote_endpoints_map.len(), 0);
        let log_to_insert = SSHDLog::new(&FIRST_LOG).unwrap();
        assert_ne!(structured_log.add_ip_log(&log_to_insert).is_err(), true);
    }
}
