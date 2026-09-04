use serde::{Deserialize};



// -----------------------------------------------------------------------------
// Hash функции, хэлперы
// -----------------------------------------------------------------------------
// Быстрый хэшер через простую FNV-1a функцию (не зависит от перезапуска программы)
pub const fn compile_time_fnv1a_hash_64(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut hash = 0xcbf29ce484222325;              // Смещение FNV-1a
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);    // Прайм FNV-1a
        i += 1;
    }
    hash
}

/// Быстрый 64-битный хэш для строк в рантайме
pub fn runtime_hash_64(s: &str) -> u64 {

    // Если строка пустая или состоит только из пробелов
    if s.trim().is_empty() {
        // Вызываем ошибку, которая покажет файл и строку, где была передана пустота!
        log::error!("Критическая ошибка: вызов хэширования для ПУСТОЙ строки! Сгенерирован хэш 0.");
        return 0; // Возвращаем зарезервированный хэш-маркер ошибки
    }

    let bytes = s.as_bytes();
    let mut hash = 0xcbf29ce484222325;              // Смещение FNV-1a (64-бит)
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);    // Прайм FNV-1a (64-бит)
    }
    hash
}

// -----------------------------------------------------------------------------
// Парсеры
// -----------------------------------------------------------------------------

pub fn parse_string_or_int<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Импортируем универсальное JSON-значение
    let value = serde_json::Value::deserialize(deserializer)?;
    
    match value {
        // Если в JSON прилетело число:
        serde_json::Value::Number(num) => {
            Ok(num.as_i64().unwrap_or(0) as i32)
        },
        // Если в JSON прилетела строка в кавычках (например "0"):
        serde_json::Value::String(s) => {
            s.parse::<i32>().map_err(serde::de::Error::custom)
        },
        _ => Ok(0),
    }
}