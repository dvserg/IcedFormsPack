// -----------------------------------------------------------------------------
// Модуль inspector_prop_editors
// Содержит реализацию редакторов приложения
// -----------------------------------------------------------------------------
use iced::Theme;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, checkbox, column, pick_list, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};
use std::str::FromStr;

use crate::core::*;
use crate::ui::UiPalette;
use crate::widgets::compact_counter::{Axis, CompactCounter, CounterStyle};

// -----------------------------------------------------------------------------
/// Редактор числовых свойств на базе счетчика (+/-)
// -----------------------------------------------------------------------------
/// Обезличенный базовый счетчик.
/// Принимает текущее число, шаг и ФУНКЦИЮ обратного вызова, которая генерирует нужное Message.

pub fn generic_counter<'a, F>(
    current_value: f32,
    options: &OptionsCounter,
    is_dark: bool,
    on_change: F,
) -> Element<'a, Message, Theme, iced::Renderer>
// Исправлено: 4 generic-аргумента
where
    F: 'static + Clone + Fn(f32) -> Message,
{
    let palette = UiPalette::get_palette(is_dark);

    CompactCounter::new(current_value)
        // Если в OptionsCounter есть эти поля, берем их оттуда.
        // Если нет — ставим дефолтные, как в коде ниже:

        .axis(Axis::Horizontal)
        .style(CounterStyle::PlusMinus)     // например, CounterStyle::PlusMinus
        //.theme(options.theme)             // например, CounterTheme::Dark
        .wrap(false) 
        .decimals(1)                        // количество знаков после запятой
        // Математика границ и шага:
        .range(options.min, options.max, options.step)  // Общий бордюр вокруг всего коунтера
        .border(palette.border_element, 1.0, 4.0)       // Фон самого коунтера (делаем в цвет панели)
        .background(palette.bg_panel)           // Текст центрального числа
        .text_color(palette.text_main)          // Различимые кнопки: задаем им контрастный фон плашек bg_element
        .button_background(palette.bg_element)  // Цвет самих стрелочек внутри кнопок
        .button_color(palette.text_main)
        // Цвет стрелочек при наведении (в светлой теме делаем черным, в темной — чисто белым)
        .button_hover_color(if palette.bg_panel.r > 0.5 {
            Color::BLACK // Для Light темы
        } else {
            Color::WHITE // Для Dark темы
        })
        // Передаем коллбэк:
        .on_change(on_change)
        .into()
}

// -----------------------------------------------------------------------------
/// Типизированные счетчики
// -----------------------------------------------------------------------------
/// Счетчик типа f32
/// Просто вызывает внутри себя универсальный generic_counter.
pub fn counter_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey,
    current_value: f32,
    options: &OptionsCounter,
    is_dark: bool,
) -> Element<'a, Message, Theme> {
    let w_id = widget_id.clone();
    let p_key = prop_key.clone();

    // Перенаправляем вызов в универсальную функцию, жестко зашивая старое сообщение
    generic_counter(current_value, options, is_dark, move |new_val| {
        // Значения сохраняем в текстовом f32
        //let formatted_string = format!("{:.1}", new_val);
        Message::UpdateProperty {
            widget_id: w_id.clone(),
            property_key: p_key.clone(),
            value: PropertyValue::Float(new_val), // formatted_string,
        }
    })
}

/// Счетчик типа usize
/// Просто вызывает внутри себя универсальный generic_counter с приведенем типа.
pub fn usize_counter_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey,
    current_value: usize,
    options: &OptionsCounter,
    is_dark: bool,
) -> Element<'a, Message, Theme> {
    let w_id = widget_id.clone();
    let p_key = prop_key.clone();

    // Конвертируем и считаем в f32
    //let float_val: f32 = utils::cast_pixels_2_f32(current_value);

    // Перенаправляем вызов в универсальную функцию, жестко зашивая сообщение
    generic_counter(current_value as f32, options, is_dark, move |new_val| {
        // Значения сохраняем в iced::Pixels
        //let formatted_string = format!("{:.1}", new_val);
        Message::UpdateProperty {
            widget_id:    w_id.clone(),
            property_key: p_key.clone(),
            // Возвращаем в usize
            value:        PropertyValue::USize(new_val as usize),
        }
    })
}

