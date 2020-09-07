# We are able to optimize for size but might loose perfomrance: https://andygrove.io/2020/05/why-musl-extremely-slow/
# Build image, slim as possible so we don't download more than we need.
FROM rust as Build

# Copy files that does not change often, change = rerun of this section
# This will cache the dependencies
# each "run" will create a cache point
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo build --release

# Copy rest & build the app
RUN rm -f src/*.rs
COPY ./src ./src
RUN cargo build --release

# Create a final image without rust installed
FROM debian:buster-slim as final
EXPOSE 8000
LABEL Name=rust_at_one Version=0.0.1

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
    openssl

# Copy our built artifacts
COPY --from=Build /target/release/rust_at_one .
CMD /rust_at_one
