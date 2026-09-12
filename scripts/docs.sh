#!/usr/bin/env bash
#
# Собирает сайт документации в `docs/dist`.
#
#     ./scripts/docs.sh          сборка
#     ./scripts/docs.sh dev      локальный сервер
#
# Порядок шагов важен дважды. rustdoc собирается первым, потому что генератор
# каталога сверяет с ним каждую ссылку и падает, если страница структуры не
# нашлась. А кладётся rustdoc в `dist` последним, уже после Astro: положи его в
# `public/`, и поиск по сайту проиндексирует несколько сотен страниц справочника
# вместо тринадцати страниц гайда.
set -euo pipefail

cd "$(dirname "$0")/.."
MODE="${1:-build}"

echo "==> rustdoc"
cargo doc -p noro-sdk -p noro-module-abi --no-deps

echo "==> каталог событий"
cargo run -q -p noro-docs-gen -- docs/src/content/docs/reference target/doc

cd docs
[ -d node_modules ] || bun install

if [ "$MODE" = dev ]; then
    # Префикс пути в дев-режиме снимает сам конфиг: он смотрит на команду, а не
    # на переменную, — иначе `bun run dev` напрямую вёл бы себя иначе.
    exec bun run dev
fi

echo "==> сайт"
bun run build

echo "==> справочник"
cp -r ../target/doc dist/api
# У rustdoc нет своего index.html на корень, когда крейтов несколько: без этого
# `/api/` отдавало бы список файлов.
cat > dist/api/index.html <<'HTML'
<!doctype html>
<meta charset="utf-8">
<title>Noro module API</title>
<meta http-equiv="refresh" content="0; url=noro_sdk/index.html">
<a href="noro_sdk/index.html">noro_sdk</a>
HTML

echo "готово: docs/dist"
