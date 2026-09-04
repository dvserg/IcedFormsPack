use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
//use log::{info, warn};

pub use crate::core::{PropertyRegistry, PropertyType, utils};

// -----------------------------------------------------------------------------
// Структуры иерархии виджетов и их свойств
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WidgetMeta {
    pub id: String,

    #[serde(deserialize_with = "utils::parse_string_or_int")]
    pub local_index: i32,
}

// Схема объектного разбора 'Length'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonLength {
    pub mode: String, // "Fill", "Shrink", "Fixed"
    pub pixels: f32,
}

// Схема объектного разбора отступов 'Padding'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonPadding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

// Схема объектного разбора скруглений 'Radius'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

// Схема разбора Float
// Гарантирует наличие точки и нуля (нотация f32)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "serde_json::Value")]
pub struct JsonFloat(pub String);

impl From<serde_json::Value> for JsonFloat {
    fn from(val: serde_json::Value) -> Self {
        // Вытаскиваем число, даже если в JSON оно записано как целое 8 или float 8.0
        let num = val.as_f64().unwrap_or(0.0);
        // Добавляем один знак после запятой
        JsonFloat(format!("{:.1}", num))
    }
}

// Схема разбора Color
// Парсит HEX и хранит готовый код для Rust/Iced
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "String")] // При десериализации сначала читаем обычную строку
pub struct JsonColor(pub String);

impl From<String> for JsonColor {
    fn from(hex: String) -> Self {
        let clean = hex.trim();
        if clean == "transparent" {
            return JsonColor("iced::Color::TRANSPARENT".to_string());
        }
        let digits = clean.trim_start_matches('#');
        if digits.len() != 6 {
            return JsonColor("iced::Color::BLACK".to_string());
        }
        // Распаршиваем 16-ричные байты в диапазон 0.0..1.0
        let r = u8::from_str_radix(&digits[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&digits[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&digits[4..6], 16).unwrap_or(0) as f32 / 255.0;

        // Сразу сохраняем как готовый, идеальный текст для генератора!
        JsonColor(format!(
            "iced::Color::from_rgb({:.3}, {:.3}, {:.3})",
            r, g, b
        ))
    }
}

// -----------------------------------------------------------------------------
// Свойства виджета с динамическими типами
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(from = "serde_json::Value", into = "serde_json::Value")]
pub struct WidgetProperties {
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl WidgetProperties {
    // Инициализация хранилища свойств
    pub fn new() -> Self {
        Self {
            fields: BTreeMap::new(),
        }
    }

    /// Безопасное извлечение свойств (Защита от сбоев типов)
    /// Ищет свойство и возвращает его как строковый срез &str
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.fields.get(key).and_then(|v| v.as_str())
    }

    /// Безопасное извлечение чисел (Защита от пустых кавычек "")
    /// Ищет свойство, автоматически срезает кавычки, если число прилетело строкой,
    /// и возвращает чистое f32. При пустом вводе "" отдает дефолтный ноль.
    pub fn get_f32(&self, key: &str) -> f32 {
        let Some(json_val) = self.fields.get(key) else {
            return 0.0;
        };

        match json_val {
            // Если в JSON число (например, 10.0)
            serde_json::Value::Number(num) => num.as_f64().unwrap_or(0.0) as f32,
            // Если в Инспекторе очистили поле и прилетела строка "" или "12.5"
            serde_json::Value::String(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    0.0
                } else {
                    trimmed.parse::<f32>().unwrap_or(0.0)
                }
            }
            _ => 0.0,
        }
    }

