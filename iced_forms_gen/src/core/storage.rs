use std::collections::BTreeMap;
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use crate::core::{PropertyType};



// -----------------------------------------------------------------------------
// Карточка метаданных конкретного свойства
// -----------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct PropertyMetadata {
    pub name: String,             // Системное имя ("padding", "border_width")
    pub raw_type_name: String,
    pub prop_type: PropertyType,  // Строгий enum-вариант (PropertyType::Padding, PropertyType::F32)
}

// -----------------------------------------------------------------------------
// Тип-хранилище реестра свойств (PropertyRegistry)
// -----------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct PropertyRegistry {
    // Внутренняя закрытая карта для защиты целостности данных
    data: BTreeMap<String, PropertyMetadata>,
}

impl PropertyRegistry {
    /// Инициализация пустого CAD-реестра
    pub fn new() -> Self {
        Self { data: BTreeMap::new() }
    }

    /// ПОИСК ПО ИМЕНИ: Возвращает карточку метаданных для конкретного свойства
    pub fn get(&self, prop_name: &str) -> Option<&PropertyMetadata> {
        self.data.get(prop_name)
    }

    /// ПОЛУЧЕНИЕ СТРОГОГО ТИПА: Напрямую выплевывает Enum-вариант по имени поля
    pub fn get_type(&self, prop_name: &str) -> PropertyType {
        self.data.get(prop_name)
            .map(|meta| meta.prop_type)
            //.unwrap_or(PropertyType::String) // Безопасный откат
            .unwrap_or(PropertyType::Unknown) // Безопасный откат
    }

    /// ВАЛИДАЦИЯ: Проверяет, зарегистрировано ли вообще такое свойство в Schema
    pub fn contains(&self, prop_name: &str) -> bool {
        self.data.contains_key(prop_name)
    }

    /// КОНТРОЛЬ: Возвращает общее количество зарегистрированных в макете типов
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// ПРОВЕРКА ЦЕЛОСТНОСТИ: Сверяет реальное JSON-значение с ожидаемым типом Schema
    pub fn validate_value_type(&self, prop_name: &str, json_value: &serde_json::Value) -> bool {
        let expected_type = self.get_type(prop_name);
        match expected_type {
            PropertyType::Bool      => json_value.is_boolean(),
            PropertyType::F32 
            | PropertyType::Pixels  => json_value.is_number(),
            PropertyType::Padding 
            | PropertyType::Length 
            | PropertyType::Radius  => json_value.is_object(),
            PropertyType::String  
            | PropertyType::Color   => json_value.is_string(),
            PropertyType::Vertical  => {
                if let Some(s) = json_value.as_str() {
                    let clean = s.trim();
                    // Сверяем строго по списку легальных вариантов Iced 0.14
                    matches!(clean, "Top" | "top" | "Start" | "start" | "Center" | "center" | "Middle" | "middle" | "Bottom" | "bottom" | "End" | "end")
                } else {
                    false
                }
            }
            PropertyType::Horizontal => {
                if let Some(s) = json_value.as_str() {
                    let clean = s.trim();
                    // Сверяем строго по списку легальных вариантов Iced 0.14
                    matches!(clean, "Left" | "left" | "Start" | "start" | "Center" | "center" | "Middle" | "middle" | "Right" | "right" | "End" | "end")
                } else {
                    false
                }
            }

            _ => false,
        }
    }
}


// -----------------------------------------------------------------------------
// РУЧНАЯ ДЕСЕРИАЛИЗАЦИЯ: Бесшовно превращает {"padding": "Padding"} в структуру
// -----------------------------------------------------------------------------
impl<'de> Deserialize<'de> for PropertyRegistry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Считываем плоскую карту строк из JSON файла
        let raw_map = BTreeMap::<String, String>::deserialize(deserializer)?;
        let mut registry_data = BTreeMap::new();

        // В цикле парсим каждую строчку и собираем карточки PropertyMetadata
        for (prop_name, type_str) in raw_map {
            //let prop_type = PropertyType::from_str(&type_str).unwrap_or(PropertyType::Unknown);
            let prop_type = match PropertyType::from_str(&type_str) {
                // Успех: тип распознан
                Ok(parsed_type) => parsed_type, 
                Err(_) => {
                    log::warn!("deserialize: Нераспознанный тип данных '{}'. Пробуем применить как 'String'", type_str);
                    // Явно изолируем поле безопасным типом-заглушкой
                    PropertyType::String 
                }
            };
            
            let metadata = PropertyMetadata {
                name: prop_name.clone(),
                raw_type_name: type_str,
                prop_type,
            };
            registry_data.insert(prop_name, metadata);
        }

        Ok(PropertyRegistry { data: registry_data })
    }
}

// Зеркальная сериализация для сохранения совместимости форматов
impl Serialize for PropertyRegistry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut raw_map = BTreeMap::new();
        for (name, meta) in &self.data {
            raw_map.insert(name.clone(), meta.raw_type_name.clone());
        }
        raw_map.serialize(serializer)
    }
}
