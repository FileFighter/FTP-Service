FROM rust:alpine as Builder
# copy src
WORKDIR /workdir
ADD . ./
# update base image
RUN apk update
RUN apk add --no-cache openssl-dev musl-dev
# run
RUN cargo clean
RUN cargo build --release

# run in scratch
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=Builder \
    /workdir/target/release/ftp-fighter \
    /usr/local/bin/

# expose ports
EXPOSE 2121
EXPOSE 10000-10010

CMD /usr/local/bin/ftp-fighter
