mod ufw_status;
mod utils;

use crate::ufw::ufw_status::{UFWStatus, UFWStatusError};
use std::process::{Command, ExitStatus, Output};

#[derive(Debug)]
pub enum UFWError {
    UFWNotInstalled,
    UFWCommandFailed,
    UFWPrivilegeError,
    UFWStatusError(UFWStatusError),
}

#[derive(Debug)]
pub struct UFW {
    ufw_location: String,
}

impl UFW {
    #[cfg(not(target_os = "linux"))]
    pub fn new() {
        panic!("UFW not supported on this OS");
    }

    #[cfg(target_os = "linux")]
    pub fn new() -> Result<UFW, UFWError> {
        let ufw_location = Command::new("which")
            .args(["ufw"])
            .output()
            .or_else(|_| Err(UFWError::UFWNotInstalled))?;

        Self::check_status_ok(&ufw_location, UFWError::UFWNotInstalled)?;

        let location =
            String::from_utf8(ufw_location.stdout).or_else(|_| Err(UFWError::UFWNotInstalled))?;
        if location.is_empty() {
            return Err(UFWError::UFWNotInstalled);
        }

        return Ok(UFW {
            ufw_location: location,
        });
    }

    pub fn status(&self) -> Result<UFWStatus, UFWError> {
        let cmd = Command::new("sudo")
            .args(["ufw", "status", "verbose"])
            .output()
            .or_else(|e| {
                println!("{:?}", e);
                return Err(UFWError::UFWCommandFailed);
            })?;

        Self::check_status_ok(&cmd, UFWError::UFWCommandFailed)?;

        let status =
            UFWStatus::new(cmd.stdout.to_owned()).or_else(|e| Err(UFWError::UFWStatusError(e)))?;

        todo!()
    }

    fn check_status_ok(output: &Output, err_to_throw: UFWError) -> Result<(), UFWError> {
        if !output.status.success() || !output.stderr.is_empty() {
            println!("{:?}", &output);
            return Err(err_to_throw);
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::ufw::UFW;

    #[test]
    fn test_ufw_location() {
        let ufw = UFW::new().unwrap();
        assert_ne!(ufw.ufw_location, String::from(""));
    }

    #[test]
    fn test_ufw_status() {
        let ufw = UFW::new().unwrap();
        let status = ufw.status().unwrap();
    }
}
