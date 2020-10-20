FROM rust:1.47

ENV USER=root

RUN mkdir -p /app/

WORKDIR /app/

COPY . /app/

RUN apt-get install libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

RUN apt-get install curl

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

EXPOSE 8080
