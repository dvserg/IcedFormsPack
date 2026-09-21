// -----------------------------------------------------------------------------
// Модуль App
// Содержит реализацию основного процесса App Iced
// -----------------------------------------------------------------------------
use std::cell::{UnsafeCell};
use iced::Theme;

use crate::APP_TITLE;
use crate::core::*;
use crate::ui::*;

pub struct OverlayContentFn(pub Box<dyn Fn(&Factory) -> iced::Element<'static, Message, Theme>>);

// Вручную пишем Debug, чтобы Iced-структура App могла компилироваться с #[derive(Debug)]
impl std::fmt::Debug for OverlayContentFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("OverlayContentFn")
    }
}

// Состояние приложения
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Toolbox,
    Canvas,
    Inspector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectorPane {
    Props,
    Layers,
}

#[derive(Debug, Clone)]
pub struct DesignerState {
    // Текущий виджет
    pub selected_widget_id: Option<String>,

    // Текущее свойство
    pub selected_property_key: Option<PropertyKey>,

    // Стиль и тема интерфейса
    pub ui_style: UIStyle,

    // // Текущая цветовая тема
    // pub is_dark_theme: bool,
    // // Стиль отбражения интерфейса
    // pub ui_style: RenderStyle,

    // Состояние PaneGrid (расположение панелей)
    pub panes: iced::widget::pane_grid::State<Pane>,

    // Вложенное состояние PaneGrid для правой панели: Props (верх) / Layers (низ)
    pub inspector_panes: iced::widget::pane_grid::State<InspectorPane>,

    // Текущая ширина левой панели (sidebar) и правой панели (inspector) в пикселях.
    pub sidebar_width_px: f32,
    pub inspector_width_px: f32,

    // Последняя известная ширина окна (нужна для пересчёта пропорций)
    pub last_window_width: Option<u32>,

    // Шаблоны
    pub template_path: String, 
    // pub template_folders: Vec<String>,
    // pub template_exclude: Vec<String>,
    // Конфигурация шаблонов Tera
    //pub template_config:  String,


}

impl Default for DesignerState {
    fn default() -> Self {
        // Создаём конфигурацию PaneGrid с тремя колонками: Toolbox 20% | Canvas 60% | Inspector 20%
        use iced::widget::pane_grid::{Axis, Configuration};

        let config = Configuration::Split {
            axis: Axis::Vertical,
            ratio: 0.2, // left = 20%
            a: Box::new(Configuration::Pane(Pane::Toolbox)),
            b: Box::new(Configuration::Split {
                axis: Axis::Vertical,
                ratio: 0.75, // of remaining: canvas = 75% of right part -> 0.75 * 0.8 = 0.6 overall
                a: Box::new(Configuration::Pane(Pane::Canvas)),
                b: Box::new(Configuration::Pane(Pane::Inspector)),
            }),
        };

        let panes = iced::widget::pane_grid::State::with_configuration(config);

        // Вложенный PaneGrid для инспектора: верх — Props (2/3), низ — Layers (1/3)
        let inspector_config = Configuration::Split {
            axis: Axis::Horizontal, // горизонтальная ось — деление вдоль вертикали (top/bottom)
            ratio: 2.0 / 3.0,       // Props = 2/3 top, Layers = 1/3 bottom
            a: Box::new(Configuration::Pane(InspectorPane::Props)),
            b: Box::new(Configuration::Pane(InspectorPane::Layers)),
        };

        let inspector_panes = iced::widget::pane_grid::State::with_configuration(inspector_config);

        Self {
            selected_widget_id: None,
            selected_property_key: None,
            //is_dark_theme: false,
            ui_style: UIStyle::default(),
            panes,
            inspector_panes,
            // Изначальные фиксированные ширины — 170px для сайдбара и 280px для инспектора
            sidebar_width_px: 170.0,
            inspector_width_px: 280.0,
            last_window_width: None,

            template_path: String::from("./templates"),
        }
    }
}

// Главная форма приложения
#[derive(Debug, Default)]
pub struct App {
    // Фабрика элементов
    pub factory: Factory,

    // Состояние приложения
    pub state: DesignerState,

    // Всплывающий оверлей (для модальных окон и редакторов)
    pub overlay_operation:    OverlayAction,
    pub overlay_content_code: UnsafeCell<String>,             // Контент диалога 'Code'
}

