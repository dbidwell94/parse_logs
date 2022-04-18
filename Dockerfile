FROM ubuntu:21.04 AS builder

#RUN rm /bin/sh && ln -s /bin/bash /bin/sh
#
## Create a .profile
#RUN echo 'PATH=$PATH:/foo/bar' > ~/.profile
#
## Create a .bash_profile
#RUN echo 'PATH=$PATH:/hello-world' > ~/.bash_profile

SHELL ["/bin/bash", "-c"]

ENV DEBIAN_FRONTEND=noninteractive
ENV HOST_USER_ID=1000
ENV HOST_GROUP_ID=1000
ENV HOST_USERNAME=user

RUN apt-get upgrade && apt-get update -yq
RUN apt-get install systemd iptables ufw curl gcc sudo -yq

RUN groupadd --gid ${HOST_GROUP_ID} ${HOST_USERNAME}
RUN adduser --disabled-password --gecos '' --uid ${HOST_USER_ID} --gid ${HOST_GROUP_ID} ${HOST_USERNAME}

VOLUME ["/app"]
WORKDIR /app
COPY ./test.sh .

RUN chmod +x ./test.sh
RUN chown 777 ./test.sh
RUN su ${HOST_USERNAME}
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN $HOME/.cargo/bin/rustup component add llvm-tools-preview
RUN $HOME/.cargo/bin/cargo install grcov
RUN exit


FROM builder as tester

ENTRYPOINT ["./test.sh"]