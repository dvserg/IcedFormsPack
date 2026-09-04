// -----------------------------------------------------------------------------
// Модуль serialization
// Содержит реализацию сериализации/десеарилизации данных JSON для ипорта и экспорта
// -----------------------------------------------------------------------------
//use std::fs::File;
//use std::io::{Write, Read};
//use std::path::Path;
use serde_json::json;
use std::collections::BTreeMap;

//use iced::{Color, Pixels, Padding, Font, Length};
//use iced::border::Radius;
//use iced::alignment::{Horizontal, Vertical};

use crate::core::Factory;
use crate::core::PropertyKey;
use crate::core::utils::*;

// -----------------------------------------------------------------------------
// СЕРИАЛИЗАЦИЯ
// -----------------------------------------------------------------------------

/// Функция сканирует VTable-хранилище, распаковывает Box<dyn Any>
/// и с помощью утилит utils.rs переводит все параметры в текстовый JSON-формат.
pub fn serialize_to_json_string(factory: &Factory) -> Result<String, std::io::Error> {
    log::info!(
        "serialize_to_json_string: Извлечение данных VTable через Factory::get_as_string..."
    );

    // Открываем RefCell на чтение PropertyStorage внутри фабрики
    let storage_guard = factory.get_field_values();//.borrow();
    let mut root_json_map = BTreeMap::new();

    // Перебираем все виджеты по их числовым u64 хэшам
    for (widget_hash, prop_map) in &storage_guard.data {
        // Извлекаем строковое имя виджета (например, "widget_1") из реестра имён        
        let widget_name = storage_guard
            .names
            .get(widget_hash)
            .cloned()
            .unwrap_or_else(|| format!("widget_hash_{}", widget_hash));

        let mut widget_properties_json = BTreeMap::new();

        // Перечисляем все свойства текущего виджета
        for (prop_hash, _any_boxed) in prop_map {
            // Заглядываем в thread_local реестр метаданных, чтобы узнать имя и точный ключ свойства
            let (prop_name, prop_key) = crate::core::ALL_PROPERTY_TOKENS.with(|tokens| {
                let guard = tokens.borrow();
                guard
                    .iter()
                    .find(|m| m.hash == *prop_hash)
                    // Собираем полноценный PropertyKey для вызова метода фабрики
                    .map(|meta| {
                        (
                            meta.name.to_string(),
                            crate::core::storage::PropertyKey::from_dynamic(meta.name),
                        )
                    })
                    .unwrap_or_else(|| {
                        // Резервный откат на случай, если токен не зарегистрирован в макросе
                        let name = format!("prop_hash_{}", prop_hash);
                        let key = crate::core::storage::PropertyKey::from_dynamic(&name);
                        (name, key)
                    })
            });

            // Вызываем метод Factory::get_as_string. Он сам сделает downcast к правильному f32, f32 или Length.
            // В качестве дефолта передаем пустую строку.
            let string_value = factory.get_as_string(&widget_name, prop_key, "");

            // Обрабатываем типы данных для JSON, чтобы примитивы оставались примитивами
            // (bool и чистые числа f32 переводим обратно в JSON типы, чтобы убрать лишние кавычки "")
            let json_value = if string_value == "true" {
                json!(true)
            } else if string_value == "false" {
                json!(false)
            } else if let Ok(parsed_float) = string_value.parse::<f32>() {
                // Если строка успешно распарсилась в число — пишем её как число!
                json!(parsed_float)
            } else {
                // В противном случае (для Length, Padding, Color) пишем готовую утилитную строку
                json!(string_value)
            };

            // Игнорируем пустые или битые свойства
            if !string_value.is_empty() {
                widget_properties_json.insert(prop_name, json_value);
            }
        }

        // Фиксируем собранную карту за конкретным именем виджета
        root_json_map.insert(widget_name, widget_properties_json);
    }

    // Форматируем JSON строку
    let pretty_json_string = serde_json::to_string_pretty(&root_json_map).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Serde Critical Error: {}", e),
        )
    })?;

    Ok(pretty_json_string)
}

