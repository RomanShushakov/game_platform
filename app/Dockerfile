FROM rust:1.45

ENV USER=root

RUN mkdir -p /app/

WORKDIR /app/

RUN cargo init

COPY . /app/

RUN apt install libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

EXPOSE 8080

# CMD ["cargo", "run", "--release"]
