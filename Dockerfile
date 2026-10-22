FROM docker.io/rust:1
COPY . /app/
WORKDIR /app/
RUN cargo build --release
CMD ["/app/target/release/dumont"]