/// Счетчик типа Pixels
/// Просто вызывает внутри себя универсальный generic_counter с приведенем типа.
pub fn pixel_counter_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey,
    current_value: iced::Pixels,
    options: &OptionsCounter,
    is_dark: bool,
) -> Element<'a, Message, Theme> {
    let w_id = widget_id.clone();
    let p_key = prop_key.clone();

    // Конвертируем и считаем в f32
    let float_val: f32 = utils::cast_pixels_2_f32(current_value);

    // Перенаправляем вызов в универсальную функцию, жестко зашивая сообщение
    generic_counter(float_val, options, is_dark, move |new_val| {
        // Значения сохраняем в iced::Pixels
        //let formatted_string = format!("{:.1}", new_val);
        Message::UpdateProperty {
            widget_id: w_id.clone(),
            property_key: p_key.clone(),
            // Возвращаем в Pixels
            value: PropertyValue::Pixels(iced::Pixels(new_val)),
        }
    })
}

/// Счетчик типа Radius
/// Просто вызывает внутри себя универсальный generic_counter с приведенем типа.
pub fn radius_counter_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey,
    current_value: iced::border::Radius,
    options: &OptionsCounter,
    is_dark: bool,
) -> Element<'a, Message, Theme> {
    let w_id = widget_id.clone();
    let p_key = prop_key.clone();

    // Конвертируем и считаем в f32
    let float_val: f32 = utils::cast_radius_2_f32(current_value);

    // Перенаправляем вызов в универсальную функцию, жестко зашивая сообщение
    generic_counter(float_val, options, is_dark, move |new_val| {
        // Значения сохраняем в iced::Pixels
        //let formatted_string = format!("{:.1}", new_val);
        Message::UpdateProperty {
            widget_id: w_id.clone(),
            property_key: p_key.clone(),
            // Возвращаем в Radius
            value: PropertyValue::Radius(iced::border::Radius::new(iced::Pixels(new_val))),
        }
    })
}

// -----------------------------------------------------------------------------
// Комбинированный редактор Size iced::Length (Fill, Short, Fixed(0.0))
// Слева название, справа выпадающий список и появляющийся на Fixed счетчик f32
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeSpec {
    Fill,
    Shrink,
    Fixed(i32),
}

// Реализация парсинга (вместо from_string)
impl FromStr for SizeSpec {
    type Err = (); // Ошибку не возвращаем, при сбое отдаем безопасный дефолт

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        // 1. Очищаем от пробелов по краям и приводим к нижнему регистру
        let clean = src.trim().to_lowercase();

        // 2. Проверяем текстовые режимы
        if clean == "fill" {
            return Ok(SizeSpec::Fill);
        }
        if clean == "shrink" {
            return Ok(SizeSpec::Shrink);
        }

        // 3. Проверяем составной формат "fixed:150"
        if clean.starts_with("fixed:") {
            if let Some(num_str) = clean.split(':').nth(1) {
                // Пытаемся распарсить как f32 (чтобы "25.0" сработало), а затем округляем до i32
                if let Ok(num_f32) = num_str.trim().parse::<f32>() {
                    return Ok(SizeSpec::Fixed(num_f32 as i32));
                }
            }
        }

        // 4. Проверяем чистое число (например, "25.0" или "120")
        if let Ok(num_f32) = clean.parse::<f32>() {
            return Ok(SizeSpec::Fixed(num_f32 as i32));
        }

        // 5. Безопасный дефолт, если пришла совсем некорректная строка
        Ok(SizeSpec::Fill)
    }
}

// Реализация сериализации (вместо метода to_string)
impl std::fmt::Display for SizeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SizeSpec::Fill => write!(f, "Fill"),
            SizeSpec::Shrink => write!(f, "Shrink"),
            SizeSpec::Fixed(val) => write!(f, "Fixed:{}", val),
        }
    }
}

