// -----------------------------------------------------------------------------
// Модуль inspector
// Содержит реализацию инспектора компонетов и свойств приложения
// -----------------------------------------------------------------------------
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length, Theme};

pub use crate::app::App;
pub use crate::core::*;
pub use crate::ui::inspector_panel_tree::render_layers_tree;
pub use crate::ui::{UiPalette, render_style};
pub use crate::ui::{inspector_panel_edit, inspector_panel_prop, inspector_prop_editors};

// Рендеринг инспектора
pub fn render_inspector<'a>(app: &'a App) -> Element<'a, Message, Theme> {
    // Текущая тема приложения
    let is_dark = app.is_dark_theme();

    let inspector = container( render_inspector_panel(&app, is_dark) )
        //.height(Length::FillPortion(2)) // Занимает 2/3 высоты панели
        .padding(0);

    inspector.into()
}

/// Рендеринг верхней панели инспектора
pub fn render_inspector_panel<'a>(app: &'a App, is_dark: bool) -> Element<'a, Message, Theme> {

    // Получаем текущую палитру приложения
    let ui_style = app.get_ui_style();
    let palette  = UiPalette::get_style_palette(ui_style);

    let factory = app.get_factory();
    let app_state = app.get_state();

    // Главная вертикальная колонка инспектора
    let mut inspector_column = column![].spacing(0).padding(0);

    // Создаем кнопку переключения режимов конструктора
    // inspector_column = inspector_column.push(render_style::render_mode_toggle(
    //     factory.is_design_mode(),
    //     Message::MenuEvent(MenuAction::ToggleDesignMode),
    // ));

    // Добавляем базовый заголовок
    //let header_content = render_style::render_header("Свойства элемента:", app.get_ui_style(), is_dark);
    //let header_content = render_style::render_header("Свойства виджета:", app.get_ui_style(), is_dark);

    let header_content =
        render_style::render_header("Свойства компонента:", app.get_ui_style());
    inspector_column = inspector_column.push(iced::widget::space::vertical().height(10));
    inspector_column = inspector_column.push(header_content);

    // Подготавливаем колонку для таблицы свойств с CAD-паддингом под скроллбар
    let mut table_content = column![]
        .spacing(3)
        .padding(options::padding_from(0.0, 0.0, 0.0, 0.0));

    // Проверяем фокус: выбран ли какой-нибудь виджет на холсте?
    if let Some(widget_id) = &app_state.selected_widget_id {
        if let Some(blueprint) = factory.get_blueprint(widget_id.clone()) {
            let w_type = blueprint.widget_type();

            // Информация о выбранном виджете
            let widget_card = static_card(
                text(format!("📦 {} [ {} ]", widget_id, w_type))
                    .size(13)
                    .into(),
                is_dark,
            );
            inspector_column = inspector_column.push(widget_card);
            inspector_column = inspector_column.push(iced::widget::space::vertical().height(1));

            // Добавляем блок кнопок управления Вверх/Вниз
            let updown_btn = static_card(render_updown_button(is_dark).into(), is_dark);
            inspector_column = inspector_column.push(updown_btn);
            inspector_column = inspector_column.push(iced::widget::space::vertical().height(1));

            /*
            let prop_key_str: Option<&str> = match &app_state.selected_property_key {
                Some(key) => Some(&key.name), // Просто берем ссылку на String через &!
                None => None,
            };
            */

            // Собираем таблицу списка доступных свойств
            table_content = table_content.push(inspector_panel_prop::build_properties_table(
                &widget_id,
                app,
            ));

            // Оборачиваем таблицу свойств в прокручиваемую scrollable-зону (Верхний этаж)
            let top_scrollable_zone = scrollable(table_content)
                .width(Length::Fill)
                .height(Length::Fill); // Занимает всё доступное место до нижнего редактора

            inspector_column = inspector_column.push(top_scrollable_zone);

            // ---------------------------------------------------------------------
            // Зона редактора: Интегрируем динамическую панель активного редактора
            // ---------------------------------------------------------------------
            let bottom_editor_zone = inspector_panel_edit::build_bottom_editor_zone(app);

            inspector_column = inspector_column.push(bottom_editor_zone);
        }
    } else {
        // Если фокуса нет — выводим отцентрированную заглушку во весь рост
        inspector_column = inspector_column.push(
            container(
                text("Выберите элемент в дереве слоев\nдля настройки его свойств")
                    .size(12)
                    .align_x(iced::Alignment::Center)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(palette.text_muted),
                    }),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center),
        );
    }

    // Возвращаем полностью собранную панель инспектора
    inspector_column.into()
}

// -----------------------------------------------------------------------------
// Хелперы
// -----------------------------------------------------------------------------

// Формирует панель с кнопками Вверх <> Вниз
pub fn render_updown_button<'a>(is_dark: bool) -> Element<'a, Message, Theme> {
    let mut row: iced::widget::Row<'a, Message, Theme> = row![]
        .spacing(4)
        .padding(iced::Padding {
            top: 4.0,
            bottom: 4.0,
            left: 0.0,
            right: 0.0,
        })
        .align_y(iced::Alignment::Center);

    row = row.push(text("Позиция:").size(13).width(Length::Fixed(110.0)));

    row = row.push(
        row![
            build_updown_button(
                "▲ Вверх",
                Message::MenuEvent(MenuAction::MoveUpWidget),
                is_dark
            ),
            build_updown_button(
                "▼ Вниз",
                Message::MenuEvent(MenuAction::MoveDownWidget),
                is_dark
            ),
        ]
        .spacing(4)
        .width(Length::Fill),
    );

    row.into()
}

