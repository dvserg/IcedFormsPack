// -----------------------------------------------------------------------------
// Модуль widget_bp
// Содержит реализацию трейтов и интерфейсов для модулей blueprints
// -----------------------------------------------------------------------------
use iced::{Element, Theme};
use std::rc::Rc;

//use crate::blueprints;
use crate::core::Factory;
use crate::core::Message;
use crate::core::PropertyKey;
use crate::core::meta::CommonWidgetMeta;
//use crate::ui::inspector::Message;

// =============================================================================
// ГЛOБАЛЬНЫЕ СOНСТАНТЫ КАТЕГOРИЙ ВИДЖЕТOВ
// =============================================================================
pub const CAT_BASE:    &'static str = "Основное";
pub const CAT_CONTAIN: &'static str = "Контейнеры";
pub const CAT_INPUTS:  &'static str = "Управление";

// =============================================================================
// Трейт метаданных
// =============================================================================
pub trait HasCommonMeta {
    /// Получение неизменяемой ссылки на метаданные
    fn get_meta(&self) -> &CommonWidgetMeta;

    /// Получение ИЗМЕНЯЕМОЙ ссылки на метаданные
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta;
}

// =============================================================================
// Главный !однопоточный! трейт виджетов
// =============================================================================
// Создаем трейт-интерфейс для всех будущих чертежей
pub trait WidgetBlueprint: HasCommonMeta + std::fmt::Debug {
    // Обязательная константа для макроса inventory
    // Задает единое константное имя блюпринта, для избежания
    // несовпадения текстовых имен при импорте/экспорте
    //const WIDGET_TYPE: &'static str;

    /// Возвращает тип виджета (например, "text", "button")
    /// Тип в тексте должен совпадать с написание типа в Rust
    //fn widget_type(&self) -> &str;
    fn widget_type(&self) -> &'static str;

    // Инициализация из VTable
    fn from_vtab(&self, _factory: &Factory) { 
        log::trace!("from_vtab: Заглушка: Инициализации свойств 
            блюпринта <{}> виджета '{}' из VTable.", 
            self.widget_type(),
            self.get_id());       
    }

    // Список полей для инспектора
    fn editable_properties(&self) -> Vec<PropertyKey>;

    /// Мутирует внутреннее состояние виджета (ввод букв, клики, выбор в комбобоксе)
    //fn update_action(&self, factory: &crate::core::Factory, action: &dyn std::any::Any);

