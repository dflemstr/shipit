FROM jimmycuadra/rust:v1.0.0-beta.4
MAINTAINER "David Flemström <david.flemstrom@gmail.com>"

WORKDIR /usr/src/shipit
RUN apt-get update && \
  apt-get install -y libprotobuf-dev libzmq3-dev && \
  rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock /usr/src/shipit/
COPY src /usr/src/shipit/src
RUN cargo build --release
RUN cp target/release/shipit /usr/bin/shipit

ENV RUST_LOG=shipit=info
WORKDIR /
ENTRYPOINT ["/usr/bin/shipit"]
