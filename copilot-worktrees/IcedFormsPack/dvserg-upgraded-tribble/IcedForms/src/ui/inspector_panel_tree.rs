// -----------------------------------------------------------------------------
// Модуль inspector_panel_tree
// Содержит реализацию панели дерева виджетов
// -----------------------------------------------------------------------------
use iced::Theme;
use iced::widget::{Column, Row, button, column, container, row, scrollable, space, text};
use iced::{Alignment, Color, Element, Length};

use crate::app::App;
use crate::core::*;
use crate::ui::{UiPalette, render_style, uitheme};

/// Рендеринг дерева слоев
pub fn render_layers_tree<'a>(app: &'a App) -> Element<'a, Message, Theme> {
    // Получаем текущую тему приложения
    let is_dark = app.is_dark_theme();

    // Получаем палитру приложения
    let palette = UiPalette::get_palette(is_dark);

    // получаем состояние приложения и ссылку на фабрику
    let app_state = app.get_state();
    let factory = app.get_factory();

    // Создаем колонку для дерева панели
    let mut layers_col = column![].spacing(4);

    // Разделитель сверху над панелью
    //layers_col = layers_col.push(iced::widget::rule::horizontal(6.0));

    layers_col = layers_col.push(render_style::render_header(
        "Дерево слоев:",
        app.get_ui_style()
    ));

    // Коллекция элементов дерева
    let mut tree_items = Vec::new();

    // Запускаем рекурсивный обход с корня (parent_id == ""), глубина 0
    build_hierarchy_tree(
        0,
        "".to_string(),
        app_state.selected_widget_id.as_deref(),
        app, 
        //&factory,
        //is_dark,
        // Передаем коллекцию в рекурсивную функцию
        &mut tree_items,
    );

    // Стром колонку дерева
    let layers_tree = Column::with_children(tree_items)
        .spacing(2)
        // Применяем отступы для сдвига контента от ползунка скролла
        .padding(options::padding_from(0.0, 0.0, 0.0, 0.0));

    // Формируем отступ над деревом
    layers_col = layers_col.push(iced::widget::space::vertical().height(1));
    //layers_col = layers_col.push(iced::widget::rule::horizontal(1.0));

    // Оборачиваем дерево в скролл, чтобы длинный список слоев можно было крутить отдельно,
    // и возвращаем его как контейнер с FillPortion(1) (пропорционально занимает 1/N часть правого sidebar)
    layers_col = layers_col.push(
        scrollable(layers_tree).height(Length::FillPortion(1)), // Занимает пропорциональную высоту
    );

    // Оборачиваем всю панель дерева в контейнер и применяем текущую палитру приложения
    // Иерархия элементов: container < column < scrollable < tree-column
    container(layers_col)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| container::Style {
            // Даем проявиться фоновой теме приложения
            //background: Some(iced::Background::Color(palette.bg_panel)),
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            text_color: Some(palette.text_main), // Каскадно пробрасываем цвет текста по умолчанию
            ..Default::default()
        })
        .into()
}

/// Рекурсивная функция сборки иерархии дерева
pub fn build_hierarchy_tree<'a>(
    depth: u32,
    parent_id: String,
    selected_widget_id: Option<&str>,
    app: &App,

    //factory: &'a Factory,
    //is_dark: bool,

    // Коллекция узлов дерева
    tree_items: &mut Vec<iced::Element<'a, Message, iced::Theme>>,
) {
    
    let factory  = app.get_factory();
    let is_dark  = app.is_dark_theme();
    let ui_style = app.get_ui_style();

    // Получить текущую палитру
    let palette = UiPalette::get_palette(is_dark);

    // Получаем отфильтрованный вектор ссылок на Arc-чертежи детей через метод фабрики
    let children_blueprints = factory.get_blueprints_by_parent(parent_id.as_str());

    // Цикл по полученному списку детей для построения списка на данном уровне иерархии
    for blueprint_arc in children_blueprints {
        let id = blueprint_arc.get_id();
        let w_type = blueprint_arc.widget_type();

        // Проверяем статус выделения строки
        let is_selected = selected_widget_id == Some(id.as_str());

        // Вызов хелпера: Передаем только id, тип и глубину!
        let button_content = build_tree_item_row(id.clone(), w_type, depth);

        // Сборка строки(кнопки) дерева: Настраиваем цвета и ховеры строки здесь!
        let item_btn = button(button_content)        
            .width(Length::Fill)
            .padding(2)
            .style(uitheme::style_item_button(ui_style, is_selected))
            .on_press(Message::MenuEvent(MenuAction::SelectWidget(id.clone())));

        tree_items.push(item_btn.into());

        // Рекурсивное построение списка дочерних элементов для текущего виджета
        let _children = build_hierarchy_tree(
            depth + 1,
            id,
            selected_widget_id,
            app,
            //factory,
            //is_dark,
            // Передаем коллекцию на уровень ниже для заполнения
            tree_items,
        );
    }
}

/// Хэлпер: Строит строку дерева ( без применения стилей )
pub fn build_tree_item_row<'a>(
    id: String,
    w_type: &'a str,
    depth: u32,
) -> Row<'a, Message, iced::Theme> {
    // Автоматически определяем иконку по типу виджета
    let icon = uitheme::get_widget_icon(w_type);
    let ui_item_style = uitheme::UIListTileStyle::default();

    // Возвращаем чистый Row, где уложены ТОЛЬКО отступ и текстовые блоки
    row![
        // Сдвиг вправо на 15 пикселей за каждый уровень вложенности
        space::Space::new().width(Length::Fixed((depth * 15) as f32)),

        // Иконка типа
        text(icon).size(ui_item_style.item_icon_size).font(uitheme::FONT_MATERIAL),

        // Разделительный микро-зазор между иконкой и ID
        space::Space::new().width(Length::Fixed(ui_item_style.item_spacing.0)),

        // Название (ID виджета)
        text(id).size(ui_item_style.item_text_size),

        // Зазор перед типом виджета
        space::Space::new().width(Length::Fixed(ui_item_style.item_spacing.0)),

        // Тип виджета в квадратных скобках (пока без цвета!)
        text(format!("[ {} ]", w_type)).size(11)
    ]
    .spacing(0)
    .align_y(Alignment::Center)
}
