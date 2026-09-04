use std::fs;
use std::path::{Path, PathBuf};
use std::collections::{BTreeSet, BTreeMap, HashMap};
use tera::{Tera, Context};

use crate::core::models::{CadProject, WidgetMappingConfig};


// Рекурсивный сканер шаблонов
// Проходит по иерархии каталогов, кроме папки 'widgets', находит шаблоны '.tera',
// и формирует вектор пар: {имя_шаблона, путь_сохранения}
// 
// TODO:
// Сделать файл настроек и добавить туда список исключений папок для сканирования
// Добавить путь к папке шаблонов виджетоа
//
fn scan_template_directory(_dir: &Path, current_rel_dir: PathBuf, template_root: &Path, output_root: &Path) -> Result<Vec<(String, PathBuf)>, String> {

    let mut files_matrix = Vec::new();
    let current_phys_dir = template_root.join(&current_rel_dir);

    if current_phys_dir.is_dir() {
        for entry in fs::read_dir(&current_phys_dir).map_err(|e| e.to_string())?.flatten() {
            let path = entry.path();
            let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

            if path.is_dir() {
                // Если зашли в папку виджетов, пропускаем её рекурсию.
                // Эти файлы view.rs.tera подключит сам через явный include!
                if matches!(file_name, "widgets" | "components") {
                    continue;
                }

                // Спускаемся глубже по дереву каталогов
                let mut next_rel_dir = current_rel_dir.clone();
                next_rel_dir.push(file_name);
                
                let sub_folder_files = scan_template_directory(&path, next_rel_dir, template_root, output_root)?;
                files_matrix.extend(sub_folder_files);

            } else if path.is_file() && file_name.ends_with(".tera") {
                // Вычисляем чистый относительный путь от корня шаблонов
                let rel_path = path.strip_prefix(template_root)
                    .map_err(|e| format!("Ошибка вычисления префикса пути: {}", e))?;

                // Принудительно преобразуем имя ключа в веб-стандарт с прямым слэшем!
                let template_name_key = rel_path.to_string_lossy().replace('\\', "/");

                // Вычисляем целевой путь сохранения: срезаем суффикс ".tera" (ровно 5 символов)
                let mut target_file_name = file_name.to_string();
                target_file_name.truncate(target_file_name.len() - 5);

                let mut target_path = output_root.join(rel_path);
                target_path.set_file_name(target_file_name);

                // Записываем пару в сборочную матрицу
                files_matrix.push((template_name_key, target_path));
            }
        }
    }
    Ok(files_matrix)
}

// Функция сканирования модулей ядра
// Сканирует папку шаблонов по пути "templates/src/core/", вырезает расширения '.rs.tera'
// формирует вектор импортов
fn scan_core_modules(template_dir: &Path) -> Vec<String> {
    let mut core_modules = Vec::new();
    let core_templates_path = template_dir.join("src").join("core");

    // Если папка физически существует на диске — собираем её файлы в реальном времени
    if let Ok(entries) = fs::read_dir(&core_templates_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    if file_name.ends_with(".rs.tera") {
                        // Отрезаем суффикс ".rs.tera", получая чистое имя модуля для "pub mod"
                        let mod_name = file_name.replace(".rs.tera", "");
                        core_modules.push(mod_name);
                    }
                }
            }
        }
    } else {
        log::warn!("scan_core_modules: Папка шаблонов ядра {:?} не обнаружена.", core_templates_path);
    }

    // Сортируем, обеспечивая алфавитный порядок импортов
    core_modules.sort();
    core_modules
}


// Функция-хелпер сканирования файлов-шаблонов элементов
fn scan_widget_modules(template_dir: &Path) -> HashMap<String, String> {
    let mut widget_registry = HashMap::new();
    let widgets_path = template_dir.join("widgets");

    if let Ok(entries) = fs::read_dir(&widgets_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    if file_name.ends_with(".tera") {
                        // Вырезаем ".tera" (например, "button.tera" -> "button")
                        let widget_name = file_name.replace(".tera", "");
                        
                        // Каноничный путь для веб-стандарта Tera (прямые слэши)
                        let template_key = format!("widgets/{}", file_name);
                        
                        widget_registry.insert(widget_name, template_key);
                    }
                }
            }
        }
    } else {
        log::warn!("scan_widget_modules: Папка виджетов {:?} не обнаружена.", widgets_path);
    }

    widget_registry
}


// Функция нормальзации синонимов

fn normalize_widget_type<'a>(raw_type: &'a str, config: &'a WidgetMappingConfig) -> &'a str {
    // Ищем имя в хэш-карте алиасов TOML-файла
    if let Some(canonical_name) = config.aliases.get(raw_type) {
        canonical_name.as_str()
    } else {
        raw_type // Если синонима нет, возвращаем как есть
    }
}

