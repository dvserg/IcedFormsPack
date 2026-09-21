//! # Ультракомпактный счетчик (CompactCounter) для Iced 0.14
//!
//! Этот модуль предоставляет универсальный, плотный и гибко настраиваемый виджет счетчика,
//! поддерживающий дробные значения `f32`, встроенные темы оформления и циклическую прокрутку.
//!
//! ## Основные возможности:
//! - **Ультракомпактность:** Минимальные внутренние отступы и оптимизированный размер шрифта.
//! - **Оси (Axis):** Поддержка как горизонтального (`Horizontal`), так и вертикального (`Vertical`) расположения.
//! - **Стили стрелок:** Переключение между символами `+ / -`, `◄ / ►` и `▼ / ▲`.
//! - **Встроенные темы:** Готовые стили `Neon` (киберпанк), `Dark` (графит), `Flat` (без рамок) и `Default`.
//! - **Управление границами:** Валидация лимитов `min`, `max` и настройка шага `step` одной строкой через `.range()`.
//! - **Цикличность (Wrap):** Опция автоматического сброса значения по кругу при достижении границ.
//! - **Инкапсуляция:** Самостоятельно вычисляет новые значения и возвращает готовый результат в родительский `update`.
//!
//! ## Пример базового использования:
//! ```rust
//! CompactCounter::new(self.value)
//!     .theme(CounterTheme::Dark)
//!     .range(0.0, 100.0, 1.0) // min, max, step
//!     .on_change(Message::ValueChanged)
//! ```

use iced::widget::button::Status;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Font, Length, Theme};

/// Направление расположения элементов счетчика
///
/// - `Horizontal`: Кнопка минус, значение, кнопка плюс расположены в линию.
/// - `Vertical`: Кнопка плюс (или вверх) сверху, значение по центру, кнопка минус снизу.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

/// Стили символов для кнопок изменения значения.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterStyle {
    PlusMinus, // Отображает "-" и "+"
    LeftRight, // Отображает "◄" и "►"
    UpDown,    // Отображает "▼" и "▲"
}

/// Готовые встроенные темы для быстрой расцветки счетчика
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterTheme {
    Default, // Стандартный стиль под общую тему приложения Iced
    Dark,    // Стильный темный блок (Slate/Dark)
    Neon,    // Киберпанк: черный фон, ярко-зеленый текст и стрелки
    Flat,    // Абсолютно плоский, без рамок и фона, только контрастный текст
}

/// Главная структура кастомного ультракомпактного виджета
pub struct CompactCounter<'a, Message> {
    pub value: f32,
    pub axis: Axis,
    pub style: CounterStyle,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub step: f32,
    pub wrap: bool,
    pub decimals: usize,
    pub on_change: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    pub width: Length,
    pub height: Length,
    pub button_color: Option<Color>,
    pub button_hover_color: Option<Color>,
    pub text_color: Option<Color>,
    pub background: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: f32,
    pub border_radius: f32,
    pub font: Font,
    pub button_background: Option<Color>,
}

