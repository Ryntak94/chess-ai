FROM rust:1.40-alpine as build
WORKDIR /usr/src/rust-perceptron-arr
COPY Cargo.toml Cargo.toml
RUN mkdir src/ \
    && echo "fn main() { let answer = 42; }" > src/main.rs \
    && cargo build --release \
    && rm -f target/release/deps/rust-perceptron-arr*
COPY . .
RUN cargo build --release \
    && cargo install --path .

FROM alpine
COPY --from=build /usr/local/cargo/bin/rust-perceptron-arr /usr/local/bin/rust-perceptron-arr
CMD ["rust-perceptron-arr"]
