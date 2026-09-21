// -----------------------------------------------------------------------------
// Виджет 'pick_list'
// Выпадающий список — Дропдаун-меню для выбора одного элемента из текстового списка.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::widget::{pick_list, text};
use iced::{Element, Length, Padding, Pixels, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: PickListBlueprint::WIDGET_TYPE, //"pick_list",
        category: CAT_INPUTS,
        constructor: create_pick_list_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_pick_list_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "PickList");
    Box::new(PickListCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct PickListCreator;

impl WidgetCreator for PickListCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(PickListBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct PickListProps {
    pub action:         String,         // Имя системного события (действия) при изменении значения

    pub options:        String,         // Распарсенный список опций
    pub value:          String,         // Текущее выбранное значение
    pub placeholder:    String,         // Текст-заглушка

    pub width:          Length,         // Стратегия ширины Iced
    pub padding:        Padding,        // Внутренние отступы

    pub menu_height:      Length, 
    pub text_line_height: f32,

    pub font_family:    String,         // Семейство шрифта
    pub text_size:      Pixels,         // Размер шрифта
    pub font_weight:    bool,           // Начертание/жирность текста
    pub font_style:     bool,

    pub show_handle:    bool,           // Флаг отображения стрелки
}

#[derive(Debug, Clone /*, serde::Serialize, serde::Deserialize*/)]
pub struct PickListBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<PickListProps>,
}

impl HasCommonMeta for PickListBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl PickListBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "pick_list";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: PickListProps::default().into(),
        }
    }

    // ПАРСИНГ СВОЙСТВ ДЛЯ ПИКЛИСТА
    fn parse_props<'a>(&self, factory: &'a Factory) -> PickListProps {
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = PickListProps::default();

        // Текстовые свойства (через защищенные фабричные методы)
        let action:      String = factory.get_or_set(&widget_id, PROP_ACTION,      def.action);
        let placeholder: String = factory.get_or_set(&widget_id, PROP_PLACEHOLDER, "Выберите значение..".to_string());

        // Парсинг опций (строка через запятую -> Вектор String)
        let options: String = factory.get_or_set(&widget_id, PROP_OPTIONS, "Опция 1,Опция 2,Опция 3".to_string());
        //let options = utils_bp::parse_comma_separated(&options_str);

        // Текущее выбранное значение
        let value: String = factory.get_or_set(&widget_id, PROP_VALUE, def.value);
        //let value:     Option<String> = if value_raw.is_empty() { None } else { Some(value_raw.to_string()) };

        // Размеры, масштабы и геометрия
        let width:   Length  = factory.get_or_set(&widget_id, PROP_WIDTH,   def.width);
        let padding: Padding = factory.get_or_set(&widget_id, PROP_PADDING, def.padding);

        let text_line_height: f32    = factory.get_or_set(&widget_id, PROP_TEXT_LINE_HEIGHT, def.text_line_height);
        let menu_height:      Length = factory.get_or_set(&widget_id, PROP_MENU_HEIGHT,      def.menu_height);

        // Размеры текста и отступы через родной parse_pixels (возвращает f32 или Pixels в зависимости от вашей реализации Factory)
        let text_size: Pixels = factory.get_or_set(&widget_id, PROP_TEXT_SIZE, def.text_size);

        // Полный комплект текстовых стилей, собираемых на заводе атомарно
        let font_family: String = factory.get_or_set(&widget_id, PROP_FONT_FAMILY, def.font_family);
        let font_weight: bool   = factory.get_or_set(&widget_id, PROP_FONT_WEIGHT, def.font_weight);
        let font_style:  bool   = factory.get_or_set(&widget_id, PROP_FONT_STYLE,  def.font_style);

        // Флаг отображения стрелочки
        let show_handle: bool = factory.get_or_set(&widget_id, PROP_SHOW_HANDLE, def.show_handle);

        PickListProps {
            //content,
            action,

            options,
            value,
            placeholder,
            width,
            padding,
            text_size,
            font_family,
            font_weight,
            font_style,
            show_handle,
            text_line_height,
            menu_height,
        }
    }
}

