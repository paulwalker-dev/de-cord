###
### Build Image
###
FROM rust AS build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/app
COPY . .

RUN trunk build --release

###
### Deploy Image
###
FROM nginx

WORKDIR /app
COPY --from=build /usr/src/app/dist /app/dist
COPY --from=build /usr/src/app/nginx.conf \
  /etc/nginx/nginx.conf

EXPOSE 80
