use indexmap::IndexMap;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

use crate::app::App;
use crate::core::codegen_models::{
    CadProject, PropertyType, WidgetMappingConfig, WidgetMeta, WidgetNode, WidgetProperties,
};
use crate::core::*;

/// Генерирует Rust-код для отдельного виджета по его widget_id, используя данные из app и factory.
/// Возвращает строку с кодом Rust. В случае ошибки возвращает строку с комментарием и логирует ошибку.

fn collect_subtree_ids(root_id: &str, factory: &Factory) -> Vec<String> {
    let mut ids = Vec::new();
    let mut stack = vec![root_id.to_string()];

    while let Some(current_id) = stack.pop() {
        if ids.iter().any(|existing| existing == &current_id) {
            continue;
        }

        ids.push(current_id.clone());
        for child_id in factory.get_children_ids_by_parent(&current_id) {
            stack.push(child_id);
        }
    }

    ids
}

// Получаем путь к каталогу с темплейтами
pub fn get_templates_path() -> PathBuf {
    // Извлекаем путь, который прописан в конфигурационном файле TOML
    let config = APP_CONFIG.get().unwrap();
    let config_path = PathBuf::from(&config.template_path);

    // Если пользователь вручную прописал в файле настроек абсолютный путь,
    // или путь отличается от дефолтного "./templates" — доверяем конфигу
    if config_path.is_absolute() || config.template_path != "./templates" {
        return config_path;
    }

    // Если путь дефолтный, включаем нашу интеллектуальную автоматику:
    // Сценарий А: Режим разработки (запуск через `cargo run`)
    #[cfg(debug_assertions)]
    {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(|root| root.join("templates"))
            .unwrap_or_else(|| PathBuf::from("templates"))
    }

    // Сценарий Б: Релизный собранный бинарник (готовый .exe на любом ПК)
    #[cfg(not(debug_assertions))]
    {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Ищет папку "templates" строго в той же папке, где лежит сам .exe
                return exe_dir.join("templates");
            }
        }
        PathBuf::from("templates")
    }
}

// Читаем темплейты
pub fn load_templates_with_include_list(templates_dir: &Path) -> Tera {
    log::info!(
        "load_templates_with_include_list: templates_dir '{:?}'",
        templates_dir
    );

    // Получаем список целевых подкаталогов из вашего глобального конфига.
    // Пример из конфига: ["/", "components", "widgets"]
    let config = APP_CONFIG.get().unwrap();
    let include_list: Vec<&str> = config.template_include.iter().map(|s| s.as_str()).collect();

    let mut tera = Tera::default();
    let mut raw_templates: IndexMap<String, String> = IndexMap::new();

    // Проходим строго по белому списку каталогов
    for sub_dir_name in include_list {
        // Вычисляем физический путь к подкаталогу на диске
        let target_path = if sub_dir_name == "/" || sub_dir_name.is_empty() {
            templates_dir.to_path_buf() // Если указан "/", ищем прямо в корне папки шаблонов
        } else {
            templates_dir.join(sub_dir_name) // Иначе ищем в конкретной подпапке
        };

        // Запускаем сбор файлов .tera для текущего подкаталога
        collect_from_directory(&target_path, templates_dir, &mut raw_templates);
        raw_templates.sort_keys();
    }

    log::trace!(
        "load_templates_with_include_list: collected templates {:#?}",
        raw_templates.keys()
    );

    // Массовая одновременная регистрация в Tera 2 для сохранения контекста
    if let Err(e) = tera.add_raw_templates(raw_templates.into_iter()) {
        log::error!(
            "Tera 2: Критическая ошибка сборки реестра по белому списку: {}",
            e
        );
    }

    tera
}

// Вспомогательная функция, которая собирает файлы .tera ИЗ ОДНОЙ конкретной папки (БЕЗ глубокой рекурсии)
fn collect_from_directory(
    dir: &Path,
    base_templates_dir: &Path,
    raw_templates: &mut IndexMap<String, String>,
) {
    
    log::info!("collect_from_directory: Dir {:?}", dir);

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // Читаем только файлы (папки на уровне ниже мы намеренно игнорируем,
                // так как обход строго контролируется белым списком верхнего уровня)
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("tera") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        // Вычисляем относительное имя для Tera (например, "widgets/text.tera")
                        if let Ok(rel_path) = path.strip_prefix(base_templates_dir) {
                            //if let Ok(rel_path) = path.strip_prefix(dir) {
                            let template_name = rel_path.to_string_lossy().to_string();
                            let clean_name = template_name.replace('\\', "/");

                            raw_templates.insert(clean_name, content);
                        }
                    }
                }
            }
        }
    }
}

