extern crate core;

mod output_log;
mod ssh_log;

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
        self.write(data)?;
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

const PARSE_LOGS_LOG_LOCATION: &'static str = "./parse_logs.log";

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
                            _ => panic!(),
                        },
                    };

                    logger.add_log(&log)?
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
        .expect(format!("Unable to open file at {}", path.to_str().unwrap()).as_str());

    parse_stdin(stdin.lock(), &mut file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Apr 10 00:00:10 devinserver sshd[1748198]: Invalid user DudePerson from 143.198.68.239 port 56720";
        let mut log_buffer_1: Vec<u8> = vec![0];
        let mut log_buffer_2: Vec<u8> = vec![0];

        parse_stdin(input.as_bytes(), &mut log_buffer_1).unwrap();
        let log = SSHDLog::new(input).unwrap();

        {
            let mut logger = Logger::new(&mut log_buffer_2);
            logger.add_log(&log).unwrap();
        }
    }
}
