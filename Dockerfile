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

RUN apt-get upgrade && apt-get update -yq
RUN apt-get install systemd iptables ufw curl gcc sudo -yq

VOLUME ["/app"]
WORKDIR /app

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN $HOME/.cargo/bin/rustup component add llvm-tools-preview
RUN $HOME/.cargo/bin/cargo install grcov

COPY ./test.sh .
RUN chmod 777 ./test.sh
RUN chown $(id -u $USER):$(id -g $USER) ./test.sh


FROM builder as tester

ENTRYPOINT ["./test.sh"]