pub fn size_mode_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey,    // Нативно поддерживает Copy
    current_mode: Length,     // Принимает родной enum Length из Iced
    current_pixels: f32,      // Числовое значение пикселей из базы данных
    options: &OptionsCounter, // Настройки счетчика
    is_bool: bool,
) -> Element<'a, Message> {
    // Напрямую конвертируем родной iced::Length в локальный SizeSpec
    let spec = match current_mode {
        Length::Fill => SizeSpec::Fill,
        Length::Shrink => SizeSpec::Shrink,
        // Если пришел Fixed, используем актуальные пиксели из аргументов функции
        Length::Fixed(_) => SizeSpec::Fixed(current_pixels as i32),
        // Защитный фоллбек для FillPortion и т.д.
        _ => SizeSpec::Shrink,
    };

    // Опции для выпадающего списка pick_list
    let mode_options: Vec<String> = vec![
        "Fill".to_string(),
        "Shrink".to_string(),
        "Fixed".to_string(),
    ];

    // Определяем активную строку для подсветки в выпадающем списке
    let active_mode_str = match spec {
        SizeSpec::Fill => "Fill".to_string(),
        SizeSpec::Shrink => "Shrink".to_string(),
        SizeSpec::Fixed(_) => "Fixed".to_string(),
    };

    let w_id_clone = widget_id.clone();
    let p_key_raw = prop_key;

    // Создаем выпадающий список режимов размера
    let dropdown = pick_list(mode_options, Some(active_mode_str), move |selected| {
        // Формируем строковое значение для сохранения в зависимости от выбора
        let value_to_save = match selected.as_str() {
            "Fixed" => iced::Length::Fixed(current_pixels),
            "Fill" => iced::Length::Fill,
            "Shrink" | _ => iced::Length::Shrink,
        };

        Message::UpdateProperty {
            widget_id: w_id_clone.clone(),
            property_key: p_key_raw,
            // Сохраняем (Length, pixels)
            value: PropertyValue::Length(value_to_save, current_pixels),
        }
    })
    .text_size(13)
    .padding(3)
    .width(Length::Fill);

    let mut control_row = row![dropdown].spacing(4).align_y(Alignment::Center);

    // Если выбран режим Fixed — выводим дополнительный счетчик пикселей
    if let SizeSpec::Fixed(pixels) = spec {
        // Конструируем динамичесий PropertyKey
        let pixels_key_str = format!("{}:pixels", prop_key.name);
        let _pixels_key = PropertyKey::from_dynamic(&pixels_key_str);

        let w_id = widget_id.clone();
        let current_value = pixels as f32;

        let counter = generic_counter(current_value, options, is_bool, move |new_val| {
            Message::UpdateProperty {
                widget_id: w_id.clone(),
                property_key: prop_key, //pixels_key,
                // Сохраняем (Fixed(pixels), pixels)
                value: PropertyValue::Length(iced::Length::Fixed(new_val), new_val),
            }
        });

        control_row = control_row.push(counter);
    }

    control_row.into()
}

// -----------------------------------------------------------------------------

