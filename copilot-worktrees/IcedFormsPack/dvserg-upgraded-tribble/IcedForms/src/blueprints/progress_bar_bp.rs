// -----------------------------------------------------------------------------
// Виджет 'progress_bar'
// Полоса прогресса — Визуальный индикатор выполнения задачи (от 0.0 до 100.0%).
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::border::Radius;
use iced::widget::{progress_bar};
use iced::{Color, Element, Length, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        ProgressBarBlueprint::WIDGET_TYPE,
        category:    CAT_INPUTS,
        constructor: create_progress_bar_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_progress_bar_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "ProgressBar");
    Box::new(ProgressBarCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct ProgressBarCreator;

impl WidgetCreator for ProgressBarCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        // Создаем чертеж прогресс-бара
        Rc::new(ProgressBarBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct ProgressBarProps {
    pub value_float:    f32,
    pub min:            f32,
    pub max:            f32,
    pub length:         iced::Length,
    pub girth:          f32,        // Толщина полосы
    pub is_vertical:    bool,       // Вертикальный вид
    pub bg_color:       Color,
    pub bar_color:      Color,
    pub border_radius:  Radius,
    pub border_width:   f32,
    pub border_color:   Color,
}

#[derive(Debug, Clone)]
pub struct ProgressBarBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<ProgressBarProps>,
}

impl HasCommonMeta for ProgressBarBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl ProgressBarBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "progress_bar";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: ProgressBarProps::default().into(),
        }
    }

    // Парсинг свойств прогресс-бара
    fn parse_props(&self, factory: &Factory) -> ProgressBarProps {
        let widget_id = self.get_id();

        // Получаем дефолтные свойства
        let def = ProgressBarProps::default();

        let min_raw: f32 = factory.get_or_set(&widget_id, PROP_MIN,     def.min);
        let max:     f32 = factory.get_or_set(&widget_id, PROP_MAX,     def.max);
        let val_raw: f32 = factory.get_or_set(&widget_id, PROP_VAL_F32, def.value_float);

        let length:  Length = factory.get_or_set(&widget_id, PROP_LENGTH, def.length);
        let girth:   f32    = factory.get_or_set(&widget_id, PROP_GIRTH,  def.girth);

        let is_vertical: bool  = factory.get_or_set(&widget_id, PROP_IS_VERTICAL, def.is_vertical);
        let bg_color:    Color = factory.get_or_set(&widget_id, PROP_BG_COLOR,    Color::from_rgb(0.9, 0.9, 0.9));
        let bar_color:   Color = factory.get_or_set(&widget_id, PROP_BAR_COLOR,   Color::from_rgb(0.09, 0.45, 0.74),);  // Океанический синий

        let border_radius: Radius = factory.get_or_set(&widget_id, PROP_BORDER_RADIUS, def.border_radius);
        let border_width:  f32    = factory.get_or_set(&widget_id, PROP_BORDER_WIDTH,  def.border_width);
        let border_color:  Color  = factory.get_or_set(&widget_id, PROP_BORDER_COLOR,  Color::from_rgb(0.7, 0.7, 0.7),); // Цвет рамки чуть темнее фона

        // Корректируем диапазон, чтобы избежать паники в RangeInclusive (min <= max)
        // Нормализуем значение в диапазон [min .. max]
        let min = if min_raw > max { max } else { min_raw };
        let value_float = val_raw.clamp(min, max);

        ProgressBarProps {
            value_float,
            min,
            max,
            length,
            girth,
            is_vertical,
            bg_color,
            bar_color,
            border_radius,
            border_width,
            border_color,
        }
    }
}

impl Default for ProgressBarProps {
    fn default() -> Self {
        ProgressBarProps {
            // ЗНАЧЕНИЯ: Диапазон от 0.0 до 100.0, текущий прогресс на нуле
            value_float:    0.0_f32,
            min:            0.0_f32,
            max:            100.0_f32,

            // ГАБАРИТЫ: По умолчанию длина заполняет контейнер (Fill),
            // а толщина полосы (girth) в Iced составляет 12.0 пикселей.
            length:         Length::Fill,
            girth:          12.0_f32,
            is_vertical:    false, // По умолчанию горизонтальный

            // СТИЛЬ И ЦВЕТА: Системный прогресс-бар полностью завязан на палитру темы.
            // Чтобы цвета не лезли в JSON, используем маркер TRANSPARENT для перегрузки.
            bg_color:       Color::TRANSPARENT, // Будет palette.background/secondary
            bar_color:      Color::TRANSPARENT, // Будет palette.primary (синий)
            
            // В Iced по умолчанию углы полосы слегка скруглены
            border_radius:  2.0.into(),
            border_width:   0.0,
            border_color:   Color::TRANSPARENT,
        }
    }
}

