// -----------------------------------------------------------------------------
// Модуль toolbar
// Содержит реализацию функций относящися к тулбару
// -----------------------------------------------------------------------------
use iced::widget::{Button, button, column, container, row, space, text, tooltip};
use iced::{Alignment, Border, Color, Element, Length, Pixels, Padding, Renderer, Theme, border::Radius};
use iced_aw::{Menu, MenuBar, menu, menu_items};

use crate::app::App;
use crate::core::*;
use crate::core::{DialogType, MenuAction, Message};
use crate::ui::*;


/// Универсальная вспомогательная функция для создания плоских кнопок тулбара меню
pub fn toolbar_double_deck_button<'a>(
    icon_char: &'a str, // Текстовая UTF-8 иконка
    label: &'a str,     // Текстовое название кнопки
    hint: &'a str,      // Всплывающая подсказка
    message: Message,
    ui_style: UIStyle,    
) -> Element<'a, Message, Theme> {
    // Загружаем текущую палитру приложения
    let palette = UiPalette::get_style_palette(ui_style);

    // Собираем кнопку меню
    let raw_button = button(
        column![
            // Верхний этаж: Крупная, выразительная UTF-8 иконка (20px), центрованная по горизонтали
            text(icon_char)
                .width(Length::Fill)
                .font(FONT_MATERIAL)
                .size(20)
                .color(palette.text_main)
                .shaping(iced::widget::text::Shaping::Advanced)
                .align_x(Alignment::Center),
            // Вертикальный зазор между этажами
            space::vertical().height(Length::Fixed(2.0)),
            // Нижний этаж: Отлично читаемый текст подписи (12px), центрованная по горизонтали
            text(label)
                .width(Length::Fill)
                .size(11)
                .color(palette.text_main)
                .align_x(Alignment::Center),
        ]
        // Растягиваем колонку до размеров кнопки
        .width(Length::Fill)
        .spacing(0)
        .align_x(Alignment::Center),
    )
    // Задаем фиксированные размеры и атрибуты кнопки тулбара
    .width(Length::Fixed(60.0))
    .height(Length::Fixed(52.0))
    .padding(padding_from(4.0, 0.0, 4.0, 0.0))
    //.style(uitheme::toolbar_button_style(palette))
    .style(uitheme::style_toolbar_button(ui_style)
    //     move |_theme, _status| {
    //     let mut base_style = button::Style::default();
    //     style_toolbar_button(&mut base_style, &_status, ui_style);
    //     base_style
    // }
    )
    .on_press(message);

    // Добавляем тултип для живого hover-эффекта
    tooltip(
        raw_button, // Кнопка меню
        container(
            text(hint) // Текст всплывающей подсказки
                .size(11),
        )
        .padding(6)
        .style(uitheme::tooltip_style(palette)),
        tooltip::Position::Bottom, // Подсказка выплывает строго снизу под кнопкой
    )
    .gap(4.0) // Микро-отступ между краем кнопки и рамкой подсказки
    .into()
}

/// Вспомогательная функция для создания тонкого вертикального разделителя между группами кнопок
pub fn toolbar_separator<'a>(ui_style: UIStyle) -> Element<'a, Message, Theme> {
    let palette = UiPalette::get_style_palette(ui_style);

    container(column![])
        .width(Length::Fixed(1.0)) // Фиксированная толщина в 1 пиксель
        .height(Length::Fixed(32.0)) // Высота разделителя (чуть меньше высоты кнопок для эстетики)
        .style(move |_theme| iced::widget::container::Style {
            // Красим линию в цвет границ из текущей темы оформления
            background: Some(iced::Background::Color(palette.border_element)),
            ..Default::default()
        })
        .into()
}

