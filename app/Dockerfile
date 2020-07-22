FROM rust:1.45

ENV USER=root

RUN mkdir -p /app/

WORKDIR /app/

RUN cargo init

COPY . /app/

EXPOSE 8080

CMD ["cargo", "run", "--release"]
