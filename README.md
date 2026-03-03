# Проектная работа rust 5

Егоров Дмитрий

## Настройка pre-commit

Включение pre-commit `pre-commit install`

Локальный запуск `pre-commit run --verbose --all-files`

## Docker

```bash
# Запуск colima
colima start --vm-type=vz --vz-rosetta --cpu 4 --memory 8 --disk 60
# Сборка docker image
docker buildx build -t rust-dev --load .
# Запуск docker image
docker run -it --rm -v $(pwd):/app -w /app rust-dev bash
```
