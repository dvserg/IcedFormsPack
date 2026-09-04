// -----------------------------------------------------------------------------
// Модуль sidebar
// Содержит реализацию левого sidebar приложения со списком доступных виджетов
// -----------------------------------------------------------------------------
use iced::Theme;
//use iced::advanced::graphics::text::cosmic_text::skrifa::outline::LcdLayout::Horizontal;
use iced::widget::{button, column, container, row, scrollable, space, text, tooltip, rule};
use iced::{Alignment, Background, Border, Element, Length};
use std::collections::BTreeMap;

use crate::app::App;
use crate::{core::*, ui};
use crate::ui::{UiPalette, UIStyle, RenderStyle, render_style, uitheme};



// -----------------------------------------------------------------------------
// Функция генерации левой панели элементов (Toolbox)
// -----------------------------------------------------------------------------
pub fn render_sidebar<'a>(app: &App) -> Element<'a, Message, Theme> {
    // Получить фабрику
    let factory = app.get_factory();

    // Получить тип темы приложения
    let is_dark = app.is_dark_theme();
    let is_design_mode = app.is_design_mode();

    // Получение палитры
    let palette  = UiPalette::get_palette(is_dark);
    let ui_style = app.get_ui_style();
    
    // =====================================================================
    // ДИНАМИЧЕСКИЙ СКАН ИНВЕНТАРЯ И ГРУППИРОВКА:
    // =====================================================================
    // Создаем рантайм-карту: Имя_Категории -> Вектор (Системный_Идентификатор_Типа)
    let mut runtime_groups: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();

    // Сканируем макросы авторегистрации, подтягивая категории без хардкода в UI!
    for registered in inventory::iter::<AutoRegisteredWidget> {
        // Проверяем, что этот конструктор реально присутствует на складе Creators фабрики,
        // чтобы не отрисовать в сайдбаре незарегистрированный или битый компонент
        if factory.creators.contains_key(registered.name) {
            runtime_groups
                .entry(registered.category)
                .or_insert_with(Vec::new)
                .push(registered.name.to_string());
        }
    }

    // Сортируем String по алфавиту от А до Я!
    for widgets_list in runtime_groups.values_mut() {
        widgets_list.sort();
    }

    let mut is_first_group = true;

    // ВАЖНО: Уменьшаем общий базовый шаг всей колонки до минимума (было 12)
    let mut list_content = column![]
        .spacing(0)
        //.padding(options::padding_from(0.0, 14.0, 0.0, 0.0))
        .width(Length::Fill)
        .height(Length::Shrink);

    // Собираем колонку элементов по категориям
    for (category_name, widgets_list) in &runtime_groups {
        let cat_str: &'static str = *category_name;
        let list_ref: &Vec<String> = widgets_list;

        let header_string = cat_str;

        // Собираем элементы строки заголовка (Стрелочка + Текст)
        //let header_row = render_style::render_group_header(&header_string, app.get_ui_style(), is_dark);

        let group_header = render_style::render_group_header(&header_string, ui_style);

        // Собираем колонку элементов группы
        let mut buttons_column = column![].spacing(2); // Плотный шаг между кнопками

        buttons_column = buttons_column.push(group_header);

        for widget_type in list_ref.iter() {
            let friendly_label = format_widget_name(widget_type);
            // Формируем элемент списка sidebar
            let btn = sidebar_button(
                    &friendly_label, 
                    widget_type, 
                    ui_style
                );
            buttons_column = buttons_column.push(btn);
        }

        // Применяем к группе текущую тему
        let group_block = render_style::render_group_panel(
            buttons_column.into(), 
            ui_style
        );

        // Расставляем Blender-воздух между блоками
        if !is_first_group {
            let spaced_group = column![
                // 10 пикселей отступа сверху перед следующей плашкой для идеального ритма верстки
                iced::widget::space::vertical().height(Length::Fixed(6.0)),
                group_block
            ]
            .spacing(0);

            list_content = list_content.push(spaced_group);
        } else {
            list_content = list_content.push(group_block);
            is_first_group = false;
        }
    }

    // =====================================================================

    // Тулбар панели sidebar
    let sidebar_toolbar = render_sidebar_toolbar(is_design_mode, ui_style);

    // Оформляем панель тулбара
    let toolbar_panel = container(sidebar_toolbar)
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(iced::alignment::Horizontal::Right);    // Кнопка темы прижата к правой стороне

    // Применяем к тулбару текущую групповую тему
    // ** Посмотреть по другим стилям - возможно здесь это не нужно
    let toolbar_block = render_style::render_group_panel(
            toolbar_panel.into(), 
            ui_style
        );

    // Заголовок списка компонентов
    let sidebar_header = render_style::render_header("Компоненты", ui_style);

    // Главный внешний каркас панели
    let main_panel = column![
        // Панель тулбара
        toolbar_block,

        // Отступ перед списком после тулбара
        space::vertical().height(Length::Fixed(2.0)),

        // Панель компонентов
        container(sidebar_header)
            .width(Length::Fill)
            .height(Length::Shrink),

        // Отступ перед списком после заголовка
        //space::vertical().height(Length::Fixed(2.0)),

        // Список кнопок-виджетов во внутреннем скроллбаре,
        // обеспечивающем прокрутку всего списка
        container(
            scrollable(list_content)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill),
    ]
    .padding(options::padding_from(4.0, 2.0, 4.0, 2.0))
    .width(Length::Fill)
    .height(Length::Fill);

    // Оборачиваем готовый макет в контейнер
    render_style::render_panel_frame(
        main_panel.into(), //main_container.into(),
        ui_style
    ).into()
}

