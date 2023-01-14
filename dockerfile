FROM rust:1.66.1-slim AS builder

WORKDIR /usr/src/project

RUN cargo init .
COPY Cargo* ./
RUN cargo build --release 

COPY . .
RUN cargo build --release

# FROM gcr.io/distroless/static:nonroot
FROM scratch

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/project/target/release/project .

CMD ["./project"]