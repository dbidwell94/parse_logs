mod log;
use super::SSHDLog;
use crate::{Overwrite, Readable, SSHDLogError};
use log::StructuredLog;
use std::io::BufRead;

pub struct Logger<'a, W>
where
    W: Readable + Overwrite,
{
    writer: &'a mut W,
    structured_log: StructuredLog,
}

impl<'a, W> Logger<'a, W>
where
    W: Readable + Overwrite,
{
    pub fn new(writer: &'a mut W) -> Logger<'a, W>
    where
        W: Readable + Overwrite,
    {
        let sl: StructuredLog = match StructuredLog::init(&writer.read_as_str()) {
            Some(log) => log,
            None => StructuredLog::empty()
        };

        println!("Starting with {} logs", sl.count_of_addresses());

        Logger {
            writer,
            structured_log: sl,
        }
    }

    pub fn add_log(&mut self, sshd_log: &SSHDLog) -> Result<(), SSHDLogError> {
        self.structured_log.add_ip_log(&sshd_log)?;
        self.writer
            .overwrite(
                serde_json::to_string_pretty(&self.structured_log).unwrap().as_bytes()
            )
            .expect("Unable to write to writer");
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::{Logger, SSHDLog};

    const FIRST_LOG: &'static str = "Apr 11 14:11:05 devinserver sshd[2567619]: Connection closed by invalid user debian 190.1.202.12 port 52218 [preauth]";
    const SECOND_LOG: &'static str = "Apr 11 14:10:59 devinserver sshd[2567574]: Failed password for invalid user fcaecaecca from 183.214.86.14 port 36614 ssh2";

    #[test]
    fn test_log_is_initially_populated() {
        let mut buffer: Vec<u8> = vec![0];
        let sshd_log = SSHDLog::new(&FIRST_LOG).unwrap();
        {
            Logger::new(&mut buffer);
        }
        assert_eq!(buffer, vec![0]);
        {
            let mut logger = Logger::new(&mut buffer);
            logger.add_log(&sshd_log);
        }
        assert_ne!(buffer, vec![0]);
    }

    #[test]
    fn test_second_log_overwrites_first_log() {}
}
