// -----------------------------------------------------------------------------
// Модуль overlay
// Содержит реализацию всплывающих overlay-диалогов приложения
// -----------------------------------------------------------------------------

use iced::widget::{column, container, text, mouse_area, row, space, button, rule};
use iced::{Color, Element, Length, Padding, Alignment, Theme};
use iced::mouse::Interaction;        

use crate::core::{Message, OverlayAction, MenuAction};
//use crate::ui::{uitheme};
use crate::ui::*;
use crate::app::App;



//------------------------------------------------------------------------------ 
// Тексты сообщений
//------------------------------------------------------------------------------ 
pub const TEXT_DIALOG_ABOUT: &'static str =
"Визуальный Конструктор GUI\n\
Версия 1.0.0 (Асинхронное ядро Iced 0.14)\n\n\
Среда для быстрого прототипирования, \
иерархического экспорта в JSON и визуальной настройки чертежей виджетов.";

pub const TEXT_DIALOG_NEW_PROJECT: &'static str =
"Вы уверены, что хотите создать новый проект?\n\
Все несохраненные данные будут утеряны.";

pub const TEXT_DIALOG_CLEAR_PROJECT: &'static str =
"Вы уверены, что хотите удалить все элементы?\n\
Все несохраненные данные будут утеряны.";

pub const TEXT_DIALOG_EXIT: &'static str =
"Вы уверены, что хотите выйти из програмы?\n\
Все несохраненные данные будут утеряны.";

//------------------------------------------------------------------------------ 
// Создание overlay-диалогов
pub fn render_overlay<'a>(
    app: &'a App,
) -> Element<'a, Message, Theme> {
    match &app.overlay_operation {
        OverlayAction::OpenWidgetEditor(widget_id) => {
            let overlay_content = render_widget_editor(&app, widget_id.clone());                
            return wrap_in_modal(overlay_content, 8, 0.0, 0.0);
        }
        OverlayAction::OpenDialog(dialog_type) => {
            use crate::core::message;
            match dialog_type {
                message::DialogType::NewProject => {
                    // Устанавливаем абсолютные размеры окна диалога 300x400
                    return wrap_in_modal(render_dialog_new_project(&app), 0, 400.0, 300.0);
                },
                message::DialogType::ClearProject => {
                    // Устанавливаем абсолютные размеры окна диалога 300x400
                    return wrap_in_modal(render_dialog_clear_project(&app), 0, 400.0, 300.0);
                },
                message::DialogType::Exit => {
                    // Устанавливаем абсолютные размеры окна диалога 300x400
                    return wrap_in_modal(render_dialog_exit_app(&app), 0, 400.0, 300.0);
                },
                message::DialogType::Info => {

                },
                message::DialogType::About => {
                    // Устанавливаем абсолютные размеры окна диалога 300x400
                    return wrap_in_modal(render_dialog_about(&app), 0, 400.0, 300.0);
                },
                message::DialogType::Settings => {
                    return wrap_in_modal(render_dialog_code(&app), 0, 400.0, 300.0);
                },
                message::DialogType::TreeCode => {
                    // Устанавливаем парциальные размеры окна диалога 5/10 частей
                    return wrap_in_modal(render_dialog_code(&app), 5, 0.0, 0.0);
                },
                message::DialogType::WidgetCode => {
                    
                },
            }
        }
        _ => {}
    }

    return wrap_in_modal(text("Нет активного оверлея").into(), 1, 0.0, 0.0);
}

//------------------------------------------------------------------------------ 
// Диалоги
//------------------------------------------------------------------------------ 
// Создание диалога "О программе"
pub fn render_dialog_about<'a>(
    _app: &'a App,
) -> Element<'a, Message, Theme> {
    return render_dialog_info(_app, crate::ui::ICON_INFO, "Iced Forms", TEXT_DIALOG_ABOUT);
}

// Создание диалога "Новый проект"
pub fn render_dialog_new_project<'a>(
    _app: &'a App,
) -> Element<'a, Message, Theme> {
    return render_dialog_confirm (
        _app, crate::ui::ICON_WARN, "Создание нового проекта", TEXT_DIALOG_NEW_PROJECT,
        Message::MenuEvent(MenuAction::NewProjectConfirmed(true)),
        Message::OverlayEvent(OverlayAction::CloseOverlay),
    );
}