/// Универсальный сеточный редактор из 2 строк (Слева — название, справа — счетчик).
/// Поддерживает CSS-стандарты сокращенной записи чисел: "10.0" или "10.0 20.0".
pub fn duo_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey, // Принимаем плоский PropertyKey (например, PROP_PADDING)
    //current_value_str: String,    // Сырая строка из фабрики (например, "10.0 20.0 10.0 20.0")
    current_value_qf32: [f32; 4], // Не смотря, что это duo editor, принимает quad значения
    labels_csv: String,           // Видимые названия для пользователя через запятую
    options: &OptionsCounter,     // Ограничения слайдера/счетчика
    is_dark: bool,
) -> Element<'a, Message, Theme> {
    // Укажите ваш точный путь к Message
    // Test
    //println!("duo_editor: Входящее значение {} = {:?}", prop_key.name, current_value_qf32);

    let parsed_numbers = current_value_qf32;

    // Гарантированно вытаскиваем 2 значения из строки любой длины (1, 2 или 4)
    let (val_vertical, val_horizontal) = match parsed_numbers.len() {
        4 => (parsed_numbers[0], parsed_numbers[1]), // Из "A B C D" берем первые два
        3 => (parsed_numbers[0], parsed_numbers[1]),
        2 => (parsed_numbers[0], parsed_numbers[1]),
        1 => (parsed_numbers[0], parsed_numbers[0]),
        _ => (0.0, 0.0),
    };

    // Текстовые метки к 'counters'
    let csv_labels: Vec<&str> = labels_csv.split(',').map(|s| s.trim()).collect();
    let lbl_v = csv_labels.get(0).unwrap_or(&"Вертикаль");
    let lbl_h = csv_labels.get(1).unwrap_or(&"Горизонталь");

    let id_1 = widget_id.clone();
    let id_2 = widget_id.clone();

    // Генерируем счетчики с правильным PropertyKey в поле property_key
    let c1 = generic_counter(val_vertical, options, is_dark, move |v| {
        Message::UpdateProperty {
            widget_id: id_1.clone(),
            property_key: prop_key, // Данные сохраняем прямо в текущее значение
            //value:        PropertyValue::Quad(v, 0)
            value: PropertyValue::Padding(cast_vecf32_2_padding([
                v,
                val_horizontal,
                v,
                val_horizontal,
            ])),
        }
    });

    let c2 = generic_counter(val_horizontal, options, is_dark, move |v| {
        Message::UpdateProperty {
            widget_id: id_2.clone(),
            property_key: prop_key, // Данные сохраняем прямо в текущее значение
            //value:        PropertyValue::Quad(v, 1)
            value: PropertyValue::Padding(cast_vecf32_2_padding([
                val_vertical,
                v,
                val_vertical,
                v,
            ])),
        }
    });

    let labels_column = column![
        text(lbl_v.to_string()).size(13).width(Length::Fixed(80.0)),
        text(lbl_h.to_string()).size(13).width(Length::Fixed(80.0)),
    ]
    .spacing(12)
    .align_x(Alignment::Start);

    let inputs_column = column![c1, c2].spacing(6).width(Length::Fill);

    row![labels_column, inputs_column]
        .spacing(10)
        .align_y(Alignment::Center)
        .into()
}

/// Компактный редактор бинарных свойств (Да/Нет) на базе checkbox Iced 0.14
pub fn checkbox_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey, // ИСПРАВЛЕНИЕ: Принимаем плоский PropertyKey вместо сырой String!
    is_checked: bool,
) -> Element<'a, Message> {
    // Укажите ваш точный путь к Message

    let w_id_clone = widget_id.clone();
    // Извлекаем имя свойства строкой из плоского ключа для передачи в оригинальное сообщение
    let _prop_name_str = prop_key.name.to_string();

    checkbox(is_checked)
        // Событие клика вешается через метод .on_toggle в Iced 0.14
        .on_toggle(move |new_state: bool| {
            Message::UpdateProperty {
                widget_id: w_id_clone.clone(),
                property_key: prop_key, // Передаем оригинальное имя свойства строкой
                value: PropertyValue::Boolean(new_state),
            }
        })
        .size(14) // Оставляем ваш плотный компактный размер
        .into()
}

/// Универсальный выпадающий список (Generic PickList) для Инспектора свойств.
/// Работает с любым типом данных `T`, который Iced умеет выводить на экран.

pub fn generic_select_editor<'a, T, F>(
    current_value: T,
    options: Vec<T>,
    is_dark: bool,
    build_message: F, // Замыкание, которое САМО собирает нужный Message из выбранного T
) -> Element<'a, Message, Theme>
where
    T: ToString + PartialEq + Clone + 'static,
    // Теперь F возвращает полноценный Message, удовлетворяя любой вызов!
    F: Fn(T) -> Message + 'static,
{
    let palette = UiPalette::get_palette(is_dark);

    // Оборачиваем коллбэк в Rc для безопасной многократной отправки в графическом цикле
    let message_builder = std::rc::Rc::new(build_message);

    // pick_list просто транслирует выбранное значение T прямо в ваш message_builder
    pick_list(options, Some(current_value), move |selected| {
        message_builder(selected) // Вызываем переданное извне замыкание
    })
    .text_size(13)
    .padding(3)
    .width(iced::Length::Fill)
    .style(move |_theme, status| {
        let mut s = pick_list::Style {
            background: Background::Color(palette.bg_element),
            text_color: palette.text_main,
            placeholder_color: palette.text_muted,
            handle_color: palette.text_main,
            border: Border {
                color: palette.border_element,
                width: 1.0,
                radius: 4.0.into(),
            },
        };

        if status == pick_list::Status::Hovered {
            s.border.color = palette.text_main;
        }

        s
    })
    .into()
}

