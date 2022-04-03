FROM rust:latest AS builder
LABEL intermediateStageToBeDeleted=true

COPY server-rs build
RUN cd build && cargo build --release

FROM ubuntu:latest
COPY --from=builder /build/target/release/server-rs /server-rs
ENTRYPOINT ["server-rs"]