// Создание диалога "Очистить проект"
pub fn render_dialog_clear_project<'a>(
    _app: &'a App,
) -> Element<'a, Message, Theme> {
    return render_dialog_confirm (
        _app, crate::ui::ICON_WARN, "Очистка проекта", TEXT_DIALOG_CLEAR_PROJECT,
        Message::MenuEvent(MenuAction::ClearCanvasConfirmed(true)),
        Message::OverlayEvent(OverlayAction::CloseOverlay),
    );
}

// Создание диалога "Выйти из программы"
pub fn render_dialog_exit_app<'a>(
    _app: &'a App,
) -> Element<'a, Message, Theme> {
    return render_dialog_confirm (
        _app, crate::ui::ICON_WARN, "Выход из программы",TEXT_DIALOG_EXIT,
        Message::MenuEvent(MenuAction::ExitApplicationConfirmed(true)),
        Message::OverlayEvent(OverlayAction::CloseOverlay),
    );
}

// Создание диалога редактора виджета
pub fn render_widget_editor<'a>(
    app: &'a App,
    widget_id: String,
) -> Element<'a, Message, Theme> {
    let factory = app.get_factory();

    // Получаем контент окна редактирования виджета
    let content: Element<'a, Message, Theme> = if let Some(blueprint) = factory.get_blueprint_rc(widget_id.to_string()) { 
        blueprint.build_editor_content(&app.get_factory()).into() 
    } else { 
        text(format!("Редактор виджета {} не найден.", widget_id)).into() 
    };

    // Передаем контент в шаблон диалогового окна
    render_dialog_frame(app, "Редактор виджета", content, vec![])
}

//------------------------------------------------------------------------------ 
// Шаблоны
//------------------------------------------------------------------------------ 

// Шаблон диалога подтверждения действия
fn render_dialog_confirm<'a>(
    _app:  &'a App,
    icon:  &'a str,
    title: &'a str,
    dialog_text: &'a str,

    ok_message:     Message,
    cancel_message: Message,
) -> Element<'a, Message, Theme> {
    // Получаем шрифт для иконки
    let font = FONT_MATERIAL;

    // Получаем текущую палитру приложения
    let palette = UiPalette::get_style_palette(_app.get_ui_style());

    // Отступы от границ по внешнему контуру диалога и между заголовком и телом
    let dialog_border_padding = 4.0;

    // let content = container(text(dialog_text))
    //         .width(Length::Fill)
    //         .padding(Padding::from([0.0, 8.0]));

    // Контент диалоговой формы
    let content = row![
        container(text(icon).font(font).size(48.0))
            .width(Length::Shrink)
            .height(Length::Shrink)
            .align_y(Alignment::Start)
            .align_x(Alignment::Start),
        container(text(dialog_text))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from([0.0, 8.0]))
            .align_y(Alignment::Start)
            .align_x(Alignment::Start),
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    // Создаем тело модального окна    
    let modal_box = container(column![content])
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding::from([dialog_border_padding, dialog_border_padding]))
        .style(uitheme::container_panel_style(palette));

    // Передаем контент в шаблон диалогового окна
    render_dialog_frame(
        _app, title, modal_box.into(),
        vec![("ОК", ok_message), ("Отмена", cancel_message)]
    )
}

// Шаблон диалога с заголовком, иконкой и сообщением, без кнопок
pub fn render_dialog_info<'a>(
    _app:    &'a App,
    icon:    &'a str,
    title:   &'a str,
    message: &'a str,
) -> Element<'a, Message, Theme> {

    let font = FONT_MATERIAL;

    // Контент диалоговой формы
    let content = row![
        container(text(icon).font(font).size(48.0))
            .width(Length::Shrink)
            .height(Length::Shrink)
            .align_y(Alignment::Start)
            .align_x(Alignment::Start),
        container(text(message))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from([0.0, 8.0]))
            .align_y(Alignment::Start)
            .align_x(Alignment::Start),
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    // Передаем контент в шаблон диалогового окна
    render_dialog_frame(_app, title, content.into(), vec![])
}

