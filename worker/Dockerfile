FROM rust:1.64.0 as builder

RUN USER=root cargo new --bin worker
WORKDIR /worker
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/worker*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/worker

RUN apt-get update \ 
    && apt-get install -y ca-certificates  tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /worker/target/release/worker ${APP}/worker

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./worker"]