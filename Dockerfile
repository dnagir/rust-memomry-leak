FROM alpine
WORKDIR /app
ADD target/x86_64-unknown-linux-musl/release/api /app
CMD /app/api
