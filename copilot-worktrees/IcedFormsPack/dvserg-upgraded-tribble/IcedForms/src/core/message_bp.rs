// -----------------------------------------------------------------------------
// Модуль 'message_bp'
// Содержит структуру событий (event, action) виджетов
// -----------------------------------------------------------------------------
pub use std::rc::Rc;

pub use crate::app;
pub use crate::blueprints;
pub use crate::core;


#[derive(Debug, Clone)]
pub enum WidgetAction {
    /// Клик по интерактивной кнопке. Передает ID этой кнопки.
    ButtonClicked {
        widget_id: String,
    },

    /// Изменение текста пользователем (для TextInput / TextArea).
    /// Передает ID элемента и то, что ввёл пользователь.
    TextChanged {
        widget_id: String,
        text_editor_action: iced::widget::text_editor::Action,
    },

    /// Выбор элемента в выпадающем списке (ComboBox).
    /// Передает ID элемента и индекс выбранной строки.
    ComboSelected {
        widget_id: String,
        selected_index: usize,
    },

    /// Клик по галочке (Checkbox). Передает ID и новое состояние.
    CheckboxToggled {
        widget_id: String,
        is_checked: bool,
    },

    /// Обработка кликов по ссылкам в RichText
    //LinkWithIdClicked(u32),

    Markdown {
        widget_id: String,
        action: MarkdownEdit,
    },

    RichText {
        widget_id: String,
        action: RichTextEdit,
    },

    // Пустая заглушка служит для временного перекрытия событий. Выводит сообщение в лог
    NoOp,
}

// ** Удалить?

#[derive(Debug, Clone)]
pub enum OverlayEditorAction {
    /*
    Markdown{
        id: String,
        edit: MarkdownEdit,
    },
    */
    NoOp,
}


// События редактора Markdown
#[derive(Debug, Clone)]
pub enum MarkdownEdit {
    FormatBold,
    FormatItalic,
    FormatH1,
    FormatH2,
    FormatH3,
    FormatCode,
    FormatStrikethrough,
    FormatBlockquote,
    FormatList,
    FormatOrderedList,
}

// События редактора RichText
#[derive(Debug, Clone)]
pub enum RichTextEdit {
    InsertClipboardRTF,
    ApplyChanges,
    Clear,
}

// События для обработки интерактивных сложных виджетов типа TextEditor
impl crate::app::App {
    pub fn handle_widget_action(
        &mut self,
        widget_id: String,
        widget_action: WidgetAction,
    //    app: &mut app::App,
    ) -> iced::Task<core::Message> {
        
        // Пишем логирование
        log::info!("handle_widget_action: Обработка события WidgetAction: {:?}", widget_action);

        let widget_action_cl = widget_action.clone();        

        /*
        let task_to_run = match widget_action {
            WidgetAction::TextChanged { ref widget_id, .. } => {

                //let id = widget_id.clone();

                // Извлекаем блупринт из фабрики для обхода иммутабельности Rc
                // (внутри фабрики теперь сидит Dummy)
                if let Some(mut bp_rc) = self.factory.take_blueprint_for_event(widget_id) {
    
                    // Получаем &mut доступ к блупринту. 
                    let bp_mut = Rc::get_mut(&mut bp_rc).expect(
                        "КРИТИЧЕСКАЯ ОШИБКА: Указатель Rc не уникален!",
                    );

                    // Вызываем handle_event
                    let final_task = bp_mut.handle_event(&widget_action_cl, self);

                    // Возвращаем оригинальный блупринт на его законное место в фабрику
                    self.factory.put_blueprint_back(widget_id, bp_rc);

                    return final_task;
                }
 
                iced::Task::none()
            }

            WidgetAction::CheckboxToggled {
                widget_id,
                is_checked,
            } => {
                log::info!(
                    "handle_widget_action: Выполнена обработка события WidgetAction::CheckboxToggled: {} = {:?}",
                    widget_id,
                    is_checked
                );
                log::info!("*** Еще не реализована ***");
                iced::Task::none()
            }


                WidgetAction::Markdown { ref widget_id, .. } => {

                //let id = widget_id.clone();

                // Извлекаем блупринт из фабрики для обхода иммутабельности Rc
                // (внутри фабрики теперь сидит Dummy)
                if let Some(mut bp_rc) = self.factory.take_blueprint_for_event(widget_id) {
    
                    // Получаем &mut доступ к блупринту. 
                    let bp_mut = Rc::get_mut(&mut bp_rc).expect(
                        "КРИТИЧЕСКАЯ ОШИБКА: Указатель Rc не уникален!",
                    );

                    // Вызываем handle_event
                    let final_task = bp_mut.handle_event(&widget_action_cl, self);

                    // Возвращаем оригинальный блупринт на его законное место в фабрику
                    self.factory.put_blueprint_back(widget_id, bp_rc);

                    return final_task;
                } 

                iced::Task::none()
            }

            // Пустая заглушка служит для временного перекрытия событий. Выводит сообщение в лог
            WidgetAction::NoOp | _ => {
                log::error!(
                    r#"handle_widget_action: Выполнено необрабатываемое событие 'WidgetAction': {:?}. 
                    При нормальной работе программы данное сообщение не появляется. 
                    Проверьте возможные источники события."#,
                    widget_action
                );
                iced::Task::none()
            }
        };
        */
        
        // Передаем обработку события в handle конкретного виджета с указанным ID
        //  Виджет вынимаем, оставляя взамен болванку, вызываем мутабельно обработчик, 
        //  и возвращаем виджет на место
        if let Some(mut bp_rc) = self.factory.take_blueprint_for_event(&widget_id) {
    
            // Получаем &mut доступ к блупринту. 
            let bp_mut = Rc::get_mut(&mut bp_rc).expect(
                "КРИТИЧЕСКАЯ ОШИБКА: Указатель Rc не уникален!",
            );

            // Вызываем handle_event
            let final_task = bp_mut.handle_event(&widget_action_cl, self);

            // Возвращаем оригинальный блупринт на его законное место в фабрику
            self.factory.put_blueprint_back(&widget_id, bp_rc);

            return final_task;
        }

        iced::Task::none()
    }

}