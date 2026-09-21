// -----------------------------------------------------------------------------
// Виджет 'stack'
// Стек - контейнер разметки, который накладывает дочерние элементы друг
//  на друга по оси Z (слоями)
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::widget::{stack};
use iced::{Element, Length, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        StackBlueprint::WIDGET_TYPE,
        category:    CAT_CONTAIN,
        constructor: create_stack_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_stack_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Stack");
    Box::new(StackCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct StackCreator;

impl WidgetCreator for StackCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(StackBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct StackProps {
    pub width:  Length,     // Стратегия ширины Iced ("width")
    pub height: Length,     // Стратегия высоты Iced ("height")
    pub clip:   bool,       // Флаг обрезки вышедшего за рамки контента ("clip")
}

#[derive(Debug, Clone /*, serde::Serialize, serde::Deserialize*/)]
pub struct StackBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<StackProps>,
}

impl HasCommonMeta for StackBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl StackBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "stack";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: StackProps::default().into(),
        }
    }

    // Финальная атомарная функция парсинга свойств контейнера Stack
    fn parse_props(&self, factory: &Factory) -> StackProps {
        // Получить ID виджета
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = StackProps::default();

        // По умолчанию для контейнеров компоновки лучше всего задавать Fill / Fill
        let width:  Length = factory.get_or_set(&widget_id, PROP_WIDTH,  Length::Fill);
        let height: Length = factory.get_or_set(&widget_id, PROP_HEIGHT, def.height);

        // Флаг обрезки контента
        let clip: bool = factory.get_or_set(&widget_id, PROP_CLIP, def.clip);

        StackProps {
            width,
            height,
            clip,
        }
    }
}

// Значения по умолчанию
impl Default for StackProps {
    fn default() -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Shrink,
            clip: false, // По умолчанию в Iced контент обычно не обрезается
        }
    }
}

impl WidgetBlueprint for StackBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Функция возвращает статус "Принимает ли детей"
    fn can_accept_child(&self, _factory: &Factory) -> bool {
        true
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![PROP_WIDTH, PROP_HEIGHT, PROP_CLIP]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>, // Получаем выбранный ID для инспектора
    ) -> Element<'a, Message, Theme> {
        // Получаем все свойства Stack через вынесенную функцию парсинга
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Создаем базовый виджет stack![] для Iced 0.14 и применяем нативные свойства
        let mut stk = stack![]
            .width(props.width.clone())
            .height(props.height.clone())
            .clip(props.clip);

        // Ищем и рендерим ВСЕХ детей, у которых parent == self.get_id()
        let mut has_children = false;
        for (child_id, child_blueprint) in factory.blueprints_iter() {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            if parent_id == self.get_id() {
                has_children = true;
                // ВАЖНО: Пробрасываем selected_id рекурсивно вниз абсолютно каждому слою!
                let child_element = child_blueprint.build_element(factory, selected_id);
                stk = stk.push(child_element);
            }
        }

        // Защита от схлопывания пустого стэка слоев
        if !has_children {
            let placeholder = create_empty_placeholder(&self.get_id(), &self.widget_type(), Length::Shrink, Length::Shrink);
            stk = stk.push(placeholder);
        }
 
        // Формируем элемент в зависимости от режима конструктора
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            iced::widget::mouse_area(stk)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            // В обычном режиме работы — стандартный чистый stack Iced 0.14
            stk.into()
        };

        // В самом конце применяем магию подсветки из трейта в одну строчку!
        apply_design_overlay(
            element,
            factory.is_design_mode(),
            selected_id,
            &self.get_id(),
        )

    }

    // Функция возвращает динамический список имен свойств для экспорта
    // Возвращаются только имена свойств с недефолтныи значениями, которые нужно сохранить в JSON
    // Свойства с дефолтными значениями отсекаются
    fn get_exportable_property_names(&self, factory: &Factory) -> Vec<PropertyKey> {
        let mut prop_names = Vec::new();

        // Извлекаем текущие свойства стэка из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = StackProps::default();

        // Сравниваем текущие значения со значениями по умолчанию
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.height != default.height {
            prop_names.push(PROP_HEIGHT);
        }
        if current.clip != default.clip {
            prop_names.push(PROP_CLIP);
        }

        prop_names
    }    
}
