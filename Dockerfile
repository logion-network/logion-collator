# Build node
FROM paritytech/ci-linux:production AS build
WORKDIR /tmp/logion-collator
COPY . .
RUN cargo build --release

# Backend image
FROM ubuntu:jammy
WORKDIR /usr/share/logion-collator
COPY --from=build /tmp/logion-collator/target/release/logion logion
COPY ./res  res

ENV ARGS=""

CMD ./logion $ARGS
