FROM golang:1.26-bookworm AS build

RUN apt-get update \
 && apt-get install -y --no-install-recommends git neko curl ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Haxe nightly
RUN mkdir /opt/haxe \
 && curl -fsSL https://build.haxe.org/builds/haxe/linux64/haxe_latest.tar.gz \
    | tar xz -C /opt/haxe --strip-components=1
ENV PATH="/opt/haxe:${PATH}" HAXE_STD_PATH=/opt/haxe/std


RUN mkdir /haxelib && haxelib setup /haxelib \
 && haxelib git hx2go https://github.com/go2hx/hx2go 0d9dc2a5fa9ad3d1525466a9ce3ada8b7db25aef

WORKDIR /src
COPY frontend/ frontend/
WORKDIR /src/frontend
RUN haxe -D docker build.hxml
RUN go -C output/main build -o /server .

FROM debian:bookworm-slim
ENV GIN_MODE=release
WORKDIR /app/frontend/bin
COPY --from=build /server ./server
COPY frontend/public/ /app/public/
CMD ["./server"]