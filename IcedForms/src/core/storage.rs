// -----------------------------------------------------------------------------
// Модуль storage
// Содержит реализацию хранилища VTable
// -----------------------------------------------------------------------------
use std::any::Any;
use std::collections::{BTreeMap, HashMap};
//use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
//use log::{info, warn, error};

use crate::core::utils::{fnv1a_hash_64, runtime_hash_64};
//use crate::core::*;

thread_local! {
    // Реестр имен токенов свойств
    pub static ALL_PROPERTY_TOKENS: RefCell<Vec<TokenMetadata>> = const { RefCell::new(Vec::new()) };
}

// -----------------------------------------------------------------------------
// Типы данных для хранилища
// -----------------------------------------------------------------------------
// Однопоточный динамический тип
pub type SingleThreadedAny = Box<dyn Any + 'static>;

// Карта свойств виджета
// Свойства хранятся в BTreeMap по хэшу_ID: (u64)
type PropertyMap = BTreeMap<u64, SingleThreadedAny>;

/// Метаданные свойства, которые теперь хранят хэш, имя и указатель на его персональный парсер!
#[derive(Clone, Copy)]
pub struct TokenMetadata {
    pub hash: u64,
    pub name: &'static str,
    pub type_name: &'static str, // Наименование  типа токена
    pub type_hash: u64,          // Идентификатор типа токена
}

// -----------------------------------------------------------------------------
// Плоский легковесный ключ (ID + Хэш) не привязанный к типу
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PropertyKey {
    pub name: &'static str, // Наименование property
    pub hash: u64,          // Хэш наименования
}

impl PropertyKey {
    // Создает плоский константный hash ключ из ID-строки
    pub const fn declare(name: &'static str) -> Self {
        Self {
            name,
            hash: fnv1a_hash_64(name),
        }
    }

    // Превращает любую динамическую рантайм-строку в PropertyKey.
    // Сначала ищет имя в глобальном константном реестре (Zero-Allocations),
    // а если свойства там нет — делает точечную вечную ссылку.
    pub fn from_dynamic(prop_name: &str) -> Self {
        let metadata = Self::meta_from_dynamic(prop_name);
        Self::from_metadata(metadata)
    }

    pub fn from_metadata(metadata: TokenMetadata) -> Self {
        Self {
            name: metadata.name,
            hash: metadata.hash,
        }
    }

    pub fn meta_from_dynamic(prop_name: &str) -> TokenMetadata {
        let cleaned_name = prop_name.to_lowercase().replace('-', "_");
        let current_hash = fnv1a_hash_64(&cleaned_name);

        // БЫСТРАЯ ОДНОПОТОЧНАЯ КЭШ-ВЕТКА (Чтение без блокировок ОС)
        if let Some(token_found) = crate::core::ALL_PROPERTY_TOKENS.with(|tokens| {
            let read_guard = tokens.borrow();

            // Ищем токен по хэшу внутри вектора
            read_guard
                .iter()
                .find(|m| m.hash == current_hash)
                // ИСПРАВЛЕНО: Клонируем TokenMetadata (если он реализует Clone),
                // либо вручную пересобираем структуру TokenMetadata
                .map(|meta| TokenMetadata {
                    hash: meta.hash,
                    name: meta.name,
                    type_name: meta.type_name,
                    type_hash: meta.type_hash,
                })
        }) {
            // Если токен успешно извлечен из thread_local реестра — мгновенно возвращаем его!
            return token_found;
        }

        // ДИНАМИЧЕСКАЯ ВЕТКА ЗАПИСИ: Если свойства нет в кэше — открываем замок на мутацию
        crate::core::ALL_PROPERTY_TOKENS.with(|tokens| {
            let mut write_guard = tokens.borrow_mut();

            // Сначала проверяем, не добавил ли кто-то токен, пока мы шли сюда
            if let Some(meta) = write_guard.iter().find(|m| m.hash == current_hash) {
                return TokenMetadata {
                    hash: meta.hash,
                    name: meta.name,
                    type_name: meta.type_name,
                    type_hash: meta.type_hash,
                };
            }

            // Создаем вечное имя для PropertyKey строго один раз в жизни
            let static_name: &'static str = Box::leak(cleaned_name.into_boxed_str());

            // Создаем новую карточку метаданных для регистрации
            let new_meta = TokenMetadata {
                hash: current_hash,
                name: static_name,
                type_name: "String",
                type_hash: fnv1a_hash_64("String"),
            };

            // Пушаем новое динамическое свойство в глобальный однопоточный кэш
            write_guard.push(new_meta.clone()); // Убедитесь, что над TokenMetadata стоит #[derive(Clone)]

            // Возвращаем свежесозданную карточку из замыкания .with()
            new_meta
        })
    }
}

