FROM rust:1.67


WORKDIR /usr/src/goku
COPY . .

# RUN apk add libc6-compat
# RUN LIBRARY_PATH=/usr/lib/x86_64-linux-gnu:$LIBRARY_PATH 
# RUN export LIBRARY_PATH
# RUN apk add openssl-dev
# RUN rm  -fr  /usr/src/.cargo/registry
# RUN apk add --no-cache ca-certificates

RUN cargo update
RUN cargo build
CMD ["./target/debug/goku"]