// Сканер импортов
// Сканирует живое JSON-дерево виджетов проекта, сопоставляет их с реальными 
// структурами графического движка Iced 0.14 и возвращает очищенный от дубликатов список импортов.


/// Безопасно извлекает тип виджета из JSON-узла, проверяя алиасы "widget_type" и "type".
/// В случае аномалий выводит предупреждения в консоль и применяет safe-заглушку "space" [1.2].

fn extract_widget_type(widget_id: &str, widget_obj: &serde_json::Map<String, serde_json::Value>) -> String {
    // Проверяем ключ "widget_type"
    if let Some(w_type_val) = widget_obj.get("widget_type") {
        if let Some(w_type_str) = w_type_val.as_str() {
            log::trace!("extract_widget_type: Тип узла '{}' успешно определен 'widget_type' -> '{}'", widget_id, w_type_str);
            return w_type_str.to_string();
        }
    }

    // Проверяем альтернативный ключ "type"
    if let Some(type_val) = widget_obj.get("type") {
        if let Some(type_str) = type_val.as_str() {
            log::info!("extract_widget_type: Тип узла '{}' успешно определен 'type' -> '{}'", widget_id, type_str);
            return type_str.to_string();
        }
    }

    // Полный сбой структуры узла в JSON. Применяем аварийную невидимую распорку
    log::warn!("extract_widget_type: Тип узла '{}' не определен! Принудительный safe-откат к 'space'.", widget_id);
    "space".to_string()
}

// Сканер импортов
// Сканирует живое JSON-дерево виджетов проекта, сопоставляет их с реальными 
// структурами графического движка Iced 0.14 и возвращает очищенный от дубликатов список импортов.
fn collect_iced_imports(
    widgets_json: &serde_json::Value,
    config: &WidgetMappingConfig,
    imports_toml_value: &serde_json::Value // Передаем распарсенный TOML
) -> String {
    let mut flat_paths = std::collections::BTreeSet::new();
    
    // Дефолтные системные типы (сохраняем вашу базовую логику)
    flat_paths.insert("iced::Element".to_string());
    flat_paths.insert("iced::Theme".to_string());
    flat_paths.insert("iced::widget::space".to_string());

    // Извлекаем секцию [widgets] из imports.toml
    if let Some(toml_widgets) = imports_toml_value.get("widgets").and_then(|w| w.as_object()) {
        
        // Бежим строго по живой JSON-карте виджетов макета (как и было)
        if let Some(widgets_map) = widgets_json.as_object() {
            for (id, widget_val) in widgets_map { 
                let widget_obj = widget_val.as_object().cloned().unwrap_or_default();
                let w_type = extract_widget_type(id, &widget_obj);
                let clean_w_type = normalize_widget_type(&w_type, config);

                // Мгновенно вытаскиваем строку импортов из TOML по каноничному типу (например, "Text")
                if let Some(imports_raw) = toml_widgets.get(clean_w_type).and_then(|v| v.as_str()) {
                    let imports_str = imports_raw.to_string();

                    // =================================================================
                    // ВАШ ИДЕАЛЬНЫЙ КОНЕЧНЫЙ АВТОМАТ РАЗМОТКИ СКОБОК (БЕЗ ИЗМЕНЕНИЙ)
                    // =================================================================
                    let mut current_prefix = String::new();
                    let mut in_brackets = false;
                    let mut current_token = String::new();

                    for ch in imports_str.chars() {
                        match ch {
                            '{' => {
                                in_brackets = true;
                                current_prefix = current_token.trim().to_string();
                                current_token.clear();
                            }
                            '}' => {
                                in_brackets = false;
                                let sub_tokens = current_token.clone();
                                current_token.clear();
                                for sub in sub_tokens.split(',') {
                                    let clean_sub = sub.trim();
                                    if !clean_sub.is_empty() {
                                        let full_path = if current_prefix.ends_with("::") {
                                            format!("{}{}", current_prefix, clean_sub)
                                        } else {
                                            format!("{}{}", current_prefix, clean_sub)
                                        };
                                        flat_paths.insert(full_path);
                                    }
                                }
                                current_prefix.clear();
                            }
                            ',' => {
                                if in_brackets {
                                    current_token.push(ch);
                                } else {
                                    let clean = current_token.trim();
                                    if !clean.is_empty() {
                                        flat_paths.insert(clean.to_string());
                                    }
                                    current_token.clear();
                                }
                            }
                            _ => {
                                current_token.push(ch);
                            }
                        }
                    }
                    let clean = current_token.trim();
                    if !clean.is_empty() && !clean.contains('{') && !clean.contains('}') {
                        flat_paths.insert(clean.to_string());
                    }
                } else {
                    log::warn!("collect_iced_imports: Тип '{}' не найден в imports.toml", clean_w_type);
                }
            }
        }
    }

    // ТВОЙ ОРИГИНАЛЬНЫЙ БЛОК ГРУППИРОВКИ (ОСТАЕТСЯ ОДИН В ОДИН)
    let mut grouped_tree: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for mut path in flat_paths {
        path = path.trim().to_string();
        if !path.contains("::") {
            path = format!("iced::widget::{}", path);
        }
        if let Some(last_cols_idx) = path.rfind("::") {
            let base_module = path[..last_cols_idx].to_string();
            let struct_name = path[last_cols_idx + 2..].to_string();
            grouped_tree.entry(base_module).or_default().insert(struct_name);
        }
    }

    let mut final_use_lines = Vec::new();
    for (module, structures) in grouped_tree {
        if structures.len() == 1 {
            let single_struct = structures.iter().next().unwrap();
            final_use_lines.push(format!("use {}::{};", module, single_struct));
        } else {
            let struct_list: Vec<String> = structures.into_iter().collect();
            final_use_lines.push(format!("use {}::{{{}}};", module, struct_list.join(", ")));
        }
    }
    final_use_lines.join("\n")
}


