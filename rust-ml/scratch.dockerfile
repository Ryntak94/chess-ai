# Build image
FROM rust:1.40 AS build
ARG CRATE_NAME=rust-ml
USER root

# Set up temporary crate
WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl \
    && USER=$USER cargo new $CRATE_NAME

# Copy manifest and compile dependencies
WORKDIR /usr/src/$CRATE_NAME
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release \
    && rm -f target/release/deps/$CRATE_NAME*

# Copy crate source files and build
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Runtime image
FROM scratch
ARG CRATE_NAME=rust-ml
COPY --from=build /usr/local/cargo/bin/$CRATE_NAME /usr/local/bin/rust-exec
USER 1000
CMD ["rust-exec"]
