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
                    log.remote_endpoints_map.insert(remote_endpoint.address.to_owned(), remote_endpoint.to_owned());
                }

                return Some(log)
            },
            Err(_) => None
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