// Шаблон рамки диалога
pub fn render_dialog_frame<'a>(
    app:     &'a App,
    title:   &'a str,
    content: Element<'a, Message, Theme>,
    buttons: Vec<(&'a str, Message)>,
) -> Element<'a, Message, Theme> {
    // Отступы от границ по внешнему контуру диалога и между заголовком и телом
    let dialog_border_padding = 4.0;

    // Создаем заголовок диалога
    let header_row = row![
        // Название диалога
        container(text(title))
            .width(Length::Fill)
            .padding(Padding::from([0.0, 8.0])),

        // Кнопка закрытия диалога
        button(uitheme::ICON_OVERLAY_CLOSE)
            .padding(Padding::from(4.0))
            .on_press(Message::OverlayEvent(OverlayAction::CloseOverlay)),
    ]
    .align_y(Alignment::Center)
    // Отступ между заголовком и телом диалога
    .padding(Padding{top: 0.0, right: 0.0, bottom: dialog_border_padding, left: 0.0});  

    let mut body = column![header_row, rule::horizontal(2.0), content];

    // Создаем нижний бар с кнопками
    if !buttons.is_empty() {

        let mut footer = row![space::horizontal()]
            .padding(Padding::from([dialog_border_padding, 0.0]))
            .spacing(10); // Добавим отступ между кнопками

        // Пушим кнопки
        for (name, msg) in buttons {
            footer = footer.push(
                button(text(name).align_x(Alignment::Center))
                    .width(80.0)
                    .style(uitheme::style_toolbar_button(app.get_ui_style()))
                    .on_press(msg)
            );
        }

        body = body.push(footer);
    }    

    // Создаем тело модального окна    
    let body_box = container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding::from([dialog_border_padding, dialog_border_padding]))
        .style(|_theme: &Theme| {
            let palette = _theme.extended_palette();

            container::Style {
                background: Some(palette.background.base.color.into()),
                border: iced::Border {
                    color: palette.background.strong.color,                    
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        });

    body_box.into()
}

// Функция обертки содержимого редактора в модальное окно с затемнением фона
// Передаем контент и пропорции окна диалога
pub fn wrap_in_modal<'a>(
    content:     Element<'a, Message, Theme>,
    size_weight: u16,   // Пропорциональные размеры окна
    size_width:  f32,   // Фиксированные размеры окна
    size_height: f32,   // при условии 'size_weight = 0'
) -> Element<'a, Message, Theme> {
    // Вычисляем размеры окна оверлея
    let frame_width  = if size_weight != 0 { Length::FillPortion(size_weight) } else { Length::Fixed(size_width) };
    let frame_height = if size_weight != 0 { Length::FillPortion(size_weight) } else { Length::Fixed(size_height) };

    // ЗАДАЕМ ПРОПОРЦИИ ПО ШИРИНЕ (Горизонтальный ряд)
    let horizontal_row = row![
        space::horizontal().width(Length::FillPortion(1)),     // Левая распорка (%)
        container(content /*modal_box*/).width(frame_width),   // Диалог (%/Fixed)
        space::horizontal().width(Length::FillPortion(1)),     // Правая распорка (%)
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    // ЗАДАЕМ ПРОПОРЦИИ ПО ВЫСОТЕ (Вертикальная колонка)
    let proportional_layout = column![
        space::vertical().height(Length::FillPortion(1)),       // Верхняя распорка (%)
        container(horizontal_row).height(frame_height),         // Ряд с окном (%/Fixed)
        space::vertical().height(Length::FillPortion(1)),       // Нижняя распорка (%)
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    // Помещаем разметку в полноэкранный контейнер с затемнением
    let outer_overlay = container(proportional_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.65).into()),
            ..Default::default()
        });

    // Перехватываем клики по всей площади оверлея
    mouse_area(outer_overlay)
        .on_press(Message::NoOp) 
        .interaction(Interaction::Idle) // Указатель мыши для правильной работы
        .into()
}