    /// Главный метод отрисовки виджета для кадра Iced.
    fn build_element<'a>(
        &'a self,
        factory: &'a crate::core::Factory,
        selected_id: Option<&str>,
    ) -> iced::Element<'a, crate::core::Message, iced::Theme>;

    /// Функция отрисовки специального редактора для виджета
    /// Блюпринт возвращает ТОЛЬКО "начинку" (внутренности) редактора
    fn build_editor_content<'a>(&'a self, _factory: &'a Factory) -> Element<'a, Message, Theme> {
        // Дефолтная начинка, если у виджета нет специфичного редактора
        use iced::widget::text;
        text(format!(
            "Редактор для типа '{}' не реализован.",
            self.widget_type()
        ))
        .into()
    }
    // -------------------------------------------------------------------------
    // БАЗОВАЯ РЕАЛИЗАЦИЯ МЕТОДА GET_ID
    fn get_id(&self) -> String {
        // Мы вызываем метод нашего трейта и клонируем ID из метаданных
        self.get_meta().id.clone()
    }

    // БАЗОВАЯ РЕАЛИЗАЦИЯ МЕТОДА SET_ID
    fn set_id(&mut self, id: &str) {
        // Вызываем мутабельный доступ к метаданным и меняем строку
        self.get_meta_mut().id = id.to_string();
    }

    /// Проверяет, может ли этот виджет прямо сейчас принять дочерний элемент
    fn can_accept_child(&self, _factory: &Factory) -> bool {
        // По умолчанию большинство простых виджетов (Text, HRule, Space)
        // не принимают детей вообще, поэтому возвращаем false
        false
    }

    /// Базовая реализация апдейта собственных свойств блюпринта из VTable
    /// Применяется в цикле на этапе Update Iced если у данного виджета были изменения свойств
    /// Фактическая реализация выполняется в самих блюпринтах
    /// Старая схема: InspectorEditor > Update VTable > Покадровая генерация виджета с чтением VTable
    /// Новая схема:  InspectorEditor > Update VTable + Обновление InternalProps > Покадровая генерация виджета с чтением InternalProps
    /// Почему новая схема лучше:
    /// 1. Не происходит покадрового чтения VTable со всеми сопутствующими конвертациями, данные берутся сразу из InternalProps
    /// 2. Пока все...
    fn refresh_internal_props(&self, _factory: &Factory) {
        // В базовой реализации ничего не апдейтим
    }

    /// Перехватчик живых событий элемента. Реализация выполняется в блюпринтах
    /// Принимает событие, ссылку на фабрику и состояние приложения для гибких мутаций.
    fn handle_event(
        &mut self,
        widget_action: &crate::core::message_bp::WidgetAction, // Строго совпадает с трейтом!
        _app: &mut crate::app::App,
    ) -> iced::Task<crate::core::message::Message> {
        // Пишем логирование
        log::info!(
            "widget_bp::handle_event: Дефолтный 'handle_event'. Обработка события WidgetAction: {:?}",
            widget_action
        );

        // По умолчанию любой виджет просто игнорирует события и ничего не делает
        iced::Task::none()
    }

    fn get_index(&self) -> usize {    // Future
        // Мы вызываем метод нашего трейта
        self.get_meta().index
        //return String::from("0");
    }

    // Функция возвращает динамический список имен свойств для экспорта
    // Возвращаются только имена свойств с недефолтныи значениями, которые нужно сохранить в JSON
    // Свойства с дефолтными значениями отсекаются
    fn get_exportable_property_names(&self, _factory: &Factory) -> Vec<PropertyKey> {
        // По умолчанию возвращаем пустой вектор
        Vec::new()
    }
}

// =============================================================================
// Пустышка для WidgetBlueprint
// =============================================================================
#[derive(Debug)]
pub struct DummyBlueprint {
    pub meta: CommonWidgetMeta,    
}

impl DummyBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "dummy";

    pub fn new(id: String) -> Self {
        Self {
            meta: CommonWidgetMeta::new(id),
        }
    }    
}

// Реализуем обязательный супертрейт метаданных (мы его разбирали ранее)
impl HasCommonMeta for DummyBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

// Реализуем сам трейт, чтобы DummyBlueprint стал легальным dyn WidgetBlueprint
impl WidgetBlueprint for DummyBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![]
    }

    fn build_element<'a>(
        &'a self,
        _factory: &'a Factory,
        _selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        iced::widget::space().into()
    }
}

// =============================================================================
// ОДНОПОТОЧНЫЙ ТРЕЙТ ДЛЯ РЕГИСТРАЦИИ ФАБРИЧНЫХ СОЗДАТЕЛЕЙ (WidgetCreator)
// =============================================================================
/// Интерфейс для динамических фабрик-создателей конкретных виджетов.
pub trait WidgetCreator: std::fmt::Debug {
    /// Метод создает сам чертеж виджета и упаковывает его в легкий Rc-указатель
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint>;
}

// =============================================================================
// Автоматическая регистрация виджетов в реестре
// =============================================================================
// Структура-контейнер, которую линкер будет собирать по всему проекту
pub struct AutoRegisteredWidget {
    pub name: &'static str,

    // Группирующая категория для виджета
    pub category: &'static str,

    // Возвращает плоский Box.
    // Box + Send + Sync здесь используется для глобальных статических коллекций inventory!
    pub constructor: fn() -> Box<dyn WidgetCreator + Send + Sync>,
}

// КРИТИЧЕСКИ ВАЖНО: Регистрируем тип в инвентаре сразу в месте его объявления!
inventory::collect!(AutoRegisteredWidget);