/// Генерация JSON образа проекта. Собирает VTable, реестр имён и счётчик уникальности фабрики
/// в финальный, полностью готовый к записи текстовый JSON-буфер.
pub fn generate_project_json_string(factory: &Factory) -> Result<String, std::io::Error> {
    log::info!("generate_project_json_string: Подготовка данных проекта к сохранению...");

    // Fix: Order
    // Создаем вектор для сохранения порядка элементов
    let mut widgets_order = Vec::new();

    // Сериализуем массив чертежей (blueprints)
    let mut blueprints_json_map = BTreeMap::new();
    for (widget_id, blueprint) in factory.blueprints_iter() {
        // Добавляем ID в массив порядка (сохраняет порядок из IndexMap)
        widgets_order.push(widget_id.clone());

        // Каждое блупринт-состояние выгружает свой строковый тип и метаданные через трейт
        blueprints_json_map.insert(
            widget_id.clone(),
            json!({
                "type": blueprint.widget_type(),
                "meta": {
                    "id": widget_id,
                    "local_index": blueprint.get_index() // Вытаскиваем индекс слоя виджета /* Неиспользуемая сейчас фича */
                }
            }),
        );
    }

    // Вызываем даункаст для таблицы VTable
    let pretty_json_text = serialize_to_json_string(factory)?;
    let vtable_json_value: serde_json::Value =
        serde_json::from_str(&pretty_json_text).unwrap_or(json!({}));

    // Формируем единую структуру JSON
    let full_envelope = json!({
        "field_counter": factory.field_counter,
        "widgets_order": widgets_order,         // Fix: Order
        "blueprints": blueprints_json_map,     
        "field_values": vtable_json_value       // Переименовано обратно по спецификации
    });

    // Превращаем комплексную структуру в отформатированный текст
    let final_text = serde_json::to_string_pretty(&full_envelope)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    log::info!("generate_project_json_string: Данные JSON сформированы.");

    // Возвращаем готовую строку приложению
    Ok(final_text)
}

// -----------------------------------------------------------------------------
// ДЕСЕРИАЛИЗАЦИЯ
// -----------------------------------------------------------------------------

