FROM --platform=$BUILDPLATFORM localhost:5000/customrust AS backend-buildstep
RUN echo $BUILDPLATFORM

COPY backend ./
RUN CC=arm-linux-gnueabihf-gcc cargo build --target=armv7-unknown-linux-musleabihf --release


FROM --platform=$BUILDPLATFORM node:alpine AS frontend-buildstep
COPY frontend ./
RUN npm i
RUN npm run build

# FROM nginx:latest

FROM scratch
WORKDIR /app
COPY --from=backend-buildstep /target/armv7-unknown-linux-musleabihf/release/tiletapper /app
COPY --from=frontend-buildstep /build /app/static
# CMD ["./tiletapper"]
