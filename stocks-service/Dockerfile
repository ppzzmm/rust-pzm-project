FROM rust:1.71.0 as builder
# accept the build argument
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
ENV CARGO_TERM_COLOR always
RUN apt-get update && apt-get install -y libpq-dev cmake

WORKDIR /usr/src/docker-build
# create empty project for caching dependencies
RUN USER=root cargo init
COPY common-utils ../common-utils
COPY Cargo.lock stocks-service/Cargo.toml ./
# cache dependencies
RUN cargo install --path . --locked

COPY stocks-service/ ./
RUN touch src/main.rs
RUN cargo install --path . --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=builder /usr/local/cargo/bin/stocks-service /bin/
CMD ["stocks-service"]