    /// Безопасное извлечение флагов (bool)
    pub fn get_bool(&self, key: &str) -> bool {
        self.fields
            .get(key)
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Безопасный доступ к родителю (Упрощает контроль иерархии)
    pub fn parent_id(&self) -> &str {
        self.get_str("parent").unwrap_or("root")
    }

    // Предпарсинг типов для кодогенератора
    /// МЕТА-УПРАВЛЯЕМЫЙ АВТОМАТ ТИПОВ (Type-Driven Scaffolding Engine)
    /// Пробегается по сырым свойствам, сопоставляет их с реестром типов Schema
    /// и генерирует код. Автоматически логирует сбои и делает safe-откат!
    pub fn prepare_generator_codes(&mut self, registry: &PropertyRegistry) {
        log::info!("prepare_generator_codes: Выполняется предпарсинг типов..");

        // Вытаскиваем только пользовательские CAD-параметры, игнорируя служебный parent
        let keys: Vec<String> = self
            .fields
            .keys()
            .filter(|k| *k != "parent")
            .cloned()
            .collect();

        for key in keys {
            let val = &self.fields[&key];

            // Узнаем Enum-тип свойства по справочнику реестра макета
            let prop_type = registry.get_type(&key);

            log::info!(
                "prepare_generator_codes: Свойство '{}' {}",
                key,
                prop_type.as_str()
            );

            match prop_type {
                // Тип: Color (HEX-строка => iced::Color::from_rgb)
                PropertyType::Color => {
                    let mut success = false;
                    if let Some(hex_str) = val.as_str() {
                        let clean = hex_str.trim();
                        if clean.starts_with('#') || clean == "transparent" {
                            let rust_color = if clean == "transparent" {
                                "iced::Color::TRANSPARENT".to_string()
                            } else {
                                let digits = clean.trim_start_matches('#');
                                if digits.len() == 6 {
                                    let r = u8::from_str_radix(&digits[0..2], 16).unwrap_or(0)
                                        as f32
                                        / 255.0;
                                    let g = u8::from_str_radix(&digits[2..4], 16).unwrap_or(0)
                                        as f32
                                        / 255.0;
                                    let b = u8::from_str_radix(&digits[4..6], 16).unwrap_or(0)
                                        as f32
                                        / 255.0;
                                    format!("iced::Color::from_rgb({:.3}, {:.3}, {:.3})", r, g, b)
                                } else {
                                    "iced::Color::BLACK".to_string()
                                }
                            };
                            self.fields
                                .insert(format!("{}_code", key), serde_json::Value::String(rust_color));
                            success = true;
                        }
                    }

                    if !success {
                        log::warn!(
                            "prepare_generator_codes: Свойство '{}' ожидает тип Color, но содержит валидный HEX. Откат к BLACK.",
                            key
                        );
                        self.fields.insert(
                            format!("{}_code", key),
                            serde_json::Value::String("iced::Color::BLACK".to_string()),
                        );
                    }
                }

                // Типы: f32 / Pixels (Одиночные числа => Нотация float 0.0)
                PropertyType::F32 | PropertyType::Pixels => {
                    if let Some(num_val) = val.as_f64() {
                        self.fields.insert(
                            format!("{}_code", key),
                            serde_json::Value::String(format!("{:.1}", num_val)),
                        );
                    } else if let Some(str_val) = val.as_str() {
                        // Защита: если число случайно прилетело строкой из Инспектора
                        if let Ok(parsed_num) = str_val.trim().parse::<f64>() {
                            self.fields.insert(
                                format!("{}_code", key),
                                serde_json::Value::String(format!("{:.1}", parsed_num)),
                            );
                        } else {
                            log::warn!(
                                "prepare_generator_codes: Не удалось распарсить число в '{}'. Откат к 0.0",
                                key
                            );
                            self.fields.insert(
                                format!("{}_code", key),
                                serde_json::Value::String("0.0".to_string()),
                            );
                        }
                    } else {
                        log::warn!(
                            "prepare_generator_codes: Поле '{}' нарушает тип Number. Откат к 0.0",
                            key
                        );
                        self.fields.insert(
                            format!("{}_code", key),
                            serde_json::Value::String("0.0".to_string()),
                        );
                    }
                }

                // -------------------------------------------------------------
                // Padding
                // Оптимизируем входящий padding:
                // [4_f32] -> [1_f32]; [4_f32] -> [2_f32]
                // -------------------------------------------------------------
                PropertyType::Padding => {
                    // ВАРИАНТ А: Паддинг задан как сложный объект {"top": 8, "right": 16, ...}
                    if let Some(sub_obj) = val.as_object() {
                        let top = sub_obj.get("top").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let right = sub_obj.get("right").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let bottom = sub_obj
                            .get("bottom")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let left = sub_obj.get("left").and_then(|v| v.as_f64()).unwrap_or(0.0);

                        let padding_code = if top == 0.0
                            && right == 0.0
                            && bottom == 0.0
                            && left == 0.0
                        {
                            "iced::Padding::ZERO".to_string()
                        } else if top == right && right == bottom && bottom == left {
                            // ОПТИМИЗАЦИЯ 1: Если все стороны абсолютно равны
                            format!("iced::Padding::from({:.1}_f32)", top)
                        } else if top == bottom && left == right {
                            // ОПТИМИЗАЦИЯ 2: Если равны пары (Вертикаль / Горизонталь)
                            // По спецификации Iced [f32; 2] принимает: [horizontal, vertical]
                            format!("iced::Padding::from([{:.1}_f32, {:.1}_f32])", left, top)
                        } else {
                            // В противном случае — строим полную структуру
                            format!(
                                "iced::Padding {{ top: {:.1}_f32, right: {:.1}_f32, bottom: {:.1}_f32, left: {:.1}_f32 }}",
                                top, right, bottom, left
                            )
                        };
                        self.fields.insert(
                            format!("{}_code", key),
                            serde_json::Value::String(padding_code),
                        );
                    }
                    // ВАРИАНТ Б: Паддинг задан как массив в JSON [top, right, bottom, left] или [h, v]
                    else if let Some(sub_arr) = val.as_array() {
                        let padding_code = match sub_arr.len() {
                            // Если в JSON пришло два числа: [horizontal, vertical]
                            2 => {
                                let h = sub_arr.get(0).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let v = sub_arr.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);

                                if h == v {
                                    // ОПТИМИЗАЦИЯ 3: Если в двумерном массиве числа совпали (редко, но бывает)
                                    format!("iced::Padding::from({:.1}_f32)", h)
                                } else {
                                    format!("iced::Padding::from([{:.1}_f32, {:.1}_f32])", h, v)
                                }
                            }
                            // Если в JSON пришло четыре числа [top, right, bottom, left]
                            4 => {
                                let top = sub_arr.get(0).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let right = sub_arr.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let bottom = sub_arr.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let left = sub_arr.get(3).and_then(|v| v.as_f64()).unwrap_or(0.0);

                                if top == 0.0 && right == 0.0 && bottom == 0.0 && left == 0.0 {
                                    "iced::Padding::ZERO".to_string()
                                } else if top == right && right == bottom && bottom == left {
                                    // ОПТИМИЗАЦИЯ 4: Все 4 элемента массива равны
                                    format!("iced::Padding::from({:.1}_f32)", top)
                                } else if top == bottom && left == right {
                                    // ОПТИМИЗАЦИЯ 5: Элементы массива образуют пары [В, Г]
                                    format!(
                                        "iced::Padding::from([{:.1}_f32, {:.1}_f32])",
                                        left, top
                                    )
                                } else {
                                    format!(
                                        "iced::Padding {{ top: {:.1}_f32, right: {:.1}_f32, bottom: {:.1}_f32, left: {:.1}_f32 }}",
                                        top, right, bottom, left
                                    )
                                }
                            }
                            _ => "iced::Padding::ZERO".to_string(),
                        };
                        self.fields.insert(
                            format!("{}_code", key),
                            serde_json::Value::String(padding_code),
                        );
                    }
                }

                // -------------------------------------------------------------
                // Radius
                // Оптимизируем входящий radius:
                // [4_f32] -> [1_f32]; [4_f32] -> [2_f32]
                // -------------------------------------------------------------
                PropertyType::Radius => {
                    let mut code_generated = false;

                    // ВАРИАНТ А: Radius задан как сложный объект с четырьмя углами
                    if let Some(sub_obj) = val.as_object() {
                        let tl = sub_obj
                            .get("top_left")
                            .or_else(|| sub_obj.get("tl"))
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let tr = sub_obj
                            .get("top_right")
                            .or_else(|| sub_obj.get("tr"))
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let br = sub_obj
                            .get("bottom_right")
                            .or_else(|| sub_obj.get("br"))
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let bl = sub_obj
                            .get("bottom_left")
                            .or_else(|| sub_obj.get("bl"))
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);

                        let radius_code = if tl == 0.0 && tr == 0.0 && br == 0.0 && bl == 0.0 {
                            "iced::border::Radius::ZERO".to_string()
                        } else if tl == tr && tr == br && br == bl {
                            // ОПТИМИЗАЦИЯ 1: Все 4 угла абсолютно равны
                            format!("iced::border::Radius::from({:.1}_f32)", tl)
                        } else if tl == tr && br == 0.0 && bl == 0.0 {
                            // ОПТИМИЗАЦИЯ 2: Скруглен только верх (удобно для вкладок/табов)
                            format!("iced::border::top({:.1}_f32)", tl)
                        } else if bl == br && tl == 0.0 && tr == 0.0 {
                            // ОПТИМИЗАЦИЯ 3: Скруглен только низ
                            format!("iced::border::bottom({:.1}_f32)", bl)
                        } else if tl == bl && tr == 0.0 && br == 0.0 {
                            // ОПТИМИЗАЦИЯ 4: Скруглен только левый край
                            format!("iced::border::left({:.1}_f32)", tl)
                        } else if tr == br && tl == 0.0 && bl == 0.0 {
                            // ОПТИМИЗАЦИЯ 5: Скруглен только правый край
                            format!("iced::border::right({:.1}_f32)", tr)
                        } else {
                            // В противном случае собираем полный массив из 4 элементов через From/Into
                            // Порядок в Iced 0.14: [top_left, top_right, bottom_right, bottom_left]
                            format!(
                                "iced::border::Radius::from([{:.1}_f32, {:.1}_f32, {:.1}_f32, {:.1}_f32])",
                                tl, tr, br, bl
                            )
                        };
                        self.fields.insert(
                            format!("{}_code", key),
                            serde_json::Value::String(radius_code),
                        );
                        code_generated = true;
                    }
                    // ВАРИАНТ Б: Radius задан как одиночное число (например, 4.0)
                    else if let Some(num_val) = val.as_f64() {
                        let radius_code = if num_val == 0.0 {
                            "iced::border::Radius::ZERO".to_string()
                        } else {
                            // 🔥 Спецификация 0.14: Предлагает использовать лаконичный .from() вместо .new()
                            format!("iced::border::Radius::from({:.1}_f32)", num_val)
                        };
                        self.fields.insert(
                            format!("{}_code", key),
                            serde_json::Value::String(radius_code),
                        );
                        code_generated = true;
                    }
                    // ВАРИАНТ В: Защита от строк (если число прилетело из текстового поля "4")
                    else if let Some(str_val) = val.as_str() {
                        if let Ok(parsed_num) = str_val.trim().parse::<f64>() {
                            let radius_code = if parsed_num == 0.0 {
                                "iced::border::Radius::ZERO".to_string()
                            } else {
                                format!("iced::border::Radius::from({:.1}_f32)", parsed_num)
                            };
                            self.fields.insert(
                                format!("{}_code", key),
                                serde_json::Value::String(radius_code),
                            );
                            code_generated = true;
                        }
                    }
                    // ВАРИАНТ Г: Если в JSON прилетел массив из 4 чисел [tl, tr, br, bl]
                    else if let Some(sub_arr) = val.as_array() {
                        if sub_arr.len() == 4 {
                            let tl = sub_arr.get(0).and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let tr = sub_arr.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let br = sub_arr.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let bl = sub_arr.get(3).and_then(|v| v.as_f64()).unwrap_or(0.0);

                            let radius_code = if tl == 0.0 && tr == 0.0 && br == 0.0 && bl == 0.0 {
                                "iced::border::Radius::ZERO".to_string()
                            } else if tl == tr && tr == br && br == bl {
                                format!("iced::border::Radius::from({:.1}_f32)", tl)
                            } else {
                                format!(
                                    "iced::border::Radius::from([{:.1}_f32, {:.1}_f32, {:.1}_f32, {:.1}_f32])",
                                    tl, tr, br, bl
                                )
                            };
                            self.fields.insert(
                                format!("{}_code", key),
                                serde_json::Value::String(radius_code),
                            );
                            code_generated = true;
                        }
                    }

                    if !code_generated {
                        log::warn!(
                            "prepare_generator_codes: Поле '{}' не является структурой Radius. Назначен Radius::ZERO.",
                            key
                        );
                        self.fields.insert(
                            format!("{}_code", key),
                            serde_json::Value::String("iced::border::Radius::ZERO".to_string()),
                        );
                    }
                }

                // =============================================================
                // ТИПЫ: STRING / BOOL / LENGTH (Прямой проброс или простые структуры)
                // =============================================================
                PropertyType::Bool => {
                    let bool_val = val.as_bool().unwrap_or(false);
                    self.fields.insert(
                        format!("{}_code", key),
                        serde_json::Value::String(bool_val.to_string()),
                    );
                }
                PropertyType::String => {
                    let rust_string_code = match val {
                        // Если в JSON лежит честная строка
                        serde_json::Value::String(str_val) => {
                            //println!("{:?}", str_val);
                            let clean = str_val;
                            if clean.is_empty() {
                                "\"\"".to_string() // Защита: пустая строка превращается в ""
                            } else {
                                let escaped = clean
                                    .replace('\\', "\\\\")
                                    .replace('"', "\\\"")
                                    .replace('\'', "\\'");
                                format!("\"{}\"", escaped) // Оборачиваем в двойные кавычки Rust
                            }
                        }
                        // Защита от Null или пропусков в JSON: выдаем безопасную пустую строку Rust
                        serde_json::Value::Null => "\"\"".to_string(),
                        // Если по ошибке записали число или булево — кастим в строку и оборачиваем в кавычки
                        _ => format!("'\"{}\"'", val.to_string().replace('"', "\\\"")),
                    };

                    self.fields.insert(
                        format!("{}_code", key),
                        serde_json::Value::String(rust_string_code),
                    );
                }
                PropertyType::Length => {
                    let mut code_generated = false;

                    if let Some(sub_obj) = val.as_object() {
                        let mode = sub_obj
                            .get("mode")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Shrink");
                        let pixels = sub_obj
                            .get("pixels")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);

                        let len_code = match mode {
                            "Fill" => "iced::Length::Fill".to_string(),
                            // Фикс: Внутри Fixed передаем СТРОГО float-число с точкой, а не Length! [🌐]
                            "Fixed" => format!("iced::Length::Fixed({:.1})", pixels),
                            "Shrink" | "Auto" => "iced::Length::Shrink".to_string(),
                            _ => "iced::Length::Shrink".to_string(),
                        };
                        self.fields
                            .insert(format!("{}_code", key), serde_json::Value::String(len_code));
                        code_generated = true;
                    }
                    // Защита: если размер прилетел в виде обычной строки (например, "Fill" или "Shrink")
                    else if let Some(str_val) = val.as_str() {
                        let len_code = match str_val.trim() {
                            "Fill" => "iced::Length::Fill".to_string(),
                            "Shrink" | "Auto" => "iced::Length::Shrink".to_string(),
                            _ => "iced::Length::Shrink".to_string(),
                        };
                        self.fields
                            .insert(format!("{}_code", key), serde_json::Value::String(len_code));
                        code_generated = true;
                    }

                    if !code_generated {
                        log::warn!(
                            "⚠️ [Schema Сбой]: Ошибка структуры Length в '{}'. Применен Length::Shrink.",
                            key
                        );
                        self.fields.insert(
                            format!("{}_code", key),
                            serde_json::Value::String("iced::Length::Shrink".to_string()),
                        );
                    }
                }
                PropertyType::Vertical => {
                    let mut success = false;
                    if let Some(str_val) = val.as_str() {
                        let clean = str_val.trim();
                        let rust_align = match clean {
                            "Top" | "top" | "Start" | "start" => {
                                Some("iced::alignment::Vertical::Top")
                            }
                            "Center" | "center" | "Middle" | "middle" => {
                                Some("iced::alignment::Vertical::Center")
                            }
                            "Bottom" | "bottom" | "End" | "end" => {
                                Some("iced::alignment::Vertical::Bottom")
                            }
                            _ => None,
                        };

                        if let Some(code) = rust_align {
                            self.fields.insert(
                                format!("{}_code", key),
                                serde_json::Value::String(code.to_string()),
                            );
                            success = true;
                        }
                    }

                    if !success {
                        log::warn!(
                            "[Schema Сбой]: Поле выравнивания '{}' содержит неверное значение. Откат к Vertical::Top.",
                            key
                        );
                        self.fields.insert(
                            format!("{}_code", key),
                            serde_json::Value::String("iced::alignment::Vertical::Top".to_string()),
                        );
                    }
                }
                PropertyType::Horizontal => {
                    let mut success = false;
                    if let Some(str_val) = val.as_str() {
                        let clean = str_val.trim();
                        let rust_align = match clean {
                            "Left" | "left" | "Start" | "start" => {
                                Some("iced::alignment::Horizontal::Left")
                            }
                            "Center" | "center" | "Middle" | "middle" => {
                                Some("iced::alignment::Horizontal::Center")
                            }
                            "Right" | "right" | "End" | "end" => {
                                Some("iced::alignment::Horizontal::Right")
                            }
                            _ => None,
                        };

                        if let Some(code) = rust_align {
                            self.fields.insert(
                                format!("{}_code", key),
                                serde_json::Value::String(code.to_string()),
                            );
                            success = true;
                        }
                    }

                    if !success {
                        log::warn!(
                            "[Schema Сбой]: Поле выравнивания '{}' содержит неверное значение. Откат к Horizontal::Left.",
                            key
                        );
                        self.fields.insert(
                            format!("{}_code", key),
                            serde_json::Value::String(
                                "iced::alignment::Horizontal::Left".to_string(),
                            ),
                        );
                    }
                }
                _ => {
                    log::warn!(
                        "prepare_generator_codes: Свойство '{}' отсутствует в реестре типов. Пытаюсь угадать тип...",
                        key
                    );
                    // Предположим String
                    // Строки Tera выведет через кавычки нативно, пробрасываем как есть
                    let str_val = val.as_str().unwrap_or("");
                    self.fields.insert(
                        format!("{}_code", key),
                        serde_json::Value::String(str_val.to_string()),
                    );
                }
            }
        }

        log::info!("prepare_generator_codes: Предпарсинг типов завершен.");
    }
}

