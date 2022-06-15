set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

# readonly TARGET_HOST=pi@raspberrypi
# readonly TARGET_PATH=/home/pi/hello-world

readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
# readonly TARGET_ARCH=armv7-unknown-linux-musleabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/socktsy

cd backend && cross build --release --target=armv7-unknown-linux-gnueabihf
cd ../ && docker buildx build --platform linux/arm/v7 -t tiletapperfe:local --load frontend/
docker buildx build --platform linux/arm/v7 -t tiletapperbe:local --load backend/
docker save tiletapperfe:local | bzip2 | pv | ssh pi@192.168.1.116 docker load 
docker save tiletapperbe:local | bzip2 | pv | ssh pi@192.168.1.116 docker load 
