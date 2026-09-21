// -----------------------------------------------------------------------------
// Модуль mainmenu
// Содержит реализацию главного меню
// -----------------------------------------------------------------------------
use iced::widget::{Button, button, column, container, row, space, text, tooltip};
use iced::{Alignment, Border, Color, Element, Length, Padding, Renderer, Theme, border::Radius};
use iced_aw::{Menu, MenuBar, menu, menu_items};

use crate::app::App;
use crate::core::*;
use crate::core::{DialogType, MenuAction, Message};
use crate::ui::*;



/// Вспомогательная функция для создания тулбара верхнего меню
pub fn render_top_toolbar<'a>(
    ui_style: UIStyle
) -> Element<'a, Message> {
    // Загружаем палитру приложения
    let palette = UiPalette::get_style_palette(ui_style);

    // Собираем двухэтажный тулбар в один горизонтальный ряд
    let toolbar_row = row![
        // Передаем аргументы: Иконка, Текст на кнопке, Текст всплывающей подсказки (Hint), Сообщение, Палитра
        toolbar_double_deck_button(
            ICON_NEW,
            "Новый",
            "Создать новый проект (Ctrl+N)",
            Message::MenuEvent(MenuAction::NewProject),
            //Message::OverlayEvent(OverlayAction::OpenDialog(DialogType::NewProject)),
            ui_style
        ),
        toolbar_double_deck_button(
            ICON_OPEN,
            "Открыть",
            "Открыть проект из файла (Ctrl+O)",
            Message::MenuEvent(MenuAction::OpenProject),
            ui_style
        ),
        toolbar_double_deck_button(
            ICON_SAVE,
            "Сохранить",
            "Сохранить текущий проект (Ctrl+S)",
            Message::MenuEvent(MenuAction::SaveProject),
            ui_style
        ),
        toolbar_separator(ui_style),
        toolbar_double_deck_button(
            ICON_CLEAR,
            "Очистить",
            "Полностью очистить проект",
            Message::MenuEvent(MenuAction::ClearCanvas),
            //Message::OverlayEvent(OverlayAction::OpenDialog(DialogType::ClearProject)),
            ui_style
        ),
        toolbar_double_deck_button(
            ICON_DELETE,
            "Удалить",
            "Удалить выбранный виджет (Delete)",
            Message::MenuEvent(MenuAction::DeleteWidget),
            ui_style
        ),
        toolbar_separator(ui_style),
        toolbar_double_deck_button(
            ICON_EXPORT,
            "Экспорт",
            "Экспортировать проект",
            Message::MenuEvent(MenuAction::ExportProject),
            ui_style
        ),
        toolbar_separator(ui_style),
        toolbar_double_deck_button(
            ICON_CODE,
            "Код",
            "Просмотреть пример кода",
            Message::OverlayEvent(OverlayAction::OpenDialog(DialogType::TreeCode)),
            ui_style
        ),
        /*
        toolbar_separator(is_dark),
        toolbar_double_deck_button(
            ICON_SETTINGS,
            "Настройка",
            "Настройка параметров",
            Message::OverlayEvent(OverlayAction::OpenDialog(DialogType::Settings)),
            ui_style,
            is_dark
        ),
        */
        toolbar_double_deck_button(
            ICON_ABOUT,
            "Инфо",
            "О программе",
            //Message::MenuEvent(MenuAction::ShowAbout),
            Message::OverlayEvent(OverlayAction::OpenDialog(DialogType::About)),
            ui_style
        ),
        toolbar_separator(ui_style),
        toolbar_double_deck_button(
            ICON_EXIT,
            "Выход",
            "Закрыть приложение (Alt+F4)",
            Message::MenuEvent(MenuAction::ExitApplication),
            //Message::OverlayEvent(OverlayAction::OpenDialog(DialogType::Exit)),
            ui_style
        ),
    ]
    .spacing(2)
    .align_y(Alignment::Center);

    // Внешний каркас тулбара с разделительной линией снизу
    container(column![
        toolbar_row,
        // Тонкая разделительная линия в 1 пиксель под тулбаром
        container(column![]).height(1.0).width(Length::Fill)
    ])
    .width(Length::Fill)
    // Увеличиваем высоту панели под двухэтажные кнопки
    .padding(padding_from(4.0, 6.0, 0.0, 6.0))
    .style(uitheme::style_toolbar_container(ui_style))
    .into()
}

