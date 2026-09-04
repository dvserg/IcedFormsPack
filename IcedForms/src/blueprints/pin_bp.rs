// -----------------------------------------------------------------------------
// Виджет 'pin'
// (Абсолютное позиционирование) — Фиксирует дочерний элемент на жестко заданных
// координатах `X` и `Y` относительно родителя, игнорируя правила автоматического макета.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::widget::{mouse_area, pin};
use iced::{Element, Length, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
inventory::submit! {
    AutoRegisteredWidget {
        name: PinBlueprint::WIDGET_TYPE, // "pin"
        category: CAT_CONTAIN,
        constructor: create_pin_creator,
    }
}

fn create_pin_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация Pin");
    Box::new(PinCreator)
}

#[derive(Debug, Clone)]
pub struct PinCreator;

impl WidgetCreator for PinCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(PinBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct PinProps {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct PinBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<PinProps>,
}

impl HasCommonMeta for PinBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl PinBlueprint {
    const WIDGET_TYPE: &'static str = "pin";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: PinProps::default().into(),
        }
    }

    pub fn parse_props(&self, factory: &Factory) -> PinProps {
        let widget_id = self.get_id();

        // Парсим координаты смещения (по умолчанию 0.0)
        let x: f32 = factory.get_or_set(&widget_id, PROP_PIN_X, 0.0);
        let y: f32 = factory.get_or_set(&widget_id, PROP_PIN_Y, 0.0);

        PinProps { x, y }
    }
}

impl Default for PinProps {
    fn default() -> PinProps {
        PinProps {
            x: 0.0_f32,
            y: 0.0_f32,
        }
    }
}

impl WidgetBlueprint for PinBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Pin принимает ровно одного ребенка
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

        !is_occupied
    }
    */

    // Экспонируем координаты X и Y в инспектор свойств
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![PROP_PIN_X, PROP_PIN_Y]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        let widget_id = self.get_id();
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Собираем внутреннего ребенка
        let mut child_element: Option<Element<'a, Message, Theme>> = None;
        for (child_id, child_blueprint) in factory.blueprints_iter() {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            if parent_id == widget_id {
                child_element = Some(child_blueprint.build_element(factory, selected_id));
                break;
            }
        }

        // Если ребенка нет, рендерим стандартную заглушку пустого контейнера
        let inner_content = child_element.unwrap_or_else(|| {
            create_empty_placeholder(
                &widget_id,
                &self.widget_type(),
                Length::Shrink,
                Length::Shrink,
            )
        });

        // Оборачиваем во внутренний контейнер дизайн-режима или рендерим начисто
        let current_element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // Чтобы дизайнер мог кликнуть по абсолютно спозиционированному элементу,
            // оборачиваем его в mouse_area для выбора в дереве проекта
            mouse_area(inner_content)
                .interaction(iced::mouse::Interaction::Pointer)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            inner_content
        };

        // НАСТАВЛЯЕМ НА АБСОЛЮТНЫЕ КООРДИНАТЫ
        // Навешиваем pin() на элемент и передаем X и Y
        let element: Element<'a, Message, Theme> = pin(current_element).x(props.x).y(props.y).into();

        // Применяем стандартный оверлей выделения
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
    fn get_exportable_property_names(&self, _factory: &Factory) -> Vec<PropertyKey> {
        self.editable_properties()
    }

}
