FROM rust:1.80.1-slim AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev protobuf-compiler
RUN update-ca-certificates

ARG CRATE_SERVER=api-categories
ARG CRATE_CORE=sellershut-core
ARG CRATE_UTILS=sellershut-utils
ARG CRATE_INFRA=infra

WORKDIR /usr/src/app

COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./.sqlx .

WORKDIR /usr/src/app/crates

RUN cargo new ${CRATE_SERVER}
RUN cargo new --lib ${CRATE_UTILS}
RUN cargo new --lib ${CRATE_INFRA}
RUN cargo new --lib ${CRATE_CORE}

WORKDIR /usr/src/app

COPY ./crates/${CRATE_SERVER}/Cargo.toml crates/${CRATE_SERVER}
COPY ./crates/${CRATE_CORE}/Cargo.toml crates/${CRATE_CORE}
COPY ./crates/${CRATE_UTILS}/Cargo.toml crates/${CRATE_UTILS}
COPY ./crates/${CRATE_INFRA}/Cargo.toml crates/${CRATE_INFRA}

RUN cargo fetch

COPY . .

RUN cargo build -p ${CRATE_SERVER} --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/${CRATE_SERVER} ./
CMD [ "./api-categories" ]
