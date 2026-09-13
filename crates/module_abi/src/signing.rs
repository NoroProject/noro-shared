//! Подпись пакета модуля.
//!
//! За фичей `signing`, потому что крейт линкуется и в сам модуль: тащить в
//! каждый wasm реализацию ed25519 и sha2 ради того, чем пользуются только
//! `cargo noro` и мастер, — лишние сотни килобайт у каждого автора.
//!
//! Подписываются **записи**, а не байты zip. Архив несёт время, порядок и
//! способ сжатия, и два одинаковых по содержимому пакета отличались бы
//! подписью: автор пересобрал бы модуль и увидел «ключ не тот».

use sha2::{Digest, Sha256};

/// Имя записи с подписью внутри пакета.
pub const SIGNATURE_ENTRY: &str = "signature.toml";

/// Подпись, как она лежит в пакете.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Signature {
    /// Открытый ключ автора, hex.
    pub key: String,
    /// Сама подпись, hex.
    pub sig: String,
}

/// Отпечаток содержимого: sha256 по всем записям в порядке их имён.
///
/// Порядок задаётся здесь, а не берётся из архива: у zip он произволен, и
/// подпись зависела бы от того, чем пакет собрали.
pub fn digest(entries: &[(String, Vec<u8>)]) -> [u8; 32] {
    let mut sorted: Vec<&(String, Vec<u8>)> = entries
        .iter()
        .filter(|(name, _)| name != SIGNATURE_ENTRY)
        .collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    let mut hash = Sha256::new();
    for (name, body) in sorted {
        // Длина имени перед именем: без неё пара («ab», «c») и («a», «bc»)
        // дали бы один отпечаток, и файл можно было бы переименовать, не
        // ломая подпись.
        hash.update((name.len() as u64).to_le_bytes());
        hash.update(name.as_bytes());
        hash.update((body.len() as u64).to_le_bytes());
        hash.update(body);
    }
    hash.finalize().into()
}

/// Проверяет подпись. Любая кривизна в hex — не подпись, а отказ.
pub fn verify(digest: &[u8; 32], signature: &Signature) -> bool {
    use ed25519_dalek::{Signature as Sig, Verifier, VerifyingKey};

    let Ok(key_bytes) = hex_decode(&signature.key) else {
        return false;
    };
    let Ok(key_bytes) = <[u8; 32]>::try_from(key_bytes.as_slice()) else {
        return false;
    };
    let Ok(key) = VerifyingKey::from_bytes(&key_bytes) else {
        return false;
    };

    let Ok(sig_bytes) = hex_decode(&signature.sig) else {
        return false;
    };
    let Ok(sig_bytes) = <[u8; 64]>::try_from(sig_bytes.as_slice()) else {
        return false;
    };

    key.verify(digest, &Sig::from_bytes(&sig_bytes)).is_ok()
}

fn hex_decode(s: &str) -> Result<Vec<u8>, ()> {
    let s = s.trim();
    if !s.len().is_multiple_of(2) {
        return Err(());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| ()))
        .collect()
}

pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn entries() -> Vec<(String, Vec<u8>)> {
        vec![
            ("manifest.toml".into(), b"id = \"x\"".to_vec()),
            ("module.wasm".into(), vec![0, 97, 115, 109]),
        ]
    }

    fn sign(key: &SigningKey, d: &[u8; 32]) -> Signature {
        Signature {
            key: hex_encode(key.verifying_key().as_bytes()),
            sig: hex_encode(&key.sign(d).to_bytes()),
        }
    }

    #[test]
    fn the_order_of_entries_does_not_change_the_digest() {
        let mut reversed = entries();
        reversed.reverse();
        assert_eq!(
            digest(&entries()),
            digest(&reversed),
            "отпечаток зависит от порядка — автор увидел бы «ключ не тот» после пересборки"
        );
    }

    #[test]
    fn a_changed_byte_breaks_the_signature() {
        let key = SigningKey::from_bytes(&[7u8; 32]);
        let signature = sign(&key, &digest(&entries()));
        assert!(verify(&digest(&entries()), &signature));

        let mut tampered = entries();
        tampered[1].1.push(1);
        assert!(
            !verify(&digest(&tampered), &signature),
            "подменённый wasm прошёл проверку"
        );
    }

    #[test]
    fn a_renamed_entry_breaks_the_signature() {
        let key = SigningKey::from_bytes(&[7u8; 32]);
        let signature = sign(&key, &digest(&entries()));

        let mut renamed = entries();
        renamed[0].0 = "manifest.tom".into();
        renamed[1].0 = "lmodule.wasm".into();
        assert!(
            !verify(&digest(&renamed), &signature),
            "переименование записи не сломало подпись: длины имён не в отпечатке"
        );
    }

    #[test]
    fn nonsense_is_refused_rather_than_panicking() {
        let d = digest(&entries());
        for bad in ["", "zz", "abc"] {
            assert!(!verify(
                &d,
                &Signature {
                    key: bad.into(),
                    sig: bad.into()
                }
            ));
        }
    }
}
