// -----------------------------------------------------------------------------
// Виджет 'toggler'
// Переключатель-триггер — Стилизованный свитч (тумблер) On/Off, альтернатива чекбоксу.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::widget::{toggler, mouse_area};
use iced::{Color, Element, Length, Pixels, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        TogglerBlueprint::WIDGET_TYPE,
        category:    CAT_INPUTS,
        constructor: create_toggler_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_toggler_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Toggler");
    Box::new(TogglerCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct TogglerCreator;

impl WidgetCreator for TogglerCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(TogglerBlueprint::new(id, "Включить опцию".to_string()))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct TogglerProps {
    pub is_checked: bool,

    pub label:       String,
    pub width:       iced::Length,
    pub flag_size:   iced::Pixels,
    pub spacing:     iced::Pixels,
    pub text_size:   iced::Pixels,
    pub font_family: String,
    pub font_weight: bool,
    pub font_style:  bool,

    pub active_color: iced::Color,
    pub bg_color:     iced::Color,
    pub fg_color:     iced::Color,
    pub text_color:   iced::Color,
}

#[derive(Debug, Clone /*, serde::Serialize, serde::Deserialize*/)]
pub struct TogglerBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<TogglerProps>,
}

impl HasCommonMeta for TogglerBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl TogglerBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "toggler";

    pub fn new(id: String, _label: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: TogglerProps::default().into(),
        }
    }

    // Возвращает чистые типизированные данные. Текст привязан к лайфтайму фабрики ('a)
    fn parse_props<'a>(&self, factory: &'a Factory) -> TogglerProps {
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = TogglerProps::default();

        let is_checked: bool   = factory.get_or_set(&widget_id, PROP_IS_CHECKED, def.is_checked);
        let label:      String = factory.get_or_set(&widget_id, PROP_LABEL,      def.label);

        // Извлекаем свойства через наши общие пуленепробиваемые методы завода
        let flag_size: Pixels = factory.get_or_set(&widget_id, PROP_FLAG_SIZE, def.flag_size); // default: 16.0
        let width:     Length = factory.get_or_set(&widget_id, PROP_WIDTH,     def.width);
        let spacing:   Pixels = factory.get_or_set(&widget_id, PROP_SPACING,   def.spacing);

        let font_family: String = factory.get_or_set(&widget_id, PROP_FONT_FAMILY, def.font_family);
        let text_size:   Pixels = factory.get_or_set(&widget_id, PROP_TEXT_SIZE,   def.text_size);
        let font_weight: bool   = factory.get_or_set(&widget_id, PROP_FONT_WEIGHT, def.font_weight);
        let font_style:  bool   = factory.get_or_set(&widget_id, PROP_FONT_STYLE,  def.font_style);

        let active_color: Color = factory.get_or_set(&widget_id, PROP_ACTIVE_COLOR, def.active_color);
        let bg_color:     Color = factory.get_or_set(&widget_id, PROP_BG_COLOR,     def.bg_color);
        let fg_color:     Color = factory.get_or_set(&widget_id, PROP_FG_COLOR,     def.fg_color);
        let text_color:   Color = factory.get_or_set(&widget_id, PROP_TEXT_COLOR,   def.text_color);

        TogglerProps {
            is_checked,
            label,
            width,
            flag_size,
            spacing,
            text_size,
            font_family,
            font_weight,
            font_style,
            active_color,
            bg_color,
            fg_color,
            text_color,
        }
    }
}

impl Default for TogglerProps {
    fn default() -> Self {
        Self {
            is_checked:   false,
            label:        String::from("Toggler"),
            width:        Length::Shrink,
            flag_size:    Pixels(16.0),
            spacing:      Pixels(10.0),
            text_size:    Pixels(14.0),
            font_family:  String::from("System"),
            font_weight:  false,
            font_style:   false,
            active_color: Color::TRANSPARENT,
            bg_color:     Color::TRANSPARENT,
            fg_color:     Color::TRANSPARENT,
            text_color:   Color::TRANSPARENT,
        }
    }
}