impl<'a, Message> CompactCounter<'a, Message> {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            axis: Axis::Horizontal,
            style: CounterStyle::PlusMinus,
            min_value: None,
            max_value: None,
            step: 1.0,
            wrap: false,
            decimals: 1,
            on_change: None,
            width: Length::Shrink,
            height: Length::Shrink,
            button_color: None,
            button_hover_color: None,
            text_color: None,
            background: None,
            border_color: None,
            border_width: 0.0,
            border_radius: 4.0,
            font: Font::DEFAULT,
            button_background: None,
        }
    }

    pub fn theme(mut self, theme: CounterTheme) -> Self {
        match theme {
            CounterTheme::Default => {
                self.button_color = None;
                self.button_hover_color = None;
                self.text_color = None;
                self.background = None;
                self.border_color = None;
                self.border_width = 0.0;
            }
            CounterTheme::Dark => {
                self.background = Some(Color::from_rgb8(30, 34, 42));
                self.border_color = Some(Color::from_rgb8(70, 78, 90));
                self.border_width = 1.0;
                self.border_radius = 6.0;
                self.text_color = Some(Color::WHITE);
                self.button_color = Some(Color::from_rgb8(150, 160, 175));
                self.button_hover_color = Some(Color::WHITE);
                self.font = Font::MONOSPACE;
            }
            CounterTheme::Neon => {
                self.background = Some(Color::from_rgb8(10, 10, 12));
                self.border_color = Some(Color::from_rgb8(0, 255, 128));
                self.border_width = 1.5;
                self.border_radius = 4.0;
                self.text_color = Some(Color::from_rgb8(0, 255, 128));
                self.button_color = Some(Color::from_rgb8(0, 200, 100));
                self.button_hover_color = Some(Color::WHITE);
                self.font = Font::MONOSPACE;
            }
            CounterTheme::Flat => {
                self.background = None;
                self.border_color = None;
                self.border_width = 0.0;
                self.text_color = Some(Color::from_rgb8(255, 69, 0));
                self.button_color = Some(Color::from_rgb8(180, 180, 180));
                self.button_hover_color = Some(Color::from_rgb8(255, 69, 0));
            }
        }
        self
    }

    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    pub fn style(mut self, style: CounterStyle) -> Self {
        self.style = style;
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min_value = Some(min);
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max_value = Some(max);
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn decimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals;
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn on_change<F>(mut self, f: F) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn button_color(mut self, color: Color) -> Self {
        self.button_color = Some(color);
        self
    }

    pub fn button_hover_color(mut self, color: Color) -> Self {
        self.button_hover_color = Some(color);
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border(mut self, color: Color, width: f32, radius: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self.border_radius = radius;
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Задает одновременно минимальное, максимальное значение и шаг изменения
    pub fn range(mut self, min: f32, max: f32, step: f32) -> Self {
        self.min_value = Some(min);
        self.max_value = Some(max);
        self.step = step;
        self
    }

    pub fn button_background(mut self, color: Color) -> Self {
        self.button_background = Some(color);
        self
    }
}

/*
impl<'a, Message> From<CompactCounter<'a, Message>> for Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'static,
{
    fn from(counter: CompactCounter<'a, Message>) -> Self {
        let (dec_char, inc_char) = match counter.style {
            CounterStyle::PlusMinus => ("-", "+"),
            CounterStyle::LeftRight => ("◄", "►"),
            CounterStyle::UpDown => ("▼", "▲"),
        };
        let can_decrease = counter.wrap
            || counter
                .min_value
                .map_or(true, |min| counter.value > min + 0.00001);
        let can_increase = counter.wrap
            || counter
                .max_value
                .map_or(true, |max| counter.value < max - 0.00001);
        /*
        let button_styler = move |theme: &Theme, status: Status| {
            let mut style = button::subtle(theme, status);
            match status {
                Status::Hovered | Status::Pressed => {
                    if let Some(hc) = counter.button_hover_color {
                        style.text_color = hc;
                    }
                }
                Status::Active => {
                    if let Some(c) = counter.button_color {
                        style.text_color = c;
                    }
                }
                Status::Disabled => {
                    style.text_color = style.text_color.scale_alpha(0.25);
                }
            }
            style
        };
        */
        let button_styler = move |theme: &Theme, status: Status| {
            let mut style = button::subtle(theme, status);

            // Задаем цвет фона кнопок из нашей темы
            if let Some(bg) = counter.button_background {
                style.background = Some(iced::Background::Color(bg));
            }

            match status {
                Status::Hovered | Status::Pressed => {
                    if let Some(hc) = counter.button_hover_color {
                        style.text_color = hc;
                    } else if let Some(c) = counter.button_color {
                        // Если hover-цвет не задан, делаем обычный цвет чуть ярче при наведении
                        style.text_color = c;
                    }
                }
                Status::Active => {
                    if let Some(c) = counter.button_color {
                        style.text_color = c;
                    }
                }
                Status::Disabled => {
                    style.text_color = style.text_color.scale_alpha(0.25);
                    // Делаем фон заблокированной кнопки чуть блеклым
                    if let Some(bg) = counter.button_background {
                        style.background = Some(iced::Background::Color(bg.scale_alpha(0.5)));
                    }
                }
            }
            style
        };
        let mut dec_btn = button(text(dec_char).size(13).font(counter.font))
            .padding(2)
            .style(button_styler);
        if can_decrease {
            if let Some(on_change) = &counter.on_change {
                let mut target_value = counter.value - counter.step;
                if let (Some(min), Some(max)) = (counter.min_value, counter.max_value) {
                    if target_value < min - 0.00001 {
                        target_value = if counter.wrap { max } else { min };
                    }
                } else if let Some(min) = counter.min_value {
                    target_value = target_value.max(min);
                }
                dec_btn = dec_btn.on_press(on_change(target_value));
            }
        }
        let mut inc_btn = button(text(inc_char).size(13).font(counter.font))
            .padding(2)
            .style(button_styler);
        if can_increase {
            if let Some(on_change) = &counter.on_change {
                let mut target_value = counter.value + counter.step;
                if let (Some(min), Some(max)) = (counter.min_value, counter.max_value) {
                    if target_value > max + 0.00001 {
                        target_value = if counter.wrap { min } else { max };
                    }
                } else if let Some(max) = counter.max_value {
                    target_value = target_value.min(max);
                }
                inc_btn = inc_btn.on_press(on_change(target_value));
            }
        }
        let formatted_value = format!("{:.1$}", counter.value, counter.decimals);
        let mut val_text = text(formatted_value).size(13).font(counter.font);
        if let Some(tc) = counter.text_color {
            val_text = val_text.color(tc);
        }
        let inner_layout: Element<'a, Message, Theme, iced::Renderer> = match counter.axis {
            Axis::Horizontal => row![dec_btn, val_text, inc_btn]
                .spacing(4)
                .align_y(Alignment::Center)
                .into(),
            Axis::Vertical => column![inc_btn, val_text, dec_btn]
                .spacing(2)
                .align_x(Alignment::Center)
                .into(),
        };
        container(inner_layout)
            .padding(2)
            .width(counter.width)
            .height(counter.height)
            .style(move |_theme: &Theme| container::Style {
                background: counter.background.map(Background::Color),
                border: Border {
                    color: counter.border_color.unwrap_or(Color::TRANSPARENT),
                    width: counter.border_width,
                    radius: counter.border_radius.into(),
                },
                ..container::Style::default()
            })
            .into()
    }
}
*/

