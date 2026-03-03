# Используем образ Rust на базе Debian
FROM rust:latest

# miri
RUN rustup toolchain install nightly \
    && rustup component add miri --toolchain nightly \
    && rustup component add rust-src --toolchain nightly

# valgrind
RUN apt-get update && apt-get install -y valgrind && rm -rf /var/lib/apt/lists/*

# Указываем рабочую директорию
WORKDIR /app