pub fn align_items_select_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey,
    current_value: Alignment,
    options: Vec<String>,
    is_dark: bool,
) -> Element<'a, Message, Theme> {
    // Получаем палитру приложения
    let _palette = UiPalette::get_palette(is_dark);
    let w_id_clone = widget_id.clone();
    let key_for_closure = prop_key;
    let current_value_s = utils::cast_align_items_2_string(current_value);

    generic_select_editor(current_value_s, options, is_dark, move |selected| {
        let align_enum = utils::cast_string_2_align_items(&selected)
            .unwrap_or(iced::alignment::Alignment::Start);

        Message::UpdateProperty {
            widget_id: w_id_clone.clone(),
            property_key: key_for_closure,
            value: PropertyValue::AlignItems(align_enum),
        }
    })
}

pub fn align_x_select_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey,
    current_value: Horizontal,
    options: Vec<String>,
    is_dark: bool,
) -> Element<'a, Message, Theme> {
    // Получаем палитру приложения
    let _palette = UiPalette::get_palette(is_dark);
    let w_id_clone = widget_id.clone();
    let key_for_closure = prop_key;
    let current_value_s = utils::cast_align_x_2_string(current_value);

    generic_select_editor(current_value_s, options, is_dark, move |selected| {
        let align_enum =
            utils::cast_string_2_align_x(&selected).unwrap_or(iced::alignment::Horizontal::Left);

        Message::UpdateProperty {
            widget_id: w_id_clone.clone(),
            property_key: key_for_closure,
            value: PropertyValue::AlignX(align_enum),
        }
    })
}

pub fn align_y_select_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey,
    current_value: Vertical,
    options: Vec<String>,
    is_dark: bool,
) -> Element<'a, Message, Theme> {
    // Получаем палитру приложения
    let _palette = UiPalette::get_palette(is_dark);
    let w_id_clone = widget_id.clone();
    let key_for_closure = prop_key;
    let current_value_s = utils::cast_align_y_2_string(current_value);

    generic_select_editor(current_value_s, options, is_dark, move |selected| {
        let align_enum =
            utils::cast_string_2_align_y(&selected).unwrap_or(iced::alignment::Vertical::Top);

        Message::UpdateProperty {
            widget_id: w_id_clone.clone(),
            property_key: key_for_closure,
            value: PropertyValue::AlignY(align_enum),
        }
    })
}

/// Редактор фиксированных текстовых опций (Выпадающий список)
pub fn select_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey,
    current_value: String,
    options: Vec<String>,
    is_dark: bool,
) -> Element<'a, Message, Theme> {
    // Получаем палитру приложения
    let palette = UiPalette::get_palette(is_dark);

    let w_id_clone = widget_id.clone();
    let key_for_closure = prop_key;

    pick_list(options, Some(current_value), move |selected| {
        Message::UpdateProperty {
            widget_id: w_id_clone.clone(),
            property_key: key_for_closure, // Передаем готовый плоский ключ в сообщение
            value: PropertyValue::Text(selected),
        }
    })
    .text_size(13)
    .padding(3)
    .width(iced::Length::Fill)
    .style(move |_theme, status| {
        // Обязательное поле shadow для оверлея меню
        let _dropdown_menu_style = iced::widget::overlay::menu::Style {
            background: Background::Color(palette.bg_panel),
            border: Border {
                color: palette.border_element,
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: palette.text_main,
            selected_background: Background::Color(palette.bg_element),
            selected_text_color: Color::WHITE,
            shadow: iced::Shadow::default(),
        };

        //  Сборка стиля плашки БЕЗ поля menu для Iced 0.14
        let mut s = pick_list::Style {
            background: Background::Color(palette.bg_element),
            text_color: palette.text_main,
            placeholder_color: palette.text_muted,
            handle_color: palette.text_main,
            border: Border {
                color: palette.border_element,
                width: 1.0,
                radius: 4.0.into(),
            },
        };

        if status == pick_list::Status::Hovered {
            s.border.color = palette.text_main;
        }

        s
    })
    .into()
}

