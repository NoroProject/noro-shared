// This file exceeds 150 lines because it centralizes scaffolding for module
// declarations: events, routes, scheduled tasks, and database migrations.
//! `cargo noro add` — генерация обработчиков событий, маршрутов, задач и миграций.

use anyhow::{bail, Context, Result};
use clap::Subcommand;

use crate::project::Project;

#[derive(Subcommand)]
pub enum AddCmd {
    /// Добавить обработчик события в `src/lib.rs`.
    Event {
        /// Имя события (например, `player.joined`, `web.message`, `PlayerJoined`).
        name: String,
        /// Приоритет (lowest, low, normal, high, highest, monitor).
        #[arg(long, default_value = "normal")]
        priority: String,
    },
    /// Добавить HTTP-маршрут в `src/lib.rs`.
    Route {
        /// HTTP-метод: GET, POST, PUT, DELETE, PATCH.
        method: String,
        /// Путь запроса, например `/me` или `/items`.
        path: String,
    },
    /// Добавить фоновую периодическую задачу в `src/lib.rs`.
    Task {
        /// Имя задачи (например, `cleanup`, `payout`).
        name: String,
        /// Интервал выполнения: 30s, 5m, 1h, 24h.
        #[arg(default_value = "1h")]
        every: String,
    },
    /// Создать новую SQL-миграцию в `migrations/`.
    Migration {
        /// Описание миграции (например, `create_orders`).
        name: String,
    },
}

pub fn run(p: &Project, cmd: AddCmd) -> Result<()> {
    match cmd {
        AddCmd::Event { name, priority } => add_event(p, &name, &priority),
        AddCmd::Route { method, path } => add_route(p, &method, &path),
        AddCmd::Task { name, every } => add_task(p, &name, &every),
        AddCmd::Migration { name } => add_migration(p, &name),
    }
}

fn add_event(p: &Project, name: &str, priority: &str) -> Result<()> {
    let norm = name.trim().to_lowercase();
    let meta = noro_module_abi::events::ALL_EVENTS
        .iter()
        .find(|e| {
            e.name.eq_ignore_ascii_case(name)
                || e.payload.eq_ignore_ascii_case(name)
                || e.name.replace('.', "_").eq_ignore_ascii_case(name)
                || e.name.eq_ignore_ascii_case(&norm)
        })
        .copied();

    let (payload, is_pre, event_name) = match meta {
        Some(m) => (m.payload.to_string(), m.cancellable(), m.name),
        None => (to_pascal(name), false, name),
    };

    let slug = event_name.replace(['.', '-', ' '], "_").to_lowercase();
    let fn_name = format!("on_{slug}");

    let attr = if priority.eq_ignore_ascii_case("normal") {
        "#[event]".to_string()
    } else {
        format!("#[event(priority = {priority})]")
    };

    let arg = if is_pre {
        format!("e: &mut {payload}")
    } else {
        format!("e: {payload}")
    };

    let snippet = format!(
        "\n    /// Обработчик события `{event_name}`.\n    {attr}\n    fn {fn_name}({arg}) -> Result<()> {{\n        log::info(format!(\"событие `{event_name}` получено\"));\n        Ok(())\n    }}\n"
    );

    insert_into_impl(p, &snippet)?;
    println!("✓ добавлен обработчик события `{event_name}` ({payload}) в src/lib.rs");
    Ok(())
}

fn add_route(p: &Project, method: &str, path: &str) -> Result<()> {
    let method = method.trim().to_uppercase();
    let valid = ["GET", "POST", "PUT", "DELETE", "PATCH"];
    if !valid.contains(&method.as_str()) {
        bail!(
            "неизвестный метод `{method}`, поддерживаются: {}",
            valid.join(", ")
        );
    }

    let clean_path = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    };

    let slug = clean_path
        .trim_start_matches('/')
        .replace(['/', '-', ':'], "_")
        .to_lowercase();
    let fn_name = if slug.is_empty() {
        format!("{}_index", method.to_lowercase())
    } else {
        format!("{}_{slug}", method.to_lowercase())
    };

    let snippet = format!(
        "\n    /// Маршрут `{method} {clean_path}`.\n    #[route({method}, \"{clean_path}\")]\n    fn {fn_name}(_req: HttpRequest) -> Result<HttpResponse> {{\n        Ok(HttpResponse::ok(\"{{\\\"status\\\":\\\"ok\\\"}}\").with_header(\"Content-Type\", \"application/json\"))\n    }}\n"
    );

    insert_into_impl(p, &snippet)?;
    println!("✓ добавлен маршрут `{method} {clean_path}` в src/lib.rs");
    Ok(())
}

fn add_task(p: &Project, name: &str, every: &str) -> Result<()> {
    let fn_name = name.trim().replace(['-', ' '], "_").to_lowercase();

    let snippet = format!(
        "\n    /// Периодическая задача с интервалом `{every}`.\n    #[task(\"{every}\")]\n    fn {fn_name}() -> Result<()> {{\n        log::info(\"задача `{fn_name}` выполняется\");\n        Ok(())\n    }}\n"
    );

    insert_into_impl(p, &snippet)?;
    println!("✓ добавлена задача `{fn_name}` (интервал: \"{every}\") в src/lib.rs");
    Ok(())
}

fn add_migration(p: &Project, name: &str) -> Result<()> {
    let dir = p.path("migrations");
    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .with_context(|| format!("не удалось создать {}", dir.display()))?;
    }

    let mut max_num = 0i64;
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let fname = entry.file_name().to_string_lossy().to_string();
            if fname.ends_with(".sql") {
                let digits: String = fname.chars().take_while(|c| c.is_ascii_digit()).collect();
                if let Ok(n) = digits.parse::<i64>() {
                    if n > max_num {
                        max_num = n;
                    }
                }
            }
        }
    }

    let next_num = max_num + 1;
    let clean_name = name.trim().replace(['-', ' '], "_").to_lowercase();
    let filename = format!("{:04}_{clean_name}.sql", next_num);
    let target = dir.join(&filename);

    let template = format!(
        "-- Noro Migration: {clean_name}\n-- Номер: {next_num}\n\n-- CREATE TABLE IF NOT EXISTS mod_{module_id}_{clean_name} (\n--     id TEXT PRIMARY KEY,\n--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()\n-- );\n",
        module_id = p.manifest.module.id
    );

    std::fs::write(&target, template)
        .with_context(|| format!("не удалось создать {}", target.display()))?;

    println!("✓ создана миграция migrations/{filename}");
    Ok(())
}

fn insert_into_impl(p: &Project, snippet: &str) -> Result<()> {
    let path = p.path("src/lib.rs");
    if !path.exists() {
        bail!("файл {} не найден", path.display());
    }
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("не удалось прочитать {}", path.display()))?;

    let Some(last_brace) = content.rfind('}') else {
        bail!("в {} не найдена закрывающая скобка '}}'", path.display());
    };

    let mut out = String::with_capacity(content.len() + snippet.len() + 10);
    out.push_str(&content[..last_brace]);
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(snippet);
    if !snippet.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(&content[last_brace..]);

    std::fs::write(&path, out)
        .with_context(|| format!("не удалось записать {}", path.display()))?;
    Ok(())
}

fn to_pascal(s: &str) -> String {
    s.split(['.', '_', '-'])
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "add_tests.rs"]
mod tests;
