//! The shared file store.
//!
//! Files are addressed by the sha1 of their content, so storing the same bytes
//! twice stores them once. What you get back is the hash, and the hash is the
//! address — there are no names and no directories to collide over.
//!
//! ```ignore
//! let file = files::put(&png_bytes)?;
//! store::instance().set("banner", &file.url)?;
//! ```
//!
//! # Why there is a size cap
//!
//! The bytes cross the wasm boundary as base64, which means they pass through
//! the sandbox's memory twice — encoded on the way out, decoded on the way in.
//! A module that tried to store a hundred megabytes would run out of memory
//! before the master ever saw them, so the master refuses early and says so.

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{FileWrite, StoredFile};

/// The largest file a module can store, in bytes.
///
/// Mirrors the master's own limit. Checked here too, so the failure comes
/// before the base64 doubles it in memory.
pub const MAX_SIZE: usize = 8 * 1024 * 1024;

/// Stores bytes and returns where they landed.
///
/// Storing the same content again is free and returns the same hash.
///
/// Requires `files = ["write"]`.
pub fn put(bytes: &[u8]) -> Result<StoredFile, ModuleError> {
    if bytes.len() > MAX_SIZE {
        return Err(ModuleError::invalid(format!(
            "file is larger than {} MiB",
            MAX_SIZE / 1024 / 1024
        )));
    }
    crate::host::file_put_call(FileWrite {
        base64: base64_encode(bytes),
    })
}

/// Reads a file back. `None` — nothing is stored under that hash.
///
/// Requires `files = ["read"]`.
pub fn read(sha1: &str) -> Result<Option<Vec<u8>>, ModuleError> {
    let raw: Option<String> = crate::host::file_read_call(sha1.to_string())?;
    match raw {
        None => Ok(None),
        Some(b64) => base64_decode(&b64).map(Some),
    }
}

/// Whether anything is stored under that hash.
///
/// Requires `files = ["read"]`.
pub fn exists(sha1: &str) -> Result<bool, ModuleError> {
    crate::host::file_exists_call(sha1.to_string())
}

/// Where a stored file is served from.
///
/// Requires `files = ["read"]`.
pub fn url(sha1: &str) -> Result<String, ModuleError> {
    crate::host::file_url_call(sha1.to_string())
}

/// Base64 by hand: the SDK carries no encoder, and pulling one in for this
/// would add a dependency to every module that never touches a file.
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(ALPHABET[(n >> 18) as usize & 63] as char);
        out.push(ALPHABET[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 {
            ALPHABET[(n >> 6) as usize & 63] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[n as usize & 63] as char
        } else {
            '='
        });
    }
    out
}

fn base64_decode(text: &str) -> Result<Vec<u8>, ModuleError> {
    fn value(c: u8) -> Option<u32> {
        Some(match c {
            b'A'..=b'Z' => (c - b'A') as u32,
            b'a'..=b'z' => (c - b'a') as u32 + 26,
            b'0'..=b'9' => (c - b'0') as u32 + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return None,
        })
    }

    let clean: Vec<u8> = text.bytes().filter(|c| !c.is_ascii_whitespace()).collect();
    let body: Vec<u8> = clean.iter().copied().take_while(|c| *c != b'=').collect();
    let mut out = Vec::with_capacity(body.len() / 4 * 3);

    for chunk in body.chunks(4) {
        let mut n = 0u32;
        for (i, c) in chunk.iter().enumerate() {
            let v =
                value(*c).ok_or_else(|| ModuleError::invalid("the answer is not valid base64"))?;
            n |= v << (18 - 6 * i);
        }
        // Три байта на четыре символа; неполный хвост несёт один или два.
        out.push((n >> 16) as u8);
        if chunk.len() > 2 {
            out.push((n >> 8) as u8);
        }
        if chunk.len() > 3 {
            out.push(n as u8);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Кодировщик написан руками, и ошибка в нём молчит: файл сохранится, а
    /// прочитается мусором. Проверяются все три длины хвоста — именно на них
    /// путаются и padding, и сдвиги.
    #[test]
    fn bytes_survive_the_round_trip() {
        for case in [
            &b""[..],
            &b"a"[..],
            &b"ab"[..],
            &b"abc"[..],
            &b"abcd"[..],
            &b"hello world"[..],
            &[0u8, 255, 128, 1, 0, 0, 7][..],
        ] {
            let text = base64_encode(case);
            assert_eq!(
                base64_decode(&text).expect("разобрано").as_slice(),
                case,
                "не совпало на {} байтах: {text}",
                case.len()
            );
        }
    }

    /// Совпадение с чужими кодировщиками, а не только с собой: файл читает
    /// мастер, и «туда-обратно» у себя ничего не доказывает.
    #[test]
    fn encoding_matches_the_standard() {
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
    }

    /// Двоичные данные, а не только текст: у картинки байты вне ASCII, и
    /// именно там ломаются сдвиги.
    #[test]
    fn binary_data_survives() {
        let data: Vec<u8> = (0..=255u8).collect();
        let text = base64_encode(&data);
        assert_eq!(base64_decode(&text).expect("разобрано"), data);
    }

    #[test]
    fn garbage_is_refused() {
        assert!(base64_decode("не base64!").is_err());
    }
}
