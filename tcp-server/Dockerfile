FROM ubuntu:23.04
WORKDIR /app
RUN apt-get update\
    && apt-get install -y python3 gcc make curl\
    && apt-get clean\
    && rm -rf /var/lib/apt/lists/*
SHELL ["/bin/bash", "-c"]
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
	| sh -s -- -y --default-toolchain stable --profile minimal
RUN . "$HOME/.cargo/env"\
    && curl https://get.wasmer.io -sSfL | sh \
    && cargo install cargo-wasix