impl WidgetBlueprint for TogglerBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_IS_CHECKED,
            PROP_LABEL,
            PROP_FLAG_SIZE,
            PROP_WIDTH,
            //PROP_SIZE,
            PROP_SPACING,
            PROP_FONT_FAMILY,
            PROP_TEXT_SIZE,
            PROP_FONT_WEIGHT,
            PROP_FONT_STYLE,
            PROP_ACTIVE_COLOR,
            PROP_BG_COLOR,
            PROP_FG_COLOR,
            PROP_TEXT_COLOR,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые свойства через вынесенную функцию
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Собираем Font
        let current_font = utils_bp::create_iced_font(
            &props.font_family.as_str(),
            props.font_weight,
            props.font_style,
        );

        let element: Element<'a, Message, Theme> = {
            let id_clone = self.get_id().clone();
            let props_cl = props.clone();

            let mut w_toggler = toggler(props.is_checked)
                .label(props_cl.label)
                .width(props_cl.width) // Нативная ширина поддерживается тогглером
                .size(props_cl.flag_size)
                .spacing(props_cl.spacing) // Расстояние между кнопкой и текстом
                .font(current_font)
                .style(move |_theme, _status| {
                    let is_on = props_cl.is_checked;
                    let mut base_style = iced::widget::toggler::default(_theme, _status);

                    if is_on {
                        if props_cl.active_color != Color::TRANSPARENT {
                            base_style.background = iced::Background::Color(props_cl.active_color);
                        }
                    } else {
                        if props_cl.bg_color != Color::TRANSPARENT {
                            base_style.background = iced::Background::Color(props_cl.bg_color);
                        }
                    }
                    if props_cl.fg_color != Color::TRANSPARENT {
                        base_style.foreground = iced::Background::Color(props_cl.fg_color);    
                    }
                    if props_cl.text_color != Color::TRANSPARENT {
                        base_style.text_color = Some(props_cl.text_color);                        
                    }

                    base_style
                });

                // Применяем размер шрифта больше 0.0, иначе автоматически используется
                // системный размер шрифта по умолчанию ( 16.0 )
                if props.text_size.0 > 0.0 {
                    w_toggler = w_toggler.text_size(props.text_size);
                }

                if factory.is_design_mode() {
                    // -------------------------------------------------------------
                    // РЕЖИМ КОНСТРУКТОРА: Оборачиваем в mouse_area для выделения
                    // -------------------------------------------------------------

                    w_toggler = w_toggler.on_toggle(move |_new_state: bool| 
                        Message::MenuEvent(MenuAction::SelectWidget(self.get_id()))
                    );
                    
                    mouse_area(w_toggler)
                        .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                        .into()
                } else {
                    // -------------------------------------------------------------
                    // РЕЖИМ РАБОТЫ: Живой интерактивный переключатель
                    // -------------------------------------------------------------

                    w_toggler = w_toggler.on_toggle(move |new_state: bool| Message::UpdateProperty {
                        widget_id: id_clone.clone(),
                        property_key: PROP_IS_CHECKED, // Используем общий стандарт checked
                        value: PropertyValue::Boolean(new_state),
                    });

                    w_toggler.into()
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

        // Извлекаем текущие свойства тогглера из фабрики
        let current = self.parse_props(factory);
    
        // Получаем чистые дефолтные свойства для сравнения
        let default = TogglerProps::default();

        // Сравниваем текущие значения со значениями по умолчанию
        if current.is_checked != default.is_checked {
            prop_names.push(PROP_IS_CHECKED);
        }
        if current.label != default.label {
            prop_names.push(PROP_LABEL);
        }
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.flag_size != default.flag_size {
            prop_names.push(PROP_FLAG_SIZE);
        }
        if current.spacing != default.spacing {
            prop_names.push(PROP_SPACING);
        }
        if current.text_size != default.text_size {
            prop_names.push(PROP_TEXT_SIZE);
        }
        if current.font_family != default.font_family {
            prop_names.push(PROP_FONT_FAMILY);
        }
        if current.font_weight != default.font_weight {
            prop_names.push(PROP_FONT_WEIGHT);
        }
        if current.font_style != default.font_style {
            prop_names.push(PROP_FONT_STYLE);
        }
        if current.active_color != default.active_color {
            prop_names.push(PROP_ACTIVE_COLOR);
        }
        if current.bg_color != default.bg_color {
            prop_names.push(PROP_BG_COLOR);
        }
        if current.fg_color != default.fg_color {
            prop_names.push(PROP_FG_COLOR);
        }
        if current.text_color != default.text_color {
            prop_names.push(PROP_TEXT_COLOR);
        }

        prop_names
    }
}