// -----------------------------------------------------------------------------
// Хэлперы
// -----------------------------------------------------------------------------

// Вспомогательная функция: делает первую букву заглавной для красоты в UI
fn format_widget_name(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// Вспомогательная функция для создания интерактивной кнопки добавления виджета
fn sidebar_button<'a>(
    label: &str,
    widget_type: &str,
    //palette: UiPalette,
    ui_style:    UIStyle,
) -> Element<'a, Message, Theme> {

    // Получение палитры
    let palette = UiPalette::get_style_palette(ui_style);

    // Выбираем иконку динамически по типу виджета
    let icon_char     = uitheme::get_widget_icon(widget_type);
    let ui_item_style = uitheme::UIListTileStyle::default();

    // Упаковываем в кнопку UTF-8 иконку и название виджета
    let text_size = ui_item_style.item_text_size;
    let text_icon = ui_item_style.item_icon_size;

    //let mut is_hovered = false;

    let item = button(
            row![
                // Компонент иконки
                text(icon_char)
                    .font(uitheme::FONT_MATERIAL)
                    .size(text_icon)
                    // Фиксируем ширину иконки для строгого столбца основного текста
                    .width(Length::Fixed(text_icon))
                    .shaping(iced::widget::text::Shaping::Advanced)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(palette.text_muted), // Иконка чуть бледнее текста
                    }),
                // Компонент основного текста
                text(label.to_string()).size(text_size)
            ]
            // Отступ между иконкой и текстом
            .spacing(ui_item_style.item_spacing)
            .padding(ui_item_style.item_padding)
            .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .padding(0)
        .style(uitheme::style_item_button(ui_style, false)) // false - Элементы sidebar никогда не выбираются
        // Привязываем событие добавления виджета к кнопке
        .on_press(Message::MenuEvent(MenuAction::AddWidget(String::from(
            widget_type,
        ))));    

    item.into()    
}


fn render_sidebar_toolbar<'a>(is_design: bool, ui_style: UIStyle) -> Element<'a, Message, Theme> {

    // Переключение темы
    // let (theme_icon, _theme_text, theme_hint) = match ui_style.is_dark_theme {
    //     true =>  (ui::ICON_SUN,  "Светлая", "Переключить на светлую тему"),
    //     false => (ui::ICON_MOON, "Тёмная",  "Переключить на тёмную тему"),
    // };

    // // Переключение режима конструктора
    // let (mode_icon, _mode_text, mode_hint) = match is_design {
    //     true  => (ui::ICON_EYE,    "Просмотр", "Переключить в режим просмотра"),
    //     false => (ui::ICON_DESIGN, "Дизайн",   "Переключить в режим дизайна"),
    // };

    let (theme_icon, _theme_text, theme_hint) = ui::get_ui_theme_toggle_data(ui_style.is_dark_theme);
    let (mode_icon,  _mode_text,  mode_hint)  = ui::get_ui_mode_toggle_data(is_design);

    let toolbar_row = row![
        crate::ui::toolbar_micro_button( theme_icon, "", theme_hint,
            Message::MenuEvent(MenuAction::ToggleViewTheme),
            ui_style
        ),
        crate::ui::toolbar_micro_button( mode_icon, "", mode_hint,
            Message::MenuEvent(MenuAction::ToggleDesignMode),
            ui_style
        ),
    ];

    toolbar_row.into()
}