// Формирует одну кнопку панели перемещения виджетов
pub fn build_updown_button<'a>(
    label: &'a str,
    message: Message,
    is_dark: bool,
) -> Element<'a, Message, Theme> {
    // Извлекаем свежую палитру под текущий флаг темы
    let palette = UiPalette::get_palette(is_dark);

    button(text(label).size(11).align_x(iced::Alignment::Center))
        .width(Length::Fill)
        .padding(4)
        // Добавляем move, чтобы замкнуть цвета вычисленной палитры внутри стиля кнопки
        .style(
            move |_theme: &iced::Theme, status: iced::widget::button::Status| {
                let mut s = button::Style {
                    // ИСПРАВЛЕНИЕ: Задаем прозрачный цвет подложки, чтобы Iced не гасил яркость шрифта
                    background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
                    // Текст и стрелочки гарантированно станут белыми на темной теме!
                    text_color: palette.text_main,
                    border: iced::Border {
                        color: palette.border_element,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                };

                // Состояние наведения мыши (Hover) — подсвечиваем плашку
                if status == iced::widget::button::Status::Hovered {
                    s.background = Some(iced::Background::Color(palette.bg_element));
                    s.border.color = palette.text_main;
                } else {
                    // В обычном состоянии на темной теме делаем легкий фон, чтобы кнопка не терялась
                    if is_dark {
                        s.background = Some(iced::Background::Color(palette.bg_element));
                    } else {
                        s.background = Some(iced::Background::Color(iced::Color::from_rgb(
                            0.92, 0.92, 0.92,
                        )));
                    }
                }

                s
            },
        )
        .on_press(message)
        .into()
}

pub fn static_card<'a>(
    content: Element<'a, Message>, // Принимает уже созданный виджет (например, pick_list) [1.1]
    is_dark: bool,
) -> Element<'a, Message> {
    // Получаем свежую палитру под текущий режим темы
    let palette = UiPalette::get_palette(is_dark);

    container(content)
        .width(Length::Fill)
        .padding(2) // Небольшой внутренний отступ, чтобы инпуты не прилипали к краям плашки [1.1]
        .style(move |_theme| container::Style {
            // Берем bg_element из палитры — фон станет сочным и адаптивным!
            //background: Some(iced::Background::Color(palette.bg_element)),
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),

            // Привязываем рамку к border_element палитры
            border: iced::Border {
                color: palette.border_element, // Тонкая рамка вокруг плашки [1.1]
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}

pub fn property_card<'a>(
    content: Element<'a, Message>, // Принимает уже созданный виджет (например, pick_list) [1.1]
    is_dark: bool,
) -> Element<'a, Message> {
    // Получаем свежую палитру под текущий режим темы
    let palette = UiPalette::get_palette(is_dark);

    container(content)
        .width(Length::Fill)
        .padding(4) // Небольшой внутренний отступ, чтобы инпуты не прилипали к краям плашки [1.1]
        .style(move |_theme| container::Style {
            // Берем bg_element из палитры — фон станет сочным и адаптивным!
            background: Some(iced::Background::Color(palette.bg_element)),

            // Привязываем рамку к border_element палитры
            border: iced::Border {
                color: palette.border_element, // Тонкая рамка вокруг плашки [1.1]
                width: 1.0,
                radius: 4.0.into(), // Аккуратные закругленные углы [1.1]
            },
            ..Default::default()
        })
        .into()
}

// -----------------------------------------------------------------------------
// Функции-обертки оформления пунктов инспектора
// -----------------------------------------------------------------------------
/// Универсальный горизонтальный шаблон строки инспектора
/// - Размещаем текстовую метку СЛЕВА от переданным виджетом управления
pub fn inspector_row<'a>(
    label_text: String,
    content: Element<'a, Message>, // Принимает уже созданный виджет (например, pick_list) [1.1]
    is_dark: bool,
) -> Element<'a, Message> {
    // 1. Получаем свежую палитру под текущий режим темы
    let palette = UiPalette::get_palette(is_dark);

    row![
        // Левая текстовая метка с фиксированной шириной
        text(label_text)
            .size(13)
            .width(Length::Fixed(100.0))
            // ИСПРАВЛЕНИЕ: Привязываем цвет шрифта к палитре.
            // Текст автоматически станет мягким светло-серым на тёмной теме!
            .style(move |_theme| iced::widget::text::Style {
                color: Some(palette.text_muted),
            }),
        // Правый виджет управления, который мы передали в функцию
        content
    ]
    .spacing(0)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}

/// Универсальный вертикальный шаблон инспектора
/// - Размещает текстовую метку НАД переданным виджетом управления
pub fn inspector_col<'a>(
    label_text: String,
    content: Element<'a, Message>, // Принимает созданный виджет (pick_list, text_input и др.) [1.1]
    is_dark: bool,
) -> Element<'a, Message> {
    // 1. Получаем свежую палитру под текущий режим темы
    let palette = UiPalette::get_palette(is_dark);

    column![
        // Текстовая метка сверху
        text(label_text)
            .size(13)
            // ИСПРАВЛЕНИЕ: Привязываем цвет верхней подписи к палитре.
            // Текст автоматически станет мягким светло-серым на тёмной теме!
            .style(move |_theme| iced::widget::text::Style {
                color: Some(palette.text_muted),
            }),
        // Виджет управления снизу (займет всю доступную ширину)
        content
    ]
    .spacing(4) // Минимальный плотный шаг между текстом и инпутом по вертикали [1.1]
    .width(Length::Fill)
    .into()
}