// Автоматическая десериализация: Перехватывает весь JSON-блок свойств как объект!
impl From<serde_json::Value> for WidgetProperties {
    fn from(value: serde_json::Value) -> Self {
        let mut fields = BTreeMap::new();

        if let Some(obj) = value.as_object() {
            for (k, v) in obj {
                // =============================================================
                // УМНЫЙ НОРМАЛИЗАТОР ГЕОМЕТРИИ (Auto-Expansion Engine)
                // Если радиус или паддинг пришли числом, разворачиваем их в объект,
                // чтобы не злить строгий Schema-валидатор проекта!
                // =============================================================
                if (k == "border_radius" || k == "radius") && v.is_number() {
                    let num = v.as_f64().unwrap_or(0.0);
                    let mut radius_map = serde_json::Map::new();
                    radius_map.insert("top_left".to_string(), serde_json::json!(num));
                    radius_map.insert("top_right".to_string(), serde_json::json!(num));
                    radius_map.insert("bottom_right".to_string(), serde_json::json!(num));
                    radius_map.insert("bottom_left".to_string(), serde_json::json!(num));

                    fields.insert(k.clone(), serde_json::Value::Object(radius_map));
                } else if k == "padding" && v.is_number() {
                    let num = v.as_f64().unwrap_or(0.0);
                    let mut padding_map = serde_json::Map::new();
                    padding_map.insert("top".to_string(), serde_json::json!(num));
                    padding_map.insert("right".to_string(), serde_json::json!(num));
                    padding_map.insert("bottom".to_string(), serde_json::json!(num));
                    padding_map.insert("left".to_string(), serde_json::json!(num));

                    fields.insert(k.clone(), serde_json::Value::Object(padding_map));
                } else {
                    // Все остальные свойства пробрасываем как есть
                    fields.insert(k.clone(), v.clone());
                }
                // =============================================================
            }
        }
        WidgetProperties { fields }
    }
}

