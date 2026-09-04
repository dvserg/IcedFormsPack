// -----------------------------------------------------------------------------
// Виджет 'text_input'
// Поле ввода — Однострочное поле для ввода текста с поддержкой плейсхолдеров,
// выделения, копирования/вставки и фокуса. Генерирует событие `on_input`.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::alignment::Horizontal;
use iced::widget::{text, text_input, mouse_area};
use iced::{Element, Length, Padding, Pixels, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        InputBlueprint::WIDGET_TYPE, //"input",
        category:    CAT_INPUTS,
        constructor: create_input_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_input_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Input");
    Box::new(InputCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct InputCreator;

impl WidgetCreator for InputCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        // Создаем чертеж виджета
        Rc::new(InputBlueprint::new(id, "Введите текст...".to_string()))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct InputProps {
    pub action: String,

    // ДАННЫЕ И ЛОГИКА
    pub value:       String,        // Текущее текстовое значение (владеющая строка для мутации в Iced)
    pub placeholder: String,        // Текст-подсказка, отображаемый в пустом поле
    pub secure:      bool,          // Флаг маскирования ввода (режим скрытия пароля)

    // ОГРАНИЧЕНИЯ И ГЕОМЕТРИЯ
    pub width:       Length,            
    pub padding:     Padding,           

    pub line_height: f32,

    // ТИПОГРАФИКА И ВНУТРЕННЯЯ РАЗМЕТКА
    pub font_family: String,        // Семейство шрифта вводимого текста
    pub text_size:   Pixels,        // Размер шрифта вводимого текста и подсказки
    pub font_weight: bool,          // Начертание / жирность шрифта текста
    pub font_style:  bool,
    pub align_x:     Horizontal,    // Положение текста по горизонтали
}

#[derive(Debug, Clone)]
pub struct InputBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<InputProps>,
}

impl HasCommonMeta for InputBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl InputBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "input";

    pub fn new(id: String, _placeholder: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: InputProps::default().into(),
        }
    }

    // Парсинг свойств
    fn parse_props<'a>(&self, factory: &'a Factory) -> InputProps {
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = InputProps::default();

        // (*) Некоторые исходные размеры устанавливаем отличными от дефолтных для работы конструктора

        let action: String = factory.get_or_set(&widget_id, PROP_ACTION, def.action);

        // ДАННЫЕ И ПОДСКАЗКИ (Через защищенные фабричные методы)
        // Для value создаем копию String, так как Iced требует владения для мутации в рантайме
        let placeholder: String = factory.get_or_set(&widget_id, PROP_PLACEHOLDER, "Введите текст..".to_string());

        let value_str:   String = factory.get_or_set(&widget_id, PROP_VALUE, def.value);
        let value = value_str.to_string();

        // ПОВЕДЕНИЕ И ЛОГИКА (Через проверку флагов в field_values)
        let secure: bool = factory.get_or_set(&widget_id, PROP_SECURE, def.secure);

        // АДАПТИВНЫЕ РАЗМЕРЫ
        let width:   Length  = factory.get_or_set(&widget_id, PROP_WIDTH,   def.width);
        let padding: Padding = factory.get_or_set(&widget_id, PROP_PADDING, def.padding);

        let line_height: f32 = factory.get_or_set(&widget_id, PROP_LINE_HEIGHT, def.line_height);

        // ТИПОГРАФИКА И ВНУТРЕННЯЯ РАЗМЕТКА (Фабричные методы-хелперы)
        let font_family: String = factory.get_or_set(&widget_id, PROP_FONT_FAMILY, def.font_family);
        let text_size:   Pixels = factory.get_or_set(&widget_id, PROP_TEXT_SIZE,   def.text_size);          // Не может быть 0
        let font_weight: bool   = factory.get_or_set(&widget_id, PROP_FONT_WEIGHT, def.font_weight);
        let font_style:  bool   = factory.get_or_set(&widget_id, PROP_FONT_STYLE,  def.font_style);

        // Выравнивание по горизонтали
        let align_x: Horizontal = factory.get_or_set(&widget_id, PROP_ALIGN_X, def.align_x);

        InputProps {
            action,
            placeholder,
            value,
            secure,
            width,
            padding,
            line_height,
            font_family,
            text_size,
            font_weight,
            font_style,
            align_x,
        }
    }
}


impl Default for InputProps {
    // Присваиваем дефолтные значения для контроля пропущенных значений и значений по умолчанию в инспекторе
    fn default() -> InputProps {
        let set = iced::Settings::default();

        InputProps {
            action:         "".to_string(),
            placeholder:    "".to_string(),
            value:          "".to_string(),
            secure:         false,
            width:          Length::Fill,
            padding:        Padding::new(5.0),
            line_height:    1.0_f32,
            font_family:    "System".to_string(),
            text_size:      set.default_text_size,
            font_weight:    false,
            font_style:     false,            
            align_x:        Horizontal::Left,
        }
    }
}

