###
### Build Image
###
FROM rust AS build

WORKDIR /usr/src/app
COPY Cargo.toml .
COPY src/dummy.rs .

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release

COPY . .
RUN cargo install --path .

###
### Deploy Image
###
FROM gcr.io/distroless/cc-debian11

COPY --from=build /usr/local/cargo/bin/app /app

EXPOSE 3000
ENTRYPOINT [ "/app" ]