/// Редактор фиксированных текстовых опций (Выпадающий список)
pub fn parent_select_editor<'a>(
    widget_id: String,
    prop_key: PropertyKey,
    current_value: String,
    options: Vec<String>,
    is_dark: bool,
) -> Element<'a, Message, Theme> {
    // Получаем палитру приложения
    let palette = UiPalette::get_palette(is_dark);

    let w_id_clone = widget_id.clone();
    let key_for_closure = prop_key;

    pick_list(options, Some(current_value), move |selected| {
        Message::UpdateProperty {
            widget_id: w_id_clone.clone(),
            property_key: key_for_closure, // Передаем готовый плоский ключ в сообщение
            value: PropertyValue::Parent(selected),
        }
    })
    .text_size(13)
    .padding(3)
    .width(iced::Length::Fill)
    .style(move |_theme, status| {
        // Обязательное поле shadow для оверлея меню
        let _dropdown_menu_style = iced::widget::overlay::menu::Style {
            background: Background::Color(palette.bg_panel),
            border: Border {
                color: palette.border_element,
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: palette.text_main,
            selected_background: Background::Color(palette.bg_element),
            selected_text_color: Color::WHITE,
            shadow: iced::Shadow::default(),
        };

        //  Сборка стиля плашки БЕЗ поля menu для Iced 0.14
        let mut s = pick_list::Style {
            background: Background::Color(palette.bg_element),
            text_color: palette.text_main,
            placeholder_color: palette.text_muted,
            handle_color: palette.text_main,
            border: Border {
                color: palette.border_element,
                width: 1.0,
                radius: 4.0.into(),
            },
        };

        if status == pick_list::Status::Hovered {
            s.border.color = palette.text_main;
        }

        s
    })
    .into()
}

/// Дефолтный текстовый редактор параметров формы (Обычная строка ввода)
pub fn text_editor(
    widget_id: String,
    prop_key: PropertyKey,
    current_value: String,
) -> Element<'static, Message> {
    let _id = widget_id.clone();
    let _pn = prop_key.name;

    // Создаем независимые копии для перемещения в замыкание
    let id_for_closure = widget_id.clone();
    // Поскольку PropertyKey реализует легкий трейт Copy, мы просто копируем его без аллокаций в куче!
    let key_for_closure = prop_key;

    text_input("Значение...", &current_value)
        // Сохраняем оригинальную структуру сообщения, передавая туда PropertyKey напрямую
        .on_input(move |new_val| Message::UpdateProperty {
            widget_id: id_for_closure.clone(),
            property_key: key_for_closure, // Передаем готовый плоский ключ (например, PROP_TEXT)
            value: PropertyValue::Text(new_val),
        })
        .size(13)
        .padding(3)
        .into()
}

