// -----------------------------------------------------------------------------
// Модуль utils
// Содержит реализацию утилит для основных модулей (core, ui)
// -----------------------------------------------------------------------------
use iced::alignment::{Horizontal, Vertical};
use iced::{Alignment, Length, Padding, Pixels, border::Radius};
use log::{error};

// -----------------------------------------------------------------------------
// Hash функции, хэлперы
// -----------------------------------------------------------------------------
// Быстрый стабильный хэшер через простую FNV-1a функцию (не зависит от перезапуска программы)
pub const fn fnv1a_hash_64(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut hash = 0xcbf29ce484222325; // Смещение FNV-1a
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3); // Прайм FNV-1a
        i += 1;
    }
    hash
}

/// Быстрый стабильный 64-битный хэш для строк в рантайме
pub fn runtime_hash_64(s: &str) -> u64 {
    // Если строка пустая или состоит только из пробелов
    if s.trim().is_empty() {
        // Вызываем ошибку, которая покажет файл и строку, где была передана пустота!
        error!("Критическая ошибка: вызов хэширования для ПУСТОЙ строки! Сгенерирован хэш 0.");
        return 0; // Возвращаем зарезервированный хэш-маркер ошибки
    }

    let bytes = s.as_bytes();
    let mut hash = 0xcbf29ce484222325; // Смещение FNV-1a (64-бит)
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3); // Прайм FNV-1a (64-бит)
    }
    hash
}

// -----------------------------------------------------------------------------
// Приведение типов (cast)
// Используются для внутренних преобразований
// -----------------------------------------------------------------------------
// Color
// Вспомогательная функция приведения Hex-строк в iced::Color

/// Превращает строковое HEX-представление (включая "transparent") в объект Color
pub fn cast_hex_2_color(hex: &str) -> Option<iced::Color> {
    let clean_hex = hex.trim();

    // ПЕРЕХВАТ ПРОЗРАЧНОСТИ:
    // Если пришла строка "transparent" — мгновенно отдаем чистый прозрачный цвет
    if clean_hex == "transparent" {
        return Some(iced::Color::TRANSPARENT);
    }

    // Проверяем наличие решетки. Если её нет, но длина 6 — разрешаем парсить без неё
    let hex_digits = clean_hex.trim_start_matches('#');

    // Строка должна содержать строго 6 символов (RRGGBB)
    if hex_digits.len() != 6 {
        return None;
    }

    // Безопасно парсим каждый байт из 16-ричной системы в f32 диапазон 0.0..1.0
    let r = u8::from_str_radix(&hex_digits[0..2], 16).ok()? as f32 / 255.0;
    let g = u8::from_str_radix(&hex_digits[2..4], 16).ok()? as f32 / 255.0;
    let b = u8::from_str_radix(&hex_digits[4..6], 16).ok()? as f32 / 255.0;

    // Возвращаем полностью непрозрачный цвет (a: 1.0) для обычных HEX-кодов
    Some(iced::Color { r, g, b, a: 1.0 })
}

