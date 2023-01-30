FROM postgres:15.1-bullseye
COPY ./docker/hello.txt /
# COPY ./hello_world1/target/release/hello_world1-pg15/usr/share/postgresql/15/extension/* /usr/share/postgresql/15/extension
# COPY ./hello_world1/target/release/hello_world1-pg15/usr/lib/postgresql/15/lib/* /usr/lib/postgresql/15/lib
RUN apt-get update && apt-get upgrade -y
RUN apt-get install libssl-dev 
RUN  apt install libc6
RUN apt-get install -y -q build-essential curl
# RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# RUN apt install rustc -y
# RUN apt install binutils-x86-64-linux-gnu -y

RUN apt-get install wget -y
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    rustArch=x86_64-unknown-linux-gnu \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.66.1

RUN set -eux; \
    # %%ARCH-CASE%%; \
    url="https://static.rust-lang.org/rustup/archive/1.25.1/${rustArch}/rustup-init"; \
    wget "$url"; \
    # echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;
# RUN apt install binutils-common libbinutils -y
RUN mkdir /hello_world1
COPY ./hello_world1 /hello_world1
RUN echo "Hello World"
# RUN source "$HOME/.cargo/env"
# RUN rustup update -y
WORKDIR /hello_world1
RUN cargo --version
RUN cargo install --locked cargo-pgx
RUN cargo build
RUN cargo pgx package