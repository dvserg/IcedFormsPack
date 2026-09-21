// -----------------------------------------------------------------------------
// Виджет 'opaque'
// Непрозрачный слой — Блокирует прокликивание сквозь элемент. Клики мыши не проходят
// к виджетам, расположенным под ним на координатной сетке.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::widget::opaque;
use iced::{Element, Length, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
inventory::submit! {
    AutoRegisteredWidget {
        name:        OpaqueBlueprint::WIDGET_TYPE,
        category:    CAT_CONTAIN,
        constructor: create_opaque_creator,
    }
}

// Функция-помощник для создания Arc
fn create_opaque_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация 'Opaque'");
    Box::new(OpaqueCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct OpaqueCreator;

impl WidgetCreator for OpaqueCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(OpaqueBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct OpaqueProps {
    pub width:  Length,
    pub height: Length,
}

#[derive(Debug, Clone)]
pub struct OpaqueBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<OpaqueProps>,
}

impl HasCommonMeta for OpaqueBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl OpaqueBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "opaque";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: OpaqueProps::default().into(),
        }
    }

    // Парсинг свойств с использованием хелперов Factory
    pub fn parse_props(&self, factory: &Factory) -> OpaqueProps {
        let widget_id = self.get_id();

        // Получаем дефолтные свойства
        let def = OpaqueProps::default();

        // Opaque принимает размеры для ограничения области перехвата событий
        let width:  Length = factory.get_or_set(&widget_id, PROP_WIDTH,  def.width);
        let height: Length = factory.get_or_set(&widget_id, PROP_HEIGHT, def.height);

        OpaqueProps { width, height }
    }
}

impl Default for OpaqueProps {
    fn default() -> OpaqueProps {
        OpaqueProps {
            width:  Length::Shrink,
            height: Length::Shrink,
        }
    }
}

impl WidgetBlueprint for OpaqueBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Функция возвращает статус Принимает ли детей (Opaque может иметь только 1 ребенка)
    fn can_accept_child(&self, factory: &Factory) -> bool {
        let widget_id = self.get_id();
        let can_accept = factory.get_blueprints_by_parent(&widget_id).is_empty();
        can_accept // Может принять детей (одного), если их список пустой (True)
    }
    /*
    fn can_accept_child(&self, factory: &Factory) -> bool {
        let is_occupied = factory.blueprints.keys().any(|child_id| {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();
            parent_id == self.get_id()
        });

        // True если свободный, False если уже занят дочерним элементом
        !is_occupied
    }
    */

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![PROP_WIDTH, PROP_HEIGHT]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        let widget_id   = self.get_id();
        //let props       = self.props.borrow(); //self.parse_props(factory);
        //let is_selected = selected_id == Some(widget_id.as_str());

        // Собираем внутреннее содержимое (дочерний элемент)
        let mut child_element: Option<Element<'a, Message, Theme>> = None;
        for (child_id, child_blueprint) in factory.blueprints_iter() {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            if parent_id == widget_id {
                child_element = Some(child_blueprint.build_element(factory, selected_id));
                break;
            }
        }

        // Создаем плейсхолдер если дочерний элемент отсутствует
        let inner_content = child_element.unwrap_or_else(|| {
            create_empty_placeholder(&widget_id, &self.widget_type(), Length::Shrink, Length::Shrink)
        });

        // Настоящая сборка Opaque для обоих режимов
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // В режиме дизайнера мы оборачиваем контент в mouse_area,
            // чтобы по нему можно было кликнуть для выделения в дереве инспектора.
            // Если оставить чистый opaque, дизайнер не сможет выбрать этот контейнер.
            iced::widget::mouse_area(inner_content)
                .interaction(iced::mouse::Interaction::Pointer)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            // В пользовательском режиме применяем нативный виджет непрозрачности ядра Iced 0.14.2
            opaque(inner_content).into()
        };

        // Дизайн-оверлей для отрисовки границ выделения компонента
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

        // Извлекаем текущие свойства mouse_area из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = OpaqueProps::default();

        // Получаем чистые дефолтные свойства для сравнения
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.height != default.height {
            prop_names.push(PROP_HEIGHT);
        }

        prop_names
    }      
}
