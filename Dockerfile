FROM rustlang/rust:nightly as builder

ENV USER=root
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=development


WORKDIR /rustfinder
COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs ./build.rs
RUN apt-get update && apt-get install -y \
    libpq-dev \
    build-essential

# RUN cargo install diesel_cli --no-default-features --features mysql


RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

RUN cargo build --release
COPY .env ./.env
ENV DB_HOST=$DB_HOST
COPY src ./src

# COPY ./app var/www/html
RUN rm -f target/release/deps/rustfinder*
RUN cargo build --release

# RUN rustup default nightly

# WORKDIR /var/www/html/


FROM debian:11.6

ENV ROCKET_ENV=production

EXPOSE 8000

# builderイメージからアプリのコードのみをコピーして /rustfinderに配置する
COPY --from=builder ./rustfinder/target/release/rustfinder /usr/local/bin/rustfinder

ENV RUST_BACKTRACE=full
# RUN apk add --no-cache ca-certificates
# CMD ["cargo", "run"]
CMD ["rustfinder"]
# カレントディレクトリで「docker build -t rustfinder:1 -f .docker/rust/Dockerfile .」とすると
# ./docker/rust/Dockerfileでのビルドがカレントディレクトリで実行され、上位ディレクトリ（/app）が参照できるようになる