// Зеркальная конвертация в  JSON
impl From<WidgetProperties> for serde_json::Value {
    fn from(props: WidgetProperties) -> Self {
        let mut map = serde_json::Map::new();
        for (k, v) in props.fields {
            map.insert(k, v);
        }
        serde_json::Value::Object(map)
    }
}

// -----------------------------------------------------------------------------
// Узел виджета
// Содержит данные виджета: тип, метаданные и свойства
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WidgetNode {
    #[serde(rename = "type")]
    pub widget_type: String, // Например, "button", "column"
    pub meta: WidgetMeta,

    // Свойства
    #[serde(default)]
    pub properties: WidgetProperties,
}

// -----------------------------------------------------------------------------
//
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CadProject {
    // Глобальный рантайм-счетчик фабрики имен
    pub field_counter: usize,

    // Изолированное хранилище реестра Schema-типов свойств
    // Пример ("padding" => "Padding")
    #[serde(rename = "types_registry")]
    pub property_registry: PropertyRegistry,

    // FIX: Order
    //pub widgets: BTreeMap<String, WidgetNode>,
    pub widgets: indexmap::IndexMap<String, WidgetNode>,

    // FIX: Order добавлено
    // Порядок виджетов
    pub widgets_order: Vec<String>,
}

impl CadProject {
    // Инициализация нового проекта
    pub fn new() -> Self {
        Self {
            field_counter: 0,
            property_registry: PropertyRegistry::new(),
            widgets: IndexMap::new(),
            widgets_order: Vec::new(),
        }
    }

