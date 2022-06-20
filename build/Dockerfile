FROM rust:1.61.0 AS rust_base
WORKDIR /
RUN apt update && apt install -y cmake
RUN cargo install cargo-chef

FROM rust_base AS chef
RUN mkdir /minerva
WORKDIR /minerva
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM rust_base AS builder
RUN mkdir /minerva
WORKDIR /minerva
COPY --from=chef /minerva/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim AS deploy
ARG APP=/usr/src/app
RUN apt update \
    && apt install -y ca-certificates tzdata libpq5 \
    && rm -rf /var/lib/apt/lists/*
ENV TZ=Etc/UTC \
    APP_USER=appuser
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}
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
ENV ROCKET_KEEP_ALIVE=0
ENV ROCKET_ENV=production
WORKDIR ${APP}

## Actual parts

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
