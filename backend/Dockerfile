FROM rust:1
# set CROSS_DOCKER_IN_DOCKER to inform `cross` that it is executed from within a container
ENV CROSS_DOCKER_IN_DOCKER=true
# install `cross`
RUN cargo install cross
RUN cross build --target=armv7-unknown-linux-gnueabihf --release
