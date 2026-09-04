// -----------------------------------------------------------------------------
// Модуль 'utils_bp'
// Содержит реализацию утилит и хелперов для виджетов
// -----------------------------------------------------------------------------
use iced::Font;
use iced::font::{Family, Stretch, Style, Weight};

//use iced::Shadow;
use iced::Theme;
use iced::widget::{container};
use iced::{Alignment, Border, Color, Element, Length};

use crate::core::*;

// -----------------------------------------------------------------------------
// Хелперы для 'blueprint'
// -----------------------------------------------------------------------------
// Создание iced::Font
pub fn create_iced_font(font_family: &str, is_bold: bool, is_italic: bool) -> Font {
    // Сопоставляем с встроенными шрифтами Iced (без аллокаций)
    let family = match font_family.trim() {
        "Monospace" | "monospace" => Family::Monospace,
        "SansSerif" | "sans-serif" => Family::SansSerif,
        "Serif" | "serif" => Family::Serif,
        "Cursive" | "cursive" => Family::Cursive,
        "Fantasy" | "fantasy" => Family::Fantasy,

        // Если имя кастомное (например, "Arial"), превращаем его в вечную строку 'static
        custom_name if !custom_name.is_empty() => {
            Family::Name(Box::leak(custom_name.to_string().into_boxed_str()))
        }

        // Защитный дефолт, если строка пришла пустой
        _ => Family::SansSerif,
    };

    // Конструируем и возвращаем готовый монолитный Font
    Font {
        family,
        weight: if is_bold   { Weight::Bold }  else { Weight::Normal },
        style:  if is_italic { Style::Italic } else { Style::Normal },
        stretch: Stretch::Normal, // Обязательный дефолт для структуры Font
    }
}

// Рамка подсветки виджета на холсте в режиме редактирования
pub fn apply_design_overlay<'a>(
    element:        Element<'a, Message, Theme>,
    is_design_mode: bool,
    selected_id:    Option<&str>,
    widget_id:      &str,
) -> Element<'a, Message, Theme> {

    // Если не режим редактирования, возвращаем элемент без изменений
    if !is_design_mode {
        return element;
    }

    // Оборачиваем элемент в прокси/виджет с подсветкой бордюра
    use crate::core::design_proxy;
    design_proxy::design_proxy(
        element, 
        selected_id == Some(widget_id)
    ).into()
}

/*
pub fn apply_design_overlay<'a>(
    element:     Element<'a, Message, Theme>,
    factory: &'a Factory,
    selected_id: Option<&str>,
    widget_id:   &str,
    width:       Length,
    height:      Length,
    // Подстраиваем поведение контейнера под элементы с ограничением max_width/max_height
    // Для других элементов данные параметры должны быть равны 0.0
    max_width:   f32,
    max_height:  f32,

    radius:      Radius,
) -> Element<'a, Message, Theme> {

    // Если не режим редактирования, возвращаем элемент без изменений
    if !factory.is_design_mode() {
        return element;
    }

    // Проверка, что переданный элемент выбран
    let is_selected = selected_id == Some(widget_id);

    let mut w_cont = container(element)
        .width(width)       // Задаем ширину внешнему каркасу подсветки
        .height(height)     // Задаем высоту внешнему каркасу подсветки
        .padding(2)         // Небольшой зазор для рамки выделения
        .style(move |_theme| container::Style {

            // Fix!
            //background: Some(iced::Background::Color(Color::TRANSPARENT)), //None,
            background: Some(iced::Background::Color(
                if is_selected {Color::from_rgba(0.0, 0.5, 1.0, 0.08) } else { Color::from_rgba(0.0, 0.5, 1.0, 0.02) })
            ),

            border: Border {
                color: if is_selected { Color::from_rgb(1.0, 0.0, 0.0) } else { Color::TRANSPARENT },
                width: if is_selected { 2.0 } else { 0.0 },
                radius:  Radius { //radius + Radius::new(2.0)
                    top_left:     radius.top_left + 2.0,
                    top_right:    radius.top_right + 2.0,
                    bottom_right: radius.bottom_right + 2.0,
                    bottom_left:  radius.bottom_left + 2.0,
                },
            },

            // Fix! Наследуем цвет текста без изменений
            text_color: None,
            shadow: Shadow::default(),
            snap: true,

            // Fix! Отключаем default
            // ..Default::default()
        });

    // Подстраиваем поведение контейнера под элементы с ограничением max_width/max_height
    // Для других элементов данные параметры должны быть равны 0.0
    if max_width > 0.0 {
        w_cont = w_cont.max_width(max_width);
    }
    if max_height > 0.0 {
        w_cont = w_cont.max_height(max_height);
    }

    w_cont.into()
}
*/

