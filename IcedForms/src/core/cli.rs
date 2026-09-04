use clap::Parser;
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(
    name = "iced_forms",
    author = "Serg Dvoryantsev",
    version = "1.0",
    about = "***: Прототипирование форм Iced 0.14"
)]
pub struct CliOptions {
    /// Уровень логирования для всего приложения [возможные значения: off, error, warn, info, debug, trace]
    #[arg(short, long, default_value = "warn")]
    pub log_level: LevelFilter,
}
