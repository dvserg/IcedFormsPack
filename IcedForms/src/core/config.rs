// -----------------------------------------------------------------------------
// Модуль config
// Содержит структуру конфигурации
// -----------------------------------------------------------------------------
use std::sync::OnceLock;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;



// Объявляем глобальный потокобезопасный контейнер
pub static APP_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {

    // Путь к папке с шаблонами
    pub template_path:    String,
    pub template_include: Vec<String>,
    //pub template_exclude: Vec<String>,

    // Конфигурация шаблонов Tera
    pub template_config:  String,

    // Стиль/тема оформления приложения (например: "Blender", "Figma", "VSCode")
    pub theme_style:      String,
}

impl Config {
    const CONFIG_FILE: &'static str = "iced_forms_config.toml";

    // Возвращает имя конфигурационного файла
    pub fn config_file() -> String {
        Self::CONFIG_FILE.to_string()
    }

    // Чтение конфигурационного файла
    pub fn load() -> Self {
        let config_path = Path::new(Self::CONFIG_FILE);

        log::info!("Config::load: Чтение файла конфигурации '{}'.", Self::config_file());

        // Если файла конфигурации нет, создаем его с дефолтными значениями
        if !config_path.exists() {
            log::info!("Config::load: Файл конфигурации отсутствует. Создаем файл с значениями по умолчанию");

            let default_config = Self::default();

            // Сериализуем структуру в TOML-текст
            if let Ok(toml_string) = toml::to_string_pretty(&default_config) {
                // Записываем файл на диск, игнорируя ошибку, если нет прав на запись
                let _ = fs::write(config_path, toml_string);
            }
            return default_config;
        }

        // Если файл существует, пробуем его прочитать
        if let Ok(file_content) = fs::read_to_string(config_path) {
            // Пробуем распарсить TOML в структуру Config
            if let Ok(parsed_config) = toml::from_str::<Config>(&file_content) {
                return parsed_config;
            }
        }

        // Если файл поврежден или его невозможно прочитать,
        // возвращаем дефолтную конфигурацию, чтобы защитить приложение от падения
        log::warn!("Config::load: Файл конфигурации поврежден, загружены настройки по умолчанию.");
        Self::default()
    }
}

impl Default for Config {
    // Присваиваем дефолтные значения Iced для контроля пропущенных значений и значений по умолчанию в инспекторе
    fn default() -> Config {
        Config {
            template_path:    String::from("./templates"),
            //template_exclude: Vec::new(),
            template_include: vec![ 
                String::from("widget_preview"),  //String::from("/"), //String::from("widget_preview"), 
                String::from("components"), 
                String::from("widgets") ],

            template_config: "./widget_config.toml".to_string(),
            theme_style:     "Blender".to_string(),
        }
    }
}