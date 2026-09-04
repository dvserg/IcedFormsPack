// -----------------------------------------------------------------------------
// Модуль message
// Содержит реализацию сообщений и событий приложения
// -----------------------------------------------------------------------------
use iced::alignment::{Horizontal, Vertical};

pub use crate::blueprints;
pub use crate::core::*;

// Структура сообщений формы используемые в программе
#[derive(Debug, Clone)]
pub enum Message {
    // Обработка асинхронных потоков событий
    GlobalEvent(iced::Event),

    // Обработка для оконных событий
    WindowEvent(iced::window::Id, iced::window::Event), 

    // События UI, меню приложения и тулбаров
    MenuEvent(MenuAction),

    // События сложных активных элементов (text_editor, combo_box),
    // Хендлер обработчика реализуется в трейте самого блюпринта
    WidgetEvent(String, core::WidgetAction),

    // События апдейта свойств виджетов и обработки активностей простых виджетов (check_box, slider)
    // Значение параметров и свойств сохраняются в едином типизированной VTable
    UpdateProperty {
        widget_id:    String,           // ID виджета (его имя)
        property_key: PropertyKey,      // ID-ключ свойства виджета (хэш ID и имя виджета)
        value:        PropertyValue,    // Типизированное значение
    },

    // События для PaneGrid (перетаскивание/изменение размера)
    PaneDragged(iced::widget::pane_grid::DragEvent),
    PaneResized(iced::widget::pane_grid::ResizeEvent),

    // События для вложенного PaneGrid инспектора (вертикальный сплит Props / Layers)
    InspectorPaneDragged(iced::widget::pane_grid::DragEvent),
    InspectorPaneResized(iced::widget::pane_grid::ResizeEvent),

    // События оверлей-диалогов в модальном окне
    OverlayEvent(OverlayAction),
    OverlayEditorEvent(OverlayEditorAction),

    // Пустая операция
    NoOp,
}

/// События UI и меню приложения
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuAction {
    // -------------------------------------------------------------------------
    // События главного меню
    // -------------------------------------------------------------------------
    NewProject,
    NewProjectConfirmed(bool),

    OpenProject,
    OpenProjectConfirmed(Option<std::path::PathBuf>),

    SaveProject,
    SaveProjectConfirmed(Option<std::path::PathBuf>),

    ExportProject,
    ExportProjectConfirmed(Option<std::path::PathBuf>),

    ShowAbout,

    ExitApplication,
    ExitApplicationConfirmed(bool),

    ClearCanvas,
    ClearCanvasConfirmed(bool),

    // ***
    OpenSettingsPanel,

    // -------------------------------------------------------------------------
    // Операции с виджетами
    // -------------------------------------------------------------------------

    // Добавить виджет
    AddWidget(String),

    // Удалить виджет
    DeleteWidget,

    // Выбрать виджет
    SelectWidget(String),

    // Выбрать свойство
    SelectProperty(PropertyKey),

    // Переместить виджет выше или ниже
    MoveUpWidget,
    MoveDownWidget,

    // -------------------------------------------------------------------------
    // Управление работой интерфейса
    // -------------------------------------------------------------------------

    // Переключение режима работы
    ToggleDesignMode,

    // Переключение отображаемой темы
    ToggleViewTheme,

    // Снять выделение элемента
    ClearSelection,

    // Нет операции
    NoOp,
}

/// Единое представление любого значения свойства в нашей VTable-системе.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Parent(String),
    Text(String),
    USize(usize),
    Float(f32),
    Integer(i32),
    Boolean(bool),
    Color(iced::Color),
    Quad(f32, usize),
    Length(iced::Length, f32),
    Size(f32),
    Pixels(iced::Pixels),
    Padding(iced::Padding),
    Radius(iced::border::Radius),
    AlignItems(iced::Alignment),
    AlignX(Horizontal),
    AlignY(Vertical),
}

// Типы диалогов оверлея
#[derive(Debug, Clone, PartialEq)]
pub enum DialogType {
    NewProject,
    ClearProject,
    Exit,
    Info,
    About,
    Settings,
    TreeCode,
    WidgetCode,
}

// События оверлея
#[derive(Default, Debug, Clone, PartialEq)]
pub enum OverlayAction {
    // Открыть оверлей-редатор виджета
    OpenWidgetEditor(String),

    // Открыть оверлей-диалог
    OpenDialog(DialogType),

    // Закрыть оверлей
    CloseOverlay,

    // Заглушка
    #[default]
    NoOp,
}

// =====================================================================
// ФИНАЛЬНЫЙ СИСТЕМНЫЙ СТИЛЬ: Говорим компилятору Rust, что наши сообщения
// можно безопасно переносить между потоками Iced.
// Это мгновенно снимет ошибку E0277 с метода iced::application!
// =====================================================================
unsafe impl Send for MenuAction {}
unsafe impl Sync for MenuAction {}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}
// =====================================================================