// =====================================================================
// ЖЕСТКИЙ СИСТЕМНЫЙ АККОРД: Снимаем ошибку E0277 с метода iced::application!
// Говорим компилятору: "Мы гарантируем, что App будет жить в UI-потоке".
// =====================================================================
unsafe impl Send for App {}
unsafe impl Sync for App {}
// =====================================================================
impl App {
    pub fn from_config(&mut self) {
        let config = &APP_CONFIG.get().unwrap();

        // Апдейтим UI Style (Figma, VSCode, Blender)
        self.state.ui_style.render_style =  match config.theme_style.to_lowercase().as_str() {
            "figma"  => RenderStyle::Figma,
            "vscode" => RenderStyle::VSCode,
            _        => RenderStyle::Blender,
        };
    }

    // Возвращает фабрику
    pub fn get_factory(&self) -> &Factory {
        &self.factory
    }
    pub fn get_factory_mut(&mut self) -> &mut Factory {
        &mut self.factory
    }

    // Возвращает состояние App
    pub fn get_state(&self) -> &DesignerState {
        &self.state
    }
    
    // Инициализация главной формы
    pub fn init() -> (App, iced::Task<Message>) {
        let mut app = App::default();
        app.from_config();


        // // Запрашиваем уникальный ID текущего открывающегося окна
        // let main_window_id = iced::window::Id::unique();

        // let init_task = iced::window::run(main_window_id, |window_ref| {
        //     use raw_window_handle::{HasWindowHandle, HasDisplayHandle};

        //     log::info!("[INIT] Получение хэндла главного окна.");

        //     if let (Ok(win_handle), Ok(disp_handle)) = (
        //         window_ref.window_handle(),
        //         window_ref.display_handle()
        //     ) {
        //         // РЕШЕНИЕ ОШИБКИ: Используем "голый" unsafe для побитового копирования.
        //         // Мы берем ссылки на структуры сырых перечислений внутри win_handle и disp_handle 
        //         // и принудительно копируем их, полностью уничтожая привязку к лайфтайму '1.
        //         let raw_window = unsafe { 
        //             std::ptr::read(&win_handle.as_raw() as *const raw_window_handle::RawWindowHandle) 
        //         };
        //         let raw_display = unsafe { 
        //             std::ptr::read(&disp_handle.as_raw() as *const raw_window_handle::RawDisplayHandle) 
        //         };

        //         let app_window = crate::core::os_dialogs::AppWindow {
        //             window:  raw_window,
        //             display: raw_display,
        //         };              

        //         // Безопасно сохраняем в глобальный OnceLock
        //         let _ = crate::core::os_dialogs::PARENT_WINDOW.set(app_window);
        //         log::info!("[INIT] Получен хэндл главного окна!");
        //     } else {
        //         log::info!("[INIT] Хэндл главного окна НЕ получен!");
        //     }
        // })
        // .map(|_| Message::NoOp);

        (app, iced::Task::none())
        //(app, init_task)

    }

    // Обновление состояния формы
    #[allow(unreachable_patterns)]
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            // Сообщение обработки событий меню
            Message::MenuEvent(action) => {
                return update_ui::handle_menu_action(action, self);
            }

            // Сообщение обработки событий холста
            Message::WidgetEvent(id, action) => {
                //return core::handle_widget_action(action, self);
                return self.handle_widget_action(id, action);
            }

            // Сообщение апдейта типизированных свойств
            Message::UpdateProperty {
                widget_id,
                property_key,
                value,
            } => {
                update_property::handle_property_update(
                    &mut self.factory,
                    widget_id,
                    property_key,
                    value,
                );
            }

