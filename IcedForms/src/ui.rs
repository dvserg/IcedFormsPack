// -----------------------------------------------------------------------------
// Модуль ui
// Содержит импорты модулей User Interface
// -----------------------------------------------------------------------------

pub mod mainmenu;
pub use mainmenu::*;

pub mod toolbar;
pub use toolbar::*;

pub mod uitheme;
pub use uitheme::*;

pub mod workspace;
pub use workspace::*;

pub mod sidebar;
pub use sidebar::*;

pub mod render_style;
pub use render_style::*;

// Рекурсовное построение дерева элементов
pub mod hierarchy;
pub use hierarchy::*;

//pub mod inspector;
//pub use inspector::*;

//pub mod inspector_panel_edit;
//pub use inspector_panel_edit::*;

//pub mod inspector_panel_tree;
//pub use inspector_panel_tree::*;

// Инспектор
pub mod inspector;
pub mod inspector_panel_edit;
pub mod inspector_panel_prop;
pub mod inspector_panel_tree;
pub mod inspector_prop_editors;

// Оверлеи (модальные окна, всплывающие подсказки и т.д.)
pub mod overlays;
pub use overlays::*;

pub mod dialogs;
pub use dialogs::*;