/// Полный двухрядный редактор палитры цветов с прямоугольными кнопками
pub fn color_picker_editor(
    widget_id: String,
    prop_key: PropertyKey,
    value: Color,
) -> Element<'static, Message> {
    // Укажите ваш точный путь к Message

    // Клонируем ID виджета и свойства для обхода borrow
    let widget_id_clone = widget_id.clone();

    // Так как PropertyKey реализует Copy, просто копируем его без .clone()
    let key_for_closure = prop_key;

    // Преобразуем Color в строку для представления в редакторе
    let current_value = utils::cast_color_2_hex(value);

    // Текстовое поле ввода
    let text_field = text_input("Например, #FF0000", &current_value)
        .on_input({
            let w_id = widget_id_clone.clone();
            move |val| Message::UpdateProperty {
                widget_id: w_id.clone(),
                property_key: key_for_closure, // Передаем готовый плоский ключ в сообщение
                value: PropertyValue::Color(
                    utils::cast_hex_2_color(val.as_str()).unwrap_or(iced::Color::TRANSPARENT),
                ),
            }
        })
        .size(13)
        .padding(3)
        .width(Length::Fill);

    // Вспомогательная локальная лямбда-функция для генерации прямоугольных кнопок
    let make_swatch = |hex: &'static str, rgb: Color, w_id: String, k_id: PropertyKey| {
        let content = text("")
            .width(Length::Fixed(14.0))
            .height(Length::Fixed(18.0));

        button(content)
            .padding(0)
            .style(move |_theme, _status| button::Style {
                background: Some(iced::Background::Color(rgb)),
                border: iced::Border {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                    width: 1.0,
                    radius: 1.5.into(),
                },
                ..button::Style::default()
            })
            .on_press(Message::UpdateProperty {
                widget_id: w_id,
                property_key: k_id, // Использован PropertyKey
                value: PropertyValue::Color(
                    utils::cast_hex_2_color(hex).unwrap_or(iced::Color::TRANSPARENT),
                ),
            })
    };

    // Ряд 1: Пастельные и светлые цвета (Нейтральные + Светлые акценты)
    let light_swatches = row![
        make_swatch(
            "#FFFFFF",
            Color::WHITE,
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "#F3F3F3",
            Color::from_rgb(0.95, 0.95, 0.95),
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "#E0E0E0",
            Color::from_rgb(0.88, 0.88, 0.88),
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "#FFCCCC",
            Color::from_rgb(1.0, 0.8, 0.8),
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "#CCE5FF",
            Color::from_rgb(0.8, 0.9, 1.0),
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "#D4EDDA",
            Color::from_rgb(0.83, 0.93, 0.85),
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "#FFF3CD",
            Color::from_rgb(1.0, 0.95, 0.8),
            widget_id_clone.clone(),
            key_for_closure
        ),
    ]
    .spacing(3);

    // Ряд 2: Глубокие и темные цвета (Dark Mode база + Темные акценты + Самый темный + Сброс)
    let dark_swatches = row![
        make_swatch(
            "#212529",
            Color::from_rgb(0.13, 0.15, 0.16),
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "#343A40",
            Color::from_rgb(0.2, 0.23, 0.25),
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "#721C24",
            Color::from_rgb(0.45, 0.11, 0.14),
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "#004085",
            Color::from_rgb(0.0, 0.25, 0.52),
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "#155724",
            Color::from_rgb(0.08, 0.34, 0.14),
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "#000000",
            Color::BLACK,
            widget_id_clone.clone(),
            key_for_closure
        ),
        make_swatch(
            "transparent",
            Color::TRANSPARENT,
            widget_id_clone,
            key_for_closure
        ),
    ]
    .spacing(3);

    // Сборка всего пака в единую вертикальную колонку свойств
    column![text_field, light_swatches, dark_swatches]
        .spacing(4)
        .into()
}

/// Дефолтный текстовый редактор параметров формы (Обычная строка ввода)
pub fn overlay_text_editor(
    widget_id: String,
    _prop_key:  PropertyKey,
    _current_value: String,
) -> Element<'static, Message> {
/*    let _id = widget_id.clone();
    let _pn = prop_key.name;

    // Создаем независимые копии для перемещения в замыкание
    let id_for_closure = widget_id.clone();
    // Поскольку PropertyKey реализует легкий трейт Copy, мы просто копируем его без аллокаций в куче!
    let key_for_closure = prop_key;

    text_input("Значение...", &current_value)
        // Сохраняем оригинальную структуру сообщения, передавая туда PropertyKey напрямую
        .on_input(move |new_val| Message::UpdateProperty {
            widget_id: id_for_closure.clone(),
            property_key: key_for_closure, // Передаем готовый плоский ключ (например, PROP_TEXT)
            value: PropertyValue::Text(new_val),
        })
        .size(13)
        .padding(3)
        .into()
        */

    let w_button = button("Редактор")
        .on_press(Message::OverlayEvent(OverlayAction::OpenWidgetEditor(widget_id)));
        
    w_button.into()
}