            // Глобальный перехват сообщений
            // Фильтрация и обработка выбранных сообщений
            Message::GlobalEvent(event) => {
                match event {
                    // Сообщения клавиатуры
                    iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                        key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab),
                        modifiers, // Можно проверить Shift+Tab через modifiers.shift()
                        ..
                    }) => {
                        if modifiers.shift() {
                            // Если зажат Shift, перемещаем фокус НАЗАД
                            return iced::widget::operation::focus_previous::<Message>();
                        } else {
                            return iced::widget::operation::focus_next::<Message>();
                        }
                    }

                    // Обработка изменения размера окна: пересчитываем относительные соотношения
                    iced::Event::Window(iced::window::Event::Resized(size)) => {
                        // Сохраняем последнюю ширину окна
                        let width = size.width as u32;
                        self.state.last_window_width = Some(width);

                        // Если у нас известны фиксированные пиксельные ширины — пересоберём конфигурацию PaneGrid
                        // так, чтобы sidebar и inspector занимали именно заданные пиксели, а центр растягивался.
                        let total_w = width as f32;

                        // Защита: минимально допустимая ширина окна
                        if total_w <= 0.0 {
                            return iced::Task::none();
                        }

                        // Ограничиваем фиксированные панели так, чтобы их сумма не превышала доступную ширину
                        let mut sidebar_w = self.state.sidebar_width_px;
                        let mut inspector_w = self.state.inspector_width_px;

                        if sidebar_w + inspector_w >= total_w {
                            // Если слишком много — уменьшаем инспектор, затем сайдбар
                            let excess = (sidebar_w + inspector_w) - total_w + 1.0; // +1 px запас
                            if inspector_w > excess {
                                inspector_w = (inspector_w - excess).max(80.0);
                            } else {
                                inspector_w = 80.0;
                                sidebar_w = (sidebar_w - (excess - inspector_w)).max(80.0);
                            }
                        }

                        // Вычисляем новые доли для PaneGrid (внешний split: sidebar | rest)
                        let left_ratio = (sidebar_w / total_w).clamp(0.05, 0.9);

                        // Для внутреннего split (canvas | inspector) нам нужен отношение canvas/(canvas+inspector)
                        let right_total = total_w - sidebar_w;
                        let inspector_ratio_of_right = if right_total > 0.0 {
                            let canvas_w = (total_w - sidebar_w - inspector_w).max(0.0);
                            (canvas_w / (canvas_w + inspector_w)).clamp(0.05, 0.95)
                        } else {
                            0.8
                        };

                        use iced::widget::pane_grid::{Axis, Configuration};

                        let config = Configuration::Split {
                            axis: Axis::Vertical,
                            ratio: left_ratio,
                            a: Box::new(Configuration::Pane(Pane::Toolbox)),
                            b: Box::new(Configuration::Split {
                                axis: Axis::Vertical,
                                ratio: inspector_ratio_of_right,
                                a: Box::new(Configuration::Pane(Pane::Canvas)),
                                b: Box::new(Configuration::Pane(Pane::Inspector)),
                            }),
                        };

                        self.state.panes =
                            iced::widget::pane_grid::State::with_configuration(config);

                        return iced::Task::none();
                    }

                    _ => {  }
                }
            }

            Message::WindowEvent(window_id, window_event ) => {

                match window_event {
                    iced::window::Event::Opened { .. } => {

                        // -----------------------------------------------------
                        // Перехват и сохранение хэндла окна для модальных диалогов
                        // -----------------------------------------------------
                        log::info!("[OS] Запускаем перехват хэндла...");

                        let _fn = iced::window::run(window_id, |window_ref| {
                            use raw_window_handle::{HasWindowHandle, HasDisplayHandle};

                            if let (Ok(win_handle), Ok(disp_handle)) = (
                                window_ref.window_handle(),
                                window_ref.display_handle()
                            ) {

                                // Стираем лайфтаймы через низкоуровневое побитовые копирование указателей
                                let raw_window = unsafe { 
                                    std::ptr::read(&win_handle.as_raw() as *const raw_window_handle::RawWindowHandle) 
                                };
                                let raw_display = unsafe { 
                                    std::ptr::read(&disp_handle.as_raw() as *const raw_window_handle::RawDisplayHandle) 
                                };

                                let app_window = crate::core::os_dialogs::AppWindow {
                                    window:  raw_window,
                                    display: raw_display,
                                };
                    
                                // Записываем в OnceLock для модальных диалогов
                                let _ = crate::core::os_dialogs::PARENT_WINDOW.set(app_window);
                                log::info!("[OS] Хэндл успешно сохранен в PARENT_WINDOW!");
                            }
                        })
                        .map(|_| Message::NoOp);

                        return _fn;
                        //return iced::Task::none();
                    }

                    _ => { }
                }
            },

            // Обработка событий PaneGrid (перетаскивание/изменение размера)
            Message::PaneDragged(ev) => {
                use iced::widget::pane_grid::DragEvent;
                match ev {
                    DragEvent::Dropped { pane, target } => {
                        // Выполним перенос/вставку панели в целевую позицию
                        self.state.panes.drop(pane, target);
                    }
                    DragEvent::Picked { .. } | DragEvent::Canceled { .. } => {
                        // Ничего не делаем для этих состояний
                    }
                }

                return iced::Task::none();
            }
            Message::PaneResized(ev) => {
                // Применяем новое соотношение сплита
                self.state.panes.resize(ev.split, ev.ratio);

                // Если у нас известна ширина окна — обновим фиксированные пиксели для sidebar/inspector
                if let Some(win_w) = self.state.last_window_width {
                    let total_w = win_w as f32;

                    // ev.split имеет тип pane_grid::Split — попытаемся разобрать его как внутреннюю структуру
                    // Обычно Split представляет собой tuple-struct содержащую индекс (usize).
                    // Split не дает публичного доступа к полям, извлечем индекс через Debug-строку
                    let split_idx_opt = {
                        let s = format!("{:?}", ev.split);
                        s.split('(')
                            .nth(1)
                            .and_then(|t| t.split(')').next())
                            .and_then(|n| n.parse::<usize>().ok())
                    };

                    if let Some(split_idx) = split_idx_opt {
                        if split_idx == 0 {
                            // внешний split: ev.ratio == sidebar / total
                            self.state.sidebar_width_px = (ev.ratio * total_w).max(80.0);
                        } else if split_idx == 1 {
                            // внутренний split: ev.ratio == canvas / (canvas + inspector)
                            // right_total = total_w - sidebar_width
                            let right_total = (total_w - self.state.sidebar_width_px).max(1.0);
                            let inspector_w = ((1.0 - ev.ratio) * right_total).max(80.0);
                            self.state.inspector_width_px = inspector_w;
                        }
                    }
                }

                return iced::Task::none();
            }
            // Вложенный PaneGrid инспектора — обработка перетаскивания
            Message::InspectorPaneDragged(ev) => {
                use iced::widget::pane_grid::DragEvent;
                match ev {
                    DragEvent::Dropped { pane, target } => {
                        self.state.inspector_panes.drop(pane, target);
                    }
                    DragEvent::Picked { .. } | DragEvent::Canceled { .. } => {}
                }
                return iced::Task::none();
            }
            // Вложенный PaneGrid инспектора — обработка изменения размера
            Message::InspectorPaneResized(ev) => {
                self.state.inspector_panes.resize(ev.split, ev.ratio);
                return iced::Task::none();
            }

            // События оверлеев и модальных окон
            Message::OverlayEvent(action) => {
                // Запускаем оверлей
                self.apply_overlay(action);
                return iced::Task::none();
            }
            Message::OverlayEditorEvent(_action) => {
                return iced::Task::none();
            }

            Message::NoOp => {
                log::error!("App::update: Выполнено пустое событие-заглушка 'Message::NoOp'.");
                return iced::Task::none();
            }
            // Пустая операция, ничего не выполняем
            _ => {
                log::error!(
                    "App::update: Выполнено необрабатываемое событие '{:?}'. При нормальной работе программы данное сообщение не появляется. Проверьте возможные источники события 'Message::_'",
                    message
                );
            }
        };

        // Возвращаем пустое задание
        iced::Task::none()
    }

    // Описание интерфейса формы на текущий кадр (сборка структуры формы)
    pub fn view(&self) -> iced::Element<'_, Message, iced::Theme> {
        /*
        // Замер времени рендеринга
        let render_start = std::time::Instant::now();
        let render = render_workspace(&self);
        let render_duration = render_start.elapsed();
        log::trace!("Время выполнения функции view(): {:?}", render_duration);        
        render
        */
        render_workspace(&self)
        
    }

    // Запуск цикла отрисовки окна формы
    pub fn run() -> iced::Result {
        // Шрифты
        let font_serif_bytes = uitheme::FONT_SERIF_BYTES;
        let font_matrial_bytes = uitheme::FONT_MATERIAL_BYTES;

        // Вызываем функцию главного окна программы
        // Регистрируем основные обработчики init, update, view
        // Подключаем дополнительные методы и применяем настройки
        iced::application(App::init, App::update, App::view)
            // Заголовок окна приложения
            .title(APP_TITLE)
            // Регистрация шрифтов приложения
            .font(font_matrial_bytes)
            .font(font_serif_bytes)
            // Управление темой окна приложения
            .theme(App::theme)
            // Прослушиватель асинхронных событий (подписчик)
            .subscription(App::subscription)
            .run()
    }

    // Управление темой
    pub fn theme(&self) -> iced::Theme {
        //iced::Theme::Light
        if self.is_dark_theme() {
            iced::Theme::Dark
        } else {
            iced::Theme::Light
        }
    }

    // Прослушивание асинхронных потоков событий
    fn subscription(&self) -> iced::Subscription<Message> {
        //use iced::keyboard::key::Named;
        //use iced::window::events;

        iced::Subscription::batch(vec![
            // Ваша текущая подписка на глобальные события (мышь, клавиатура и т.д.)
            iced::event::listen().map(Message::GlobalEvent),

            // Подписка на события окон (передаст ID окна и само событие)            
            iced::window::events().map(|(id, event)| Message::WindowEvent(id, event)),
        ])
    }

    // -------------------------------------------------------------------------
    // Управление состоянием
    // -------------------------------------------------------------------------

    // Полная очистка хранилища и сброс выбранных элементов
    pub fn clear_all(&mut self) {
        self.factory.clear_all();
        self.clear_selection();
    }

    // Выбор виджета
    pub fn select_widget(&mut self, widget_id: String) {
        self.state.selected_property_key = None;
        self.state.selected_widget_id = Some(widget_id.clone());
        log::info!("Выбран активным виджет '{}'", widget_id);
    }

    // Выбор свойства
    pub fn select_property(&mut self, property_key: PropertyKey) {
        if self.state.selected_widget_id.is_some() {
            self.state.selected_property_key = Some(property_key.clone());
            log::info!("Выбрано активным свойство '{}'", property_key.name);
        }
    }

    // Сброс выбранных элементов
    pub fn clear_selection(&mut self) {
        self.state.selected_property_key = None;
        self.state.selected_widget_id = None;
        log::info!("Выделение виджета успешно сброшено");
    }

    // Добавить виджет
    pub fn add_widget(&mut self, widget_type: &str) {
        let new_id = self
            .factory
            .add_widget(widget_type, self.state.selected_widget_id.as_deref());
        self.select_widget(new_id);
    }

    // Удалить виджет
    pub fn delete_widget(&mut self) {
        if let Some(ref widget_id) = self.state.selected_widget_id {
            // Передаем чистую строковую ссылку &str в метод удаления фабрики
            self.factory.delete_widget(widget_id);

            // Сразу сбрасываем выделение в стейте, так как этого виджета больше нет в живых
            self.clear_selection();
        }
    }

    pub fn moveup_widget(&mut self) {
        if let Some(widget_id) = &self.state.selected_widget_id {
            self.factory.move_widget_in_map(&widget_id, -1);
        }
    }

    pub fn movedown_widget(&mut self) {
        if let Some(widget_id) = &self.state.selected_widget_id {
            self.factory.move_widget_in_map(&widget_id, 1);
        }        
    }

    // -------------------------------------------------------------------------
    // Управление работой интерфейса
    // -------------------------------------------------------------------------

    // Переключение режима работы
    pub fn toggle_design_mode(&mut self) {
        self.factory.toggle_design_mode();
    }

    // Переключение отображаемой темы
    pub fn toggle_view_theme(&mut self) {
        self.state.ui_style.is_dark_theme = !self.is_dark_theme();
    }

    pub fn is_design_mode(&self) -> bool {
        self.get_factory().is_design_mode()
    }

    pub fn is_dark_theme(&self) -> bool {
        self.state.ui_style.is_dark_theme
    }

    pub fn get_ui_style(&self) -> UIStyle {
        self.state.ui_style
    }

    // -------------------------------------------------------------------------
    // Управление работой оверлеев и модальных окон
    // -------------------------------------------------------------------------
    pub fn has_active_overlay(&self) -> bool {
        self.overlay_operation != OverlayAction::NoOp
    }

    pub fn apply_overlay(&mut self, action: OverlayAction) {
        match action {
            // Закрываем оверлей
            OverlayAction::CloseOverlay => {
                self.close_overlay();
            }
            _ => {
                // Разрешаем поле оверлея для диалога
                self.overlay_operation = action;
            }
        }
    }

    /*
    pub fn apply_editor_overlay(&mut self, action: OverlayEditorAction) {
        match action {
            _ => {
            }
        }
    }
    */

    pub fn close_overlay(&mut self) {
        self.overlay_operation    = OverlayAction::NoOp;
        self.overlay_content_code = UnsafeCell::new(String::new());
    }

}