/// Корневой оркестратор кодогенерации
/// Создает чистый снимок проекта и обогащает его строковыми кодами типов
/// через методы моделей, распределяет контекст и запускает пакетный рендеринг.
pub fn render_iced_layout_template(
    project: &CadProject, 
    template_dir: &Path, 
    output_dir: &Path, 
    no_fmt: bool
) -> Result<(), String> {
    log::info!("codegen::render_iced_layout_template: Инициализация конвейера...");

    // Инициализируем конфигурацию: Считываем внешний конфиг файл TOML 
    let config = WidgetMappingConfig::load_from_file("widget_config.toml");

    // Инициализируем рантайм Tera под спецификацию Tera 2.x
    let template_mask = template_dir.join("**/*");
    let mask_str = template_mask.to_string_lossy();
    
    // ИСПРАВЛЕНО ДЛЯ TERA 2: Конструктор пустой, загрузка через метод расширения
    let mut tera = Tera::default();
    if let Err(error) = tera.load_from_glob(&mask_str) {
        log::error!(
            "Критическая ошибка синтаксиса в шаблонах Tera:\n{}",
            error
        );
        std::process::exit(1); // Завершаем программу
    };

    // Настройка автоэкранирования (как и было исправлено ранее)
    tera.autoescape_on([".sql", ".j2"]);

    // Упреждающий кастинг типов по Schema-реестру
    let mut local_project = project.clone();    
    for widget in local_project.widgets.values_mut() {
        
        log::info!(
            "render_iced_layout_template: Запуск предпарсинга типов для '{} => {}'", 
            widget.meta.id, 
            widget.widget_type
        );
        widget.properties.prepare_generator_codes(&project.property_registry);
        
    }
    

    // Сериализуем подготовленный бинарный снимок проекта для Tera
    let mut widgets_json = serde_json::to_value(&local_project.widgets)
        .map_err(|e| format!("Ошибка serialization виджетов: {}", e))?;

    if let Some(widgets_map) = widgets_json.as_object_mut() {
        for (id, widget_val) in widgets_map {
            if let Some(widget_obj) = widget_val.as_object_mut() {
                // Извлекаем сырой тип, проверяя widget_type или type с логами
                let raw_type = extract_widget_type(id, widget_obj);
                
                // Прогоняем сырую строку через наш внешний TOML-список синонимов
                let canonical_type = normalize_widget_type(&raw_type, &config).to_string();
                
                log::info!(
                    "Нормализация узла '{}': '{}' -> '{}'", 
                    id, 
                    raw_type, 
                    canonical_type
                );
                
                // Запечатываем чистый тип в оба поля. Теперь view.rs.tera увидит ТОЛЬКО каноничные строки!
                widget_obj.insert("widget_type".to_string(), serde_json::Value::String(canonical_type.clone()));
                widget_obj.insert("type".to_string(), serde_json::Value::String(canonical_type));
            }
        }
    }

    // =========================================================================
    // Сортировка элементов рекурсивно сверху-вниз (к корню) 
    // с сохранение порядка элементов внутри группы
    // =========================================================================
    //let mut widgets_order = project.widgets_order.clone();
    let widgets_order = project.get_post_order_layout("root");

    // Сборка корневых элементов
    let mut root_ids = Vec::new();
    let mut last_widget_id = String::new();

    // FIX: Order
    // Итерируемся по исходной IndexMap проекта    
    for id in &widgets_order {
    //for (id, node) in &project.widgets {

        let Some(node) = project.widgets.get(id) else {
            log::warn!("Виджет '{}' из widgets_order не найден на складе widgets!", id);
            // Пропускаем итерацию, если виджет вдруг оказался битым или удаленным
            continue; 
        };
        
        // Запоминаем последний гарантированный ID виджета по локальному порядку
        last_widget_id = id.clone();

        // Достаем parent_id через инкапсулированный метод
        let parent_val = node.properties.parent_id();

        // Проверяем условия корня
        if parent_val == "root" || parent_val.is_empty() || parent_val == "None" || parent_val == "canvas" {
            root_ids.push(id.clone());
        }
    }

    // =====================================================================
    // ИСПРАВЛЕНО: Сбор и группировка импортов через imports.toml
    // =====================================================================
    let imports_toml_path = template_dir.join("imports.toml");
    let toml_content = fs::read_to_string(&imports_toml_path)
        .unwrap_or_else(|_| {
            log::warn!("render_iced_layout_template: Файл конфигурации импортов {:?} не найден, откат к базовому TOML.", imports_toml_path);
            "[widgets]".to_string()
        });
        
    let imports_toml_value: serde_json::Value = toml::from_str(&toml_content)
        .map_err(|e| format!("Ошибка синтаксиса в файле imports.toml: {}", e))?;

    // Вызываем обновленную функцию, передавая туда готовую JSON-структуру TOML-файла
    let iced_imports_block = collect_iced_imports(&widgets_json, &config, &imports_toml_value);

    // =====================================================================
    // Заполнение контент-контекста рендеринга
    // =====================================================================
    let mut context = Context::new();
    context.insert("widgets", &widgets_json);
    context.insert("widgets_order", &widgets_order);
    context.insert("root_ids", &root_ids);
    context.insert("last_widget_id", &last_widget_id);
    context.insert("iced_imports_block", &iced_imports_block); // Передаем готовые отформатированные use-строки
    context.insert("field_counter", &project.field_counter);
    context.insert("project_title", "Iced Forms App");
    context.insert("crate_name", "iced_forms_gen");

    let core_modules = scan_core_modules(template_dir);
    context.insert("core_modules", &core_modules);

    // Запускаем рекурсивное сканирование структуры шаблонов в папках
    let files_to_render = scan_template_directory(template_dir, PathBuf::new(), template_dir, output_dir)?;

    // Цикл пакетного рендеринга и диагностики
    for (template_name, target_path) in files_to_render {
        log::info!("Рендеринг компонента проекта: {} -> {:?}", template_name, target_path);
        if let Some(parent_dir) = target_path.parent() {
            fs::create_dir_all(parent_dir).map_err(|e| e.to_string())?;
        }
        
        let raw_content = match tera.render(&template_name, &context) {
            Ok(content) => content
                .lines()
                // Удаляет trailing whitespace с пустых строк перед форматированием
                .map(|line| line.trim_end()) 
                .collect::<Vec<&str>>()
                .join("\n"),
            Err(err) => {
                let mut error_report = format!(
                    "Критический сбой рендеринга шаблона '{}'!\n[Ошибка Tera]: {}", 
                    template_name, err
                );
                let mut current_cause = std::error::Error::source(&err);
                let mut step = 1;
                while let Some(cause) = current_cause {
                    error_report.push_str(&format!("\n └─ [Сбой #{}] {}", step, cause));
                    current_cause = cause.source();
                    step += 1;
                }
                log::error!(" {}", error_report);
                return Err(error_report);
            }
        };

        // Перехват флага --no-fmt для отключения rustfmt
        let final_content = if target_path.extension().and_then(|s| s.to_str()) == Some("rs") && !no_fmt {
            use crate::core::rustfmt_wrapper;
            rustfmt_wrapper::rustfmt(&raw_content).unwrap_or_else(|err| {
                log::error!("[Генератор] Предупреждение: rustfmt не смог отформатировать {:?}", target_path);
                log::error!("Детали ошибки:\n{}\n{:?}", err, raw_content); 
                raw_content
            })
        } else {
            // В противном случае (другой тип файла или включен --no-fmt) оставляем код "как есть"
            raw_content
            //rustfmt_wrapper::rustfmt(&raw_content).unwrap_or(raw_content)
        };
        
        fs::write(&target_path, final_content).map_err(|e| e.to_string())?;
    }

    log::info!("Структура проекта успешно сгенерирована.");
    Ok(())
}

