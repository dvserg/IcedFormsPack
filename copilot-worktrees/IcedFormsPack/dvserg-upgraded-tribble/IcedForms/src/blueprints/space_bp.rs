// -----------------------------------------------------------------------------
// Виджет 'space'
// Пространство — Компонент, который позволяет добавить отступ между другими виджетами.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::widget::{container, space};
use iced::{Element, Length, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        SpaceBlueprint::WIDGET_TYPE,
        category:    CAT_BASE,
        constructor: create_space_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_space_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Space");
    Box::new(SpaceCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct SpaceCreator;

impl WidgetCreator for SpaceCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(SpaceBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct SpaceProps {
    pub width:  Length,
    pub height: Length,
}

#[derive(Debug, Clone /*, serde::Serialize, serde::Deserialize*/)]
pub struct SpaceBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<SpaceProps>,
}

impl HasCommonMeta for SpaceBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl SpaceBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "space";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: SpaceProps::default().into(),
        }
    }

    // Вынесенный парсинг свойств с переиспользованием хелперов Factory
    fn parse_props(&self, factory: &Factory) -> SpaceProps {

        log::trace!("parse_props: Парсинг свойств блюпринта <{}> виджета '{}' из VTable.", self.widget_type(), self.get_id());

        // Получить текущий ID виджета
        let widget_id = self.get_id();

        // Читаем ширину и высоту (по умолчанию fill, если не задано)
        let width:  Length = factory.get_or_set(&widget_id, PROP_WIDTH,  Length::Fixed(50.0));
        let height: Length = factory.get_or_set(&widget_id, PROP_HEIGHT, Length::Fixed(50.0));

        SpaceProps { width, height }
    }
}

// Реализация значений по умолчанию
impl Default for SpaceProps {
    fn default() -> Self {
        Self {
            width:  Length::Shrink,  // Либо Length::Fixed(0.0)
            height: Length::Shrink, // Либо Length::Fixed(0.0)
        }
    }
}


impl WidgetBlueprint for SpaceBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![PROP_WIDTH, PROP_HEIGHT]
    }

    // Реализация апдейта собственных свойств блюпринта из VTable
    // crate::impl_refresh_props!(SpaceBlueprint, SpaceProps);

    // Рендеринг элемента
    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем свойства из блюпринта
        let props = self.parse_props(factory);

        // Формируем элемент в зависимости от режима конструктора
        let element: Element<'a, Message, Theme> = {
            // В обычном режиме работы — стандартное невидимое пространство
            let w_space = space::Space::new()
                .width(props.width)
                .height(props.height);                

            if factory.is_design_mode() {
                // Окрашиваем пустое пространство в нежный пастельный цвет в конструкторе
                let space_container = container(w_space).style(|_theme| container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.2, 0.6, 1.0, 0.15,
                    ))),
                    border: iced::Border {
                        color: iced::Color::from_rgba(0.2, 0.6, 1.0, 0.4),
                        width: 1.0,
                        radius: 2.0.into(),
                    },
                    ..Default::default()
                });

                iced::widget::mouse_area(space_container)
                    .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    .into()
            } else {
                w_space.into()
            }
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

        // Извлекаем текущие свойства пространства из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = SpaceProps::default();

        // Сравниваем свойства строго по вашему списку editable_properties
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.height != default.height {
            prop_names.push(PROP_HEIGHT);
        }

        prop_names
    }    
}
