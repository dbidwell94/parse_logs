mod ssh_log;
use self::ssh_log::{SSHDLog, SSHDLogError};
use chrono::RoundingError::TimestampExceedsLimit;
use std::io::{self, BufRead, Write};

enum LogType {
    SSHD(SSHDLog),
}

fn parse_stdin<R>(mut reader: R)
where
    R: BufRead,
{
    let mut str_buffer: String = String::new();

    loop {
        match reader.read_line(&mut str_buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return ();
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
                    // println!("{:?}", log);
                }
            }
            Err(_) => return,
        }
        str_buffer = String::new();
    }
}

fn main() {
    let stdin = io::stdin();

    parse_stdin(stdin.lock());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = b"Apr 10 00:00:10 devinserver sshd[1748198]: Invalid user ashishyadav from 143.198.68.239 port 56720\n";
        parse_stdin(&input[..]);
    }
}
