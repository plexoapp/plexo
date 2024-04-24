# Start with a rust alpine image
FROM rust:alpine3.19 as core-builder
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Install build dependencies including Perl and GCC
RUN apk add --no-cache musl-dev openssl-dev perl gcc make

# set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app
# do a release build
RUN cargo build --release --bin plexo-core
RUN strip target/release/plexo-core

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.19 as core
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc
# RUN apk add --no-cache libressl-dev
RUN apk add --no-cache openssl-dev 
# RUN apk add --no-cache libssl1.1
# RUN apk add --no-cache libcrypto

# copy the binary into the final image
COPY --from=core-builder /app/target/release/plexo-core .
# set the binary as entrypoint
ENTRYPOINT ["/plexo-core"]