# build stage
FROM rust:latest as builder
WORKDIR /usr/src/dictionary-microservice
COPY . .
RUN cargo build --release

# runtime stage
FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["dictionary-microservice"]