pub fn render_main_menu_aw<'a>(app: &App) -> Element<'a, Message> {
    use iced::alignment::Horizontal;

    let ui_style = app.get_ui_style();
    let palette  = UiPalette::get_style_palette(ui_style);
    let bg_color = palette.bg_panel;

    // Кастомный паддинг для пунктов выпадающего списка (Верх/Низ: 6px, Право/Лево: 12px)
    //let item_padding = Padding::ZERO.top(6.0).bottom(6.0).left(12.0).right(12.0);
    //let item_padding = Padding::ZERO.top(2.0).bottom(2.0).left(4.0).right(4.0);
    let item_padding = Padding::ZERO.top(0.0).bottom(0.0).left(4.0).right(4.0); // Blender

    // Настройка текстовых кнопок (убираем фон, оставляем плоский текст)
    //let menu_btn_style = button::text;

    // -------------------------------------------------------------------------
    // Хелперы создания главных и выпадающих пунктов меню
    // -------------------------------------------------------------------------
    // Внутренний отступ для самих шапок меню верхнего уровня
    //let header_padding = Padding::ZERO.top(4.0).bottom(4.0).left(8.0).right(8.0);
    let header_padding = Padding::ZERO.top(2.0).bottom(2.0).left(8.0).right(8.0);       // Blender

    let text_header_helper = move |label: &'static str| -> iced::widget::Button<
        '_,
        Message,
        iced::Theme,
        iced::Renderer,
    > {
        // Создаем чистый текстовый виджет
        let label_widget = text(label).size(12).width(Length::Shrink);

        // Оборачиваем его в контейнер с отступами шапки.
        button(label_widget)
            .padding(header_padding)
            .style(uitheme::style_mainmenu_button(ui_style))
            .on_press(Message::MenuEvent(MenuAction::NoOp))

        // container(label_widget)
        //     .padding(header_padding)
        //     .width(Length::Shrink)
        //     .height(Length::Shrink)
        //     .style(|theme| {container::transparent(theme)})
    };

    let menu_item_helper = move |icon: &'static str, label: &'static str, msg: Message| {
        // Собираем виджет для иконки
        let icon_widget = text(icon)
            .size(16)
            .font(FONT_MATERIAL)
            .width(Length::Shrink); // Иконка занимает ровно столько места, сколько ей нужно

        // Собираем виджет для текста пункта меню (дефолтный шрифт, размер 13)
        let label_widget = text(label)
            .size(11)
            .width(Length::Fill) // Текст растягивается, заполняя всю оставшуюся ширину кнопочной плашки
            .align_x(Horizontal::Left);

        // Компонуем их горизонтально внутри Row с небольшим аккуратным зазором
        let button_content = row![icon_widget, label_widget,]
            .spacing(8)                         // Зазор в 8 пикселей между иконкой и текстом (стиль VS Code / Figma)
            .align_y(iced::Alignment::Center)   // Выравниваем иконку и текст строго по центру относительно друг друга
            .width(Length::Fill);

        // Помещаем собранный ряд внутрь нашей нативной кнопки
        button(button_content)
            .padding(item_padding)
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(uitheme::style_mainmenu_button(app.get_ui_style())) // Адаптивный стиль ховеров для светлой/темной темы            
            //.style(get_menu_button_style(app.get_ui_style())) // Адаптивный стиль ховеров для светлой/темной темы
            .on_press(msg)
    };

    // -------------------------------------------------------------------------
    // Выпадающий список "Файл"
    // -------------------------------------------------------------------------
    let file_menu: Menu<'a, Message, Theme, Renderer> = Menu::new(menu_items!(
        (menu_item_helper(
            ICON_NEW,
            "Новый проект",
            Message::MenuEvent(MenuAction::NewProject)
        )),
        (menu_item_helper(
            ICON_OPEN,
            "Открыть проект",
            Message::MenuEvent(MenuAction::OpenProject)
        )),
        (menu_item_helper(
            ICON_SAVE,
            "Сохранить проект",
            Message::MenuEvent(MenuAction::SaveProject)
        )),
        (menu_item_helper(
            ICON_EXIT,
            "Выход",
            Message::MenuEvent(MenuAction::ExitApplication)
        )),
    ))
    // Фиксированная ширина
    .width(Length::Fixed(180.0))
    .offset(6.0);   // Расстояние до выпадающего меню

    // -------------------------------------------------------------------------
    // Выпадающий список "Редактировать"
    // -------------------------------------------------------------------------
    let edit_menu = Menu::new(menu_items!(
        (menu_item_helper(
            "📋",
            "Удалить виджет",
            Message::MenuEvent(MenuAction::DeleteWidget)
        )),
        (menu_item_helper(
            "📋",
            "Очистить проект",
            Message::MenuEvent(MenuAction::ClearCanvas)
        )),
        (menu_item_helper("📋", "Переключить режим дизайна", Message::NoOp)),
    ))
    .width(Length::Fixed(240.0))
    .offset(6.0);   // Расстояние до выпадающего меню

    // -------------------------------------------------------------------------
    // Выпадающий список "Режим"
    // -------------------------------------------------------------------------

    // let view_mode = if app.is_design_mode() {
    //     "Включить режим просмотра"
    // } else {
    //     "Включить режим проектирования"
    // };
    // let view_theme = if !app.is_dark_theme() {
    //     "Включить темную тему"
    // } else {
    //     "Включить светлую тему"
    // };

    // let icon_mode = if app.is_design_mode() {
    //      ICON_EYE
    //  } else {
    //      ICON_DESIGN
    //  };
    // let icon_theme = if !app.is_dark_theme() {
    //     ICON_MOON
    // } else {
    //     ICON_SUN
    // };

    let (theme_icon, theme_text, _theme_hint) = uitheme::get_ui_theme_toggle_data(app.is_dark_theme());
    let (mode_icon,  mode_text,  _mode_hint)  = uitheme::get_ui_mode_toggle_data(app.is_design_mode());
    
    let mode_menu = Menu::new(menu_items!(
        (menu_item_helper(
            mode_icon,
            mode_text,
            Message::MenuEvent(MenuAction::ToggleDesignMode)
        )),
        (menu_item_helper(
            theme_icon,
            theme_text,
            Message::MenuEvent(MenuAction::ToggleViewTheme)
        )),
    ))
    .width(Length::Fixed(260.0))
    .offset(6.0);   // Расстояние до выпадающего меню

    // -------------------------------------------------------------------------
    // Выпадающий список "Сервис"
    // -------------------------------------------------------------------------
    let service_menu = Menu::new(menu_items!(
        (menu_item_helper(ICON_SETTINGS, "Настройка", Message::NoOp)),
        (menu_item_helper(ICON_EXPORT, "Экспорт структуры (JSON)", Message::NoOp)),
    ))
    .width(Length::Fixed(220.0))
    .offset(6.0);   // Расстояние до выпадающего меню

    // -------------------------------------------------------------------------
    // Выпадающий список "Помощь"
    // -------------------------------------------------------------------------
    let about_menu = Menu::new(menu_items!(
        (menu_item_helper(
            ICON_ABOUT,
            "О программе",
            Message::MenuEvent(MenuAction::ShowAbout)
        )),
    ))
    .width(Length::Fixed(140.0))
    .offset(6.0);   // Расстояние до выпадающего меню

    // -------------------------------------------------------------------------
    // ГЛАВНАЯ СБОРКА MENU_BAR
    // -------------------------------------------------------------------------

    //let menu_bar = MenuBar::with_state(menu_state, menu_items!(
    let menu_bar = MenuBar::new(menu_items!(
        (text_header_helper("Файл"),   file_menu),
        (text_header_helper("Редактировать"), edit_menu),
        (text_header_helper("Режим"),  mode_menu),
        (text_header_helper("Сервис"), service_menu),
        (text_header_helper("Помощь"), about_menu),
    ))
    .style(uitheme::style_dropdown_menu(ui_style))
    .draw_path(menu::DrawPath::Backdrop)
    .spacing(4.0)                   // Расстояние между вкладками верхнего уровня
    .safe_bounds_margin(8.0);       // Граница оверлея вокруг выпадающего меню

    // Заворачиваем всё меню в красивую тонкую горизонтальную плашку во всю ширину
    container(menu_bar)
        .width(Length::Fill)
        //.padding(Padding::ZERO.left(10.0).right(10.0)) // Небольшой отступ от краев экрана
        .padding(Padding::ZERO.left(8.0).right(8.0)) // Небольшой отступ от краев экрана
        .style(uitheme::style_menu_container(ui_style))
        .into()
}


