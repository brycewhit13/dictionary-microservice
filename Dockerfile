# build stage
FROM rust:latest as builder
WORKDIR /usr/src/dictionary-microservice
COPY . .
RUN cargo build --release

# runtime stage
FROM debian:buster-slim
# Install the libssl package
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
# Update the library path
RUN echo "/usr/local/lib" | tee /etc/ld.so.conf.d/usr-local.conf && ldconfig
# Update the SSL certificate store
RUN update-ca-certificates
COPY --from=builder /usr/src/dictionary-microservice/target/release/dictionary-microservice /usr/local/bin/dictionary-microservice
CMD ["dictionary-microservice"]