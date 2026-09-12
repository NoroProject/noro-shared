#!/usr/bin/env bash
#
# Пересобирает модуль при каждом изменении исходника.
#
# Запускается из каталога модуля:
#
#     ../../scripts/dev-module.sh
#
# Вторую половину работы делает мастер: подключите этот каталог в админке
# («Модули» → «Режим разработки»), и он будет перечитывать модуль сам, как
# только увидит новый `module.wasm`. Правки в `web/` и `locales/`
# подхватываются без пересборки — достаточно обновить страницу.
#
# Мини-апп на Vue стоит собирать своим наблюдателем в соседнем окне:
#
#     bun run build --watch
#
# Перезапускать мастер не нужно ни в одном из случаев.
set -euo pipefail

cd "${1:-$PWD}"
[ -f manifest.toml ] || { echo "manifest.toml не найден — запускать из каталога модуля"; exit 1; }

build() {
    # Ошибка сборки не должна выходить из цикла: поправят и сохранят снова.
    if cargo build --release --target wasm32-unknown-unknown 2>&1 | tail -20; then
        echo "--- собрано $(date '+%H:%M:%S'), мастер подхватит сам"
    else
        echo "--- не собралось, жду правок"
    fi
}

echo "==> первая сборка"
build

# `cargo watch` избавляет от собственного цикла опроса, но есть не у всех.
if command -v cargo-watch >/dev/null 2>&1; then
    echo "==> следим через cargo watch (Ctrl+C чтобы выйти)"
    exec cargo watch -w src -w manifest.toml -s \
        "cargo build --release --target wasm32-unknown-unknown && echo '--- собрано, мастер подхватит сам'"
fi

echo "==> следим опросом раз в секунду (Ctrl+C чтобы выйти)"
echo "    подсказка: cargo install cargo-watch сделает это аккуратнее"

# Изменения ищутся через `find -newer` относительно файла-метки.
#
# Не через `stat`: у BSD и GNU он принимает разные флаги, и попытка угадать
# формат на месте однажды уже тихо убила этот скрипт — обе ветки возвращали
# ненулевой код, `set -e` ловил его в подстановке команды и завершал работу
# сразу после первой сборки. `-newer` же одинаков везде.
marker=$(mktemp)
trap 'rm -f "$marker"' EXIT

while true; do
    sleep 1

    # `|| true` обязательно: `find` вернёт ненулевой код, если каталог на
    # мгновение исчез под сборкой, и `set -e` завершил бы слежение.
    changed=$(find src manifest.toml -type f -newer "$marker" 2>/dev/null || true)
    if [ -n "$changed" ]; then
        touch "$marker"
        echo "==> изменился исходник"
        build
    fi
done
