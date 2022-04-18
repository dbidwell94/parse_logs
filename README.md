# parselogs
A rust binary used to parse logs generated from /var/log/auth.log on Unix systems

## How does it work?
The Program reads changes from `/var/log/auth.log` and writes to a database in `/etc/parselogs/parselogs.json`
If certain conditions are met, then the Program will then use `ufw` to block IP addresses from accessing the System.

## Badges

Testing Code Coverage  |  -
:---------------------:|:----------------:
[![codecov](https://codecov.io/gh/dbidwell94/parse_logs/branch/master/graph/badge.svg?token=OSFZUE3WUZ)](https://codecov.io/gh/dbidwell94/parse_logs) | -
