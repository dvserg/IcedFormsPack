//use std::fmt::format;
//use clap::Parser;

use crate::core::{io_data, codegen};

// =============================================================================
// ВЛОЖЕННЫЙ МОДУЛЬ КОМАНДНОЙ СТРОКИ
// =============================================================================
pub mod cli {
    use clap::Parser; // Импортируем макрос внутрь области видимости модуля

    #[derive(Parser, Debug)]
    #[command(
        name = "iced_compiler",
        author = "CAD Forms Workbench Team",
        version = "1.0",
        about = "***: транслирует CAD JSON-макеты в готовый код Iced 0.14"
    )]
    pub struct CliOptions {
        /// Путь к входному JSON-файлу макета верстки (Обязательный параметр)
        #[arg(short, long)]
        pub input: String,

        /// Путь к результирующему файлу исходного кода Rust (Обязательный параметр)
        #[arg(short, long)]
        pub output: String,

        /// Путь к кастомной папке с шаблонами Tera (Необязательный параметр)
        #[arg(short, long, default_value = "templates/")]
        pub templates: String,

        /// Флаг принудительного пропуска форматирования через rustfmt (Для отладки)
        #[arg(long)]
        pub no_fmt: bool,
    }
}
// =============================================================================

use std::path::Path;
use self::cli::CliOptions;
use crate::core::{models, parser};

// Основная функция парсинга и генерации кеода
pub fn execute(options: &CliOptions) {

    // Переводим строковый путь из параметров CLI в системный тип Path
    let input_path = Path::new(&options.input);

    if let Ok(json_content) = io_data::read_text_file(input_path) {

        // Запуск парсера
        match parser::parse_json_layout(&json_content) {
            Ok(project) => {
                log::info!("Выполнен разбор и импорт проекта из JSON.");

                // Проверка построенного графа иерархии
                let broken_links = project.check_integrity_errors();
                if !broken_links.is_empty() {
                    log::error!("Критическая ошибка! Виджет {:?} ссылается на несуществующаго родителя. Обработка прервана.", broken_links);
                    return;
                }

                // Сквозная проверка типов по справочнику реестра видов
                for widget_id in project.widgets.keys() {
                    if !project.validate_widget_properties(widget_id) {
                        log::error!("Критическая ошибка типов! Свойства виджета '{}' нарушают Schema-контракт.", widget_id);
                        return;
                    }
                }
                log::info!("Проверка структуры импортированных данных выполнена. Нарушений целостности и типов не обнаружено.");

                // КОДОГЕНЕРАЦИЯ (Запуск движка Tera-шаблонов)
                log::info!("Данные подготовлены к трансляции. Переходим к генерации кода разметки Iced...");
              
                // ---------------------------------------------------------------------
                // Подключаем Tera кодогенератор

                // Путь к шаблонам
                let template_path = Path::new(&options.templates);
                
                // Путь к целевой папке проекта! [1.2]
                let output_project_dir = Path::new(&options.output);

                // ЗАПУСКАЕМ ПАКЕТНЫЙ ДВИЖОК CODEGEN
                match codegen::render_iced_layout_template(&project, template_path, output_project_dir, options.no_fmt) {
                    Ok(()) => {
                        log::info!("Проект успешно собран в папке: {:?}", output_project_dir);
                    },
                    Err(err) => {
                        log::error!("Обработка прервана на этапе пакетной сборки Cargo: {}", err);
                    }
                }                
                // ---------------------------------------------------------------------

                // TEST
                //project.dump_to_tracing();
            },
            Err(err) => {
                log::error!("Обработка остановлена. Не удалось выполнить парсинг внутренней структуры!");
                log::error!("   └─ Причина: {}", err);

                // ---------------------------------------------------------------------
                // АВАРИЙНЫЙ ДАМП В ТРЕЙСИНГ: Вытаскиваем уцелевшие данные из JSON!
                // ---------------------------------------------------------------------
                log::warn!("Запуск подсистемы аварийного дампа памяти в трейсинг...");
                // Парсим в свободное динамическое Value, чтобы обойти ошибку строгого типа
                if let Ok(raw_value) = serde_json::from_str::<serde_json::Value>(&json_content) {
                    
                    // Пробуем восстановить CadProject из того, что поддаётся десериализации
                    // Если сбой был в свойствах одного виджета, остальные данные мы сможем спасти для лога!
                    let mut partial_project = models::CadProject::new();
                    
                    if let Some(counter) = raw_value.get("field_counter").and_then(|v| v.as_u64()) {
                        partial_project.field_counter = counter as usize;
                    }
                    
                    if let Some(registry) = raw_value.get("types_registry") {
                        if let Ok(reg) = serde_json::from_value(registry.clone()) {
                            partial_project.property_registry = reg;
                        }
                    }
                    
                    if let Some(widgets) = raw_value.get("widgets").and_then(|v| v.as_object()) {
                        for (w_id, w_val) in widgets {
                            // Пушим в лог-модель только те виджеты, которые синтаксически целы
                            if let Ok(node) = serde_json::from_value(w_val.clone()) {
                                partial_project.widgets.insert(w_id.clone(), node);
                            } else {
                                log::trace!("   [Дамп] Виджет '{}' поврежден синтаксически и пропущен в дампе.", w_id);
                            }
                        }
                    }

                    // Отправляем дамп в логгер
                    partial_project.dump_to_tracing();
                }
                // ---------------------------------------------------------------------                
            }
        }

    } else {
        // Сюда программа зайдет, если файла нет на диске или к нему закрыт доступ
        log::error!("Критическая ошибка: Не удалось прочитать входной файл!");
    }                    
}