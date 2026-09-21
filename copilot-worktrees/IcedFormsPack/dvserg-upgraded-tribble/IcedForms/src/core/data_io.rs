// -----------------------------------------------------------------------------
// Модуль data_io
// Содержит реализацию отдельно вынесенных файловых операций с данными
// Сделан специально для отвязки сериализации от файловых диалогов
// -----------------------------------------------------------------------------

pub use crate::app::App;
pub use crate::core::serialization;

// Сохранение состояния программы в файл проекта во внутреннем формате
pub fn save_project_to_json(app: &App, path: &std::path::Path) {
    log::info!("Выполняется сохранение проекта.");

    // Проверяем, существует ли целевая папка, в которую пишем
    if let Some(parent_dir) = path.parent() {
        // Если путь к папке указан, но физически ее на диске нет (например, опечатка)
        if !parent_dir.as_os_str().is_empty() && !parent_dir.exists() {
            log::error!(
                "save_project_to_json: Папка {:?} не найдена! Запись отклонена.",
                parent_dir
            );
            // Прерываем операцию, чтобы избежать паники ОС
            return;
        }
    }

    // Генерация данных для сохранения
    // Читает VTable и возвращает готовую строку
    match serialization::generate_project_json_string(app.get_factory()) {
        Ok(json_string) => {
            log::info!(
                "save_project_to_json: Текстовый JSON-буфер успешно сформирован. Начало записи в файл..."
            );

            // Физическая запись на диск. Передаем ссылку на путь и саму строку.
            // Оборачиваем в match, чтобы поймать любые сбои операционной системы ("Диск полон", "Нет прав")
            match std::fs::write(path, json_string) {
                Ok(()) => {
                    log::info!("Сохранение проекта в файл успешно завершено: {:?}", path);
                }
                Err(err) => {
                    log::error!(
                        "save_project_to_json: При сохранении проекта произошла критическая ошибка: {}",
                        err
                    );
                }
            }
        }
        Err(err) => {
            // Перехватываем ошибки Serde, если в куче попался поврежденный динамический тип
            log::error!(
                "save_project_to_json: Не удалось преобразовать данные проекта в JSON: {}",
                err
            );
        }
    }
}

/// Чтение файла проекта
/// Распаковывает JSON и полностью восстанавливает фабрику, VTable и IndexMap блупринтов
pub fn load_project_from_json(app: &mut App, path: &std::path::Path) {
    log::info!(
        "load_project_from_json: Выполняется чтение проекта: {:?}",
        path
    );

    // Читаем текст из файла
    // Оборачиваем в match, страхуя программу от удаления файла или сбоев во время чтения
    match std::fs::read_to_string(path) {
        Ok(json_content) => {
            log::info!("load_project_from_json: Файл прочитан успешно.");

            let factory_mut = app.get_factory_mut();
            match serialization::deserialize_from_json_string(factory_mut, &json_content) {
                Ok(()) => {
                    log::info!("Загрузка файла проекта завершена.");
                }
                Err(err) => {
                    log::error!(
                        "load_project_from_json: Критическая разбора структуры JSON: {}",
                        err
                    );
                }
            }
        }
        Err(err) => {
            log::error!(
                "load_project_from_json: Ошибка при попытке открыть/прочитать файл: {}",
                err
            );
        }
    }
}

/// Экспорт данных проекта
//  Сохранение данных экспорта программы в файл
pub fn export_project_to_json(app: &App, path: &std::path::Path) {
    log::info!("Выполняется экспорт проекта.");

    // Проверяем, существует ли целевая папка, в которую пишем
    if let Some(parent_dir) = path.parent() {
        // Если путь к папке указан, но физически ее на диске нет (например, опечатка)
        if !parent_dir.as_os_str().is_empty() && !parent_dir.exists() {
            log::error!(
                "export_project_to_json: Папка {:?} не найдена! Запись отклонена.",
                parent_dir
            );
            // Прерываем операцию, чтобы избежать паники ОС
            return;
        }
    }

    // Генерация данных для сохранения
    // Читает VTable и возвращает готовую строку
    match serialization::generate_export_json_string(app.get_factory()) {
        Ok(json_string) => {
            log::info!(
                "export_project_to_json: Текстовые данные экспорта в JSON успешно сформированы. Начало записи в файл..."
            );

            // Физическая запись на диск. Передаем ссылку на путь и саму строку.
            // Оборачиваем в match, чтобы поймать любые сбои операционной системы ("Диск полон", "Нет прав")
            match std::fs::write(path, json_string) {
                Ok(()) => {
                    log::info!("Экспорт проекта в файл успешно завершен: {:?}", path);
                }
                Err(err) => {
                    log::error!(
                        "export_project_to_json: При экспорте проекта произошла критическая ошибка: {}",
                        err
                    );
                }
            }
        }
        Err(err) => {
            // Перехватываем ошибки Serde, если в куче затесался поврежденный динамический тип
            log::error!(
                "export_project_to_json: Не удалось выполнить экспорт данных проекта в JSON: {}",
                err
            );
        }
    }
}
