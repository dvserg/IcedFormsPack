// -----------------------------------------------------------------------------
// Виджет 'slider'
// Ползунок — Горизонтальная полоса с бегунком для плавного выбора
// числовых значений в диапазоне.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::border::Radius;
use iced::widget::{slider};
use iced::{Element, Length, Color, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: SliderBlueprint::WIDGET_TYPE, //"slider",
        category: CAT_INPUTS,
        constructor: create_slider_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_slider_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Slider");
    Box::new(SliderCreator)
}

// Конструктор blueprint для виджета

#[derive(Debug, Clone)]
pub struct SliderCreator;

impl WidgetCreator for SliderCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(SliderBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct SliderProps {
    pub value_float: f32,           // Текущее числовое значение ("value")
    pub min:         f32,           // Минимальная граница ползунка ("min")
    pub max:         f32,           // Максимальная граница ползунка ("max")
    pub step:        f32,           // Дискретность/шаг ползунка ("step")
    pub width:       Length,        // Стратегия ширины Iced ("width")

    pub bg_color:      Color,       // Цвет дорожки
    pub border_width:  f32,     
    pub border_color:  Color,   
    pub border_radius: Radius,

    pub rail_width:    f32,         // Ширина полосы

    pub is_handle_rectangle: bool,  // Тип бегунка (круг/квадрат)
    pub handle_color:        Color, // Цвет бегунка
    //pub handle_border
}

#[derive(Debug, Clone)]
pub struct SliderBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<SliderProps>,
}

impl HasCommonMeta for SliderBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl SliderBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "slider";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: SliderProps::default().into(),

        }
    }

    // Финальная атомарная функция парсинга свойств ползунка
    fn parse_props(&self, factory: &Factory) -> SliderProps {
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = SliderProps::default();

        // Безопасно получаем текущее значение и зажимаем его в рамки [min, max]
        let value_raw: f32 = factory.get_or_set(&widget_id, PROP_VAL_F32, def.value_float);

        // Извлекаем границы диапазона и шаг
        let min_raw: f32 = factory.get_or_set(&widget_id, PROP_MIN,  def.min);
        let max:     f32 = factory.get_or_set(&widget_id, PROP_MAX,  def.max);
        let step:    f32 = factory.get_or_set(&widget_id, PROP_STEP, def.step);

        // Адаптивные размеры (слайдеры по умолчанию растягивают по ширине Fill)
        let width: Length = factory.get_or_set(&widget_id, PROP_WIDTH, def.width);

        // Корректируем диапазон, чтобы избежать паники в RangeInclusive (min <= max)
        // Корректируем значение в границы диапазона
        let min = if min_raw > max { max } else { min_raw };
        let value_float = value_raw.clamp(min, max);

        // Стиль
        let bg_color:       Color  = factory.get_or_set(&widget_id, PROP_BG_COLOR,      def.bg_color);
        let border_radius:  Radius = factory.get_or_set(&widget_id, PROP_BORDER_RADIUS, def.border_radius);
        let border_width:   f32    = factory.get_or_set(&widget_id, PROP_BORDER_WIDTH,  def.border_width);
        let border_color:   Color  = factory.get_or_set(&widget_id, PROP_BORDER_COLOR,  def.border_color);

        let rail_width:     f32    = factory.get_or_set(&widget_id, PROP_RAIL_WIDTH, def.rail_width);

        let is_handle_rectangle: bool  = factory.get_or_set(&widget_id, PROP_IS_HANDLE_RECTANGLE, def.is_handle_rectangle);
        let handle_color:        Color = factory.get_or_set(&widget_id, PROP_ACTIVE_COLOR,        def.handle_color);

        SliderProps {
            value_float,
            min,
            max,
            step,
            width,

            bg_color,
            border_width,
            border_color,
            border_radius,

            rail_width,

            is_handle_rectangle,
            handle_color,
        }
    }
}

impl Default for SliderProps {
    fn default() -> Self {
        SliderProps {
            // БИЗНЕС-ЛОГИКА: Диапазон от 0.0 до 100.0, текущее значение на нуле
            value_float:        0.0_f32,
            min:                0.0_f32,
            max:                100.0_f32,
            
            // Шаг изменения значения по умолчанию равен 1.0
            step:               1.0_f32,

            // ГАБАРИТЫ: Слайдер по умолчанию растягивается на 100% ширины контейнера
            width:              Length::Fill,

            bg_color:           Color::TRANSPARENT,
            border_width:       0.0_f32,
            border_color:       Color::TRANSPARENT,
            border_radius:      Radius::from(2.0_f32),

            // Ширина полосы
            rail_width:         4.0_f32,

            is_handle_rectangle: false,
            handle_color:        Color::TRANSPARENT,
        }
    }
}