// -----------------------------------------------------------------------------
// Типизированный хэш ключ
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyToken<T> {
    pub name: &'static str,
    pub hash: u64,
    // Виртуальное поле для реализации технологии шаблона <T>
    // Нужно только на этапе компиляции. Размер 0 байт
    _marker: PhantomData<T>,
}

impl<T> PropertyToken<T> {
    pub const fn declare(name: &'static str) -> Self {
        Self {
            name,
            hash: fnv1a_hash_64(name),
            _marker: PhantomData,
        }
    }
}

// -----------------------------------------------------------------------------
// Единое хранилище свойств виджетов
// -----------------------------------------------------------------------------

// Поле data  - содержит дерево карт свойств по каждому виджету hash-ID ключом
// Поле names - список текстовых ID виджетов и хэшей ключей к ним,
// нужно для обратного поиска и экспорта
#[derive(Debug, Default)]
pub struct PropertyStorage {
    // ID виджета (u64 хэш) -> Карта его свойств
    pub data: BTreeMap<u64, PropertyMap>,

    // Реестр имен для экспорта в JSON
    pub names: HashMap<u64, String>,
}

impl PropertyStorage {
    /// Создает новое пустое хранилище
    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
            data: BTreeMap::new(),
        }
    }

    // Очистка хранилища
    pub fn clear_all(&mut self) {
        // Стираем все свойства всех зарегистрированных виджетов
        self.data.clear();

        // Полностью очищаем таблицу текстовых имён для JSON
        self.names.clear();

        log::info!(
            "PropertyStorage: Хранилище VTable-свойств и реестр имён JSON успешно обнулены."
        );
    }

    /// Чтение значения по составному ключу (Имя виджета + Токен свойства)
    /// Метод принимает обычную иммутабельную ссылку &self
    pub fn get<T: 'static + Clone>(&self, widget_id: &str, token: PropertyToken<T>) -> Option<T> {
        let widget_hash = runtime_hash_64(widget_id);

        self.data
            .get(&widget_hash)
            .and_then(|prop_map| prop_map.get(&token.hash))
            .and_then(|any_boxed| any_boxed.downcast_ref::<T>().cloned())
    }

    /// Чтение напрямую по хэшу u64 (используется универсальными темплейтами)
    pub fn get_by_hash<T: 'static + Clone>(&self, widget_id: &str, prop_hash: u64) -> Option<T> {
        let widget_hash = runtime_hash_64(widget_id);

        // Поиск идет напрямую по готовым числам u64!
        self.data
            .get(&widget_hash)
            .and_then(|prop_map| prop_map.get(&prop_hash))
            .and_then(|any_boxed| any_boxed.downcast_ref::<T>().cloned())
    }

    /// Запись значения по составному ключу (Имя виджета + Токен свойства).
    pub fn set<T: 'static>(&mut self, widget_id: &str, token: PropertyToken<T>, value: T) {
        let widget_hash = runtime_hash_64(widget_id);

        // Нативно и без блокировок записываем имя виджета
        self.names
            .entry(widget_hash)
            .or_insert_with(|| widget_id.to_string());

        // Нативно заходим в BTreeMap и вставляем Box<dyn Any>
        let prop_map = self.data.entry(widget_hash).or_default();
        prop_map.insert(token.hash, Box::new(value) as SingleThreadedAny);
    }

    /// Запись напрямую по хэшу u64 (используется универсальными темплейтами).
    pub fn set_by_hash<T: 'static>(&mut self, widget_id: &str, prop_hash: u64, value: T) {
        let widget_hash = runtime_hash_64(widget_id);

        self.names
            .entry(widget_hash)
            .or_insert_with(|| widget_id.to_string());

        let prop_map = self.data.entry(widget_hash).or_default();
        prop_map.insert(prop_hash, Box::new(value) as SingleThreadedAny);
    }

    /// Полностью удаляет свойства виджета и его имя из реестра по u64 хэшу
    pub fn remove_widget_by_hash(&mut self, widget_hash: u64) {
        // Стираем VTable-карту свойств, привязанную к этому виджету
        self.data.remove(&widget_hash);

        // Удаляем его имя из реестра имён для JSON-экспорта
        self.names.remove(&widget_hash);

        log::info!(
            "PropertyStorage: Свойства и имя виджета с хэшем {} успешно удалены.",
            widget_hash
        );
    }
}

// -----------------------------------------------------------------------------
// Хэлперы
// -----------------------------------------------------------------------------

