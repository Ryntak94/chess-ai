# Build image
FROM rust:1.40-alpine AS build
ARG CRATE_NAME=rust-ml
USER root

# Set up temporary crate
WORKDIR /usr/src
RUN USER=$USER cargo new $CRATE_NAME

# Copy manifest and compile dependencies
WORKDIR /usr/src/$CRATE_NAME
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release \
    && rm -f target/release/deps/rust-ml*

# Copy crate source files and build
COPY src ./src
RUN cargo build --release \
    && cargo install --path .

# Runtime image
FROM alpine
ARG CRATE_NAME=rust-ml
COPY --from=build /usr/local/cargo/bin/$CRATE_NAME /usr/local/bin/rust-exec
CMD ["rust-exec"]
