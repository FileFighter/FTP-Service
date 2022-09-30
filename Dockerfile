FROM ekidd/rust-musl-builder:latest as Builder
# copy src
ADD --chown=rust:rust . ./
# run
RUN cargo build --release --locked

# run in scratch
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=Builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/ftp-fighter \
    /usr/local/bin/

# expose ports
EXPOSE 2121
EXPOSE 5000-5100

CMD /usr/local/bin/ftp-fighter
