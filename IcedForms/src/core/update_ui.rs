// -----------------------------------------------------------------------------
// Модуль update
// Содержит реализацию обработчиков для событий прилождения
// -----------------------------------------------------------------------------
//use log::{info, warn, error};

pub use crate::app::App;
pub use crate::core::*;
//pub use crate::core::os_dialogs::*;

// -----------------------------------------------------------------------------
// Диспетчеры обработки сообщений UI
// -----------------------------------------------------------------------------

pub fn handle_menu_action(action: MenuAction, app: &mut App) -> iced::Task<Message> {
    // Пишем логирование меню
    log::info!(
        "handle_menu_action: Обработка события MenuAction: {:?}",
        action
    );

    let task_to_run = match action {
        // Запрос создания нового проекта
        MenuAction::NewProject => {
            return on_new_project();
        }
        // Обработка подтверждения создания нового проекта
        MenuAction::NewProjectConfirmed(is_confirmed) => {
            do_new_project(is_confirmed, app);
            iced::Task::done(Message::OverlayEvent(OverlayAction::CloseOverlay))
        }
        // Запрос чтения проекта
        MenuAction::OpenProject => {
            return on_open_project();
        }
        // Обработка подтверждения чтения проекта
        MenuAction::OpenProjectConfirmed(opt_path) => {
            do_open_project(app, opt_path);
            iced::Task::none()
        }
        // Зарпос чтения проекта
        MenuAction::SaveProject => {
            return on_save_project();
        }
        // Обработка подтверждения чтения проекта
        MenuAction::SaveProjectConfirmed(opt_path) => {
            do_save_project(app, opt_path);
            iced::Task::none()
        }
        // Удалить все виджеты
        // Отличается от NewProject сохранением счетчика
        // ??? Х3 зачем нужно, возможно в будущем пригодится ???
        MenuAction::ClearCanvas => {
            return on_clear_canvas();
        }
        MenuAction::ClearCanvasConfirmed(is_confirmed) => {
            do_clear_canvas(is_confirmed, app);
            iced::Task::done(Message::OverlayEvent(OverlayAction::CloseOverlay))
        }
        // Запрос экспорта данных
        MenuAction::ExportProject => {
            return on_export_project();
        }
        // Обработка подтверждения экспорта данных
        MenuAction::ExportProjectConfirmed(opt_path) => {
            do_export_project(app, opt_path);
            iced::Task::none()
        }
        // Запрос выхода из приложения
        MenuAction::ExitApplication => {
            return on_exit();
        }
        // Обработка подтверждения выхода из приложения
        MenuAction::ExitApplicationConfirmed(is_confirmed) => {
            do_exit(is_confirmed);            
            iced::Task::done(Message::OverlayEvent(OverlayAction::CloseOverlay))
        }
        // Показать окно 'О приложении'
        MenuAction::ShowAbout => {
            return on_about();
        }

        // -------------------------------------------------------------------------
        // Операции с виджетами
        // -------------------------------------------------------------------------

        // Добавить новый виджет
        MenuAction::AddWidget(widget_type) => {
            on_add_widget(app, &widget_type);
            iced::Task::none()
        }
        // Удалить виджет
        MenuAction::DeleteWidget => {
            on_del_widget(app);
            iced::Task::none()
        }

        // Выбрать виджет
        MenuAction::SelectWidget(widget_id) => {
            on_select_widget(app, &widget_id);
            iced::Task::none()
        }
        // Выбрать свойство виджета
        MenuAction::SelectProperty(prop_key) => {
            on_select_property(app, prop_key);
            iced::Task::none()
        }

        MenuAction::MoveUpWidget => {
            on_moveup_widget(app);
            iced::Task::none()
        }
        MenuAction::MoveDownWidget => {
            on_movedown_widget(app);
            iced::Task::none()
        }


        // -------------------------------------------------------------------------
        // Управление работой интерфейса
        // -------------------------------------------------------------------------

        // Переключение режима работы
        MenuAction::ToggleDesignMode => {
            on_toggle_design_mode(app);
            iced::Task::none()
        }

        // Переключение отображаемой темы
        MenuAction::ToggleViewTheme => {
            on_toggle_view_theme(app);
            iced::Task::none()
        }

        // Переключение отображаемой темы
        MenuAction::ClearSelection => {
            on_clear_selection(app);
            iced::Task::none()
        }

        MenuAction::NoOp => {
            log::info!(
                "handle_menu_action: Выполнено необрабатываемое событие 'MenuAction::NoOp'. Данное событие ничего не обрабатывает и только выводит данное сообщение."
            );
            //return iced::Task::done(Message::MouseMoved {
            //iced::Point::new(100.0, 200.0);
            //});
            //iced::widget::operation::move_cursor_to::<Message>(iced::widget::Id::unique(), 0);
            iced::Task::none()
        }
        _ => {
            log::error!(
                "handle_menu_action: Выполнено необрабатываемое событие 'MenuAction::_'. При нормальной работе программы данное сообщение не появляется. Проверьте возможные источники события 'MenuAction::NoOp'"
            );
            iced::Task::none()
        }
    };

    task_to_run
}