// Рамка подсветки виджета на холсте в режиме редактирования
/*
pub fn apply_design_overlay<'a>(
    element: Element<'a, Message, Theme>,
    factory: &'a Factory,
    selected_id: Option<&str>,
    widget_id: &str,
    width: Length,
    height: Length,
    radius: Radius,
    // Всегда добавлять для некоторых элементов. В обычном состоянии 'false'
    _always_rend: bool,
) -> Element<'a, Message, Theme> {
    let is_design = factory.is_design_mode();

    // Если не режим редактирования и не указано всегда рендерить, возвращаем элемент без изменений
    //if !factory.is_design_mode() && !always_rend{
    //    return element;
    //}

    // Элемент выбран
    let is_selected = selected_id == Some(widget_id);

    // Вычисляем параметры динамически, но контейнер оставляем ВСЕГДА
    let padding_value = if is_design { 2 } else { 0 };

    let background_color = if is_design {
        if is_selected {
            Color::from_rgba(0.0, 0.5, 1.0, 0.08)
        } else {
            Color::from_rgba(0.0, 0.5, 1.0, 0.02)
        }
    } else {
        Color::TRANSPARENT // Невидим в обычном режиме
    };

    let border_color = if is_design && is_selected {
        Color::from_rgb(1.0, 0.0, 0.0) // Красная рамка для выделенного элемента
    } else {
        Color::TRANSPARENT // Невидим в остальных случаях
    };

    let border_width = if is_design && is_selected { 2.0 } else { 0.0 };

    let final_radius = if is_design {
        Radius {
            top_left: radius.top_left + 2.0,
            top_right: radius.top_right + 2.0,
            bottom_right: radius.bottom_right + 2.0,
            bottom_left: radius.bottom_left + 2.0,
        }
    } else {
        radius // В рантайме сохраняем оригинальное скругление
    };

    // Исправление паники: Контейнер-обертка рендерится всегда.
    // Тип возвращаемого элемента для KeyedColumn стабилен на 100%.
    container(element)
        .width(width)
        .height(height)
        .padding(padding_value)
        .style(move |_theme| container::Style {
            background: Some(iced::Background::Color(background_color)),
            border: Border {
                color: border_color,
                width: border_width,
                radius: final_radius,
            },
            text_color: None,
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}
*/

// УНИВЕРСАЛЬНЫЙ ХЕЛПЕР для подстановки в пустые компоненты
pub fn create_empty_placeholder<'a>(
    widget_name: &str, // Имя для отображения (например, "Windget_1")
    widget_type: &str, // Тип для отображения (например, "MouseArea" или "Container")
    width: Length,     // Передаем настроенную ширину
    height: Length,    // Передаем настроенную высоту
) -> Element<'a, Message, Theme> {
    container(
        iced::widget::text(format!("📦 {} [{}] (Пусто)", widget_name, widget_type))
            .size(9)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .width(width)
    .height(height)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    // Стиль плашки редактора (легкий полупрозрачный фон с тонкой рамкой-пунктиром)
    .style(|_theme| container::Style {
        background: Some(Color::from_rgba(0.0, 0.5, 1.0, 0.05).into()), // Чуть синеватый фон
        border: Border {
            color: Color::from_rgba(0.0, 0.5, 1.0, 0.3), // Полупрозрачная граница
            width: 1.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    })
    .into()
}

/// Разбирает строку с разделителем-запятой в очищенный вектор строк
pub fn parse_comma_separated(s: &str) -> Vec<String> {
    s.split(',')
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty()) // Опционально: убирает пустые элементы, если пользователь случайно поставил лишнюю запятую
        .collect()
}
