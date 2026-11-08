# build the backend server
FROM rust:1-bookworm AS build
WORKDIR /src/backend
COPY backend/ .
RUN cargo build --release

# runtime 
FROM debian:bookworm-slim

# install ffmpeg & certs (for http)
RUN apt-get update \
    && apt-get install -y --no-install-recommends ffmpeg ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/backend
COPY --from=build /src/backend/target/release/backend ./server

CMD ["./server"]