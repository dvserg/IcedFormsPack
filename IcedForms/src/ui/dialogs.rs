// -----------------------------------------------------------------------------
// Модуль dialogs
// Содержит реализацию вызова overlay диалогов
// -----------------------------------------------------------------------------
use iced::{Element, Theme, Length};
use iced::widget::{text, scrollable};

use crate::core::Message;
use crate::ui::*;
use crate::app::App;


// Создание диалога "Settings"
pub fn render_dialog_settings<'a>(
    _app: &'a App,
) -> Element<'a, Message, Theme> {

    let content = text("Settings");

    // Передаем контент в шаблон диалогового окна
    render_dialog_frame(
        _app,
        "Iced Forms", content.into(), 
        vec![]
    )
}

// Создание диалога "Code"
pub fn render_dialog_code<'a>(
    _app:    &'a App,
//    content: &str,
) -> Element<'a, Message, Theme> {

    // Получаем изменяемую ссылку
    let content_code: &mut String = unsafe { &mut *_app.overlay_content_code.get() };
    
    // Проверяем и генерируем текст в один проход
    if content_code.is_empty() {
        *content_code = crate::core::codegen::generate_widget_tree_code(_app);
    }

    let content = scrollable(text(& *content_code).size(13.0))
        .width(Length::Fill)
        .height(Length::Fill);

    // Передаем контент в шаблон диалогового окна
    render_dialog_frame(
        _app,
        "Iced Forms code </>", 
        content.into(), 
        vec![]
    )
}