impl<'a, Message> From<CompactCounter<'a, Message>> for Element<'a, Message, Theme, iced::Renderer>
where
    Message: Clone + 'static,
{
    fn from(counter: CompactCounter<'a, Message>) -> Self {
        /*
        let make_counter_text = |string_content: String, font_size: f32, text_font: iced::Font| {
            text(string_content)
                .size(font_size)                    // Размер f32, который идеально примет Iced 0.14
                .font(text_font)                    // Шрифт (например, counter.font или FONT_MATERIAL)
                .width(Length::Fill)                // Растягиваем текстовый блок на всю ширину кнопки/контейнера
                .height(Length::Fill)               // Растягиваем на всю доступную высоту
                .align_x(iced::alignment::Horizontal::Center) // Выравнивание строго по центру горизонтали [🌐]
                .align_y(iced::alignment::Vertical::Center)   // Выравнивание строго по центру вертикали [🌐]
        };
        */

        // Определение символов стрелочек на основе стиля
        let (dec_char, inc_char) = match counter.style {
            //CounterStyle::PlusMinus => ("-", "+"),
            CounterStyle::PlusMinus => ("–", "+"),
            CounterStyle::LeftRight => ("◄", "►"),
            CounterStyle::UpDown => ("▼", "▲"),
        };

        // Логика проверки границ для блокировки кнопок
        let can_decrease = counter.wrap
            || counter
                .min_value
                .map_or(true, |min| counter.value > min + 0.00001);

        let can_increase = counter.wrap
            || counter
                .max_value
                .map_or(true, |max| counter.value < max - 0.00001);

        // ---------------------------------------------------------------------
        // АДАПТИВНАЯ МАТЕМАТИКА ГЕОМЕТРИИ (СТРОГО В f32 ДЛЯ ICED 0.14)
        // ---------------------------------------------------------------------
        // Считываем физическую высоту. Если Fixed(px) — берем её, если Shrink/Fill — дефолтные 24.0 пикселя
        let target_height_px = match counter.height {
            iced::Length::Fixed(px) => px,
            _ => 24.0,
        };

        // Пропорционально вычисляем размеры: шрифт 55%, отступы по 8% от общей высоты
        let dynamic_font_size: f32 = (target_height_px * 0.55).round();
        let dynamic_btn_padding: f32 = (target_height_px * 0.08).round().max(1.0);
        let dynamic_container_padding: f32 = (target_height_px * 0.08).round().max(1.0);
        // ---------------------------------------------------------------------

        // Стилизатор кнопок (Hover / Active / Disabled)
        let button_styler = move |theme: &Theme, status: Status| {
            let mut style = button::subtle(theme, status);

            // Задаем цвет фона кнопок из нашей темы
            if let Some(bg) = counter.button_background {
                style.background = Some(iced::Background::Color(bg));
            }

            match status {
                Status::Hovered | Status::Pressed => {
                    if let Some(hc) = counter.button_hover_color {
                        style.text_color = hc;
                    } else if let Some(c) = counter.button_color {
                        // Если hover-цвет не задан, делаем обычный цвет чуть ярче при наведении
                        style.text_color = c;
                    }
                }
                Status::Active => {
                    if let Some(c) = counter.button_color {
                        style.text_color = c;
                    }
                }
                Status::Disabled => {
                    style.text_color = style.text_color.scale_alpha(0.25);
                    // Делаем фон заблокированной кнопки чуть блеклым
                    if let Some(bg) = counter.button_background {
                        style.background = Some(iced::Background::Color(bg.scale_alpha(0.5)));
                    }
                }
            }
            style
        };

        // Сборка кнопки декремента (Минус / Назад)
        let mut dec_btn = button(
            text(dec_char)
                .size(dynamic_font_size)
                .font(counter.font)
                .line_height(text::LineHeight::Relative(1.2)),
        )
        .padding(dynamic_btn_padding)
        .style(button_styler);

        if can_decrease {
            if let Some(on_change) = &counter.on_change {
                let mut target_value = counter.value - counter.step;
                if let (Some(min), Some(max)) = (counter.min_value, counter.max_value) {
                    if target_value < min - 0.00001 {
                        target_value = if counter.wrap { max } else { min };
                    }
                } else if let Some(min) = counter.min_value {
                    target_value = target_value.max(min);
                }
                dec_btn = dec_btn.on_press(on_change(target_value));
            }
        }

        // Сборка кнопки инкремента (Плюс / Вперед)
        let mut inc_btn = button(
            text(inc_char)
                .size(dynamic_font_size)
                .font(counter.font)
                .line_height(text::LineHeight::Relative(1.2)),
        )
        .padding(dynamic_btn_padding)
        .style(button_styler);

        if can_increase {
            if let Some(on_change) = &counter.on_change {
                let mut target_value = counter.value + counter.step;
                if let (Some(min), Some(max)) = (counter.min_value, counter.max_value) {
                    if target_value > max + 0.00001 {
                        target_value = if counter.wrap { min } else { max };
                    }
                } else if let Some(max) = counter.max_value {
                    target_value = target_value.min(max);
                }
                inc_btn = inc_btn.on_press(on_change(target_value));
            }
        }

        // Форматирование числового значения на основе decimals
        let formatted_value = format!("{:.1$}", counter.value, counter.decimals);
        let mut val_text = text(formatted_value)
            .size(dynamic_font_size)
            .font(counter.font);
        if let Some(tc) = counter.text_color {
            val_text = val_text.color(tc);
        }

        // Компоновка осей (Оси Axis)
        let inner_layout: Element<'a, Message, Theme, iced::Renderer> = match counter.axis {
            Axis::Horizontal => row![dec_btn, val_text, inc_btn]
                .spacing(4)
                .align_y(Alignment::Center)
                .into(),
            Axis::Vertical => column![inc_btn, val_text, dec_btn]
                .spacing(2)
                .align_x(Alignment::Center)
                .into(),
        };

        // Финальная обертка в контейнер с применением внешней темы
        container(inner_layout)
            .padding(dynamic_container_padding)
            .width(counter.width)
            .height(counter.height)
            .style(move |_theme: &Theme| container::Style {
                background: counter.background.map(Background::Color),
                border: Border {
                    color: counter.border_color.unwrap_or(Color::TRANSPARENT),
                    width: counter.border_width,
                    radius: counter.border_radius.into(),
                },
                ..container::Style::default()
            })
            .into()
    }
}

// -----------------------------------------------------------------------------
// Helper
// -----------------------------------------------------------------------------
pub fn compact_counter<'a, Message>(value: f32) -> CompactCounter<'a, Message> {
    CompactCounter::new(value)
}
