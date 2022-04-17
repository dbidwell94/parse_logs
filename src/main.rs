extern crate core;

mod output_log;
mod ssh_log;
mod test_helpers;
mod ufw;

use self::ssh_log::{SSHDLog, SSHDLogError};
use output_log::Logger;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Error, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub trait Overwrite {
    fn overwrite(&mut self, data: &[u8]) -> Result<(), Error>;
}

pub trait Readable {
    fn read_as_str(&mut self) -> String;
}

impl Overwrite for File {
    fn overwrite(&mut self, data: &[u8]) -> Result<(), Error> {
        self.set_len(0)?;
        self.seek(SeekFrom::Start(0))?;
        self.write_all(data)?;
        return Ok(());
    }
}

impl Overwrite for Vec<u8> {
    fn overwrite(&mut self, data: &[u8]) -> Result<(), Error> {
        self.clear();
        self.resize(data.len(), 0);
        let mut index: usize = 0;
        for datum in data {
            self[index] = datum.to_owned();
            index += 1;
        }
        return Ok(());
    }
}

impl Readable for File {
    fn read_as_str(&mut self) -> String {
        let mut buffer = String::new();
        self.read_to_string(&mut buffer).unwrap();
        return buffer;
    }
}

impl Readable for Vec<u8> {
    fn read_as_str(&mut self) -> String {
        return String::from_utf8(self.clone()).unwrap();
    }
}

const PARSE_LOGS_LOG_LOCATION: &'static str = "/etc/parselogs/logdb.json";

fn parse_stdin<R, W>(mut reader: R, writer: &mut W) -> Result<(), SSHDLogError>
where
    R: BufRead,
    W: Overwrite + Readable,
{
    let mut str_buffer: String = String::new();
    let mut logger = Logger::new(writer);

    loop {
        match reader.read_line(&mut str_buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return Ok(());
                }
                let split_strings = str_buffer.split("\n");
                for split_string in split_strings {
                    let log = match SSHDLog::new(split_string) {
                        Ok(v) => v,
                        Err(error) => match error {
                            SSHDLogError::LogParseError => {
                                continue;
                            }
                            SSHDLogError::IpParseError => {
                                continue;
                            }
                            _ => panic!(),
                        },
                    };

                    match logger.add_log(&log) {
                        Ok(_) => {}
                        Err(e) => match e {
                            SSHDLogError::IpParseError => {}
                            _ => {}
                        },
                    }
                }
            }
            Err(_) => return Ok(()),
        }
        str_buffer = String::new();
    }
}

fn main() -> Result<(), SSHDLogError> {
    let stdin = io::stdin();

    let path = Path::new(PARSE_LOGS_LOG_LOCATION);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .append(false)
        .open(path)
        .expect(
            format!(
                "Unable to open file at {} with RW permissions",
                path.to_str().unwrap()
            )
            .as_str(),
        );

    parse_stdin(stdin.lock(), &mut file)
}

#[cfg(test)]
mod tests {
    use super::{parse_stdin, Logger, SSHDLog};
    use crate::test_helpers::test_helpers;
    use test_helpers::MockWriter;

    #[test]
    fn it_works() {
        let input = "Apr 10 00:00:10 devinserver sshd[1748198]: Invalid user DudePerson from 143.198.68.239 port 56720";
        let mut mock_writer = MockWriter::new();
        let mut mock_writer_2 = MockWriter::new();

        parse_stdin(input.as_bytes(), &mut mock_writer).unwrap();
        let log = SSHDLog::new(input).unwrap();

        {
            let mut logger = Logger::new(&mut mock_writer_2);
            logger.add_log(&log).unwrap();
        }
    }

    #[test]
    fn test_invalid_input_doesnt_log() {
        let invalid_input = "INVALID INPUT";
        let mut mock_writer = MockWriter::new();
        parse_stdin(invalid_input.as_bytes(), &mut mock_writer).unwrap();
        // overwrite should only have been called once when the data was flushed
        assert_eq!(mock_writer.get_overwrite_calls(), &1usize);
    }

    #[test]
    fn test_invalid_ip_address() {
        let input = "Apr 10 00:00:10 devinserver sshd[1748198]: Invalid user DudePerson from 143.198.68.23934 port 56720";
        let mut mock_writer = MockWriter::new();
        parse_stdin(input.as_bytes(), &mut mock_writer).unwrap();
        // overwrite should have been called twice for valid log and a flush
        assert_eq!(mock_writer.get_overwrite_calls(), &2usize);
    }

    #[test]
    fn test_read_for_file_impl() {}
}