pub fn generate_widget_tree_code(_app: &App) -> String {
    log::info!("generate_widget_tree_code: Инициализация TERA конвейера для widget scope...");

    // Получаем путь к темплейтам
    let template_dir = get_templates_path();

    //let effective_template_root = ensure_widget_preview_root(&template_dir);
    let effective_template_root = template_dir.clone();

    let config_path = effective_template_root.join("widget_config.toml");
    let config = if config_path.exists() {
        WidgetMappingConfig::load_from_file(config_path.to_string_lossy().as_ref())
    } else {
        WidgetMappingConfig::default()
    };

    // Подготавливаем исходные данные CadProject для Tera
    let mut project = match prepare_code_project(_app) {
        Ok(project) => project,
        Err(err) => {
            log::warn!("prepare_code_project: {}", err);
            CadProject::new()
        }
    };

    // Строим список корневых виджетов
    let roots_from_project: Vec<String> = project
        .widgets_order
        .iter()
        .filter(|id| {
            project
                .widgets
                .get(*id)
                .map(|node| {
                    let parent_val = node.properties.parent_id();
                    parent_val == "root"
                        || parent_val.is_empty()
                        || parent_val == "None"
                        || parent_val == "canvas"
                })
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    // Поиск выделенного виджета, для которого будет строиться код,
    // либо если выделенного нет - выбираем список корневых виджетов
    let factory = _app.get_factory();
    let selected_root = _app.state.selected_widget_id.clone();
    let mut scope_root_ids = match selected_root {
        Some(ref selected_id) if project.widgets.contains_key(selected_id) => {
            vec![selected_id.clone()]
        }
        _ => roots_from_project,
    };

    // На случай если итоговый список пуст - берем все виджеты по порядку
    if scope_root_ids.is_empty() {
        scope_root_ids = project.widgets_order.iter().cloned().collect::<Vec<_>>();
    }

    // Рекурсивно спускаемся по дереву вниз и строим плоский список виджетов
    let mut scope_ids = Vec::new();
    for root_id in &scope_root_ids {
        // Проходим по деревьям для всех корневых элементов
        for id in collect_subtree_ids(root_id, &factory) {
            if !scope_ids.contains(&id) {
                scope_ids.push(id);
            }
        }
    }

    // Eсли на предыдущих этапах не удалось построить список -
    // копируем весь список проекта
    if scope_ids.is_empty() {
        scope_ids = project.widgets_order.clone();
    }

    project.widgets.retain(|id, _| scope_ids.contains(id));
    project.widgets_order.retain(|id| scope_ids.contains(id));

    // Сериализуем подготовленный бинарный снимок проекта для Tera
    let mut widgets_json = match serde_json::to_value(&project.widgets) {
        Ok(value) => value,
        Err(err) => {
            log::error!(
                "generate_widget_tree_code: Ошибка serialisation widgets: {}",
                err
            );
            serde_json::Value::Object(serde_json::Map::new())
        }
    };

    if let Some(widgets_map) = widgets_json.as_object_mut() {
        for (id, widget_val) in widgets_map {
            if let Some(widget_obj) = widget_val.as_object_mut() {
                // Извлекаем сырой тип, проверяя widget_type или type с логами
                let raw_type = extract_widget_type(id, widget_obj);

                // Прогоняем сырую строку через наш внешний TOML-список синонимов
                let canonical_type = normalize_widget_type(&raw_type, &config).to_string();

                // Запечатываем чистый тип в оба поля. Теперь view.rs.tera увидит ТОЛЬКО каноничные строки!
                widget_obj.insert(
                    "widget_type".to_string(),
                    serde_json::Value::String(canonical_type.clone()),
                );
                widget_obj.insert(
                    "type".to_string(),
                    serde_json::Value::String(canonical_type),
                );
            }
        }
    }

    // Imports
    let mut last_widget_id = scope_root_ids.last().cloned().unwrap_or_default();
    if project.widgets_order.last().is_some() {
        last_widget_id = project.widgets_order.last().unwrap().clone();
    }

    let imports_toml_path = effective_template_root.join("imports.toml");
    let toml_content = match fs::read_to_string(&imports_toml_path) {
        Ok(content) => content,
        Err(_) => {
            log::warn!(
                "generate_widget_tree_code: Файл конфигурации импортов {:?} не найден, откат к базовому TOML.",
                imports_toml_path
            );
            "[widgets]".to_string()
        }
    };

    let imports_toml_value: serde_json::Value = match toml::from_str(&toml_content) {
        Ok(value) => value,
        Err(err) => {
            log::warn!(
                "generate_widget_tree_code: Ошибка синтаксиса imports.toml: {}",
                err
            );
            serde_json::Value::Object(serde_json::Map::new())
        }
    };

    let iced_imports_block = collect_iced_imports(&widgets_json, &config, &imports_toml_value);

    //let mut tera = load_templates_with_skip(&effective_template_root);
    let mut tera = load_templates_with_include_list(&effective_template_root);

    tera.autoescape_on([".sql", ".j2"]);

    // =====================================================================
    // Заполнение контент-контекста рендеринга
    // =====================================================================
    let mut generated = String::new();

    //let template_name = format!("widget_preview/preview.tera");

    // Ищем в реестре Tera шаблон, который заканчивается на "preview.tera"
    let template_name = tera
        .get_template_names()
        .find(|name| name.ends_with("preview.tera") || *name == "preview.tera")
        .map(|name| name.to_string())
        .unwrap_or_else(|| {
            // Аварийный вариант, если в папках вообще нет такого файла
            String::from("preview.tera")
        });

    let mut context = Context::new();
    context.insert("widgets", &widgets_json);
    context.insert("widgets_order", &project.widgets_order);
    context.insert("root_ids", &scope_root_ids);
    context.insert("last_widget_id", &last_widget_id);
    context.insert("iced_imports_block", &iced_imports_block);
    context.insert("field_counter", &project.field_counter);
    context.insert("project_title", "Iced Forms App");
    context.insert("crate_name", "IcedForms");

    // Цикл пакетного рендеринга и диагностики
    match tera.render(&template_name, &context) {
        Ok(content) => {
            let content_clean = content
                .lines()
                .map(|line| line.trim_end())
                .filter(|line| !line.is_empty())
                .collect::<Vec<&str>>()
                .join("\n");
            generated.push_str(&content_clean);
        }
        Err(err) => {
            let msg = format!("// '{}': {}\n", &template_name, err);
            log::error!("{}", msg);
            generated.push_str(&msg);
        }
    }

    //println!("Generated: \n{}", generated);

    if generated.trim().is_empty() {
        "// Нет виджетов для генерации.".to_string()
    } else {
        format_fragment(&generated)
    }
}

fn normalize_widget_type<'a>(raw_type: &'a str, config: &'a WidgetMappingConfig) -> &'a str {
    if let Some(canonical_name) = config.aliases.get(raw_type) {
        canonical_name.as_str()
    } else {
        raw_type
    }
}

fn extract_widget_type(
    widget_id: &str,
    widget_obj: &serde_json::Map<String, serde_json::Value>,
) -> String {
    if let Some(w_type_val) = widget_obj.get("widget_type") {
        if let Some(w_type_str) = w_type_val.as_str() {
            return w_type_str.to_string();
        }
    }

    if let Some(type_val) = widget_obj.get("type") {
        if let Some(type_str) = type_val.as_str() {
            return type_str.to_string();
        }
    }

    log::warn!(
        "extract_widget_type: Тип узла '{}' не определен! Принудительный safe-откат к 'space'.",
        widget_id
    );
    "space".to_string()
}

fn collect_iced_imports(
    widgets_json: &serde_json::Value,
    config: &WidgetMappingConfig,
    imports_toml_value: &serde_json::Value,
) -> String {
    let mut flat_paths = BTreeSet::new();
    flat_paths.insert("iced::Element".to_string());
    flat_paths.insert("iced::Theme".to_string());
    flat_paths.insert("iced::widget::space".to_string());

    if let Some(toml_widgets) = imports_toml_value
        .get("widgets")
        .and_then(|w| w.as_object())
    {
        if let Some(widgets_map) = widgets_json.as_object() {
            for (id, widget_val) in widgets_map {
                let widget_obj = widget_val.as_object().cloned().unwrap_or_default();
                let w_type = extract_widget_type(id, &widget_obj);
                let clean_w_type = normalize_widget_type(&w_type, config);

                if let Some(imports_raw) = toml_widgets.get(clean_w_type).and_then(|v| v.as_str()) {
                    let mut current_prefix = String::new();
                    let mut in_brackets = false;
                    let mut current_token = String::new();

                    for ch in imports_raw.chars() {
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
                                        let full_path = format!("{}{}", current_prefix, clean_sub);
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
                            _ => current_token.push(ch),
                        }
                    }

                    let clean = current_token.trim();
                    if !clean.is_empty() && !clean.contains('{') && !clean.contains('}') {
                        flat_paths.insert(clean.to_string());
                    }
                }
            }
        }
    }

    let mut grouped_tree: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for mut path in flat_paths {
        path = path.trim().to_string();
        if !path.contains("::") {
            path = format!("iced::widget::{}", path);
        }
        if let Some(last_cols_idx) = path.rfind("::") {
            let base_module = path[..last_cols_idx].to_string();
            let struct_name = path[last_cols_idx + 2..].to_string();
            grouped_tree
                .entry(base_module)
                .or_default()
                .insert(struct_name);
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

// Подготавливает исходные данные для проекта Tera
pub fn prepare_code_project(_app: &App) -> Result<CadProject, String> {
    let factory = _app.get_factory();
    let mut project = CadProject::new();

    project.field_counter = factory.get_field_counter();
    //project.widgets_order = factory.get_blueprint_keys();
    project.widgets_order = factory.get_blueprints_id_build_order();

    for (index, widget_id) in project.widgets_order.iter().enumerate() {
        let Some(bp) = factory.get_blueprint_rc(widget_id.clone()) else {
            continue;
        };

        let mut properties = WidgetProperties::new();
        let parent_id = factory
            .get::<String>(widget_id, crate::core::PROP_PARENT)
            .unwrap_or_default();
        properties.set_parent(&parent_id);
        project
            .property_registry
            .register("parent".to_string(), PropertyType::String);

        for prop_key in bp.get_exportable_property_names(factory) {
            let value = factory.get_as_string(widget_id, prop_key, "");
            let code = factory.get_as_code(widget_id, prop_key, "");
            let key_str = prop_key.name.to_string();
            properties
                .fields
                .insert(key_str.clone(), serde_json::Value::String(value.clone()));
            properties.fields.insert(
                format!("{}_code", key_str.clone()),
                serde_json::Value::String(code),
            );
            project
                .property_registry
                .register(key_str.clone(), PropertyType::String);
        }

        let widget_node = WidgetNode {
            widget_type: bp.widget_type().to_string(),
            meta: WidgetMeta {
                id: widget_id.clone(),
                local_index: index as i32,
            },
            properties,
        };

        project.widgets.insert(widget_id.clone(), widget_node);
    }

    Ok(project)
}

pub fn format_fragment(raw_code: &str) -> String {
    // Заворачиваем в валидную функцию
    let wrapped = format!("fn __wrapper() {{\n{}\n}}", raw_code);

    // Парсим строку в структуру файла AST через syn
    match syn::parse_file(&wrapped) {
        Ok(file) => {
            // Форматируем с помощью prettyplease
            let formatted_wrapped = prettyplease::unparse(&file);

            // Получаем строки отформатированного текста
            let mut lines: Vec<&str> = formatted_wrapped.lines().collect();
            if lines.len() >= 2 {
                // Срезаем обертку `fn __wrapper() {` и закрывающую `}`
                lines.remove(0);
                lines.pop();
            }
            return lines.join("\n");
        }
        Err(err) => {
            log::error!(
                "format_fragment: Ошибка парсинга syn в генераторе кода: {}",
                err
            );
            log::error!("format_fragment: Исходный текст:\n{}", &wrapped);
        }
    }

    raw_code.to_string() // Если синтаксис совсем сломан — отдаем как есть
}

// -----------------------------------------------------------------------------
// Factory::get_as_code
// Реализация функции Factory возвращает представление кода Rust для типа данных
// -----------------------------------------------------------------------------

impl Factory {
    pub fn get_as_code<'a>(
        &'a self,
        widget_id: &str,
        prop_key: PropertyKey,
        default_value: &str,
    ) -> String {
        let t_hash = storage::get_prop_type_hash(prop_key);
        let key = prop_key.hash;

        //-----------------------------------------------------------------
        // ЛОКАЛЬНЫЕ КОНСТАНТЫ КОМПИЛЯЦИИ:
        // Считаются 1 раз при сборке файла!
        // Они совпадают с тем, что генерирует stringify!() в макросе!
        // Поэтому все константы создаем голым именем соблюдая регистр
        //-----------------------------------------------------------------
        // !!! Сделай глобальные константы типов в одном месте !!!

        const TYPE_STRING: u64 = fnv1a_hash_64("String");
        const TYPE_USIZE: u64 = fnv1a_hash_64("usize");
        const TYPE_FLOAT: u64 = fnv1a_hash_64("f32");
        const TYPE_BOOL: u64 = fnv1a_hash_64("bool");
        const TYPE_LENGTH: u64 = fnv1a_hash_64("Length");
        const TYPE_PADDING: u64 = fnv1a_hash_64("Padding");
        const TYPE_PIXELS: u64 = fnv1a_hash_64("Pixels");
        const TYPE_RADIUS: u64 = fnv1a_hash_64("Radius");
        const TYPE_COLOR: u64 = fnv1a_hash_64("Color");
        const TYPE_FONT: u64 = fnv1a_hash_64("Font");
        const TYPE_ALIGN_ITEMS: u64 = fnv1a_hash_64("Alignment");
        const TYPE_HORIZONTAL: u64 = fnv1a_hash_64("Horizontal");
        const TYPE_VERTICAL: u64 = fnv1a_hash_64("Vertical");

        // В зависимости от типа извлекаем данные и превращаем их в строку
        let mut type_name = "";
        match t_hash {
            TYPE_USIZE => {
                if let Some(val) = self.get_by_hash::<usize>(widget_id, key) {
                    return val.to_string();
                }
                type_name = "usize";
            }
            TYPE_FLOAT => {
                if let Some(val) = self.get_by_hash::<f32>(widget_id, key) {
                    let val_fmt = format!("{:.1}", val);
                    return val_fmt;
                }
                type_name = "Float";
            }
            TYPE_BOOL => {
                if let Some(val) = self.get_by_hash::<bool>(widget_id, key) {
                    return val.to_string();
                }
                type_name = "Bool";
            }
            TYPE_LENGTH => {
                if let Some(val) = self.get_by_hash::<iced::Length>(widget_id, key) {
                    return code_from_length(val);
                }
                type_name = "Length";
            }
            TYPE_PADDING => {
                if let Some(val) = self.get_by_hash::<iced::Padding>(widget_id, key) {
                    return code_from_padding(val);
                }
                type_name = "Padding";
            }
            TYPE_PIXELS => {
                if let Some(val) = self.get_by_hash::<iced::Pixels>(widget_id, key) {
                    return code_from_pixels(val);
                }
                type_name = "Pixels";
            }
            TYPE_RADIUS => {
                if let Some(val) = self.get_by_hash::<iced::border::Radius>(widget_id, key) {
                    return code_from_radius(val);
                }
                type_name = "Radius";
            }
            TYPE_COLOR => {
                if let Some(val) = self.get_by_hash::<iced::Color>(widget_id, key) {
                    return code_from_color(val);
                }
                type_name = "Color";
            }
            TYPE_FONT => {
                // Код для демо
                return "Font".to_string();
            }
            TYPE_ALIGN_ITEMS => {
                if let Some(val) = self.get_by_hash::<iced::Alignment>(widget_id, key) {
                    return code_from_aling(val);
                }
                type_name = "AlignItems";
            }
            TYPE_HORIZONTAL => {
                if let Some(val) = self.get_by_hash::<iced::alignment::Horizontal>(widget_id, key) {
                    return code_from_align_x(val);
                }
                type_name = "Horizontal";
            }
            TYPE_VERTICAL => {
                if let Some(val) = self.get_by_hash::<iced::alignment::Vertical>(widget_id, key) {
                    return code_from_align_y(val);
                }
                type_name = "Vertical";
            }
            TYPE_STRING | _ => {
                if let Some(val) = self.get_by_hash::<String>(widget_id, key) {
                    // Экранируем все кавычки и слеши
                    let serde_str = serde_json::Value::String(val);
                    return serde_str.to_string();
                }
                type_name = "String";
            }
            _ => {
                log::warn!(
                    r#"Factory::get_as_code: Не найден зарегистрированный тип для '{}:{}'
                    Будет возвращено значение по умолчанию '{}'."#,
                    widget_id,
                    prop_key.name,
                    default_value.to_string()
                );
            }
        }

        // Внимание! Не добавлен тип или не соответствуют тип в макросе и в парсере
        log::warn!(
            "Factory::get_as_code: Не удалось преобразовать <{}> '{}:{}' в строку. Проверьте соответствие типа PropetyKey.",
            type_name,
            widget_id,
            prop_key.name
        );

        // Если свойства нет в базе или тип неизвестен — возвращаем дефолт
        default_value.to_string()
    }

    // Возвращает сортированный список ID блупринтов с сохранением локального порядка
    // Проходим от корней до листьев и строим список в обратном порядке, как виджеты должны создаваться.
    // Сначала создаем конечные виджеты (листья), затем группирующие (корни).
    // При этом порядок одноуровневых детей сохраняем
    pub fn get_blueprints_id_build_order<'a>(&'a self) -> Vec<String> {
        // Рекурсивная подфункция
        fn get_childs_recursive(factory: &Factory, parent_id: &str) -> Vec<String> {
            let child_widgets = factory.get_blueprint_keys_by_parent(parent_id);
            let mut final_widgets = Vec::new();

            if !child_widgets.is_empty() {
                for w in &child_widgets {
                    final_widgets.extend(get_childs_recursive(factory, &w));
                }
            }
            final_widgets.extend(child_widgets);
            final_widgets
        }

        get_childs_recursive(self, "root")
    }
}

// -----------------------------------------------------------------------------
// Code From хелперы
// Функции возвращают строку с представлением кода Rust для данного типа
// -----------------------------------------------------------------------------
fn code_from_length(value: iced::Length) -> String {
    return match value {
        iced::Length::Fixed(p) => format!("Length::Fixed({:.1})", p),
        iced::Length::Fill => String::from("Length::Fill"),
        iced::Length::Shrink => String::from("Length::Shrink"),
        iced::Length::FillPortion(u) => format!("Length::FillPortion({})", u),
    };
}

pub fn code_from_color(color: iced::Color) -> String {
    // ОБРАТНЫЙ ПЕРЕХВАТ ПРОЗРАЧНОСТИ:
    // Если альфа-канал равен нулю, то это прозрачный цвет!
    // Возвращаем маркер "transparent", чтобы текстовое поле не сваливалось в `#000000`
    if color.a == 0.0 {
        return String::from("iced::Color::TRANSPARENT");
    }

    // Обычные цвета переводим в стандартный формат #RRGGBB
    let r = color.r;
    let g = color.g;
    let b = color.b;

    // Возвращаем полностью непрозрачный цвет (a: 1.0) для обычных HEX-кодов
    String::from(format!(
        "iced::Color::from_rgb({:.3}, {:.3}, {:.3})",
        r, g, b
    ))
}

pub fn code_from_padding(padding: iced::Padding) -> String {
    if padding.top == 0.0 && padding.right == 0.0 && padding.bottom == 0.0 && padding.left == 0.0 {
        return String::from("iced::Padding::ZERO");
    }
    format!("Padding::from([{:.1}, {:.1}])", padding.top, padding.left)
}

pub fn code_from_pixels(value: iced::Pixels) -> String {
    if value.0 == 0.0 {
        return String::from("iced::Pixels::ZERO");
    }
    format!("Pixels({:.1})", value.0)
}

pub fn code_from_radius(value: iced::border::Radius) -> String {
    if value.top_left == 0.0 {
        return String::from("iced::border::Radius::ZERO");
    }
    format!("iced::border::Radius::new({:.1})", value.top_left)
}

pub fn code_from_aling(value: iced::Alignment) -> String {
    match value {
        iced::Alignment::Start => String::from("Alignment::Start"),
        iced::Alignment::Center => String::from("Alignment::Center"),
        iced::Alignment::End => String::from("Alignment::End"),
    }
}

pub fn code_from_align_x(value: iced::alignment::Horizontal) -> String {
    match value {
        iced::alignment::Horizontal::Left => String::from("Horizontal::Left"),
        iced::alignment::Horizontal::Center => String::from("Horizontal::Center"),
        iced::alignment::Horizontal::Right => String::from("Horizontal::Right"),
    }
}

pub fn code_from_align_y(value: iced::alignment::Vertical) -> String {
    match value {
        iced::alignment::Vertical::Top => String::from("Vertical::Top"),
        iced::alignment::Vertical::Center => String::from("Vertical::Center"),
        iced::alignment::Vertical::Bottom => String::from("Vertical::Bottom"),
    }
}
