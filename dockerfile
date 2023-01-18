FROM rust:1.66.1-slim AS builder

WORKDIR /usr/src/project

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install musl-dev musl-tools openssl -y

RUN cargo init .
COPY Cargo* ./
RUN cargo build --release 

RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/server_card_rs*


RUN cargo build --release --target x86_64-unknown-linux-musl

# FROM gcr.io/distroless/static:nonroot
FROM scratch

COPY --from=builder /usr/src/project/target/release/server-card-rs .

EXPOSE 3000

CMD ["/server-card-rs"]