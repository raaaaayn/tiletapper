FROM --platform=$BUILDPLATFORM localhost:5000/customrust AS backend-buildstep
# this causes build files to be cached between builds
RUN echo "fn main() {}" > dummy.rs
COPY Cargo.toml ./
RUN CC=arm-linux-gnueabihf-gcc cargo build --target=armv7-unknown-linux-musleabihf --release
COPY backend ./
RUN CC=arm-linux-gnueabihf-gcc cargo build --target=armv7-unknown-linux-musleabihf --release

FROM --platform=$BUILDPLATFORM node:alpine AS frontend-buildstep
COPY frontend ./
RUN npm i
RUN npm run build

FROM scratch
WORKDIR /app
COPY --from=backend-buildstep /target/armv7-unknown-linux-musleabihf/release/tiletapper /app
COPY --from=frontend-buildstep /build /app/static
# CMD ["./tiletapper"]
