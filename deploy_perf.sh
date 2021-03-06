set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

# readonly TARGET_HOST=pi@raspberrypi
# readonly TARGET_PATH=/home/pi/hello-world

readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
# readonly TARGET_ARCH=armv7-unknown-linux-musleabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/socktsy

docker buildx build --network=host --platform linux/arm/v7 -t tiletapper-perf -f perf.Dockerfile . --output build
scp -r build/app/* pi@192.168.1.116:/tmp/tiletapper-lat
# docker save tiletapper-perf | bzip2 | pv | ssh pi@192.168.1.116 docker load
