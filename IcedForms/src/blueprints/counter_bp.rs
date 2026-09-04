use crate::widgets;
//use crate::widgets::compact_counter::{Axis, CompactCounter, CounterStyle, CounterTheme};
//use std::cell::{RefCell};
use iced::Element;
use iced::Theme;
use iced::widget::button;

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: CounterBlueprint::WIDGET_TYPE, //"counter",
        category: CAT_INPUTS,
        constructor: create_counter_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_counter_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Counter");
    Box::new(CounterCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct CounterCreator;

impl WidgetCreator for CounterCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(CounterBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных числовых свойств
#[derive(Debug, Clone)]
pub struct CounterProps {
    pub value: f32,
    pub min:   f32,
    pub max:   f32,
    pub step:  f32,
    pub scale: f32,
    pub size:  f32,
}

#[derive(Debug, Clone /*, serde::Serialize, serde::Deserialize*/)]
pub struct CounterBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<CounterProps>,
}

impl HasCommonMeta for CounterBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl CounterBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "counter";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: CounterProps::default().into(),
        }
    }

    fn parse_props(&self, factory: &Factory) -> CounterProps {
        // Идентификатор виджета
        let widget_id = self.get_id();

        let value_raw: f32 = factory.get_or_set(&widget_id, PROP_VAL_F32, 0.0);
        let min_raw:   f32 = factory.get_or_set(&widget_id, PROP_MIN,     0.0);
        let max_val:   f32 = factory.get_or_set(&widget_id, PROP_MAX,     1000.0);
        let step:      f32 = factory.get_or_set(&widget_id, PROP_STEP,    1.0);
        let scale:     f32 = factory.get_or_set(&widget_id, PROP_SCALE,   1.5);
        // Базовый размер
        let size:      f32 = 20.0;

        // Проверка минимальной границы
        // Значение должно входить в диапазон
        let min_val = if min_raw > max_val { max_val } else { min_raw };
        let value   = value_raw.clamp(min_val, max_val);

        CounterProps {
            value,
            min: min_val,
            max: max_val,
            step,
            scale,
            size,
        }
    }
}

impl Default for CounterProps {
    // Присваиваем дефолтные значения для контроля пропущенных значений и значений по умолчанию в инспекторе
    fn default() -> CounterProps {
        CounterProps {
            value:  0.0_f32,
            min:    0.0_f32,
            max:    1000.0_f32,
            step:   1.0_f32,
            scale:  1.5_f32,
            size:   20.0,
        }
    }
}
// -----------------------------------------------------------------------------
// Реализация Трейт-Контракта WidgetBlueprint
// -----------------------------------------------------------------------------
//#[typetag::serde]
impl WidgetBlueprint for CounterBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![PROP_VALUE, PROP_MIN, PROP_MAX, PROP_STEP, PROP_SCALE]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые типизированные свойства
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Рассчитываем пропорциональные размеры на базе коэффициента scale
        let _text_size   = 16.0 * props.scale;
        let _btn_padding = (5.0 * props.scale) as u16;
        let _box_width   = 40.0 * props.scale;
        let _row_spacing =  5.0 * props.scale;

        let _width = 80.0 * props.scale;
        let height = props.size * props.scale;

        let w_counter = widgets::compact_counter(props.value)
            // тестируем по горизонталь
            .width(iced::Length::Shrink)
            .height(iced::Length::Fixed(height))
            .range(props.min, props.max, props.step);

        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // -------------------------------------------------------------
            // РЕЖИМ РАБОТЫ: Проектирование
            // -------------------------------------------------------------
            button(w_counter)
                .padding(0)
                .style(|_theme, _status| button::Style {
                    background: None,
                    border: iced::Border::default(),
                    ..button::Style::default()
                })
                //.on_press(Message::SelectWidget { widget_id: self.get_id().clone() })
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            // -------------------------------------------------------------
            // РЕЖИМ РАБОТЫ: Интерактивный
            // -------------------------------------------------------------
            w_counter
                .on_change(move |new_value| Message::UpdateProperty {
                    widget_id: self.get_id(),
                    property_key: PROP_VAL_F32,
                    value: PropertyValue::Float(new_value),
                })
                .into()
        };

        // В самом конце применяем магию подсветки из трейта в одну строчку!
        apply_design_overlay(
            element,
            factory.is_design_mode(),
            selected_id,
            &self.get_id(),
        )
    }
}
