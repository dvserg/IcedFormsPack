// -----------------------------------------------------------------------------
// Модуль core
// Содержит импорты основного ядра проекта
// -----------------------------------------------------------------------------

pub mod config;
pub use config::*;

pub mod storage;
pub use storage::{ALL_PROPERTY_TOKENS, TokenMetadata};
pub use storage::*;

pub mod meta;
pub use meta::*;

pub mod message;
pub use message::{Message, MenuAction, PropertyValue, OverlayAction, DialogType};

pub mod message_bp;
pub use message_bp::*;

pub mod widget_bp;
pub use widget_bp::*;

pub mod factory;
pub use factory::*;

pub mod macros;
//pub use macros::*;

// Hash ключи для свойств
pub mod prop_keys;
pub use prop_keys::*;

pub mod utils;
pub use utils::*;

pub mod utils_bp;
pub use utils_bp::*;

pub mod options;
pub use options::*;

pub mod os_dialogs;
pub use os_dialogs::*;

// Обработка событий
pub mod update_property;
pub mod update_ui;

//
pub mod data_io;
pub mod serialization;

pub mod design_proxy;

pub mod cli;
pub use cli::CliOptions;

pub mod codegen;
pub use codegen::*;

pub mod codegen_models;
pub use codegen_models::*;