/// Универсальная вспомогательная функция для создания плоских кнопок тулбара меню
pub fn toolbar_small_button<'a>(
    icon_char: &'a str,     // Текстовая UTF-8 иконка
    _label:    &'a str,     // Текстовое название кнопки
    hint:      &'a str,     // Всплывающая подсказка
    message:  Message,
    ui_style: UIStyle,
) -> Element<'a, Message, Theme> {
    // // Загружаем текущую палитру приложения
    // let palette = UiPalette::get_style_palette(ui_style);

    // // Собираем кнопку меню
    // let raw_button = button(
    //     text(icon_char)
    //         .font(FONT_MATERIAL)
    //         .color(palette.text_main)
    //         .size(22)
    //         .width(Length::Fill)
    //         .shaping(iced::widget::text::Shaping::Advanced)
    //         .align_x(Alignment::Center)
    //         .align_y(Alignment::Center)

    // )
    // // Задаем фиксированные размеры и атрибуты кнопки тулбара
    // .width(Length::Fixed(28.0))
    // .height(Length::Fixed(28.0))
    // .padding(padding_from(4.0, 0.0, 4.0, 0.0))
    // .style(uitheme::style_toolbar_button(ui_style))
    // .on_press(message);

    // // Добавляем тултип для живого hover-эффекта
    // tooltip(
    //     raw_button, // Кнопка меню
    //     container(
    //         text(hint) // Текст всплывающей подсказки
    //             .size(11),
    //     )
    //     .padding(6)
    //     .style(uitheme::tooltip_style(palette)),
    //     tooltip::Position::Bottom, // Подсказка выплывает строго снизу под кнопкой
    // )
    // .gap(4.0) // Микро-отступ между краем кнопки и рамкой подсказки
    // .into()

    toolbar_button(icon_char, _label, hint, message, ui_style, 22.0, 28.0)
}

/// Универсальная вспомогательная функция для создания плоских кнопок тулбара меню
pub fn toolbar_micro_button<'a>(
    icon_char: &'a str,     // Текстовая UTF-8 иконка
    _label:    &'a str,     // Текстовое название кнопки
    hint:      &'a str,     // Всплывающая подсказка
    message:  Message,
    ui_style: UIStyle,
) -> Element<'a, Message, Theme> {
    // // Загружаем текущую палитру приложения
    // let palette = UiPalette::get_style_palette(ui_style);

    // // Собираем кнопку меню
    // let raw_button = button(
    //     text(icon_char)
    //         .font(FONT_MATERIAL)
    //         .color(palette.text_main)
    //         .size(16)
    //         .width(Length::Fill)
    //         .shaping(iced::widget::text::Shaping::Advanced)
    //         .align_x(Alignment::Center)
    //         .align_y(Alignment::Center)
    // )
    // // Задаем фиксированные размеры и атрибуты кнопки тулбара
    // .width(Length::Fixed(24.0))
    // .height(Length::Fixed(24.0))
    // .padding(padding_from(4.0, 0.0, 4.0, 0.0))
    // .style(uitheme::style_toolbar_button(ui_style))
    // .on_press(message);

    // // Добавляем тултип для живого hover-эффекта
    // tooltip(
    //     raw_button, // Кнопка меню
    //     container(
    //         text(hint) // Текст всплывающей подсказки
    //             .size(11),
    //     )
    //     .padding(6)
    //     .style(uitheme::tooltip_style(palette)),
    //     tooltip::Position::Bottom, // Подсказка выплывает строго снизу под кнопкой
    // )
    // .gap(4.0) // Микро-отступ между краем кнопки и рамкой подсказки
    // .into()

    toolbar_button(icon_char, _label, hint, message, ui_style, 16.0, 24.0)
    
}

/// Универсальный шаблон функция для создания плоских кнопок тулбара
pub fn toolbar_button<'a>(
    icon_char: &'a str,     // Текстовая UTF-8 иконка
    _label:    &'a str,     // Текстовое название кнопки
    hint:      &'a str,     // Всплывающая подсказка
    message:   Message,
    ui_style:  UIStyle,
    text_size: f32,
    btn_size:  f32,
) -> Element<'a, Message, Theme> {
    // Загружаем текущую палитру приложения
    let palette = UiPalette::get_style_palette(ui_style);

    let text_size_px = Pixels::from(text_size);
    let btn_size_f32 = btn_size as f32;

    // Собираем кнопку меню
    let raw_button = button(
        text(icon_char)
            .font(FONT_MATERIAL)
            .color(palette.text_main)
            .size(text_size_px)
            .width(Length::Fill)
            .shaping(iced::widget::text::Shaping::Advanced)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
    )
    // Задаем фиксированные размеры и атрибуты кнопки тулбара
    .width(Length::Fixed(btn_size_f32))
    .height(Length::Fixed(btn_size_f32))
    .padding(padding_from(4.0, 0.0, 4.0, 0.0))
    .style(uitheme::style_toolbar_button(ui_style))
    .on_press(message);

    // Добавляем тултип для живого hover-эффекта
    tooltip(
        raw_button, // Кнопка меню
        container(
            text(hint) // Текст всплывающей подсказки
                .size(11),
        )
        .padding(6)
        .style(uitheme::tooltip_style(palette)),
        tooltip::Position::Bottom, // Подсказка выплывает строго снизу под кнопкой
    )
    .gap(4.0) // Микро-отступ между краем кнопки и рамкой подсказки
    .into()
}