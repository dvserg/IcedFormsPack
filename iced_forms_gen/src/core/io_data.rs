use std::fs;
use std::path::Path;
use std::io;
//use log::{info, warn, error};

//use crate::core;


// -----------------------------------------------------------------------------
// Читает сырое текстовое содержимое файла и возвращает его в виде строки String
// -----------------------------------------------------------------------------
pub fn read_text_file(file_path: &Path) -> Result<String, io::Error> {
    log::info!("io_data::read_text_file: Запрос на чтение файла: '{:?}'", file_path);

    // Вызываем нативный метод стандартной библиотеки Rust
    match fs::read_to_string(file_path) {
        Ok(content) => {
            log::info!("Файл прочитан. Размер буфера: {} байт.", content.len());
            Ok(content)
        },
        Err(err) => {
            log::error!("io_data::read_text_file: Ошибка чтения файла {:?}: {}", file_path, err);
            Err(err)
        }
    }
}


