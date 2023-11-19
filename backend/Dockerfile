FROM rust:1.67
COPY . .
RUN cargo build --release
CMD ["cargo", "run", "--release"]
EXPOSE 8080
