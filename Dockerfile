FROM alpine as builder

RUN apk update && \
    apk install -y \
    cargo \
    rust

COPY ./src /src
WORKDIR /src
RUN cargo build --release

FROM alpine as final

COPY --from=builder /src/target/release/main /usr/local/bin
COPY --from=builder /src/target/release/generate-yaml /usr/local/bin