impl WidgetBlueprint for ProgressBarBlueprint {
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
            PROP_LENGTH,
            PROP_GIRTH,
            PROP_IS_VERTICAL,
            PROP_BG_COLOR,
            PROP_BAR_COLOR,
            PROP_BORDER_WIDTH,
            PROP_BORDER_COLOR,
            PROP_BORDER_RADIUS,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>, // Сквозная передача выбранного ID виджета
    ) -> Element<'a, Message, Theme> {
        // Получаем настройки ProgressBar из фабрики через вынесенную функцию парсинга
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Корректируем диапазон, чтобы избежать паники в RangeInclusive (min <= max)
        let min_val = if props.min > props.max {
            props.max
        } else {
            props.min
        };

        // Гарантируем, что значение находится строго внутри границ
        let current_value = props.value_float.clamp(min_val, props.max);

        // Создаем базовый виджет ProgressBar Iced 0.14
        // Нативно применяем ширину через .length() и толщину через .girth()
        let props_cl = props.clone();
        let mut base_progress = progress_bar(min_val..=props.max, current_value)
            .length(props_cl.length.clone())   // Управляет размером полосы (длиной)
            .girth(props_cl.girth)             // Нативно управляет высотой/толщиной полосы
            .style(move |_theme: &Theme| {
                let palette = _theme.extended_palette();

                let bg_color  = if props_cl.bg_color  == Color::TRANSPARENT { palette.background.strong.color } else { props_cl.bg_color };
                let bar_color = if props_cl.bar_color == Color::TRANSPARENT { palette.primary.base.color } else { props_cl.bar_color };

                progress_bar::Style {
                    // Задаем цвет и рамку для задней дорожки-трека (незаполненной части)
                    background: iced::Background::Color(bg_color/*props.bg_color*/),

                    // Задаем цвет для заполняющей линии прогресса
                    bar: iced::Background::Color(bar_color),

                    // В Iced 0.14 рамка и скругление описываются структурой iced::Border
                    border: iced::Border {
                        color:  props_cl.border_color,
                        width:  props_cl.border_width,
                        radius: props_cl.border_radius.into(), // Конвертируем f32 в тип Radius
                    },
                }
            });

        //let mut width = props.length.clone();
        //let mut height = props.girth.clone();

        if props.is_vertical {
            base_progress = base_progress.vertical();

            //width = iced::Length::Fixed(props.girth.clone());
            //height = props.progress_bar_size.clone();
        }

        // Формируем элемент в зависимости от режима конструктора
        let element: Element<'a, Message, Theme> = if factory.is_design_mode(){
            // В РЕЖИМЕ КОНСТРУКТОРА:
           // Чтобы дизайнер мог кликнуть по абсолютно спозиционированному элементу,
            // оборачиваем его в mouse_area для выбора в дереве проекта
            iced::widget::mouse_area(base_progress)
                .interaction(iced::mouse::Interaction::Pointer)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            // В РЕЖИМЕ РАБОТЫ: Обычный системный прогресс-бар
            base_progress.into()
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

        // 1. Извлекаем текущие свойства прогресс-бара из фабрики
        let current = self.parse_props(factory);
        
        // 2. Получаем чистые дефолтные свойства для сравнения
        let default = ProgressBarProps::default();

        // 3. Сравниваем свойства в точном соответствии с editable_properties
        if current.value_float != default.value_float {
            prop_names.push(PROP_VAL_F32);
        }
        if current.min != default.min {
            prop_names.push(PROP_MIN);
        }
        if current.max != default.max {
            prop_names.push(PROP_MAX);
        }
        if current.length != default.length {
            prop_names.push(PROP_LENGTH);
        }
        if current.girth != default.girth {
            prop_names.push(PROP_GIRTH);
        }
        if current.is_vertical != default.is_vertical {
            prop_names.push(PROP_IS_VERTICAL);
        }
        if current.bg_color != default.bg_color {
            prop_names.push(PROP_BG_COLOR);
        }
        if current.bar_color != default.bar_color {
            prop_names.push(PROP_BAR_COLOR);
        }
        if current.border_radius != default.border_radius {
            prop_names.push(PROP_BORDER_RADIUS);
        }
        if current.border_width != default.border_width {
            prop_names.push(PROP_BORDER_WIDTH);
        }
        if current.border_color != default.border_color {
            prop_names.push(PROP_BORDER_COLOR);
        }

        prop_names
    }    
}
