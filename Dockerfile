FROM rust:latest

ENV XDG_RUNTIME_DIR=/tmp

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN apt-get update && apt-get install -y libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev

COPY . .

RUN cargo build --release

CMD ["./target/release/open_scape"]