/// Функция принимает JSON-текст проекта, восстанавливает VTable,
/// возвращает счетчик field_counter и реактивно формирует Rc-чертежи в таблице.
pub fn deserialize_from_json_string(
    factory: &mut Factory,
    json_text: &str,
) -> Result<(), std::io::Error> {
    log::info!("deserialize_from_json_string: Разбор текстовой строки JSON.");

    // Распаковываем JSON-структуру
    let envelope: serde_json::Value = serde_json::from_str(json_text).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Неверный формат JSON: {}", e),
        )
    })?;

    // Восстанавливаем генератор уникальности ID виджетов
    let loaded_counter = envelope["field_counter"].as_u64().unwrap_or(0) as usize;
    factory.field_counter = loaded_counter;

    let widgets_order = &envelope["widgets_order"];
    let blueprints_data = &envelope["blueprints"];
    let field_values_data = &envelope["field_values"];

    // Очищаем текущий холст перед загрузкой нового
    factory.clear_blueprints();

    // -------------------------------------------------------------------------
    // Восстановливаем свойства в таблицу VTABLE
    log::info!("deserialize_from_json_string: Восстановление VTable-свойств...");

    // Очищаем старую VTable перед восстановлением
    /*{
        let mut storage_guard = factory.field_values.borrow_mut();
        storage_guard.data.clear();
        storage_guard.names.clear();
    }*/

    if let Some(widgets_values_map) = field_values_data.as_object() {
        for (widget_id, properties_map) in widgets_values_map {
            // Записываем текстовое имя виджета в реестр имен VTable по его числовому хэшу
            let widget_hash = crate::core::utils::runtime_hash_64(widget_id);
            factory
                .get_field_values_mut()
                //.borrow_mut()
                .names
                .insert(widget_hash, widget_id.clone());

            if let Some(props) = properties_map.as_object() {
                for (prop_name, json_val) in props {
                    // Создаем PropertyKey по имени параметра. Он заглянет в ALL_PROPERTY_TOKENS
                    // и автоматически узнает правильный тип данных из макроса
                    let prop_meta_key = PropertyKey::meta_from_dynamic(prop_name);
                    let prop_key = PropertyKey::from_metadata(prop_meta_key);
                    let prop_type_name = prop_meta_key.type_name.to_string(); // "f32", "Length", "Padding" и т.д.

                    // Превращаем JSON-значение в строку для отправки в утилиты парсинга
                    let val_str = match json_val {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        serde_json::Value::Number(n) => n.to_string(),

                        // Пропускаем некорректные вложенные структуры
                        //_ => continue,
                        unsupported_json => {
                            log::error!(
                                "[Парсер] Пропущено некорректное свойство в JSON! \
                                Виджет: '{}', Параметр: '{}', Значение: {:?}",
                                widget_id,
                                prop_name,
                                unsupported_json
                            );
                            continue;
                        }
                    };

                    // Парсинг матрицы типов (Используем функции utils::cast_string_2_*)
                    match prop_type_name.as_str() {
                        "String" => {
                            factory.set::<String>(widget_id, prop_key, val_str);
                        }
                        "f32" => {
                            //if let Ok(val) = val_str.parse::<f32>() { factory.set::<f32>(widget_id, prop_key, val); }
                            match val_str.parse::<f32>() {
                                Ok(val) => {
                                    factory.set::<f32>(widget_id, prop_key, val);
                                }
                                Err(e) => {
                                    log::error!(
                                        "[Парсер f32] Ошибка конвертации! Виджет: '{}', Свойство: '{}', \
                                        Исходная строка '{}'. Ошибка: {}",
                                        widget_id,
                                        prop_name,
                                        val_str,
                                        e
                                    );
                                }
                            }
                        }
                        "bool" => {
                            //if let Ok(val) = val_str.parse::<bool>() { factory.set::<bool>(widget_id, prop_key, val); }
                            match val_str.parse::<bool>() {
                                Ok(val) => {
                                    factory.set::<bool>(widget_id, prop_key, val);
                                }
                                Err(e) => {
                                    log::error!(
                                        "[Парсер bool] Ошибка конвертации! Виджет: '{}', Свойство: '{}', \
                                        Исходная строка '{}'. Ошибка: {}",
                                        widget_id,
                                        prop_name,
                                        val_str,
                                        e
                                    );
                                }
                            }
                        }
                        "Length" => {
                            let val = cast_string_2_length(&val_str);
                            factory.set::<iced::Length>(widget_id, prop_key, val);
                        }
                        "Pixels" => {
                            let val = cast_string_2_pixels(&val_str);
                            factory.set::<iced::Pixels>(widget_id, prop_key, val);
                        }
                        "Padding" => {
                            let parsed_option = cast_string_2_padding(&val_str);
                            let val: iced::Padding = parsed_option.unwrap_or(iced::Padding::ZERO);
                            factory.set::<iced::Padding>(widget_id, prop_key, val);
                        }
                        "Radius" => {
                            let val = cast_string_2_radius(&val_str);
                            factory.set::<iced::border::Radius>(widget_id, prop_key, val);
                        }
                        "Color" => {
                            // Если функция возвращает Option<Color>, распаковываем её с дефолтом TRANSPARENT
                            let val =
                                cast_hex_2_color(&val_str).unwrap_or(iced::Color::TRANSPARENT);
                            factory.set::<iced::Color>(widget_id, prop_key, val);
                        }
                        "Horizontal" => {
                            let val = cast_string_2_align_x(&val_str)
                                .unwrap_or(iced::alignment::Horizontal::Left);
                            factory.set::<iced::alignment::Horizontal>(widget_id, prop_key, val);
                        }
                        "Vertical" => {
                            let val = cast_string_2_align_y(&val_str)
                                .unwrap_or(iced::alignment::Vertical::Top);
                            factory.set::<iced::alignment::Vertical>(widget_id, prop_key, val);
                        }
                        _ => {
                            log::warn!(
                                "   [VTable] Неизвестный тип данных для парсинга: '{}'",
                                prop_type_name
                            );
                        }
                    }
                }
            }
        }
    }

    // -------------------------------------------------------------------------
    // Восстановление иерархии чертежей
    log::info!("deserialize_from_json_string: Реактивная сборка Rc-чертежей...");

    // Fix: Order
    // Восстанавливаем таблицу блюпринтов в исходном порядке как в векторе JSON
    if let Some(order_array) = widgets_order.as_array() {
    //if let Some(blueprints_map) = blueprints_data.as_object() {

        // Fix: Order
        // Итерируемся по упорядоченным ID виджетов
        for id_value in order_array {
        //for (widget_id, bp_info) in blueprints_map {

            // Преобразуем JSON-строку в обычный Rust &str / String
            if let Some(widget_id_str) = id_value.as_str() {
                let widget_id = widget_id_str.to_string();
        
                // Ищем данные блупринта в карте по конкретному упорядоченному ID
                let bp_info = &blueprints_data[&widget_id];

                // Вытаскиваем строковый тип виджета (например, "button", "text_editor")
                if let Some(widget_type) = bp_info["type"].as_str() {
                    // Находим нужный Rc-креатор, зарегистрированный через инвентарь фабрики
                    if let Some(creator) = factory.creators.get(widget_type) {
                        // Запрашиваем креатор сформировать изолированный блупринт
                        let blueprint_rc = creator.create_blueprint(widget_id.clone());

                        // Восстанавливаем встроенные свойства блюпринта из VTable (если такие реализованы)
                        blueprint_rc.from_vtab(&factory);

                        // Добавляем созданный чертеж в таблицу IndexMap
                        factory.insert_blueprint(&widget_id, blueprint_rc);

                        log::info!("   [Создан] Виджет '{}' типа '{}'", widget_id, widget_type);
                    } else {
                        log::warn!(
                            "   [Пропущен] Креатор для типа '{}' не обнаружен на фабрике!",
                            widget_type
                        );
                    }
                }
            }
        }
    } else {
        log::error!("Поле 'widgets_order' отсутствует или не является массивом!");
        return Ok(());
    }

    log::info!("deserialize_from_json_string: Проект успешно восстановлен.");
    Ok(())
}

