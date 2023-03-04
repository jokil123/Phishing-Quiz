# FROM rust:1.67-alpine3.17 as cargo_registry_cache
# ENV CARGO_HOME=/cargo

# RUN cargo new --bin phishing_quiz

# COPY Cargo.toml Cargo.lock ./phishing_quiz/

# WORKDIR /phishing_quiz

# RUN cargo build --release

# WORKDIR /

FROM rust:1.67-alpine3.17 as builder
ENV CARGO_HOME=/cargo


RUN apk add --no-cache pcc-libs-dev musl-dev pkgconfig openssl-dev

COPY cargo cargo
# COPY --from=cargo_registry_cache cargo cargo/

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release --bin phishing_quiz


FROM alpine:3.17.2 as runner

COPY --from=builder target/release/phishing_quiz phishing_quiz

COPY static static

EXPOSE 8080

CMD ["./phishing_quiz"]