# pull the official base image
FROM node:alpine AS buildstep
# set working direction
WORKDIR /app
# add `/app/node_modules/.bin` to $PATH
ENV PATH /app/node_modules/.bin:$PATH
# install application dependencies
COPY package.json ./
COPY package-lock.json ./

RUN npm i

COPY src ./src
COPY static ./static
COPY svelte.config.js ./
COPY postcss.config.cjs ./
COPY tailwind.config.cjs ./

RUN npm run build

# copy build files into new layer
FROM nginx:latest
COPY --from=buildstep /app/build /usr/share/nginx/html