impl WidgetBlueprint for InputBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Переопределяем метод: инпут ОБЯЗАН принимать фокус ввода!
    //fn can_accept_focus(&self) -> bool {
    //    true
    //}

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_ACTION,
            PROP_PLACEHOLDER,
            PROP_VALUE,
            PROP_SECURE,
            PROP_WIDTH,
            PROP_PADDING,
            PROP_FONT_FAMILY,
            PROP_TEXT_SIZE,
            PROP_FONT_WEIGHT,
            PROP_FONT_STYLE,
            PROP_LINE_HEIGHT,
            PROP_ALIGN_X,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем все свойства виджета через вынесенную функцию парсинга
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Собираем Font
        let current_font = utils_bp::create_iced_font(
            &props.font_family.as_str(),
            props.font_weight,
            props.font_style,
        );

        // Логика текста для режима конструктора: если поле пароля и не пустое — маскируем звездочками
        /*
        let display_text = if props.value.is_empty() {
            props.placeholder.to_string()
        } else if props.secure {
            "•".repeat(props.value.chars().count())
        } else {
            props.value.clone()
        };
        */

        // Создаем базовый элемент (в зависимости от режима)
        let element: Element<'a, Message, Theme> = {
            // РЕЖИМ РАБОТЫ: Обычное интерактивное поле ввода
            let id_clone = self.get_id().clone();
                    
            // Если disabled — не передаем замыкание on_input, чтобы заблокировать ввод в Iced
            let mut w_input = text_input(&props.placeholder, &props.value)
                .width(props.width.clone())
                //.size(props.text_size)
                .align_x(props.align_x)
                .padding(props.padding)
                .font(current_font)
                .secure(props.secure)
                .line_height(text::LineHeight::Relative(props.line_height));

            // Применяем размер шрифта больше 0.0, иначе автоматически используется системный
            if props.text_size.0 > 0.0 {
                w_input = w_input.size(props.text_size);
            }
            // Высота строки текста должна быть болше нуля
            if props.line_height > 0.0 {
                w_input = w_input.line_height(text::LineHeight::Relative(props.line_height));
            }

            if factory.is_design_mode() {
                // -------------------------------------------------------------
                // РЕЖИМ КОНСТРУКТОРА: Пассивный режим text_input
                // -------------------------------------------------------------
                // Оборачиваем в mouse_area и навешиваем на оба элемента событие выделения при клике
                mouse_area(w_input)
                    .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    .on_release(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    //.interaction(iced::mouse::Interaction::Idle)
                    //.interaction(iced::mouse::Interaction::Text) 
                    .into()

            } else {
                // -------------------------------------------------------------
                // РЕЖИМ РАБОТЫ: Интерактивный режим text_input Iced 0.14
                // -------------------------------------------------------------
                w_input = w_input.on_input(move |new_val: String| Message::UpdateProperty {
                        widget_id: id_clone.clone(),
                        property_key: PROP_VALUE,
                        value: PropertyValue::Text(new_val),
                    });
                w_input.into()
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

        // Извлекаем текущие свойства текстового поля из фабрики
        let current = self.parse_props(factory);
        
        // Получаем дефолтные свойства для сравнения
        let default = InputProps::default();

        // Сравниваем свойства строго по списку editable_properties
        if current.action != default.action {
            prop_names.push(PROP_ACTION);
        }
        if current.placeholder != default.placeholder {
            prop_names.push(PROP_PLACEHOLDER);
        }
        if current.value != default.value {
            prop_names.push(PROP_VALUE);
        }
        if current.secure != default.secure {
            prop_names.push(PROP_SECURE);
        }
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.padding != default.padding {
            prop_names.push(PROP_PADDING);
        }
        if current.font_family != default.font_family {
            prop_names.push(PROP_FONT_FAMILY);
        }
        if current.text_size != default.text_size {
            prop_names.push(PROP_TEXT_SIZE);
        }
        if current.font_weight != default.font_weight {
            prop_names.push(PROP_FONT_WEIGHT);
        }
        if current.font_style != default.font_style {
            prop_names.push(PROP_FONT_STYLE);
        }
        if current.line_height != default.line_height {
            prop_names.push(PROP_LINE_HEIGHT);
        }
        if current.align_x != default.align_x {
            prop_names.push(PROP_ALIGN_X);
        }

        prop_names
    }

}
