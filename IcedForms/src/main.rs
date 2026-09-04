// -----------------------------------------------------------------------------
// Модуль main
// Содержит главный модуль приложения
// -----------------------------------------------------------------------------
use clap::Parser;

pub mod app;
pub mod blueprints;
pub mod core;
pub mod ui;
pub mod logger;
pub mod widgets;

pub use crate::app::App;
pub use crate::core::{ALL_PROPERTY_TOKENS, CliOptions, Config, APP_CONFIG};

pub const APP_TITLE: &'static str = "Iced Forms";



use tracing_subscriber::layer::SubscriberExt;

fn main() {

    // Подключаем слой логирования Tracy к глобальному подписчику
    let subscriber = tracing_subscriber::registry()
        .with(tracing_tracy::TracyLayer::default());

    tracing::subscriber::set_global_default(subscriber)
        .expect("Не удалось установить глобальный логгер");

    // Запуск автоматического парсинга аргументов
    let options = CliOptions::parse();

    // Инициализация объявленных констант и статических переменных
    core::init_builtin_properties();

    logger::init(options.log_level);
    log::info!("Старт программы");

    // Инициализируем и читаем конфиг
    let loaded_config = Config::load();
    APP_CONFIG.set(loaded_config).unwrap();    

    // Создаем и запускаем приложение
    let _ = app::App::run();

    log::info!("Завершение программы");
}
