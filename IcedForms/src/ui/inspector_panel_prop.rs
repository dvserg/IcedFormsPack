// -----------------------------------------------------------------------------
// Модуль inspector_panel_props
// Содержит реализацию панели списка свойств виджета
// -----------------------------------------------------------------------------
use iced::widget::{Column, button, column, row, container, text};
use iced::{Alignment, Color, Element, Length, Theme};

pub use crate::app::App;
use crate::core::*;
use crate::ui::{UiPalette, UIStyle, RenderStyle, render_style, uitheme};

/*
/// ПОЛНОСТЬЮ ИЗОЛИРОВАННАЯ ФУНКЦИЯ СБОРКИ НИЖНЕЙ ЗОНЫ РЕДАКТОРА СВОЙСТВ
pub fn build_bottom_editor_zone<'a>(
    widget_id: &'a str,                         // ID текущего выделенного виджета
    selected_property_name: Option<&String>,     // Активное выбранное свойство (если есть)
    w_factory: &'a crate::WidgetFactory,       // Ссылка на фабрику параметров
    is_dark: bool,
) -> Element<'a, Message, Theme> {

    let palette = UiPalette::get_palette(is_dark);

    // 1. Собираем базовую колонку панели редактора
    let mut editor_panel = column![].spacing(0);

    // Добавляем шапку редактора
    editor_panel = editor_panel.push(
        inspector::render_header("Редактор свойств", is_dark)
    );

    // 2. Диспетчеризация контента: выбрано ли свойство в таблице?
    if let Some(active_prop) = selected_property_name {
        // Вытаскиваем живое значение свойства из базы данных factory
        let current_value: &'a str = w_factory.get_property(widget_id, active_prop)
            .map(|s| s.as_str())
            .unwrap_or("");

        // Вызываем изолированную функцию подбора нужного редактора (инпут, каунтер, пиклист)
        let active_editor = super::panel_edit::build_active_property_editor(
            active_prop.as_str(),
            widget_id,
            current_value,
            is_dark,
            w_factory,
        );

        // Оборачиваем весь динамический редактор в красивую карточку-плашку темы
        let bounded_editor = inspector::property_card(active_editor, is_dark);
        editor_panel = editor_panel.push(bounded_editor);

    } else {
        // Если свойство не выбрано — выводим красивую CAD-заглушку, используя класс Muted из темы
        editor_panel = editor_panel.push(
            container(
                text("Выберите свойство в таблице выше")
                    .size(12)
                    .style(TextStyle::Muted) // Текст автоматически станет наклонным/серым из темы
            )
            .padding(10)
        );
    }

    // 3. Финальная обертка всей зоны в контейнер с адаптивной разделительной рамкой сверху
    container(editor_panel)
        .width(Length::Fill)
        .padding(6)
        .style(move |_theme| container::Style {
            border: iced::Border {
                // Линия станет мягкой темно-серой на темной теме и аккуратной светло-серой на светлой
                color: palette.border_element,
                width: 1.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}
*/

/// ПОЛНОСТЬЮ ИЗОЛИРОВАННАЯ ФУНКЦИЯ СБОРКИ ТАБЛИЦЫ СПИСКА СВОЙСТВ ВИДЖЕТА
pub fn build_properties_table<'a>(
    widget_id: &str,                      // ID текущего выделенного виджета
    app: &App,
) -> Column<'a, Message, Theme> {

    let selected_property_name: Option<&str> = match app.get_state().selected_property_key {
        Some(key) => Some(&key.name), // Просто берем ссылку на String через &!
        None => None,
    };    

    let w_factory = app.get_factory();
    let is_dark   = app.is_dark_theme();

    // Инициализируем палитру и пустую вертикальную колонку-таблицу
    let ui_style = app.get_ui_style();
    let palette  = UiPalette::get_style_palette(ui_style);

    //let palette = UiPalette::get_palette(is_dark);
    let ui_item_style = uitheme::UIListTileStyle::default();

    let mut table_content = column![].spacing(2).width(Length::Fill);

    // Достаем чертеж и тип виджета из фабрики по widget_id!
    if let Some(blueprint_arc) = w_factory.get_blueprint(widget_id.to_string()) {
        let w_type = blueprint_arc.widget_type();

        // Извлекаем вектор редактируемых свойств из чертежа
        let mut p_table = blueprint_arc.editable_properties().clone();

        // Добавляем свойства "parent" принудительно для всех, кроме корневого элемента
        if w_type != "root" {
            p_table.push(PROP_PARENT);
        }

        // Динамически генерируем интерактивные строки таблицы параметров
        for prop_key in p_table {
            //let prop_key  = PropertyKey::from_dynamic(prop);//.to_string();
            let is_active = selected_property_name == Some(prop_key.name);
            let friendly_label = friendly_label(prop_key);

            // Читаем текущее строковое значение этого свойства для отображения
            let val_preview = w_factory.get_as_string(widget_id, prop_key, "н/д");

            // Собираем строчку таблицы
            let row_content = row![
                // Название свойства
                text(friendly_label).size(ui_item_style.item_text_size).width(Length::Fixed(110.0)),
                // Контент свойства
                container(               
                    text(val_preview)
                        .size(ui_item_style.item_text_size)
                        .width(Length::Fill)
                        .wrapping(text::Wrapping::None)
                        .line_height(text::LineHeight::Relative(1.0))                        
                        // .style(move |_theme| iced::widget::text::Style {
                        //      color: Some(if is_active { palette.text_main } else { palette.text_muted })
                        // })
                )
                .max_height(32.0)   // Ограничиваем высоту строки свойства
                .clip(true),
            ]
            .spacing(8)
            .align_y(Alignment::Center)            
            .into();

            // Обертка в интерактивную CAD-плашку
            let table_row_btn = property_row_wrapper(
                row_content, 
                prop_key, 
                app.get_ui_style(), 
                is_active                
            );

            table_content = table_content.push(table_row_btn);
        }
    }

    table_content
}

// -----------------------------------------------------------------------------
/// Декоратор для строк таблицы свойств.
/// Принимает готовое содержимое строки, имя свойства для отправки события и флаг активности.
/// Возвращает красивую интерактивную кнопку-карточку.
pub fn property_row_wrapper<'a>(
    content: Element<'a, Message, Theme>, // Готовый внутренний row_content
    property_key: PropertyKey,            // Имя свойства для события клика
    ui_style:     UIStyle,
    is_selected:  bool,                   // Флаг: выделено ли это свойство сейчас
) -> Element<'a, Message, Theme> {
    // Получаем свежую палитру под текущий режим темы
    //let palette = UiPalette::get_palette(is_dark);

    button(content)
        .padding(iced::Padding::from([3.0, 4.0]))
        .width(Length::Fill)
        .style(uitheme::style_item_button(ui_style, is_selected))
        // Привязываем стандартное сообщение выбора свойства
        .on_press(Message::MenuEvent(MenuAction::SelectProperty(property_key)))
        .into()
}
