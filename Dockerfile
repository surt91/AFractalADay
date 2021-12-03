FROM rust:1.56 as builder
WORKDIR /usr/src/fractal
COPY . .
RUN cargo build --release --features "binaries"

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl1.1 imagemagick optipng && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/fractal/target/release/a_fractal_a_day /usr/local/bin/fractal
ENTRYPOINT ["fractal"]