/// Функция генерирует стиль для выпадающих окошек меню на основе темы оформления
pub fn get_dropdown_menu_style(ui_style: UIStyle) -> impl Fn(&Theme) -> menu::Style {

    let palette = UiPalette::get_style_palette(ui_style);

    let bg_color     = palette.bg_panel;
    let border_color = palette. border_element;

    move |_theme: &Theme| menu::Style {
        // Стилизуем выпадающее меню
        menu_background: iced::Background::Color(bg_color),
        menu_border: Border {
            color:   border_color,
            width:   1.0,
            radius:  Radius::new(4.0),
            ..Default::default()
        },

        // Верхний бар меню делаем полностью прозрачным, чтобы он сливался с шапкой
        bar_background: iced::Background::Color(Color::TRANSPARENT),
        bar_border:     Border::default(),
        ..Default::default()
    }
}

pub fn menu_button<'a>(
    label: &'a str,
    action: MenuAction,
) -> Button<'a, Message, iced::Theme, iced::Renderer> {
    // Внутренний отступ, чтобы пункты меню выглядели просторно (Верх/Низ: 6px, Право/Лево: 12px)
    let item_padding = Padding::ZERO.top(4.0).bottom(4.0).left(8.0).right(8.0);

    // Создаем кнопку. Если в вашей версии button() принимает строку напрямую:
    button(label)
        .padding(item_padding)
        .style(button::text) // Делаем кнопку плоской (без серых рамок и стандартных фонов)
        .width(Length::Fill) // Растягиваем кнопку на всю ширину выпадающей плашки
        .on_press(Message::MenuEvent(action)) // Привязываем наше единое сообщение меню
}


