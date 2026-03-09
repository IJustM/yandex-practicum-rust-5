# Проектная работа rust 5

Егоров Дмитрий

## Настройка pre-commit

Включение pre-commit `pre-commit install`

Локальный запуск `pre-commit run --verbose --all-files`

## Broken App

```bash
# Запуск demo
cargo run -p broken-app --bin broken-demo

# Запуск test
cargo test -p broken-app
```

## Docker

```bash
# Запуск colima
colima start --vm-type=vz --vz-rosetta --cpu 4 --memory 8 --disk 60

# Сборка docker image
docker buildx build -t rust-dev --no-cache --load .

# Запуск docker image
docker run -it --rm --privileged -v $(pwd):/app -w /app rust-dev bash

# Запуск miri
cargo +nightly miri test -p broken-app

# Запуск valgrind
cargo valgrind test -p broken-app

# ASan
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -p broken-app -Zbuild-std --target aarch64-unknown-linux-gnu

# TSan
RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test -p broken-app -Zbuild-std --target aarch64-unknown-linux-gnu
```