// Вспомогательная функция для создания кнопки переключения тем оформления (Светлая/Тёмная)
// fn theme_toggle_button<'a>(is_dark: bool, palette: UiPalette) -> Element<'a, Message, Theme> {
//     let (icon_char, theme_text, theme_hint) = if is_dark {
//         ("\u{e81a}", "Светлая", "Переключить на светлую тему")
//     } else {
//         ("\u{e51c}", "Тёмная", "Переключить на тёмную тему")
//     };

//     let raw_button = button(column![
//         // Верхний этаж: Крупная, выразительная иконка (20px)
//         text(icon_char)
//             .font(uitheme::FONT_MATERIAL)
//             .size(18)
//             .width(Length::Fill)
//             .shaping(iced::widget::text::Shaping::Advanced)
//             .align_x(Alignment::Center)
//             .align_y(Alignment::Center),
//         // Нижний этаж: Отлично читаемый текст подписи (12px)
//         // text(theme_text)
//         //     .size(11)
//         //     .width(Length::Fill)
//         //     .align_x(Alignment::Center),
//     ])
//     .width(Length::Fixed(28.0))
//     .height(Length::Fixed(28.0))
//     .padding(0)
//     .style(cloned_palette(palette))
//     //.on_press(Message::ToggleTheme);
//     .on_press(Message::MenuEvent(MenuAction::ToggleViewTheme));

//     // Всплывающая подсказка для кнопки
//     tooltip(
//         raw_button,
//         container(
//             // Текст всплывающей подсказки
//             text(theme_hint)
//                 .size(11)
//                 .style(move |_theme| iced::widget::text::Style {
//                     color: Some(palette.text_main),
//                 }),
//         )
//         .padding(6)
//         .style(move |_theme| iced::widget::container::Style {
//             background: Some(Background::Color(palette.bg_panel)), // Цвета подложки подсказки из палитры
//             border: Border {
//                 color: palette.border_element,
//                 width: 1.0,
//                 radius: 4.0.into(),
//             },
//             ..Default::default()
//         }),
//         // Подсказка аккуратно выплывает строго снизу под кнопкой
//         tooltip::Position::Bottom,
//     )
//     // Микро-отступ между краем кнопки и рамкой подсказки
//     .gap(4.0)
//     .into()
// }

// Вспомогательная функция генерации динамического стиля с повышенной контрастностью на тёмной теме
// fn cloned_palette(
//     palette: UiPalette,
// ) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style {
//     move |_theme, status| {
//         // Проверяем, активен ли сейчас тёмный режим (смотрим по цвету подложки)
//         let is_dark_mode = palette.bg_panel.r < 0.2;

//         // Жёстко зашиваем контрастные цвета для текста и иконки, чтобы они не блекли:
//         let text_color = if is_dark_mode {
//             iced::Color::WHITE // Кристально белый цвет для тёмной темы (кнопка "Светлая")
//         } else {
//             palette.text_main // Цвет палитры для светлой темы (кнопка "Тёмная")
//         };

//         let mut s = button::Style {
//             // Явно задаем прозрачный фон вместо None, чтобы Iced не приглушал яркость шрифта
//             background: Some(Background::Color(palette.bg_element)),
//             text_color, // Применяем яркий цвет
//             border: Border {
//                 color: iced::Color::TRANSPARENT,
//                 width: 0.0,
//                 radius: 4.0.into(),
//             },
//             ..Default::default()
//         };

//         // Эффект наведения мыши (Hover)
//         if status == button::Status::Hovered {
//             s.background = Some(Background::Color(palette.bg_element));
//             s.background = Some(Background::Color(iced::Color::TRANSPARENT));
//             s.border.color = palette.border_element;
//             s.border.width = 1.0;

//             // При наведении удерживаем максимальную яркость
//             if is_dark_mode {
//                 s.text_color = iced::Color::WHITE;
//             }
//         }
//         s
//     }
// }