/// Переводит объект Color в строковое представление для инспектора параметров
pub fn cast_color_2_hex(color: iced::Color) -> String {
    // ОБРАТНЫЙ ПЕРЕХВАТ ПРОЗРАЧНОСТИ:
    // Если альфа-канал равен нулю, то это прозрачный цвет!
    // Возвращаем маркер "transparent", чтобы текстовое поле не сваливалось в `#000000`
    if color.a == 0.0 {
        return "transparent".to_string();
    }

    // Обычные цвета переводим в стандартный формат #RRGGBB
    let r = (color.r * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = (color.g * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = (color.b * 255.0).round().clamp(0.0, 255.0) as u8;

    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

// Pixels
// Парсинг типа Pixels в значение типа f32
pub fn cast_pixels_2_f32(value: Pixels) -> f32 {
    return value.0;
}
pub fn cast_pixels_2_string(value: Pixels) -> String {
    return value.0.to_string();
}

// Padding
// Парсинг типа Padding в значение типа String "A D C D"
pub fn cast_padding_2_string(value: Padding) -> String {
    let p = value;
    format!("{:.0} {:.0} {:.0} {:.0}", p.top, p.right, p.bottom, p.left)
}
pub fn cast_padding_2_vecf32(value: Padding) -> [f32; 4] {
    [value.top, value.right, value.bottom, value.left]
}

// String
pub fn cast_string_2_padding(s: &str) -> Option<Padding> {
    // 1. Разбиваем строку по пробелам и пытаемся распарсить каждую часть в f32
    let values: Vec<f32> = s
        .split_whitespace()
        .map(|val| val.parse::<f32>())
        .collect::<Result<Vec<f32>, _>>()
        .ok()?; // Если хоть одно слово не число, выходим с None

    // 2. Применяем стандартную CSS-логику в зависимости от количества чисел
    match values.len() {
        // 1 число: одинаково для всех сторон (например, "10")
        1 => Some(Padding {
            top: values[0],
            right: values[0],
            bottom: values[0],
            left: values[0],
        }),
        // 2 числа: [вертикаль, горизонталь] (например, "10 20")
        2 => Some(Padding {
            top: values[0],
            right: values[1],
            bottom: values[0],
            left: values[1],
        }),
        // 4 числа: [top, right, bottom, left] (например, "10 15 20 25")
        4 => Some(Padding {
            top: values[0],
            right: values[1],
            bottom: values[2],
            left: values[3],
        }),
        // Любое другое количество чисел (0, 3 или > 4) считается ошибкой
        _ => None,
    }
}

// f32
pub fn cast_vecf32_2_string(value: [f32; 4]) -> String {
    value.map(|x| x.to_string()).join(", ") // Результат: "10, 15, 10, 15"    
}

/// Конвертирует массив [f32; 4] в структуру Padding для Iced.
/// Порядок элементов соответствует стандарту CSS (по часовой стрелке):
/// values[0] — Top (Верх)
/// values[1] — Right (Право)
/// values[2] — Bottom (Низ)
/// values[3] — Left (Лево)
pub fn cast_vecf32_2_padding(values: [f32; 4]) -> Padding {
    Padding {
        top: values[0],
        right: values[1],
        bottom: values[2],
        left: values[3],
    }
}

// Radius
pub fn cast_radius_2_string(value: iced::border::Radius) -> String {
    return format!("{:.0}", value.top_left);
}

// Length
pub fn cast_length_2_string(value: iced::Length) -> String {
    return match value {
        iced::Length::Fixed(p) => format!("Fixed:{}", p),
        iced::Length::Fill => "Fill".to_string(),
        iced::Length::Shrink | _ => "Shrink".to_string(),
    };
}

// -----------------------------------------------------------------------------
// Align
// -----------------------------------------------------------------------------
// AlignItems
pub fn cast_align_items_2_string(value: Alignment) -> String {
    match value {
        Alignment::Start =>  "Start".to_string(),
        Alignment::Center => "Center".to_string(),
        Alignment::End =>    "End".to_string(),
    }
}
pub fn cast_string_2_align_items(value: &str) -> Option<Alignment> {
    match value.trim().to_lowercase().as_str() {
        "start" => Some(Alignment::Start),
        "center" => Some(Alignment::Center),
        "end" => Some(Alignment::End),
        _ => None,
    }
}

// AlignX
pub fn cast_align_x_2_string(value: Horizontal) -> String {
    match value {
        Horizontal::Left => "Left".to_string(),
        Horizontal::Center => "Center".to_string(),
        Horizontal::Right => "Right".to_string(),
    }
}
pub fn cast_string_2_align_x(value: &str) -> Option<Horizontal> {
    match value.trim().to_lowercase().as_str() {
        "left" => Some(Horizontal::Left),
        "center" => Some(Horizontal::Center),
        "right" => Some(Horizontal::Right),
        _ => None,
    }
}

// Align_Y
pub fn cast_align_y_2_string(value: Vertical) -> String {
    match value {
        Vertical::Top => "Top".to_string(),
        Vertical::Center => "Center".to_string(),
        Vertical::Bottom => "Bottom".to_string(),
    }
}

pub fn cast_string_2_align_y(value: &str) -> Option<Vertical> {
    match value.trim().to_lowercase().as_str() {
        "top" => Some(Vertical::Top),
        "center" => Some(Vertical::Center),
        "bottom" => Some(Vertical::Bottom),
        _ => None,
    }
}

// -----------------------------------------------------------------------------
// Length
// -----------------------------------------------------------------------------
pub fn cast_string_2_length(s: &str) -> Length {
    // Очищаем строку от случайных пробелов и переводим в нижний регистр для отказоустойчивости
    let clean_str = s.trim().to_lowercase();

    // Проверяем текстовые маркеры автоматического растяжения
    if clean_str == "fill" {
        return Length::Fill;
    }
    if clean_str == "shrink" {
        return Length::Shrink;
    }

    // Обрабатываем формат "fillportion:Х" (если используются доли дочерних слоев)
    if clean_str.starts_with("fillportion:") {
        if let Some(num_str) = clean_str.split(':').nth(1) {
            if let Ok(factor) = num_str.trim().parse::<u16>() {
                return Length::FillPortion(factor);
            }
        }
    }

    // Обрабатываем формат "fixed:Х" или сырое число "Х"
    let target_digits = if clean_str.starts_with("fixed:") {
        clean_str.split(':').nth(1).unwrap_or("").trim()
    } else {
        &clean_str
    };

    // Пробуем распарсить оставшиеся цифры в пиксели f32
    if let Ok(pixels) = target_digits.parse::<f32>() {
        Length::Fixed(pixels)
    } else {
        // Аварийный CAD-дефолт: если в JSON прилетел битый мусор,
        // сжимаем виджет до размеров контента, чтобы не ломать отрисовку окна
        log::warn!(
            "cast_string_2_length: Не удалось распарсить строку '{}'. Применен дефолт Shrink.",
            s
        );
        Length::Shrink
    }
}

// Преобразование (String + f32) в Length
pub fn cast_text_f32_2_length(mode: &str, pixels: f32) -> Length {
    match mode.to_lowercase().trim() {
        "fill" => Length::Fill,
        "shrink" => Length::Shrink,
        // Если написано "fixed" или любая другая строка — выдаем фиксированный размер в пикселях
        _ => Length::Fixed(pixels),
    }
}

pub fn cast_f32_2_radius(value: f32) -> Radius {
    Radius::new(iced::Pixels(value))
}

// Radius
// В Radius все углы устанавливаются синхронно в одно значение f32,
// поэтому в обратном преобразовании берем один угол
pub fn cast_radius_2_f32(value: iced::border::Radius) -> f32 {
    value.top_left
}

/// ОБРАТНЫЙ ПАРСИНГ: Превращает текстовое представление размера (например, "16", "16px", "24.5 px")
/// обратно в системный тип пикселей iced::Pixels для VTable-кучи.
pub fn cast_string_2_pixels(s: &str) -> Pixels {
    //  Очищаем строку от пробелов и переводим в нижний регистр для отказоустойчивости
    let clean_str = s.trim().to_lowercase();

    // Отрезаем суффикс "px", если пользователь или экспортер его принудительно добавил
    let digits_part = if clean_str.ends_with("px") {
        clean_str.trim_end_matches("px").trim()
    } else {
        &clean_str
    };

    // Пробуем распарсить чистые цифры в число f32
    if let Ok(value_f32) = digits_part.parse::<f32>() {
        // Оборачиваем f32 в структуру Pixels
        Pixels(value_f32)
    } else {
        // Безопасный CAD-откат: если в JSON прилетел битый текст,
        // сбрасываем значение в 0.0 пикселей, чтобы не вызвать рантайм-панику ОС
        log::warn!(
            "cast_string_2_pixels: Не удалось распарсить строку '{}'. Применен дефолт 0.0px.",
            s
        );
        Pixels(0.0)
    }
}

/// ОБРАТНЫЙ ПАРСИНГ: Превращает текстовое представление скругления из JSON
/// (например, "4", "4.5", "8 8 0 0") обратно в системную структуру iced::border::Radius.
pub fn cast_string_2_radius(s: &str) -> Radius {
    // Очищаем строку от случайных пробелов
    let clean_str = s.trim();

    // Бьем строку по пробелам, чтобы проверить, переданы ли раздельные углы
    let tokens: Vec<&str> = clean_str.split_whitespace().collect();

    match tokens.len() {
        // Если передано ровно 4 числа: [Top-Left, Top-Right, Bottom-Right, Bottom-Left]
        4 => {
            let tl = tokens[0].parse::<f32>().unwrap_or(0.0);
            let tr = tokens[1].parse::<f32>().unwrap_or(0.0);
            let br = tokens[2].parse::<f32>().unwrap_or(0.0);
            let bl = tokens[3].parse::<f32>().unwrap_or(0.0);

            Radius {
                top_left: tl,
                top_right: tr,
                bottom_right: br,
                bottom_left: bl,
            }
        }
        // Во всех остальных случаях (если передано 1 число или строка вида "4px")
        _ => {
            // Отрезаем суффикс "px" для отказоустойчивости ручного ввода
            let digits_part = clean_str.to_lowercase().trim_end_matches("px").to_string();

            if let Ok(uniform_value) = digits_part.trim().parse::<f32>() {
                // Создаем равномерный радиус для всех 4-х углов сразу [🌐]
                Radius::new(uniform_value)
            } else {
                // Безопасный CAD-откат: если на диске мусор — углы будут острыми (0.0) [1.2]
                log::warn!(
                    "cast_string_2_radius: Не удалось распарсить строку '{}'. Применен дефолт 0.0.",
                    s
                );
                Radius::new(0.0)
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Парсинг
// Обрабатываются строки в типизированные значения для внешних импортов/экспортов
// -----------------------------------------------------------------------------
