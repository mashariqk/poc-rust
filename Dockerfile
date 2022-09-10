FROM rust:1.63 as builder

RUN USER=root cargo new --bin poc-rust
WORKDIR ./poc-rust
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/poc_rust*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
	&& apt-get install -y ca-certificates tzdata \
	&& rm -rf /var/lib/apt/lists/*

EXPOSE 9001

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
	&& useradd -g $APP_USER $APP_USER \
	&& mkdir -p ${APP}

COPY --from=builder /poc-rust/target/release/poc-rust ${APP}/poc-rust

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./poc-rust"]