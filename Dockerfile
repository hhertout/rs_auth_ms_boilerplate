FROM rust:1.81.0

RUN cargo install cargo-watch

WORKDIR /app

COPY . .

CMD [ "cargo", "watch", "-x", "run" ]