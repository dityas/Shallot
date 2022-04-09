FROM ubuntu:21.10

WORKDIR /home

RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt-get -y install tzdata
RUN apt-get install -y curl gcc git-all 
RUN apt-get update
RUN libssl-dev pkg-config
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN git clone https://github.com/dityas/Shallot.git

WORKDIR /home/Shallot

# Is this really needed?
# RUN curl https://getmic.ro | sh -s -- -y && mv micro /usr/bin/

# Are these?
# COPY src ./src
# COPY Cargo.toml .
# COPY Cargo.lock .

# RUN cargo build

CMD ["/bin/bash"]