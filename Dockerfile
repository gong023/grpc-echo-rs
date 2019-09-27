FROM rust:1.37-stretch as build

# create a new empty shell project
RUN USER=root cargo new --bin grpc-echo-rs

RUN wget https://github.com/Kitware/CMake/releases/download/v3.15.3/cmake-3.15.3-Linux-x86_64.sh \
      -q -O /tmp/cmake-install.sh \
      && chmod u+x /tmp/cmake-install.sh \
      && mkdir /usr/bin/cmake \
      && /tmp/cmake-install.sh --skip-license --prefix=/usr/bin/cmake \
      && rm /tmp/cmake-install.sh
ENV PATH="/usr/bin/cmake/bin:${PATH}"

WORKDIR /grpc-echo-rs

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/grpc_echo_rs*
RUN cargo build --release

# our final base
FROM rust:1.37-stretch

COPY --from=build /grpc-echo-rs/target/release/server .

 CMD ["./server"]
