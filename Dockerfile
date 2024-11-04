FROM rust:latest

WORKDIR /usr/src/application

COPY . .

RUN cargo build --release

CMD [ "./target/release/linpauldotcom"]