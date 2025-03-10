FROM rust:latest

RUN apt-get update \
    && apt-get install -y git build-essential cmake clang \
    # install cbc
    && apt-get install -y coinor-cbc coinor-libcbc-dev \
    # setup rust toolchain
    && rustup component add clippy \
    && rustup component add rust-src \
    && rustup component add rustfmt \
    && rustup component add rust-analyzer

# install highs
WORKDIR /build
RUN git clone https://github.com/ERGO-Code/HiGHS.git
WORKDIR /build/HiGHS
RUN cmake -S. -B build \
    && cmake --build build --parallel \
    && cmake --install build

WORKDIR /sandbox
COPY .  .

ENTRYPOINT [ "echo" ]
CMD [ "Hello, World!" ]
