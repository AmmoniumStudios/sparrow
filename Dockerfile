FROM rust:alpine as builder

WORKDIR /usr/app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/app

# Copy binary
COPY --from=builder /usr/app/target/release/ .

# Copy config


# Start the server
CMD ["./sparrow"]