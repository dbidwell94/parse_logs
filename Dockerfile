FROM ubuntu:21.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get upgrade && apt-get update -yq
RUN apt-get install systemd iptables ufw curl gcc -yq
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

VOLUME ["/app"]

WORKDIR /app

COPY ./test.sh .

FROM builder as tester

RUN chmod +x ./test.sh

ENTRYPOINT ["./test.sh"]