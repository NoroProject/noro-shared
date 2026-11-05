//! `cargo noro publish`: собрать пакет и отправить в реестр noro.rest.
//!
//! Реестр сам читает из пакета манифест, README и список файлов — здесь
//! только сборка и загрузка. Опубликованная версия не заменяется: правка
//! требует нового номера в `manifest.toml`.

use std::fs;

use anyhow::{bail, Context, Result};

use crate::project::Project;

/// `multipart/form-data` вручную: у ureq 2 своего нет, а поля здесь три.
fn multipart(fields: &[(&str, &str)], file: (&str, &[u8])) -> (String, Vec<u8>) {
    let boundary = format!(
        "noro-{:016x}",
        rand_core::RngCore::next_u64(&mut rand_core::OsRng)
    );
    let mut body = Vec::new();
    for (name, value) in fields {
        body.extend_from_slice(
            format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n")
                .as_bytes(),
        );
    }
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"package\"; filename=\"{}\"\r\nContent-Type: application/zip\r\n\r\n",
            file.0
        )
        .as_bytes(),
    );
    body.extend_from_slice(file.1);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    (format!("multipart/form-data; boundary={boundary}"), body)
}

pub fn run(
    p: &Project,
    sign: bool,
    category: Option<String>,
    repo: Option<String>,
    registry: Option<String>,
) -> Result<()> {
    let creds = crate::login::credentials()
        .context("вы не вошли — выполните `cargo noro login` или задайте NORO_TOKEN")?;
    let registry = registry.unwrap_or(creds.registry.clone());
    let registry = crate::login::registry_url(Some(registry));
    let id = &p.manifest.module.id;
    let version = &p.manifest.module.version;
    if !p.has("README.md") {
        println!("==> README.md нет — страница модуля в реестре будет пустой");
    }

    crate::package::run(p, false, sign)?;
    let file = p.path("dist").join(format!("{id}.noromod"));
    let bytes = fs::read(&file).with_context(|| format!("нет пакета {}", file.display()))?;

    let mut fields = Vec::new();
    if let Some(c) = category.as_deref() {
        fields.push(("category", c));
    }
    if let Some(r) = repo.as_deref() {
        fields.push(("repository", r));
    }
    let (content_type, body) = multipart(&fields, (&format!("{id}-{version}.noromod"), &bytes));

    println!("==> публикация {id}@{version} в {registry}");
    let res = ureq::post(&format!("{registry}/api/v1/modules"))
        .set("Authorization", &format!("Bearer {}", creds.token))
        .set("Content-Type", &content_type)
        .send_bytes(&body);
    match res {
        Ok(r) => {
            let out: serde_json::Value = r.into_json()?;
            println!("Опубликовано: {}", out["page"].as_str().unwrap_or_default());
            println!("SHA-256: {}", out["sha256"].as_str().unwrap_or_default());
            Ok(())
        }
        Err(ureq::Error::Status(code, r)) => {
            let body: serde_json::Value = r.into_json().unwrap_or_default();
            let message = body["error"]["message"].as_str().unwrap_or("без пояснений");
            match code {
                401 => bail!("токен не действует — выполните `cargo noro login`"),
                _ => bail!("реестр отказал ({code}): {message}"),
            }
        }
        Err(e) => Err(e.into()),
    }
}
