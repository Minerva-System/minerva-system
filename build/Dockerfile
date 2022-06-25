## Compilation and preparation

# Base container for compilation
FROM rust:1.61.0-alpine3.15 AS rust_base
WORKDIR /
RUN apk add --no-cache cmake make musl-dev file build-base libpq libpq-dev openssl-dev postgresql-dev
RUN cargo install cargo-chef
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Cargo Chef recipe generation
FROM rust_base AS chef
RUN mkdir /minerva
WORKDIR /minerva
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build all modules
FROM rust_base AS builder
RUN mkdir /minerva
WORKDIR /minerva
COPY --from=chef /minerva/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

# Base container for deploying services
FROM alpine:3.15 AS deploy
ARG APP=/usr/src/app
RUN apk add --no-cache ca-certificates tzdata libpq
COPY --from=builder /usr/lib/libgcc_s.so.1 /usr/lib/
COPY --from=builder /usr/lib/libgcc_s.so /usr/lib/
COPY --from=builder /usr/lib/libstdc++.so.6.0.28 /usr/lib/
COPY --from=builder /usr/lib/libstdc++.so.6 /usr/lib/
COPY --from=builder /usr/lib/libstdc++.so /usr/lib/
ENV TZ=Etc/UTC \
    APP_USER=appuser
RUN addgroup -g 1000 $APP_USER \
    && mkdir -p $APP \
    && adduser -u 1000 -G $APP_USER -h $APP -D $APP_USER
RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
ENV USER_SERVICE_PORT=9010
ENV SESSION_SERVICE_PORT=9011
ENV PRODUCT_SERVICE_PORT=9012
ENV STOCK_SERVICE_PORT=9013
ENV REPORT_SERVICE_PORT=9014
ENV USER_SERVICE_SERVER=user
ENV SESSION_SERVICE_SERVER=session
ENV DATABASE_SERVICE_SERVER=postgresql
ENV MONGO_SERVICE_SERVER=mongodb
ENV REDIS_SERVICE_SERVER=redis
ENV ROCKET_KEEP_ALIVE=0
ENV ROCKET_ENV=production
WORKDIR ${APP}

## Service containers

# REST
FROM deploy AS minerva_rest
EXPOSE 9000
COPY ./tenancy.toml ./tenancy.toml
COPY ./minerva-rest/Rocket.toml ./Rocket.toml
COPY --from=builder /minerva/target/release/minerva-rest ./minerva-rest
CMD ["./minerva-rest"]


# RUNONCE
FROM deploy AS minerva_runonce
COPY ./tenancy.toml ./tenancy.toml
COPY ./minerva-runonce/migrations ./migrations
COPY --from=builder /minerva/target/release/minerva-runonce ./minerva-runonce
CMD ["./minerva-runonce"]


# SESSION
FROM deploy AS minerva_session
EXPOSE 9011
COPY ./tenancy.toml ./tenancy.toml
COPY --from=builder /minerva/target/release/minerva-session ./minerva-session
CMD ["./minerva-session"]


# USER
FROM deploy AS minerva_user
EXPOSE 9010
COPY ./tenancy.toml ./tenancy.toml
COPY --from=builder /minerva/target/release/minerva-user ./minerva-user
CMD ["./minerva-user"]
