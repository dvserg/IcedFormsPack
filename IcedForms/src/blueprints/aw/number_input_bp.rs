// -----------------------------------------------------------------------------
// Библиотека AW
// Виджет 'number_input'
//
// -----------------------------------------------------------------------------
use iced::alignment::Horizontal;
use iced::widget::{button, text};
use iced::{Element, Length, Padding, Pixels, Theme};
use iced_aw::widget::NumberInput;
use std::rc::Rc;

use crate::core::*;

// -----------------------------------------------------------------------------
// Автоматическая Регистрация через Inventory
// -----------------------------------------------------------------------------
inventory::submit! {
    AutoRegisteredWidget {
        name: NumberInputBlueprint::WIDGET_TYPE,
        category: CAT_INPUTS,
        constructor: create_number_input_creator,
    }
}

fn create_number_input_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "NumberInput");
    Box::new(NumberInputCreator)
}

#[derive(Debug)]
pub struct NumberInputCreator;

impl WidgetCreator for NumberInputCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(NumberInputBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

impl HasCommonMeta for NumberInputBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

// Свойства виджета
#[derive(Debug)]
pub struct NumberInputProps {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub width: Length,
    //pub height:    Length,
    pub padding: Padding,
    pub line_height: f32,
    pub align_x: Horizontal,

    pub font_family: String,
    pub text_size: Pixels,
    pub font_weight: bool,
    pub font_style: bool,

    pub ignore_scroll: bool,
    pub ignore_buttons: bool,
}

#[derive(Debug)]
pub struct NumberInputBlueprint {
    pub meta: CommonWidgetMeta,
}

impl NumberInputBlueprint {
    pub const WIDGET_TYPE: &'static str = "number_input";

    pub fn new(id: String) -> Self {
        Self {
            meta: CommonWidgetMeta::new(id),
        }
    }

    fn parse_props(&self, factory: &Factory) -> NumberInputProps {
        // Идентификатор виджета
        let widget_id = self.get_id();

        let value_raw: f32 = factory.get_or_set(&widget_id, PROP_VAL_F32, 0.0);
        let min_raw: f32 = factory.get_or_set(&widget_id, PROP_MIN, 0.0);
        let max_val: f32 = factory.get_or_set(&widget_id, PROP_MAX, 1000.0);
        let step: f32 = factory.get_or_set(&widget_id, PROP_STEP, 1.0);
        //let scale:     f32 = factory.get_or_set(&widget_id, PROP_SCALE,   1.5);
        // Базовый размер
        //let size:      f32 = 20.0;
        let width: Length = factory.get_or_set(&widget_id, PROP_WIDTH, Length::Fixed(120.0));
        //let height:    Length  = factory.get_or_set(&widget_id, PROP_HEIGHT,    Length::Fixed(32.0));
        let padding: Padding =
            factory.get_or_set(&widget_id, PROP_PADDING, Padding::from([8.0, 16.0]));
        let align_x: Horizontal = factory.get_or_set(&widget_id, PROP_ALIGN_X, Horizontal::Left);

        let font_family: String =
            factory.get_or_set(&widget_id, PROP_FONT_FAMILY, String::from("System"));
        let text_size: Pixels = factory.get_or_set(&widget_id, PROP_TEXT_SIZE, Pixels(16.0));
        let font_weight: bool = factory.get_or_set(&widget_id, PROP_FONT_WEIGHT, false);
        let font_style: bool = factory.get_or_set(&widget_id, PROP_FONT_STYLE, false);

        let line_height: f32 = factory.get_or_set(&widget_id, PROP_LINE_HEIGHT, 1.0);

        let ignore_scroll: bool = factory.get_or_set(&widget_id, PROP_IGNORE_SCROLL, false);
        let ignore_buttons: bool = factory.get_or_set(&widget_id, PROP_IGNORE_BUTTONS, false);

        // Проверка минимальной границы
        // Значение должно входить в диапазон
        let min_val = if min_raw > max_val { max_val } else { min_raw };
        let value = value_raw.clamp(min_val, max_val);

        NumberInputProps {
            value,
            min: min_val,
            max: max_val,
            step,

            width,
            padding,
            align_x,

            font_family,
            text_size,
            font_weight,
            font_style,

            line_height,
            ignore_scroll,
            ignore_buttons,
        }
    }
}

// -----------------------------------------------------------------------------
// Реализация Трейт-Контракта WidgetBlueprint
// -----------------------------------------------------------------------------
impl WidgetBlueprint for NumberInputBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_VALUE,
            PROP_MIN,
            PROP_MAX,
            PROP_STEP,
            PROP_WIDTH,
            PROP_PADDING,
            PROP_ALIGN_X,
            PROP_FONT_FAMILY,
            PROP_TEXT_SIZE,
            PROP_FONT_WEIGHT,
            PROP_FONT_STYLE,
            PROP_LINE_HEIGHT,
            PROP_IGNORE_SCROLL,
            PROP_IGNORE_BUTTONS,
        ]
    }

    /// Отрисовка интерактивного счетчика iced_aw на холсте верстака
    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые типизированные свойства
        let props = self.parse_props(factory);

        //let widget_id = self.get_id();

        let current_font = utils_bp::create_iced_font(
            &props.font_family.as_str(),
            props.font_weight,
            props.font_style,
        );

        // Инициализируем сам виджет NumberInput из библиотеки iced_aw
        // Мы передаем текущее значение, лимиты и шаг. При изменении (клик на стрелочки или ввод)
        // шлем наше дженерик-сообщение WidgetAction обратно в ядро диспетчера
        //let id_clone = widget_id.clone();
        let mut base_number_input =
            NumberInput::new(&props.value, props.min..=props.max, move |new_value| {
                Message::UpdateProperty {
                    widget_id: self.get_id(),
                    property_key: PROP_VAL_F32,
                    value: PropertyValue::Float(new_value),
                }
            })
            .step(props.step)
            .width(props.width)
            .padding(props.padding)
            .align_x(props.align_x)
            .font(current_font)
            .ignore_scroll(props.ignore_scroll)
            .ignore_buttons(props.ignore_buttons);

        // Применяем размер шрифта больше 0.0, иначе автоматически используется
        // системный размер шрифта по умолчанию ( 16.0 )
        if props.text_size.0 > 0.0 {
            base_number_input = base_number_input.set_size(props.text_size);
        }
        if props.line_height > 0.0 {
            base_number_input =
                base_number_input.line_height(text::LineHeight::Relative(props.line_height));
        }

        // -------------------------------------------------------------
        // РЕЖИМ КОНСТРУКТОРА: Событие выделения виджета
        // -------------------------------------------------------------
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            button(base_number_input)
                .width(props.width)
                .height(Length::Shrink)
                .padding(0)
                .style(|_theme, _status| button::Style {
                    background: None,
                    border: iced::Border::default(),
                    ..Default::default()
                })
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            base_number_input.into()
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