    // Проверка структуры связей
    pub fn check_integrity_errors(&self) -> Vec<String> {
        let mut broken_widgets = Vec::new();

        for (widget_id, node) in &self.widgets {
            // Вызываем наш собственный метод инкапсулированного контейнера параметров!
            let parent_id = node.properties.parent_id();

            if parent_id != "root" && !parent_id.is_empty() && !self.widgets.contains_key(parent_id)
            {
                broken_widgets.push(widget_id.clone());
            }
        }
        broken_widgets
    }

    // Валидация свойств
    pub fn validate_widget_properties(&self, widget_id: &str) -> bool {
        let Some(node) = self.widgets.get(widget_id) else {
            log::error!(
                "validate_widget_properties: Ошибка получения node '{}'.",
                widget_id
            );
            return false;
        };

        let raw_props = match serde_json::to_value(&node.properties) {
            Ok(v) => v,
            Err(_) => return false,
        };

        let Some(prop_obj) = raw_props.as_object() else {
            return false;
        };

        for (prop_name, json_val) in prop_obj {
            if !json_val.is_null() {
                if !self.property_registry.contains(prop_name) {
                    log::error!(
                        "validate_widget_properties: Ошибка проверки наличия свойства '{}'.",
                        prop_name
                    );
                    return false;
                }
                if !self
                    .property_registry
                    .validate_value_type(prop_name, json_val)
                {
                    log::error!(
                        "validate_widget_properties: Ошибка валидации типа свойства '{}' -> {}.",
                        prop_name,
                        json_val
                    );
                    return false;
                }
            }
        }
        true
    }

