// -----------------------------------------------------------------------------
// Модуль logger
// Содержит все, что относится к логированию
// -----------------------------------------------------------------------------

use flexi_logger::{
    Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, Naming, WriteMode,
    detailed_format,
};
use log::Record;
use log::LevelFilter;
//use log::{info, warn};


pub fn init(level: LevelFilter) {
    // Превращаем LevelFilter в строку, приводим к нижнему регистру (например, "trace")
    let level_str = level.to_string().to_lowercase();

    // Формируем гибкое правило: по умолчанию warn, а для проекта — выбранный уровень
    let filter_spec = format!("warn, IcedForms={}", level_str);


    //env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // НАСТРОЙКА И БЛОКИРОВКА ЛОГОВ:
    // "warn" — дефолтный уровень для всего (глушит лишний info от Iced/wgpu)
    // "IcedForms=info" — включает подробный info! вывод только для вашего проекта
    //Logger::try_with_str("warn, IcedForms=info")
    Logger::try_with_str(filter_spec)
        .unwrap()
        // Настройка записи в файл
        .log_to_file(
            FileSpec::default()
                .directory("logs") // Все логи будут складываться в папку logs/
                .basename("editor"), // Имя файла будет editor_YYYY-MM-DD_HH-MM-SS.log
        )
        // ГЛАВНАЯ СТРОКА: Дублируем логи и в файл, и в консоль (stderr) одновременно!
        .duplicate_to_stderr(Duplicate::All)
        .format(detailed_format)
        .format_for_stderr(console_format)
        // Ротация файлов (чтобы логи не весили гигабайты)
        .rotate(
            Criterion::Size(10 * 1024 * 1024), // Ротация при достижении файла 10 МБ
            Naming::Timestamps,
            Cleanup::KeepLogFiles(5), // Хранить только последние 5 файлов логов
        )
        .write_mode(WriteMode::Direct)
        .start()
        .unwrap();
}

pub fn console_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(w, "[{}] {}\n", record.level(), record.args())
}
