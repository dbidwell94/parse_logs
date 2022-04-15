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
            None => StructuredLog::empty(),
        };

        println!("Starting with {} logs", sl.count_of_addresses());

        Logger {
            writer,
            structured_log: sl,
        }
    }

    pub fn add_log(&mut self, sshd_log: &SSHDLog) -> Result<(), SSHDLogError> {
        self.structured_log.add_ip_log(&sshd_log)?;
        return Ok(self.write_data_to_writer());
    }

    fn write_data_to_writer(&mut self) {
        self.writer
            .overwrite(
                serde_json::to_string_pretty(&self.structured_log)
                    .unwrap()
                    .as_bytes(),
            )
            .expect("Unable to write to writer");
    }
}

impl<'a, W> Drop for Logger<'a, W>
where
    W: Readable + Overwrite,
{
    fn drop(&mut self) {
        self.write_data_to_writer();
    }
}

#[cfg(test)]
mod logger_tests {
    use crate::output_log::log::StructuredLog;
    use crate::{Logger, Overwrite, Readable, SSHDLog};
    use std::io::Error;

    struct MockWriter {
        overwrite_calls: usize,
        read_calls: usize,
        data: Vec<u8>,
    }

    impl MockWriter {
        pub fn new() -> Self {
            MockWriter {
                overwrite_calls: 0,
                read_calls: 0,
                data: Vec::new(),
            }
        }

        pub fn get_overwrite_calls(&self) -> &usize {
            &self.overwrite_calls
        }

        pub fn get_read_calls(&self) -> &usize {
            &self.read_calls
        }
    }

    impl Overwrite for MockWriter {
        fn overwrite(&mut self, data: &[u8]) -> Result<(), Error> {
            self.overwrite_calls += 1;
            self.data.overwrite(data)
        }
    }

    impl Readable for MockWriter {
        fn read_as_str(&mut self) -> String {
            self.read_calls += 1;
            self.data.read_as_str()
        }
    }

    const FIRST_LOG: &'static str = "Apr 11 14:11:05 devinserver sshd[2567619]: Connection closed by invalid user debian 190.1.202.12 port 52218 [preauth]";
    const SECOND_LOG: &'static str = "Apr 11 14:10:59 devinserver sshd[2567574]: Failed password for invalid user fcaecaecca from 183.214.86.14 port 36614 ssh2";

    #[test]
    fn test_log_is_initially_populated() {
        let mut mock_writer = MockWriter::new();
        assert_eq!(mock_writer.get_overwrite_calls(), &0usize);
        assert_eq!(mock_writer.data.len(), 0);
        let test_log = SSHDLog::new(&FIRST_LOG).unwrap();
        {
            let mut logger = Logger::new(&mut mock_writer);
            logger.add_log(&test_log).unwrap();
        }
        let mut base_written_log = StructuredLog::empty();
        base_written_log.add_ip_log(&test_log).unwrap();

        let written_log_to_test =
            StructuredLog::init(&String::from_utf8(mock_writer.data.to_owned()).unwrap()).unwrap();

        assert_eq!(
            base_written_log.count_of_addresses(),
            written_log_to_test.count_of_addresses()
        );
    }

    #[test]
    fn test_log_flushes_data_when_destroyed() {
        let mut mock_writer = MockWriter::new();
        assert_eq!(mock_writer.get_overwrite_calls(), &0usize);
        assert_eq!(mock_writer.get_read_calls(), &0usize);
        {
            let mut logger = Logger::new(&mut mock_writer);
            let log = SSHDLog::new(&FIRST_LOG).unwrap();
            logger.add_log(&log).unwrap();
        }
        assert_eq!(mock_writer.get_read_calls(), &1usize);
        assert_eq!(mock_writer.get_overwrite_calls(), &2usize);
    }
}
