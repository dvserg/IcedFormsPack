use crate::core::{models};



/// СИНТАКСИЧЕСКИЙ РАЗБОР
/// Принимает сырую строку текста макета, проверяет её валидность и разворачивает
/// строго типизированную структуру CadProject со всеми VTable-параметрами.
pub fn parse_json_layout(json_content: &str) -> Result<models::CadProject, String> {
    log::info!("parser::parse_json_layout: Запуск синтаксического анализа строки JSON...");

    // Проверяем, что на вход пришёл не пустой файл
    let trimmed_content = json_content.trim();
    if trimmed_content.is_empty() {
        return Err("Входной текстовый буфер JSON пуст!".to_string());
    }

    // Вызываем потоковый десериализатор serde_json
    // Благодаря кастомному трейту Deserialize внутри PropertyRegistry, 
    // serde самостоятельно распарсит плоскую карту "types_registry" из JSON, 
    // проверит строки через FromStr и запишет в готовые Enum-варианты PropertyType
    match serde_json::from_str::<models::CadProject>(trimmed_content) {
        Ok(project) => {
            log::info!("   [Успех] Парсинг проекта успешно завершен.");
            log::info!("   └─ Загружено свойств в Schema: {}", project.property_registry.len());
            log::info!("   └─ Загружено живых виджетов:    {}", project.widgets.len());
//println!("PROJECT: {:#?}", &project);
            // Возвращаем полностью собранный, валидный CAD-проект
            Ok(project)
        },
        Err(err) => {
            // Формируем ошибку с указанием строки и символа сбоя
            let error_msg = format!(
                "Критическая ошибка синтаксиса JSON макета (Строка {}, Колонка {}): {}",
                err.line(),
                err.column(),
                err
            );
            log::error!("parser::parse_json_layout: {}", error_msg);
            Err(error_msg)
        }
    }
}