// -----------------------------------------------------------------------------
// Обработчики событий
// -----------------------------------------------------------------------------

// Создать новый проект
fn on_new_project() -> iced::Task<Message> {
    log::info!("Запрос на создание нового проекта. Вызов асинхронного диалога...");
    // Передаем рантайму Iced задачу: вызвать окно подтверждения в правильном потоке,
    // а логический результат (true/false) вернуть в сообщение NewProjectConfirmed
    // return iced::Task::perform(
    //     async {
    //         crate::core::os_dialogs::show_confirm_box(
    //             "Создание нового проекта",
    //             "Вы уверены, что хотите создать новый проект?\nВсе несохраненные изменения будут утеряны.",
    //         )
    //     },
    //     |was_confirmed| Message::MenuEvent(MenuAction::NewProjectConfirmed(was_confirmed)),
    // );
    return iced::Task::done(Message::OverlayEvent(OverlayAction::OpenDialog(DialogType::NewProject)));
}
fn do_new_project(is_confirmed: bool, app: &mut App) {
    // Эта часть кода использовалась для работы системных диалогов
    // Убрана по причине введения оверлей-диалогов (системные в Linux требуют отдельных библиотек - инннахрен)
    // if is_confirmed {
    //     log::info!("Создание нового проекта. Выполнена полная очистка холста...");

    //     // Стираем все виджеты из фабрики (IndexMap и field_values)
    //     app.clear_all();
    // } else {
    //     log::info!("Создание нового проекта отменено.");
    // }    

    log::info!("Создание нового проекта. Выполнена полная очистка холста...");

    // Стираем все виджеты из фабрики (IndexMap и field_values)
    app.clear_all();
}

// Прочитать проект
fn on_open_project() -> iced::Task<Message> {
    log::info!("Запрос на открытие сохраненного проекта. Вызов асинхронного диалога...");
    //self.active_menu = None; // Закрываем оверлеи меню

    // Передаем рантайму Iced задачу: выполнить функцию load_project_from_json в фоне,
    // а результат (Option<Factory>) завернуть в сообщение Message::ProjectLoaded

    return iced::Task::perform(
        async {
            crate::core::os_dialogs::show_open_dialog("Открыть проект конструктора", "", "json")
        },
        |selected_path| Message::MenuEvent(MenuAction::OpenProjectConfirmed(selected_path)),
    );

    //iced::Task::none()
}
fn do_open_project(app: &mut App, path: Option<std::path::PathBuf>) {
    log::info!("do_open_project: Обработка открытия сохраненного проекта.");
    if let Some(path) = path {
        log::info!(
            "do_open_project: Получен валидный путь для чтения: {:?}",
            path
        );
        data_io::load_project_from_json(app, &path);
    } else {
        // Ветка None: Сюда мы попадаем строго тогда, когда пользователь нажал "Отмена"
        log::warn!("do_open_project: Чтение проекта отменена пользователем.");
    }
}

// Сохранить проект
fn on_save_project() -> iced::Task<Message> {
    log::info!("Запрос на сохранение проекта. Вызов асинхронного диалога...");
    //self.active_menu = None; // Закрываем оверлеи меню

    // Передаем рантайму Iced задачу: выполнить функцию load_project_from_json в фоне,
    // а результат (Option<Factory>) завернуть в сообщение Message::ProjectLoaded

    return iced::Task::perform(
        async {
            crate::core::os_dialogs::show_save_dialog("Сохранить проект конструктора", "", "json")
        },
        |selected_path| Message::MenuEvent(MenuAction::SaveProjectConfirmed(selected_path)),
    );

    //iced::Task::none()    //iced::Task::none()
}
fn do_save_project(app: &mut App, path: Option<std::path::PathBuf>) {
    log::info!("Обработка сохранения проекта.");
    if let Some(path) = path {
        log::info!(
            "do_save_project: Получен валидный путь для записи: {:?}",
            path
        );
        data_io::save_project_to_json(app, &path);
    } else {
        // Ветка None: Сюда мы попадаем строго тогда, когда пользователь нажал "Отмена"
        log::warn!("do_save_project: Запись проекта отменена пользователем.");
    }
}