    // Возвращает Enum-тип для конкретного свойства
    pub fn get_property_type(&self, prop_name: &str) -> PropertyType {
        self.property_registry.get_type(prop_name)
    }

    // -------------------------------------------------------------------------
    // Выгрузка в трейсинг: Формирует детальный текстовый слепок
    // импортированного проекта и отправляет его в отладочный лог-поток уровня TRACE
    // -------------------------------------------------------------------------
    pub fn dump_to_tracing(&self) {
        log::trace!("=================================================================");
        log::trace!(" CAD_PROJECT STATE DUMP (Слепок состояния CadProject перед сбоем)");
        log::trace!("=================================================================");
        log::trace!(
            " Глобальный счетчик фабрики элементов: {}",
            self.field_counter
        );
        log::trace!("-----------------------------------------------------------------");

        // Выгружаем зарегистрированную Schema типов
        log::trace!(
            " РЕЕСТР ТИПИЗИРОВАННЫХ ИМЕН СВОЙСТВ (Загружено: {}):",
            self.property_registry.len()
        );
        // Выгрузим типы через сериализацию в JSON-строку:
        if let Ok(json_registry) = serde_json::to_string_pretty(&self.property_registry) {
            for line in json_registry.lines() {
                log::trace!("    {}", line);
            }
        }
        log::trace!("-----------------------------------------------------------------");

        // Выгружаем дерево виджетов и их свойства
        log::trace!(" ДЕРЕВО ВИДЖЕТOВ (Найдено: {}):", self.widgets.len());
        for (widget_id, node) in &self.widgets {
            log::trace!(
                "      [Виджет ID: '{}'] -> Тип блупринта: '{}'",
                widget_id,
                node.widget_type
            );
            log::trace!("      ├─ Слой (local_index): {}", node.meta.local_index);
            log::trace!("      └─ VTable-свойства:");

            // Переводим свойства конкретного виджета в JSON-текст для лога
            if let Ok(props_json) = serde_json::to_string_pretty(&node.properties) {
                for line in props_json.lines() {
                    // Пропускаем пустые строки и фигурные скобки для компактности вывода
                    if line.trim() != "{" && line.trim() != "}" {
                        log::trace!("          {}", line.trim());
                    }
                }
            }
        }
        log::trace!("=================================================================");
    }

