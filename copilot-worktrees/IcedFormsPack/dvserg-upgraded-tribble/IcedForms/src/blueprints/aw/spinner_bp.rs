// -----------------------------------------------------------------------------
// Библиотека AW
// Виджет 'spinner'
// Представляет собой бегающий по кругу индикатор загрузки
// -----------------------------------------------------------------------------
use iced::widget::button;
use iced::{Element, Length, Theme};
use iced_aw::widget::Spinner;
use std::rc::Rc;
//use log::{info, warn};

use crate::core::*;

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: SpinnerBlueprint::WIDGET_TYPE,
        category: CAT_BASE,
        constructor: create_spinner_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_spinner_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "ButtonBox");
    Box::new(SpinnerCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug)]
pub struct SpinnerCreator;

impl WidgetCreator for SpinnerCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(SpinnerBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

impl HasCommonMeta for SpinnerBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

// Свойства виджета
#[derive(Debug)]
pub struct SpinnerProps {
    pub size: f32,
}

#[derive(Debug)]
pub struct SpinnerBlueprint {
    pub meta: CommonWidgetMeta,
}

impl SpinnerBlueprint {
    // Наша жесткая константа типа для инвентаря фабрики:
    pub const WIDGET_TYPE: &'static str = "spinner";

    pub fn new(id: String) -> Self {
        Self {
            meta: CommonWidgetMeta::new(id),
        }
    }

    // ВЫНЕСЕННАЯ ФУНКЦИЯ ПАРСИНГА СВОЙСТВ
    fn parse_props<'a>(&self, factory: &'a Factory) -> SpinnerProps {
        // Идентификатор виджета
        let widget_id = self.get_id();

        let size: f32 = factory.get_or_set(&widget_id, PROP_SIZE, 24.0);

        SpinnerProps { size }
    }
}

// -----------------------------------------------------------------------------
// Реализация Контракта Трейта WidgetBlueprint
// -----------------------------------------------------------------------------
impl WidgetBlueprint for SpinnerBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![PROP_SIZE]
    }

    /// Отрисовка анимированного спиннера iced_aw на холсте
    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые типизированные свойства
        let props = self.parse_props(factory);

        //let widget_id = self.get_id();

        let size = Length::Fixed(props.size);

        let base_spinner = Spinner::new().width(size).height(size);

        // -------------------------------------------------------------
        // РЕЖИМ КОНСТРУКТОРА: Событие выделения виджета
        // -------------------------------------------------------------
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            button(base_spinner)
                .width(size)
                .height(size)
                .padding(0)
                .style(|_theme, _status| button::Style {
                    background: None,
                    border: iced::Border::default(),
                    ..Default::default()
                })
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            base_spinner.into()
        };

        // В самом конце применяем магию подсветки из трейта в режиме конструктора
        apply_design_overlay(
            element,
            factory.is_design_mode(),
            selected_id,
            &self.get_id(),
        )
    }
}
