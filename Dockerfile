FROM ubuntu:21.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive
ENV USER_ID=1000
ENV GROUP_ID=1000
ENV USERNAME=user

RUN apt-get upgrade && apt-get update -yq
RUN apt-get install systemd iptables ufw curl gcc sudo -yq

RUN groupadd --gid ${GROUP_ID} ${USERNAME}
RUN adduser --disabled-password --gecos '' --uid ${USER_ID} --gid ${GROUP_ID} ${USERNAME}

VOLUME ["/app"]
WORKDIR /app
COPY ./test.sh .

RUN chmod +x ./test.sh
RUN chown 777 ./test.sh
RUN su ${USERNAME}
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN exit


FROM builder as tester

ENTRYPOINT ["./test.sh"]