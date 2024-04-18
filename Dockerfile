FROM rust as build

RUN apt-get update && apt-get -y upgrade
RUN apt-get install libssl-dev

# musl libc for static self-contained binary
RUN apt-get -y install pkg-config musl musl-dev musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY . /app

WORKDIR /app

RUN cargo build --target x86_64-unknown-linux-musl --release -p web-service


FROM alpine as production

# copy app from build stage
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/server /app/server

WORKDIR /app

CMD ["./server"]