pub fn get_prop_type_hash(key: PropertyKey) -> u64 {
    // Очищаем имя от рантайм-суффиксов CAD-инспектора параметров (например, "padding:0" -> "padding")
    let clean_name = match key.name.split_once(':') {
        Some((root, _)) => root,
        None => key.name,
    };

    // Вычисляем хэш чистого корня для точного поиска
    let root_hash = fnv1a_hash_64(clean_name);

    // Блок .with() вернет Option<u64>. Если хэш типа найден — мы сразу отдадим его наружу!
    if let Some(type_hash) = crate::core::ALL_PROPERTY_TOKENS.with(|tokens| {
        // Наносекундный захват ссылки на чтение без блокировок ОС
        let guard = tokens.borrow();

        // Ищем метаданные по хэшу чистого корня свойства
        guard
            .iter()
            .find(|m| m.hash == root_hash)
            .map(|meta| meta.type_hash) // Извлекаем число u64 (meta.type_hash)
    }) {
        // Если хэш типа успешно извлечен из thread_local реестра — мгновенно возвращаем его!
        return type_hash;
    }
    // =====================================================================

    // БЕЗОПАСНЫЙ ОТКАТ: Если свойство абсолютно новое и кастомное — по умолчанию считаем его String
    log::warn!(
        "get_prop_type_hash: Свойство '{}' не найдено в реестре. Откат к String.",
        key.name
    );

    fnv1a_hash_64("String")
}



// =============================================================================
// Test
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*; // Импортируем PropertyStorage, PropertyToken и хелперы из родительского модуля
    use crate::core::storage::{PropertyKey, PropertyStorage, PropertyToken}; // Корректируйте пути под ваш проект

    /// Вспомогательная функция для генерации чистых тестовых токенов свойств
    fn init_test_tokens() -> (
        PropertyToken<String>,
        PropertyToken<i32>,
        PropertyToken<bool>,
    ) {
        (
            PropertyToken::<String>::declare("prop_value"),
            PropertyToken::<i32>::declare("grid_step"),
            PropertyToken::<bool>::declare("is_visible"),
        )
    }

    #[test]
    fn test_vtable_write_and_read() {
        // Инициализируем хранилище и токены
        let mut storage = PropertyStorage::new();
        let (prop_text, prop_int, prop_bool) = init_test_tokens();

        let widget_id = "button_main";

        // 1. ЗАПИСЬ: Тестируем метод .set() для разных типов данных
        storage.set(widget_id, prop_text.clone(), "Привет, Робот!".to_string());
        storage.set(widget_id, prop_int.clone(), 42);
        storage.set(widget_id, prop_bool.clone(), true);

        // 2. ЧТЕНИЕ: Проверяем, что все типы прочитались и распаковались без ошибок
        let read_text = storage.get(widget_id, prop_text);
        let read_int = storage.get(widget_id, prop_int);
        let read_bool = storage.get(widget_id, prop_bool);

        // Проверяем утверждения (Asserts)
        assert_eq!(read_text, Some("Привет, Робот!".to_string()));
        assert_eq!(read_int, Some(42));
        assert_eq!(read_bool, Some(true));

        // Проверяем, что в реестре имен JSON-экспорта правильно сохранился текстовый ID
        let widget_hash = crate::core::utils::runtime_hash_64(widget_id);
        assert_eq!(
            storage.names.get(&widget_hash),
            Some(&widget_id.to_string())
        );
    }

    #[test]
    fn test_vtable_write_by_hash_and_dynamic_keys() {
        let mut storage = PropertyStorage::new();
        let widget_id = "text_editor_1";

        // Генерируем динамический ключ (эмулируем клик пользователя в инспекторе параметров)
        let dynamic_key = PropertyKey::from_dynamic("custom-padding");

        // Извлекаем хэш-число свойства u64
        let prop_hash = dynamic_key.hash;

        // ЗАПИСЬ НАПРЯМУЮ ПО ХЭШУ (Как в методе apply_action_ / update_property)
        storage.set_by_hash::<String>(widget_id, prop_hash, "20px".to_string());

        // ЧТЕНИЕ НАПРЯМУЮ ПО ХЭШУ
        let read_value = storage.get_by_hash::<String>(widget_id, prop_hash);

        assert_eq!(read_value, Some("20px".to_string()));
    }

    #[test]
    fn test_vtable_clear_and_delete() {
        let mut storage = PropertyStorage::new();
        let (prop_text, _, _) = init_test_tokens();

        let widget_1 = "widget_1";
        let widget_2 = "widget_2";

        // Заполняем базу данных VTable
        storage.set(widget_1, prop_text.clone(), "Элемент 1".to_string());
        storage.set(widget_2, prop_text.clone(), "Элемент 2".to_string());

        // Проверяем частичное удаление одного виджета по хэшу
        let hash_1 = crate::core::utils::runtime_hash_64(widget_1);
        storage.remove_widget_by_hash(hash_1);

        assert!(storage.get::<String>(widget_1, prop_text.clone()).is_none());
        assert!(storage.get::<String>(widget_2, prop_text.clone()).is_some());

        // Проверяем полную очистку всего хранилища (Кнопка "Новый проект")
        storage.clear_all();

        assert!(storage.data.is_empty());
        assert!(storage.names.is_empty());
    }
}
