use iced::widget::{column, container, mouse_area, pane_grid, stack};
use iced::{Element, Length, Theme};
use iced::widget::themer;     // Импортируем виджет смены темы

use crate::app::{App, InspectorPane, Pane};
use crate::core::{MenuAction, OverlayAction, Message, PROP_PARENT};
use crate::ui::inspector::render_inspector;
use crate::ui::{
    UiPalette,
    render_main_menu_aw, render_sidebar, render_top_toolbar, 
    overlays, render_style
};

/// Вспомогательная функция для рабочего пространства главной формы программы
pub fn render_workspace<'a>(app: &'a App) -> Element<'a, Message, Theme> {

    // Текущая тема приложения (темная/светлая)
    let is_dark = app.is_dark_theme();

    // Загружаем палитру приложения для текущего стиля
    let palette = UiPalette::get_style_palette(app.get_ui_style());

    // Верхнее меню
    let top_menu = render_main_menu_aw(app);

    // Верхний тулбар
    let top_toolbar = render_top_toolbar(app.get_ui_style());

    // На базе PaneGrid собираем основную часть главного окна 'body'
    let body_grid_row = pane_grid(
        &app.get_state().panes,
        |_pane_id, pane_state, _is_maximized| {
            match *pane_state {
                Pane::Toolbox => pane_grid::Content::new(render_sidebar(&app)),
                Pane::Canvas => {
                    // Собираем панель холста
                    let mut app_column = column![]
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .spacing(0.0);

                    // Build project widget blueprints
                    for (id, blueprint_arc) in app.get_factory().blueprints_iter() {
                        let parent_id: String =
                            app.get_factory().get(id, PROP_PARENT).unwrap_or_default();
                        if parent_id.is_empty() || parent_id == "root" {
                            app_column = app_column.push(blueprint_arc.build_element(
                                &app.factory,
                                app.get_state().selected_widget_id.as_deref(),
                            ));
                        }
                    }

                    let interactive_canvas = mouse_area(app_column)
                        .on_press(Message::MenuEvent(MenuAction::ClearSelection));

                    //let light_palette_only = UiPalette::light();

                    // ---------------------------------------------------------
                    // Центральный канвас
                    // ---------------------------------------------------------
                    let center_app_panel = 
                        render_style::render_panel_frame(

                            container(interactive_canvas)
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .padding(4.0)
                                // >>> Используем прозрачный стиль для контейнера канваса <<<
                                .style(|theme| {
                                    let base_style = container::transparent(&iced::Theme::Light);
                                    base_style                                
                                }).into(),

                                app.get_ui_style()
                        );

                    pane_grid::Content::new(
                        //center_app_panel
                        // Для центральной панели используем системную палитру iced::Theme::Light
                        themer(Some(iced::Theme::Light), center_app_panel)
                    )
                }
                Pane::Inspector => {
                    // Вложенный PaneGrid: сверху — Props (инспектор), снизу — Layers (дерево)
                    let nested =
                        pane_grid(&app.get_state().inspector_panes, |_nid, nstate, _nmax| {
                            match *nstate {
                                InspectorPane::Props => pane_grid::Content::new(
                                    // Отрисовка только верхней части инспектора (свойства)
                                    render_style::render_panel_frame(
                                        container(render_inspector(&app))
                                            .width(Length::Fill)
                                            .height(Length::Fill)
                                            .into(),
                                        app.get_ui_style()
                                    )
                                ),
                                InspectorPane::Layers => pane_grid::Content::new(
                                    // Перенесенное дерево слоев в отдельную панель
                                    render_style::render_panel_frame(
                                        container(crate::ui::inspector_panel_tree::render_layers_tree(
                                            &app,
                                        ))
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .into(),
                                        app.get_ui_style()
                                    ),
                                ),
                            }
                        })
                        .width(Length::Fill)
                        .height(Length::Fill)
                        // Вложенные события должны отправлять отдельные сообщения
                        .on_drag(Message::InspectorPaneDragged)
                        .on_resize(6, Message::InspectorPaneResized);

                    pane_grid::Content::new(nested)
                }
            }
        },
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .on_drag(Message::PaneDragged)
    .on_resize(6, Message::PaneResized);

    // Оборачиваем в стек для реализации всплывающих диалогов и модальных окон поверх холста
    let body_stack_row = stack![body_grid_row]
        .width(Length::Fill)
        .height(Length::Fill);

    // Оборачиваем body в контейнер и задаем главную тему body приложения
    let body_row = container(body_stack_row)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme: &Theme| {
            let mut base_style = container::transparent(_theme);
            // Задаем фон body-зоны приложения
            //base_style.background = Some(iced::Background::Color(palette.bg_panel));
            base_style.background = Some(iced::Background::Color(palette.bg_color));
            base_style
        });
    
    // Собираем всё приложение целиком: Меню сверху, всё остальное под ним
    let main_application_layout = column![
        top_menu, // Всегда прижато к верху экрана
        top_toolbar,
        body_row, // Занимает всё оставшееся пространство снизу
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    // Слои главного окна
    let mut main_application_layers = stack![main_application_layout]
        .width(Length::Fill)
        .height(Length::Fill);

    // Добавляем overlay окна диалогов
    if app.overlay_operation.clone() != OverlayAction::NoOp {
        main_application_layers = main_application_layers.push(overlays::render_overlay(app));
    }

    // Возвращаем собранную структуру главного окна приложения
    main_application_layers.into()
}
