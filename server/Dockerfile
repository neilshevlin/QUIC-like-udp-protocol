FROM rust:1.64.0 as builder

RUN USER=root cargo new --bin server
WORKDIR /server
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/server*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/server

RUN apt-get update \ 
    && apt-get install -y ca-certificates  tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /server/target/release/server ${APP}/server

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./server"]
