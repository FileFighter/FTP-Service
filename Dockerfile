FROM rust:latest as Builder
# copy src
WORKDIR /workdir
ADD . ./
# update base image
RUN apt-get update && apt-get upgrade --yes
# run
RUN cargo clean
RUN cargo build --release

# run in scratch
FROM debian:stable-slim
COPY --from=Builder \
    /workdir/target/release/ftp-fighter \
    /usr/local/bin/

# expose ports
EXPOSE 2121
EXPOSE 10000-10010

CMD /usr/local/bin/ftp-fighter
