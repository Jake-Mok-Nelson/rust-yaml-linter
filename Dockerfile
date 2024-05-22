FROM alpine as builder

RUN apk add --no-cache cargo rust


COPY ./ /src
WORKDIR /src
RUN cargo build --release

FROM alpine as final

RUN apk add --no-cache musl libgcc

COPY --from=builder /src/target/release/main /usr/local/bin/rust-yaml-linter
COPY --from=builder /src/target/release/generate-yaml /usr/local/bin/

CMD ["rust-yaml-linter"]
