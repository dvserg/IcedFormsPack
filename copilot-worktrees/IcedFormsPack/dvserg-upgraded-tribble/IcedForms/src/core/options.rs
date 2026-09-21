// -----------------------------------------------------------------------------
// Модуль options
// Содержит реализацию опций для редактора инспектора
// -----------------------------------------------------------------------------
// Содержит опции для inspector

use iced::Padding;


pub fn padding_from(top: f32, right: f32, bottom: f32, left: f32) -> Padding {
    Padding {
        top,
        right,
        bottom,
        left,
    }
}

pub fn text_alignment_options() -> Vec<String> {
    vec![
        "Left".to_string(),
        "Center".to_string(),
        "Right".to_string(),
    ]
}

// "align_y"
pub fn row_align_items_options() -> Vec<String> {
    vec![
        "Top".to_string(),
        "Center".to_string(),
        "Bottom".to_string(),
    ]
}

// "align_items"
pub fn align_items_options() -> Vec<String> {
    vec!["Start".to_string(), "Center".to_string(), "End".to_string()]
}

// "align_x"
pub fn col_align_items_options() -> Vec<String> {
    vec![
        "Left".to_string(),
        "Center".to_string(),
        "Right".to_string(),
    ]
}

pub fn font_family_options() -> Vec<String> {
    vec![
        "System".to_string(),
        "Monospace".to_string(),
        "Serif".to_string(),
    ]
}

pub fn mouse_area_cursor_options() -> Vec<String> {
    vec![
        "None".to_string(),
        "Hidden".to_string(),
        "Idle".to_string(),
        "ContextMenu".to_string(),
        "Help".to_string(),
        "Pointer".to_string(),
        "Progress".to_string(),
        "Wait".to_string(),
        "Cell".to_string(),
        "Crosshair".to_string(),
        "Text".to_string(),
        "Alias".to_string(),
        "Copy".to_string(),
        "Move".to_string(),
        "NoDrop".to_string(),
        "NotAllowed".to_string(),
        "Grab".to_string(),
        "Grabbing".to_string(),
        "ResizingHorizontally".to_string(),
        "ResizingVertically".to_string(),
        "ResizingDiagonallyUp".to_string(),
        "ResizingDiagonallyDown".to_string(),
        "ResizingColumn".to_string(),
        "ResizingRow".to_string(),
        "AllScroll".to_string(),
        "ZoomIn".to_string(),
        "ZoomOut".to_string(),
    ]
}

pub fn scroll_options() -> Vec<String> {
    vec![
        "vertical".to_string(),
        "horizontal".to_string(),
        "both".to_string(),
    ]
}

pub fn font_weight_options() -> Vec<String> {
    vec!["Normal".to_string(), "Bold".to_string()]
}

pub fn font_style_options() -> Vec<String> {
    vec!["Normal".to_string(), "Italic".to_string()]
}

pub fn svg_content_fit_options() -> Vec<String> {
    vec![
        "Contain".to_string(),
        "Cover".to_string(),
        "Fill".to_string(),
        "None".to_string(),
    ]
}

// ----- Опции для Counter -----

// Структура опций для Counter
pub struct OptionsCounter {
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

// Формирует опции Counter
pub fn counter_options(min: f32, max: f32, step: f32) -> OptionsCounter {
    OptionsCounter { min, max, step }
}

// Радиус скругления [0..100] шаг 2
// "radius": ( 0.0, 100.0, 2.0 )
pub fn radius_options() -> OptionsCounter {
    counter_options(0.0, 100.0, 2.0)
}

// Масштаб виджета [0.5 .. 3.0] шаг 0.1
// "scale": ( 0.5, 3.0, 0.1 )
pub fn scale_options() -> OptionsCounter {
    counter_options(-5.0, 5.0, 0.1)
}

// Аспект размера [0.0 .. 10.0] шаг 0.1
pub fn aspect_options() -> OptionsCounter {
    counter_options(0.0, 10.0, 0.1)
}

// Размер max_width и max_height: от 0 до 2000 пикселей
pub fn max_size_options() -> OptionsCounter {
    counter_options(0.0, 2000.0, 4.0)
}

// Текст от 8 до 72 пикселей
// "text_size": ( 8.0, 72.0, 1.0 )
pub fn text_size_options() -> OptionsCounter {
    counter_options(8.0, 72.0, 1.0)
}

// Внутренние отступы [0..100] шаг 2
// "padding": | "spacing" => (0.0, 100.0, 2.0)
pub fn padding_options() -> OptionsCounter {
    counter_options(0.0, 100.0, 2.0)
}

// Внешние отступы [0..100] шаг 2
// "spacing": ( 0.0, 100.0, 2.0 )
pub fn spacing_options() -> OptionsCounter {
    counter_options(0.0, 100.0, 2.0)
}

/// Опции для счетчика размеров виджетов в пикселях (Ширина / Высота)
pub fn size_options() -> OptionsCounter {
    counter_options(1.0, 2000.0, 2.0)
}

// Опции для толщины рамки [0..20] шаг 1
pub fn border_width_options() -> OptionsCounter {
    counter_options(0.0, 20.0, 1.0)
}

pub fn line_height_options() -> OptionsCounter {
    counter_options(1.0, 3.0, 0.1)
}

// Опции для толщины рамки [0..100] шаг 1
pub fn thickness_options() -> OptionsCounter {
    counter_options(1.0, 100.0, 1.0)
}

// Опции для отсутпа вокруг скроллбара рамки [0..16] шаг 1
pub fn scrollbar_margin_options() -> OptionsCounter {
    counter_options(0.0, 16.0, 1.0)
}

// Опции для ширины скроллбара рамки [2..20] шаг 1
pub fn scrollbar_width_options() -> OptionsCounter {
    counter_options(1.0, 100.0, 1.0)
}

// Опции для SVG прозрачность 'opacity' [0.0 .. 1.0] шаг 0.1
pub fn svg_opacity_options() -> OptionsCounter {
    counter_options(0.0, 1.0, 0.1)
}

// Ширина контента (длина строки) TextEditor
pub fn content_width_options() -> OptionsCounter {
    counter_options(0.0, 2000.0, 5.0)
}

// Параметры вращения в гразусах [0..360], шаг 5
pub fn rotation_options() -> OptionsCounter {
    counter_options(0.0, 360.0, 5.0)
}

// Опции для заполнения rule [0..100] шаг 2
pub fn rule_fill_percent_options() -> OptionsCounter {
    counter_options(2.0, 100.0, 2.0)
}

// Опции для заполнения rule [0..100] шаг 2
pub fn columns_options() -> OptionsCounter {
    counter_options(1.0, 100.0, 1.0)
}
