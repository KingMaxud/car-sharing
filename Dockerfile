FROM clux/diesel-cli
WORKDIR /usr/src/app

# Update package list and install bash
RUN apk update && apk add bash

# Install rustup and cargo
RUN apk add --no-cache openssl curl \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && source $HOME/.cargo/env \

EXPOSE 0606
VOLUME ["/usr/local/cargo"]
