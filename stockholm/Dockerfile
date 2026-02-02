FROM rust:1.93 AS builder
WORKDIR /usr/src/stockholm
COPY . .
RUN cargo install --path .

FROM debian:stable
COPY --from=builder /usr/local/cargo/bin/stockholm /usr/local/bin/stockholm

CMD ["/bin/bash"]
