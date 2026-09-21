use clap::Parser;

pub mod core;
pub use crate::core::{base, cli, logger,};



fn main() {
    logger::init();
    log::info!("Старт программы");

    // Запуск парсинга аргументов
    // Если пользователь забыл передать параметры или ввёл --help,
    // clap напечатает справку в консоль и завершит программу.
    if std::env::args().len() == 1 {
        use clap::CommandFactory;
        
        let mut cmd = cli::CliOptions::command(); // Теперь это скомпилируется!
        cmd.print_help().unwrap();
        println!();
        std::process::exit(0);
    }

    let options = cli::CliOptions::parse();

    log::info!("Параметры CLI успешно приняты:");
    log::info!("   └─ Входной макет JSON: {}",  options.input);
    log::info!("   └─ Выходной код Rust:  {}",  options.output);
    log::info!("   └─ Папка шаблонов Tera: {}", options.templates);

    base::execute(&options);
    log::info!("Завершение программы");
}

