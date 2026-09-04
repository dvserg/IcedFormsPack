//use log::{info, warn};
use flexi_logger::{Logger, FileSpec, Duplicate, Criterion, Naming, Cleanup, detailed_format, DeferredNow, WriteMode};
use log::Record;



pub fn init() {

    // НАСТРОЙКА И БЛОКИРОВКА ЛОГОВ:
    // "warn" — дефолтный уровень для всего (глушит лишний info от Iced/wgpu)
    // "IcedForms=info" — включает подробный info! вывод только для вашего проекта
    Logger::try_with_str("warn, iced_forms_gen=trace")
        .unwrap()

        // Настройка записи в файл
        .log_to_file(
            FileSpec::default()
                .directory("logs")          // Все логи будут складываться в папку logs/
                .basename("editor")         // Имя файла будет editor_YYYY-MM-DD_HH-MM-SS.log
        )

        // ГЛАВНАЯ СТРОКА: Дублируем логи и в файл, и в консоль (stderr) одновременно!
        .duplicate_to_stderr(Duplicate::All)
        .format(detailed_format)
        .format_for_stderr(console_format)

        // Ротация файлов (чтобы логи не весили гигабайты)
        .rotate(
            Criterion::Size(10 * 1024 * 1024), // Ротация при достижении файла 10 МБ
            Naming::Timestamps,
            Cleanup::KeepLogFiles(5),          // Хранить только последние 5 файлов логов
        )
        .write_mode(WriteMode::Direct)
        .start()
        .unwrap();

}

pub fn init_logger() -> flexi_logger::LoggerHandle {
    Logger::try_with_str("warn, IcedForms=info")
        .unwrap()
        .format(detailed_format)
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .basename("editor")
        )
        .duplicate_to_stderr(Duplicate::All)
        .format_for_stderr(console_format)
        .rotate(
            Criterion::Size(10 * 1024 * 1024),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(5),
        )
        .start()    // Запускаем логер
        .unwrap()   // Извлекаем LoggerHandle наружу!
}

pub fn console_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow, 
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(w, "[{}] {}\n", record.level(), record.args())
}