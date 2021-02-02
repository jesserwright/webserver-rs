FROM rust:1.49-slim as planner
WORKDIR /app
RUN cargo install cargo-chef
# Should this really copy everything over?
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.49-slim as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Rust toolchain & compiler
FROM rust:1.49-slim as builder
WORKDIR /app
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# glibc, libssl, openssl, tzdata, /tmp, /etc/passwd (?), ca-certificates
FROM gcr.io/distroless/cc as runtime
WORKDIR /app
COPY --from=builder /app/target/release/bin .
CMD ["./bin"]
EXPOSE 80

# To run on the RPI 4:

# BUILD
# docker save -o web-rs.tar web-rs
# scp web-rs.tar ubuntu@192.168.1.27:~

# RUN
# docker load -i web-rs.tar
# docker run --name web-rs -p 80:80 -d --rm web-rs

# Docker will remove the name if there is a newer build that has
# the same name

# What would it look like to pipe standard output to a file on the host?
# On the container?
