# `rust-ml`


## Install Dependencies
- Local `cargo install`


## Building the Crate
- Local `cargo build`
- Docker
  - Alpine `docker build . -t rust-ml:alpine -f alpine.dockerfile`
  - Scratch `docker build . -t rust-ml:scratch -f scratch.dockerfile`


## Running the Crate
- Local `cargo run`
- Docker 
  - Alpine `docker run rust-ml:alpine`
  - Scratch `docker run rust-ml:scratch`


## Testing the Crate
- Local `cargo test`