// -----------------------------------------------------------------------------
// ЭКСПОРТ
// -----------------------------------------------------------------------------
pub fn generate_export_json_string(factory: &Factory) -> Result<String, std::io::Error> {
    log::info!(
        "serialization::generate_export_json_string: ГНачата сериализация проекта для экспорта в JSON..."
    );

    // Создаем вектор для сохранения порядка элементов
    let mut widgets_order = Vec::new();

    // =====================================================================
    // Генерация справичников типов (Types Registry Schema)
    // =====================================================================
    let mut types_registry = BTreeMap::new();

    // Сканируем наш thread_local реестр, созданный макросом declare_properties!
    crate::core::ALL_PROPERTY_TOKENS.with(|tokens| {
        let guard = tokens.borrow();
        for token in guard.iter() {
            // Исключаем пустые или тестовые маркеры
            if !token.name.is_empty() {
                types_registry.insert(token.name.to_string(), token.type_name.to_string());
            }
        }
    });

    // =====================================================================
    // Сборка виджетов со свойствами
    // =====================================================================
    let mut widgets_json_map = BTreeMap::new();

    // Перебираем склад чертежей (blueprints) холста
    for (widget_id, blueprint) in factory.blueprints_iter() {
        // Добавляем ID в массив порядка (сохраняет порядок из IndexMap)
        widgets_order.push(widget_id.clone());

        // Базовый конверт метаданных виджета
        let mut widget_envelope = json!({
            "type": blueprint.widget_type(),
            "meta": {
                "id": widget_id,
                "local_index": blueprint.get_index()
            }
        });

        let mut properties_json_map = BTreeMap::new();

        // Открываем RefCell хранилища VTable
        let storage_guard = factory.get_field_values();
        let widget_hash   = crate::core::utils::runtime_hash_64(widget_id);

        // (+++) 
        //Вызываем метод фильтрации недефолтных значений у текущего блупринта.
        let mut allowed_properties: Vec<String> = blueprint
            .get_exportable_property_names(factory)
            .into_iter()
            .map(|key| key.name.to_string())
            .collect();

        // Также добавляем обязательные поля
        allowed_properties.push("parent".to_string());
        // (+++)

        if let Some(prop_map) = storage_guard.data.get(&widget_hash) {
            for (prop_hash, _any_boxed) in prop_map {
                // Находим точный PropertyKey свойства
                let prop_key = crate::core::ALL_PROPERTY_TOKENS.with(|tokens| {
                    let guard = tokens.borrow();
                    guard
                        .iter()
                        .find(|m| m.hash == *prop_hash)
                        .map(|meta| PropertyKey::from_dynamic(meta.name))
                        .unwrap_or_else(|| PropertyKey::from_dynamic("unknown"))
                });

                if prop_key.name == "unknown" {
                    continue;
                }

                // (+++) 
                // Если имя текущего свойства из VTable не входит в белый список блупринта -> ПРОПУСКАЕМ
                if !allowed_properties.contains(&prop_key.name.to_string()) {
                    continue;
                }
                // (+++)

                // Создаем PropertyKey по имени параметра. Он заглянет в ALL_PROPERTY_TOKENS
                // и автоматически узнает правильный тип данных из макроса
                let prop_meta_key  = PropertyKey::meta_from_dynamic(prop_key.name);
                let prop_type_name = prop_meta_key.type_name.to_string(); // "f32", "Length", "Padding" и т.д.

                // РАЗБОР СЛОЖНЫХ СТРУКТУР НА JSON-ОБЪЕКТЫ
                let json_value = match prop_type_name.as_str() {
                    // Числа и флаги пишем нативными примитивами JSON без кавычек
                    "f32" => {
                        let val: f32 = factory.get_by_hash(widget_id, *prop_hash).unwrap_or(0.0);
                        json!(val)
                    }
                    "bool" => {
                        let val: bool = factory.get_by_hash(widget_id, *prop_hash).unwrap_or(false);
                        json!(val)
                    }
                    "Pixels" => {
                        let val: iced::Pixels = factory
                            .get_by_hash(widget_id, *prop_hash)
                            .unwrap_or(iced::Pixels(0.0));
                        json!(val.0) // Вытаскиваем чистое f32 через .0
                    }

                    // Сложный отступ Padding раскладываем на чёткий CSS-объект!
                    "Padding" => {
                        let val: iced::Padding = factory
                            .get_by_hash(widget_id, *prop_hash)
                            .unwrap_or(iced::Padding::ZERO);
                        json!({
                            "top":    val.top,
                            "right":  val.right,
                            "bottom": val.bottom,
                            "left":   val.left
                        })
                    }

                    // Скругление углов Radius раскладываем по 4-м углам!
                    "Radius" => {
                        let val: iced::border::Radius = factory
                            .get_by_hash(widget_id, *prop_hash)
                            .unwrap_or(iced::border::Radius::new(0.0));
                        json!({
                            "top_left":     val.top_left,
                            "top_right":    val.top_right,
                            "bottom_right": val.bottom_right,
                            "bottom_left":  val.bottom_left
                        })
                    }

                    // Размеры Length (Fill, Shrink, Fixed) превращаем в понятный полиморфный объект! [1.2]
                    "Length" => {
                        let val: iced::Length = factory
                            .get_by_hash(widget_id, *prop_hash)
                            .unwrap_or(iced::Length::Shrink);
                        match val {
                            iced::Length::Fill => json!({ "mode": "Fill", "pixels": 0.0 }),
                            iced::Length::Shrink => json!({ "mode": "Shrink", "pixels": 0.0 }),
                            iced::Length::Fixed(p) => json!({ "mode": "Fixed", "pixels": p }),
                            _ => json!({ "mode": "Shrink", "pixels": 0.0 }),
                        }
                    }

                    // Цвета Color переводим в HEX строку для удобства веб-интеграций
                    "Color" => {
                        let val: iced::Color = factory
                            .get_by_hash(widget_id, *prop_hash)
                            .unwrap_or(iced::Color::TRANSPARENT);
                        json!(cast_color_2_hex(val))
                    }

                    // Все остальные перечисления и строки (String, Horizontal, Vertical) пишем обычным текстом
                    _ => {
                        let string_value = factory.get_as_string(widget_id, prop_key, "");
                        json!(string_value)
                    }
                };

                properties_json_map.insert(prop_key.name.to_string(), json_value);
            }
        }

        // Инжектируем чистый структурированный объект свойств внутрь виджета
        if let Some(widget_obj) = widget_envelope.as_object_mut() {
            widget_obj.insert("properties".to_string(), json!(properties_json_map));
        }

        widgets_json_map.insert(widget_id.clone(), widget_envelope);
    }

    // =====================================================================
    // УПАКОВКА В СИСТЕМНЫЙ КОНВЕРТ CAD SCHEMA
    // =====================================================================
    let full_envelope = json!({
        "field_counter":  factory.field_counter,
        "widgets_order":  widgets_order,

        "types_registry": types_registry,
        "widgets":        widgets_json_map
    });

    // Переводим в красивую JSON строку
    let final_text = serde_json::to_string_pretty(&full_envelope)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    log::info!(
        "✅ serialization::generate_project_json_string: Монолитный JSON и Schema успешно собраны."
    );
    Ok(final_text)
}
