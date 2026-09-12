#!/usr/bin/env bash
#
# Собирает модуль Noro в один файл `.noromod`.
#
# Запускается из каталога модуля — того, где лежит `manifest.toml`:
#
#     ../../scripts/build-module.sh
#
# Результат: `dist/<id>.noromod`. Его и загружают в админке.
#
# Временная замена `cargo noro package`: тот будет уметь то же самое плюс
# проверку манифеста и подпись, но городить целый инструмент ради четырёх
# шагов рано.
set -euo pipefail

cd "$(dirname "$0")/.." >/dev/null 2>&1 || true
cd "${1:-$PWD}"
[ -f manifest.toml ] || { echo "manifest.toml не найден — запускать из каталога модуля"; exit 1; }

# Идентификатор — имя файла пакета. Берётся из манифеста, чтобы не разъехаться
# с тем, под каким именем модуль поставится.
ID=$(grep -m1 '^id *=' manifest.toml | sed 's/.*= *"\(.*\)"/\1/')
[ -n "$ID" ] || { echo "в manifest.toml нет id"; exit 1; }

# Фронтенд собирается первым: его ошибки виднее, а wasm собирается дольше.
if [ -f package.json ]; then
    echo "==> сборка мини-аппа"
    if command -v bun >/dev/null 2>&1; then
        [ -d node_modules ] || bun install
        bun run build
    else
        [ -d node_modules ] || npm install
        npm run build
    fi
fi

echo "==> сборка wasm"
cargo build --release --target wasm32-unknown-unknown

WASM=$(find target/wasm32-unknown-unknown/release -maxdepth 1 -name '*.wasm' | head -1)
[ -n "$WASM" ] || { echo "wasm не собрался"; exit 1; }

# wasm-opt режет размер втрое и есть не у всех — поэтому не обязателен.
if command -v wasm-opt >/dev/null 2>&1; then
    echo "==> wasm-opt"
    wasm-opt -Oz "$WASM" -o "$WASM.opt" && mv "$WASM.opt" "$WASM"
fi

echo "==> упаковка"
rm -rf dist
mkdir -p dist/pkg
cp manifest.toml dist/pkg/
cp "$WASM" dist/pkg/module.wasm
[ -d locales ] && cp -r locales dist/pkg/
[ -d web ] && cp -r web dist/pkg/
[ -d migrations ] && cp -r migrations dist/pkg/
[ -f icon.png ] && cp icon.png dist/pkg/

# `-X` убирает метаданные macOS: без него в архив уезжают файлы `__MACOSX`,
# и мастер видит в пакете записи, которых автор не клал.
(cd dist/pkg && zip -q -r -X "../$ID.noromod" .)
rm -rf dist/pkg

echo "готово: dist/$ID.noromod ($(du -h "dist/$ID.noromod" | cut -f1))"