    /// Встроенная рекурсивная функция сортировки
    /// Возвращает плоский список элементов по widgets_order для реализации ПОРЯДКА построения
    /// элементов в коде и группировки элементов у предков
    /// Порядок Листья -> Ветви -> Корень сохраняет очередность элементов в контейнерах согласно widgets_order
    fn sort_recursive(&self, current_parent_id: &str, final_result: &mut Vec<String>) {
        // Выбираем все виджеты, которые являются прямыми детьми для переданного current_parent_id,
        let mut direct_children = Vec::new();

        for id in &self.widgets_order {
            if let Some(node) = self.widgets.get(id) {
                let parent = node.properties.parent_id();

                // Проверяем условия совпадения родителя.
                // Если ищем корень, то родителем может считаться "", "root", "None" или "canvas"
                let is_match = if current_parent_id == "" || current_parent_id == "root" {
                    parent == "root" || parent.is_empty() || parent == "None" || parent == "canvas"
                } else {
                    parent == current_parent_id
                };

                if is_match {
                    direct_children.push(id.clone());
                }
            }
        }

        // Защита от бесконечной рекурсии (если в данных случайно возникнет кольцевая связь)
        // Если у этого узла нет детей, мы просто выходим из этого шага
        if direct_children.is_empty() && (current_parent_id != "" && current_parent_id != "root") {
            return;
        }

        // Запускаем итерацию по полученному массиву (прямых детей) и рекурсивно опрашиваем наличие потомков.
        for child_id in &direct_children {
            // Передаем в рекурсию ID текущего дочернего элемента как нового родителя.
            self.sort_recursive(child_id, final_result);
        }

        // По окончании этой итерации добавляем групповые узлы (прямых детей)
        // во второй (общий) массив и возвращаем результат (через mut-ссылку)
        for child_id in direct_children {
            // Проверяем, чтобы элемент не добавился дважды (в случае сложных пересечений)
            if !final_result.contains(&child_id) {
                final_result.push(child_id);
            }
        }
    }

    /// Публичная обёртка для запуска топологического пост-ордер обхода.
    /// На вход принимает "" или "root", возвращает плоский отсортированный массив ID виджетов.
    pub fn get_post_order_layout(&self, root_id: &str) -> Vec<String> {
        let mut final_order = Vec::new();
        self.sort_recursive(root_id, &mut final_order);
        final_order
    }
}

// -----------------------------------------------------------------------------
// Структура данных конфигурации виджетов
// -----------------------------------------------------------------------------
#[derive(Debug, Deserialize, Clone, Default)]
pub struct WidgetMappingConfig {
    /// Раздел [aliases] из файла конфигурации
    pub aliases: HashMap<String, String>,
    /// Раздел [templates] из файла конфигурации
    pub templates: HashMap<String, String>,
}

impl WidgetMappingConfig {
    /// Метод автоматической безопасной загрузки файла конфигурации с диска
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Self {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(config) = toml::from_str(&content) {
                log::info!(
                    "WidgetMappingConfig: Внешний файл синонимов и шаблонов успешно загружен."
                );
                return config;
            }
        }
        log::warn!(
            "WidgetMappingConfig: Файл конфигурации не найден или поврежден. Использую встроенную схему."
        );
        Self::default()
    }
}
