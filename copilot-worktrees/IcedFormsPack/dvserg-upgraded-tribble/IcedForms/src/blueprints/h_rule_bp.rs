// -----------------------------------------------------------------------------
// Виджет 'rule::Horizontal'
// Графический виджет горизонтальной разделительной линии.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::widget::{rule, mouse_area};
use iced::{Element, Pixels, Color, Theme};
use iced::border::Radius;
use iced::widget::rule::FillMode;

use crate::core::*;
use crate::core::{MenuAction, Message};


// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        HRuleBlueprint::WIDGET_TYPE,
        category:    CAT_BASE,
        constructor: create_h_rule_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_h_rule_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "HRule");
    Box::new(HRuleCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct HRuleCreator;

impl WidgetCreator for HRuleCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(HRuleBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct HRuleProps {
    pub thickness:     Pixels,
    pub fill_percent:  f32,         // Размер в % от полной ширины
    pub color:         Color,
    pub border_radius: Radius,
}

#[derive(Debug, Clone)]
pub struct HRuleBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<HRuleProps>,
}

impl HasCommonMeta for HRuleBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl HRuleBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "h_rule";

    pub fn new(id: String) -> Self {
        Self {
            meta: CommonWidgetMeta::new(id),
            //props: HRuleProps::default().into(),
        }
    }

    pub fn parse_props(&self, factory: &Factory) -> HRuleProps {
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = HRuleProps::default();

        let thickness:     Pixels = factory.get_or_set(&widget_id, PROP_THICKNESS,     Pixels(2.0));
        let fill_percent:  f32    = factory.get_or_set(&widget_id, PROP_FILL_PERCENT,  def.fill_percent);
        let color:         Color  = factory.get_or_set(&widget_id, PROP_COLOR,         def.color);
        let border_radius: Radius = factory.get_or_set(&widget_id, PROP_BORDER_RADIUS, def.border_radius);

        HRuleProps { 
            thickness,
            fill_percent,
            color,
            border_radius, 
        }
    }
}

impl Default for HRuleProps {
    // Присваиваем дефолтные значения для контроля пропущенных значений и значений по умолчанию в инспекторе
    fn default() -> HRuleProps {
        HRuleProps {
            thickness:     Pixels(1.0),
            fill_percent:  100.0_f32,
            color:         Color::TRANSPARENT,
            border_radius: 0.0.into(),
        }
    }
}

impl WidgetBlueprint for HRuleBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_THICKNESS,
            PROP_FILL_PERCENT,
            PROP_COLOR,
            PROP_BORDER_RADIUS,
        ]
    }

    // Реализация апдейта собственных свойств блюпринта из VTable
    //crate::impl_refresh_props!(VRuleBlueprint, HRuleProps);

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые свойства через вынесенную функцию
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Создаем стандартный горизонтальный разделитель Iced 0.14
        let base_rule = rule::horizontal(props.thickness)
            .style(move |theme: &Theme| {
                // Получаем схему по умолчанию для rule
                let mut base_style = rule::default(theme);

                if props.color != Color::TRANSPARENT {
                    base_style.color = props.color;
                }
                base_style.fill_mode = if props.fill_percent == 100.0 { FillMode::Full } else { FillMode::Percent(props.fill_percent) };
                base_style.radius    = props.border_radius;

                base_style
            });

        // Формируем элемент в зависимости от режима
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // В РЕЖИМЕ КОНСТРУКТОРА:
            // Оборачиваем в mouse_area для перехвата клика инспектором
            mouse_area(base_rule)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            // В РЕЖИМЕ РАБОТЫ: Обычная пассивная разделительная линия
            base_rule.into()
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

        let current = self.parse_props(factory);
        let default = HRuleProps::default();

        if current.thickness != default.thickness {
            prop_names.push(PROP_THICKNESS);
        }
        if current.fill_percent != default.fill_percent {
            prop_names.push(PROP_FILL_PERCENT);
        }
        if current.color != default.color {
            prop_names.push(PROP_COLOR);
        }
        if current.border_radius != default.border_radius {
            prop_names.push(PROP_BORDER_RADIUS);
        }

        prop_names
    }
}
