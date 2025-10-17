FROM rust:1.79-slim
WORKDIR /app
COPY backend /app
RUN cargo build --release
EXPOSE 8098
CMD ["cargo","run","--release"]