impl Default for PickListProps {
    fn default() -> PickListProps {
        let set = iced::Settings::default();

        PickListProps {
            //content,
            action:         String::new(),

            options:        String::new(),
            value:          String::new(),
            placeholder:    String::new(),
            width:          Length::Shrink,
            padding:        Padding::from([5.0, 10.0]),            
            text_size:      set.default_text_size,
            font_family:    "System".to_string(),
            font_weight:    false,
            font_style:     false,
            show_handle:    true,

            text_line_height: 1.0_f32, 
            menu_height:    Length::Shrink,
        }
    }
}

impl WidgetBlueprint for PickListBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            //PROP_CONTENT,
            PROP_ACTION,
            PROP_OPTIONS,
            PROP_VALUE,
            PROP_PLACEHOLDER,
            PROP_WIDTH,
            PROP_PADDING,
            PROP_MENU_HEIGHT,
            PROP_TEXT_LINE_HEIGHT,
            PROP_FONT_FAMILY,
            PROP_TEXT_SIZE,
            PROP_FONT_WEIGHT,
            PROP_FONT_STYLE,
            PROP_SHOW_HANDLE,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>, // Сквозная передача выбранного ID
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

        // Рендерим виджет в зависимости от выбранного режима
        let element: Element<'a, Message, Theme> = {
            // -------------------------------------------------------------
            // РЕЖИМ РАБОТЫ: Настоящий интерактивный PickList для пользователя
            // -------------------------------------------------------------
            let id_clone = self.get_id().clone();

            let options = if !factory.is_design_mode() { utils_bp::parse_comma_separated(&props.options) } else { vec![] };

            let mut pl = pick_list(
                options, 
                if props.value.is_empty() { None } else { Some(props.value.clone()) }, 
                move |selected_item| { 
                    Message::UpdateProperty {
                        widget_id: id_clone.clone(),
                        property_key: PROP_VALUE,
                        value: PropertyValue::Text(selected_item),
                    }
                })
                .placeholder(&props.placeholder)
                .text_size(props.text_size)
                .text_line_height(text::LineHeight::Relative(props.text_line_height))
                .padding(props.padding)
                .width(props.width.clone())
                .menu_height(props.menu_height)
                .font(current_font); // Применяем кастомный шрифт к PickList

            // Применяем скрытие стрелочки handle, если флаг равен false
            if !props.show_handle {
                pl = pl.handle(pick_list::Handle::None);
            }
            /*
            // в следующем релизе Iced пригодится
            if !factory.is_design_mode() {
                pl.on_select(move |selected_item| { 
                    Message::UpdateProperty {
                        widget_id: id_clone.clone(),
                        property_key: PROP_VALUE,
                        value: PropertyValue::Text(selected_item),
                    }
                })
            }
            */

            if factory.is_design_mode() {
                let static_pl = iced::widget::opaque(pl);
               // -------------------------------------------------------------
                // РЕЖИМ КОНСТРУКТОРА: Пассивный режим text_input
                // -------------------------------------------------------------
                // Оборачиваем в mouse_area и навешиваем на оба элемента событие выделения при клике
                iced::widget::mouse_area(static_pl)
                    .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    .on_release(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    //.interaction(iced::mouse::Interaction::Idle)
                    //.interaction(iced::mouse::Interaction::Text) 
                    .into()
            } else {
                pl.into()
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

        // 1. Извлекаем текущие свойства выпадающего списка из фабрики
        let current = self.parse_props(factory);
        
        // 2. Получаем чистые дефолтные свойства для сравнения
        let default = PickListProps::default();

        // 3. Сравниваем свойства строго по вашему списку editable_properties
        if current.action != default.action {
            prop_names.push(PROP_ACTION);
        }
        if current.options != default.options {
            prop_names.push(PROP_OPTIONS);
        }
        if current.value != default.value {
            prop_names.push(PROP_VALUE);
        }
        if current.placeholder != default.placeholder {
            prop_names.push(PROP_PLACEHOLDER);
        }
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.padding != default.padding {
            prop_names.push(PROP_PADDING);
        }
        if current.menu_height != default.menu_height {
            prop_names.push(PROP_MENU_HEIGHT);
        }
        if current.text_line_height != default.text_line_height {
            prop_names.push(PROP_TEXT_LINE_HEIGHT);
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
        if current.show_handle != default.show_handle {
            prop_names.push(PROP_SHOW_HANDLE);
        }

        prop_names
    }    
}