// Очистить canvas (очистить виджеты)
fn on_clear_canvas() -> iced::Task<Message> {
    log::info!("Запрос на удаление всех виджетов. Вызов асинхронного диалога...");

    // Передаем рантайму Iced задачу: вызвать окно подтверждения в правильном потоке,
    // а логический результат (true/false) вернуть в сообщение NewProjectConfirmed
    //return iced::Task::perform(
    //    async {

            //crate::core::os_dialogs::show_confirm_box(
            //    "Очистка проекта",
            //    "Вы уверены, что хотите удалить все элементы?\nВсе несохраненные изменения будут утеряны.",
            //)
    //    },
    //    |was_confirmed| Message::MenuEvent(MenuAction::ClearCanvasConfirmed(was_confirmed)),
    //);
    return iced::Task::done(Message::OverlayEvent(OverlayAction::OpenDialog(DialogType::ClearProject)));
    //return iced::Task::none();

}
fn do_clear_canvas(is_confirmed: bool, app: &mut App) {
    // if is_confirmed {
    //     log::info!("Очистка холста. Выполнено удаление всех виджетов...");

    //     // Стираем все виджеты из фабрики (IndexMap и field_values)
    //     app.clear_all();
    // } else {
    //     log::info!("Очистка холста отменена.");
    // }

    log::info!("Очистка холста. Выполнено удаление всех виджетов...");

    // Стираем все виджеты из фабрики (IndexMap и field_values)
    app.clear_all();
}

// Экспорт проекта
fn on_export_project() -> iced::Task<Message> {
    log::info!("Запрос на Экспорт проекта. Вызов асинхронного диалога...");

    return iced::Task::perform(
        async {
            crate::core::os_dialogs::show_save_dialog("Сохранить файл экспорта проекта", "", "json")
        },
        |selected_path| Message::MenuEvent(MenuAction::ExportProjectConfirmed(selected_path)),
    );
    //iced::Task::none()
}
fn do_export_project(app: &mut App, path: Option<std::path::PathBuf>) {
    log::info!("Обработка экспорта проекта.");
    if let Some(path) = path {
        println!("-----------------------------");
        log::info!(
            "do_export_project: Получен валидный путь для записи: {:?}",
            path
        );
        data_io::export_project_to_json(app, &path);
    } else {
        // Ветка None: Сюда мы попадаем строго тогда, когда пользователь нажал "Отмена"
        log::warn!("do_export_project: Экспорт проекта отменена пользователем.");
    }
}

// Выйти из приложения
fn on_exit() -> iced::Task<Message> {
    //use iced::advanced::widget::operation::focusable::unfocus;

    log::info!("Запрос на выход из приложения.");

    return iced::Task::done(Message::OverlayEvent(OverlayAction::OpenDialog(DialogType::Exit)));
    //return iced::Task::none();
}
fn do_exit(_is_confirmed: bool) {
    // СЮДА ПРИЛЕТАЕТ ОТВЕТ ИЗ СИСТЕМНОГО ОКНА ДА/НЕТ ДЛЯ ВЫХОДА
    // if is_confirmed {
    //     log::info!("Выход подтвержден. Завершение работы процесса...");
    //     // Корректное завершение работы приложения
    //     std::process::exit(0);
    // } else {
    //     log::info!("Выход отменен пользователем. Возврат в конструктор.");
    // }

    log::info!("Выход подтвержден. Завершение работы процесса...");

    // Корректное завершение работы приложения
    std::process::exit(0);

}

// Показать диалог 'О программе'
fn on_about() -> iced::Task<Message> {
    log::info!("Отображение диалога 'О программе'...");

    return iced::Task::done(
        Message::OverlayEvent(OverlayAction::OpenDialog(DialogType::About))
    );
}

// Добавить виджетт
fn on_add_widget(app: &mut App, widget_type: &str) {
    // Вызываем логику приложения, передавая тип и тип добавляемого виджета
    let _new_id = app.add_widget(widget_type);
}

// Удалить виджет
fn on_del_widget(app: &mut App) {
    // Вызываем логику приложения для удаления текущего виджета
    let _new_id = app.delete_widget();
}

// Выбрать виджетт
fn on_select_widget(app: &mut App, widget_id: &str) {
    app.select_widget(String::from(widget_id));
}

// Выбрать свойство
fn on_select_property(app: &mut App, prop_key: PropertyKey) {
    app.select_property(prop_key);
}

fn on_moveup_widget(app: &mut App) {
    app.moveup_widget();
}

fn on_movedown_widget(app: &mut App) {
    app.movedown_widget();
}

// -------------------------------------------------------------------------
// Управление работой интерфейса
// -------------------------------------------------------------------------

// Переключение режима работы
fn on_toggle_design_mode(app: &mut App) {
    app.toggle_design_mode();
}

// Переключение отображаемой темы
fn on_toggle_view_theme(app: &mut App) {
    app.toggle_view_theme();
}

// Снять выделение элемента
fn on_clear_selection(app: &mut App) {
    app.clear_selection();
}

// -------------------------------------------------------------------------
// Управление работой оверлеев и модальных окон
// -------------------------------------------------------------------------
//fn on_close_overlay(app: &mut App) {
//    app.close_overlay();
//}