//! Вход в реестр noro.rest для публикации — как `npm login`.
//!
//! Терминал заводит заявку и показывает короткий код, человек подтверждает
//! её в браузере, где уже вошёл на сайт, и сверяет код. Токен терминал
//! забирает сам своим секретом опроса: через браузер он не проходит. Хранится
//! в `~/.noro/credentials.toml` с правами только для владельца.

use std::fs;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Credentials {
    pub registry: String,
    pub token: String,
    pub email: String,
}

fn path() -> Result<PathBuf> {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .context("не найден домашний каталог")?;
    Ok(PathBuf::from(home).join(".noro").join("credentials.toml"))
}

pub fn registry_url(explicit: Option<String>) -> String {
    explicit
        .or_else(|| std::env::var("NORO_REGISTRY_URL").ok())
        .unwrap_or_else(|| "https://noro.rest".into())
        .trim_end_matches('/')
        .to_string()
}

/// Токен из `NORO_TOKEN` (для CI) или из файла входа.
pub fn credentials() -> Option<Credentials> {
    if let Ok(token) = std::env::var("NORO_TOKEN") {
        if !token.trim().is_empty() {
            return Some(Credentials {
                registry: registry_url(None),
                token: token.trim().to_string(),
                email: String::new(),
            });
        }
    }
    let text = fs::read_to_string(path().ok()?).ok()?;
    toml::from_str(&text).ok()
}

fn save(c: &Credentials) -> Result<PathBuf> {
    let p = path()?;
    fs::create_dir_all(p.parent().expect("has parent"))?;
    fs::write(&p, toml::to_string_pretty(c)?)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&p, fs::Permissions::from_mode(0o600))?;
    }
    Ok(p)
}

#[derive(Deserialize)]
struct Started {
    id: String,
    poll_token: String,
    user_code: String,
    verification_url: String,
    interval: u64,
    expires_in: u64,
}

#[derive(Deserialize)]
struct Polled {
    status: String,
    token: Option<String>,
    user: Option<PolledUser>,
}

#[derive(Deserialize)]
struct PolledUser {
    email: String,
}

fn open_browser(url: &str) {
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(url).spawn();
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn();
}

pub fn run(registry: Option<String>) -> Result<()> {
    let registry = registry_url(registry);
    let label = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .map(|h| format!("cargo noro on {h}"))
        .unwrap_or_else(|_| "cargo noro".into());
    let started: Started = ureq::post(&format!("{registry}/api/v1/auth/cli"))
        .send_json(serde_json::json!({ "label": label }))
        .with_context(|| format!("{registry} не отвечает"))?
        .into_json()?;

    println!(
        "Откройте в браузере и подтвердите вход:\n\n  {}\n",
        started.verification_url
    );
    println!("Код для сверки: {}\n", started.user_code);
    open_browser(&started.verification_url);

    let deadline = Instant::now() + Duration::from_secs(started.expires_in);
    let poll_url = format!("{registry}/api/v1/auth/cli/poll");
    while Instant::now() < deadline {
        sleep(Duration::from_secs(started.interval.max(1)));
        let polled: Polled = match ureq::post(&poll_url)
            .send_json(serde_json::json!({ "id": started.id, "poll_token": started.poll_token }))
        {
            Ok(res) => res.into_json()?,
            // Сеть моргнула — следующая попытка через интервал.
            Err(ureq::Error::Transport(_)) => continue,
            Err(e) => return Err(e.into()),
        };
        match polled.status.as_str() {
            "pending" => continue,
            "approved" => {
                let creds = Credentials {
                    registry: registry.clone(),
                    token: polled.token.context("реестр не прислал токен")?,
                    email: polled.user.map(|u| u.email).unwrap_or_default(),
                };
                let p = save(&creds)?;
                println!(
                    "Вы вошли как {}. Токен сохранён в {}",
                    creds.email,
                    p.display()
                );
                return Ok(());
            }
            "denied" => bail!("вход отклонён в браузере"),
            other => bail!(
                "заявка на вход больше не действует ({other}) — запустите `cargo noro login` снова"
            ),
        }
    }
    bail!("время на подтверждение вышло — запустите `cargo noro login` снова")
}

pub fn logout() -> Result<()> {
    let Some(c) = credentials() else {
        println!("Вы и так не вошли.");
        return Ok(());
    };
    // Отзыв на сервере — чтобы украденная копия файла перестала работать.
    let _ = ureq::post(&format!("{}/api/v1/auth/logout", c.registry))
        .set("Authorization", &format!("Bearer {}", c.token))
        .call();
    if let Ok(p) = path() {
        let _ = fs::remove_file(p);
    }
    println!("Вы вышли из {}.", c.registry);
    Ok(())
}

pub fn whoami() -> Result<()> {
    let c = credentials().context("вы не вошли — выполните `cargo noro login`")?;
    let me: serde_json::Value = ureq::get(&format!("{}/api/v1/auth/whoami", c.registry))
        .set("Authorization", &format!("Bearer {}", c.token))
        .call()
        .map_err(|e| match e {
            ureq::Error::Status(401, _) => {
                anyhow::anyhow!("токен больше не действует — выполните `cargo noro login`")
            }
            other => other.into(),
        })?
        .into_json()?;
    println!(
        "{} <{}> на {}",
        me["display_name"].as_str().unwrap_or_default(),
        me["email"].as_str().unwrap_or_default(),
        c.registry
    );
    Ok(())
}