impl WidgetBlueprint for SliderBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_VAL_F32, 
            PROP_MIN, 
            PROP_MAX, 
            PROP_STEP, 
            PROP_WIDTH,

            PROP_RAIL_WIDTH,

            PROP_BG_COLOR,
            PROP_BORDER_WIDTH,
            PROP_BORDER_COLOR,
            PROP_BORDER_RADIUS,
           
            PROP_IS_HANDLE_RECTANGLE,
            PROP_ACTIVE_COLOR
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>, // Сквозная передача выбранного ID
    ) -> Element<'a, Message, Theme> {
        // Получаем все свойства ползунка через вынесенную функцию парсинга
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Рендерим виджет в зависимости от выбранного режима
        let element: Element<'a, Message, Theme> = {
            let id_clone = self.get_id().clone();
            let mut w_slider = slider(props.min..=props.max, props.value_float, move |new_val| {
                if factory.is_design_mode() {
                    Message::MenuEvent(MenuAction::SelectWidget(id_clone.clone()))
                } else {
                    Message::UpdateProperty {
                        widget_id: id_clone.clone(),
                        property_key: PROP_VAL_F32,
                        value: PropertyValue::Float(new_val),
                    }
                }
            })
            .step(props.step)
            .width(props.width.clone());


            w_slider = w_slider.style(move |theme: &Theme, status: slider::Status| {
                let palette = theme.extended_palette();

                let mut base_style = slider::default(theme, status);
               
                // Ширина полосы
                base_style.rail.width = props.rail_width;

                if props.bg_color != Color::TRANSPARENT {
                    base_style.rail.backgrounds = (iced::Background::Color(props.bg_color), palette.background.strong.color.into());
                }
                if props.border_color != Color::TRANSPARENT {
                    base_style.rail.border.color  = props.border_color;
                    base_style.rail.border.width  = props.border_width;
                    base_style.rail.border.radius = props.border_radius;
                }
                // Тип ползунка
                if props.is_handle_rectangle {
                    base_style.handle.shape = slider::HandleShape::Rectangle { 
                        width: 8, 
                        border_radius: 2.0.into() 
                    };
                }                
                // Цвет ползунка
                if props.handle_color != Color::TRANSPARENT {
                    base_style.handle.background = props.handle_color.into();
                }

                base_style
            });

            if factory.is_design_mode() {
                // -------------------------------------------------------------
                // РЕЖИМ КОНСТРУКТОРА
                // -------------------------------------------------------------
                iced::widget::mouse_area(w_slider)
                    .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    .into()
            } else {
                w_slider.into()
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

        // Извлекаем текущие свойства слайдера из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = SliderProps::default();

        // Сравниваем свойства в точном соответствии с editable_properties
        if current.value_float != default.value_float {
            prop_names.push(PROP_VAL_F32);
        }
        if current.min != default.min {
            prop_names.push(PROP_MIN);
        }
        if current.max != default.max {
            prop_names.push(PROP_MAX);
        }
        if current.step != default.step {
            prop_names.push(PROP_STEP);
        }
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.rail_width != default.rail_width {
            prop_names.push(PROP_RAIL_WIDTH);
        }
        if current.bg_color != default.bg_color {
            prop_names.push(PROP_BG_COLOR);
        }
        if current.border_width != default.border_width {
            prop_names.push(PROP_BORDER_WIDTH);
        }
        if current.border_color != default.border_color {
            prop_names.push(PROP_BORDER_COLOR);
        }
        if current.border_radius != default.border_radius {
            prop_names.push(PROP_BORDER_RADIUS);
        }
        if current.is_handle_rectangle != default.is_handle_rectangle {
            prop_names.push(PROP_IS_HANDLE_RECTANGLE);
        }
        // Обратите внимание: свойство называется PROP_ACTIVE_COLOR, 
        // но сравнивается с полем структуры handle_color
        if current.handle_color != default.handle_color {
            prop_names.push(PROP_ACTIVE_COLOR);
        }

        prop_names
    }

}
