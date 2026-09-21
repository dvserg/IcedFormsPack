// -----------------------------------------------------------------------------
// Виджет 'checkbox'
// Флажок — Двухпозиционный чекбокс (галочка) для выбора опций "включено/выключено".
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::border::Radius;
use iced::widget::{checkbox, text, mouse_area};
use iced::{Color, Element, Length, Pixels, Theme};
//use log::{info, warn};

//use crate::blueprints::*;
use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: CheckboxBlueprint::WIDGET_TYPE, //"check_box",
        category: CAT_INPUTS,
        constructor: create_checkbox_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_checkbox_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "CheckBox");
    Box::new(CheckboxCreator)
}

#[derive(Debug, Clone)]
pub struct CheckboxCreator;
impl WidgetCreator for CheckboxCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        //std::sync::Arc::new(crate::blueprints::CheckboxBlueprint::new(id, "CheckBox".to_string()))
        Rc::new(CheckboxBlueprint::new(id, "CheckBox".to_string()))
    }
}
// -----------------------------------------------------------------------------

// Структура для распарсенных свойств
#[derive(Debug, Clone)]
pub struct CheckboxProps {
    pub action: String,

    //pub is_enabled:     bool,

    // Логика связи и значения (Бизнес-логика)
    pub is_checked:     bool,
    pub label:          String,

    // Геометрия и размеры виджета
    pub flag_size:      Pixels,
    pub width:          Length,
    pub spacing:        Pixels,     // Отступ от флага до текста

    // Основная Типографика подписи
    pub font_family:    String,
    pub text_size:      Pixels,     // Системный тип Pixels для Iced 0.14
    pub font_weight:    bool,
    pub font_style:     bool,
    pub wrapping:       bool,       // Перенос по строкам
    pub shaping:        bool,       // Продвинутая отрисовка текста

    pub line_height:    f32,
    pub text_color:     Color,      // Кастомный цвет надписи

    pub icon_color:     Color,
    pub bg_color:       Color,
    pub border_radius:  Radius,
    pub border_width:   f32,
    pub border_color:   Color,
}

#[derive(Debug, Clone)]
pub struct CheckboxBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<CheckboxProps>,
}

impl HasCommonMeta for CheckboxBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl CheckboxBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "check_box";

    pub fn new(id: String, _label: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: CheckboxProps::default().into(),
        }
    }

    // Парсинг свойств
    fn parse_props<'a>(&self, factory: &'a Factory) -> CheckboxProps {

        let widget_id = self.get_id();

        // Получаем дефолтные свойства
        let def = CheckboxProps::default();

        let action: String = factory.get_or_set(&widget_id, PROP_ACTION, def.action);

        // Логика связи и значения (Бизнес-логика)
        let is_checked: bool   = factory.get_or_set(&widget_id, PROP_IS_CHECKED, false);
        let label:      String = factory.get_or_set(&widget_id, PROP_LABEL,      "Checkbox".to_string());

        // Геометрия и размеры виджета
        let flag_size: Pixels = factory.get_or_set(&widget_id, PROP_FLAG_SIZE, def.flag_size);
        let width:     Length = factory.get_or_set(&widget_id, PROP_WIDTH,     def.width);
        let spacing:   Pixels = factory.get_or_set(&widget_id, PROP_SPACING,   def.spacing);

        // Основная Типографика подписи
        let font_family: String = factory.get_or_set(&widget_id, PROP_FONT_FAMILY, def.font_family);
        let text_size:   Pixels = factory.get_or_set(&widget_id, PROP_TEXT_SIZE,   def.text_size);
        let font_weight: bool   = factory.get_or_set(&widget_id, PROP_FONT_WEIGHT, def.font_weight);
        let font_style:  bool   = factory.get_or_set(&widget_id, PROP_FONT_STYLE,  def.font_style);

        // Форматирование и Рендеринг текста
        let wrapping: bool = factory.get_or_set(&widget_id, PROP_WRAPPING, def.wrapping);
        let shaping:  bool = factory.get_or_set(&widget_id, PROP_SHAPING,  def.shaping);

        let line_height: f32   = factory.get_or_set(&widget_id, PROP_LINE_HEIGHT, def.line_height); // Увеличенный межстрочный интервал по дефолту
        let text_color:  Color = factory.get_or_set(&widget_id, PROP_TEXT_COLOR,  def.text_color); //Color::from_rgb(0.7, 0.7, 0.7));

        let icon_color:  Color = def.icon_color;

        // Стиль контейнера
        let bg_color:      Color  = factory.get_or_set(&widget_id, PROP_BG_COLOR,      def.bg_color); //Color::from_rgb(0.95, 0.95, 0.95)); // Мягкий светло-серый фон
        let border_radius: Radius = factory.get_or_set(&widget_id, PROP_BORDER_RADIUS, def.border_radius);
        let border_width:  f32    = factory.get_or_set(&widget_id, PROP_BORDER_WIDTH,  1.0_f32);
        let border_color:  Color  = factory.get_or_set(&widget_id, PROP_BORDER_COLOR,  Color::from_rgb(0.7, 0.7, 0.7));

        CheckboxProps {
            action,

            is_checked,
            label,

            flag_size,
            width,
            spacing,

            font_family,
            text_size,
            font_weight,
            font_style,
            wrapping,
            shaping,

            line_height,
            text_color:     if text_color != Color::TRANSPARENT { text_color } else { def.text_color },

            icon_color,

            bg_color,
            border_radius,
            border_width,
            border_color:   if border_color != Color::TRANSPARENT { border_color } else {  Color::from_rgb(0.7, 0.7, 0.7) /*iced::Border::default().color*/ },
        }
    }
}

impl Default for CheckboxProps {
    fn default() -> CheckboxProps {

        let set = iced::Settings::default();
        let system_palette = iced::Theme::Light.palette();

        CheckboxProps {
            action:         "".to_string(),

            // Логика связи и значения (Бизнес-логика)
            is_checked:     false,
            label:          "".to_string(),

            flag_size:      set.default_text_size, //Pixels(16.0),
            width:          Length::Shrink,
            spacing:        Pixels(8.0),

            font_family:    "System".to_string(),
            text_size:      set.default_text_size, //Pixels(16.0),
            font_weight:    false,
            font_style:     false,
            wrapping:       false,
            shaping:        false,

            line_height:    1.0_f32,
            text_color:     system_palette.text, //Color::TRANSPARENT,

            icon_color:     system_palette.primary,

            bg_color:       Color::TRANSPARENT, // system_palette.primary,  // Transparent - пропускаем применение кастомного цвета
            border_radius:  Radius::from(2.0_f32),
            border_width:   0.0_f32,
            border_color:   Color::TRANSPARENT,                             // Transparent - пропускаем применение кастомного цвета
        }
    }
}

impl WidgetBlueprint for CheckboxBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_ACTION,
            PROP_IS_CHECKED,
            PROP_LABEL,
            PROP_FLAG_SIZE,
            PROP_WIDTH,
            PROP_SPACING,
            PROP_FONT_FAMILY,
            PROP_TEXT_SIZE,
            PROP_FONT_WEIGHT,
            PROP_FONT_STYLE,
            PROP_WRAPPING,
            PROP_SHAPING,
            PROP_LINE_HEIGHT,
            PROP_TEXT_COLOR,
            PROP_BG_COLOR,
            PROP_BORDER_RADIUS,
            PROP_BORDER_WIDTH,
            PROP_BORDER_COLOR,
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

        // Извлекаем размер квадратика f32 из фабрики
        //let flag_size = props.flag_size;

        // Wrapping - перенос 'label' по словам
        let wrapping_mode = if props.wrapping {
            iced::widget::text::Wrapping::Word // Если true -> перенос по словам
        } else {
            iced::widget::text::Wrapping::None // Если false -> не переносить текст
        };

        // Shaping - HarfBuzz (лигатуры, кернинг, Unicode)
        let shaping_mode = if props.shaping {
            iced::widget::text::Shaping::Advanced // Если true -> HarfBuzz (лигатуры, кернинг)
        } else {
            iced::widget::text::Shaping::Basic // Если false -> обычный быстрый вывод
        };

        // Высота строки текста не должна быть мегьше или равна нулю
        let current_line_height = if props.line_height > 0.0 {
            props.line_height
        } else {
            1.0
        };

        // Собираем Font
        let current_text_font = utils_bp::create_iced_font(
            &props.font_family.as_str(),
            props.font_weight,
            props.font_style,
        );

        let element: Element<'a, Message, Theme> = {
            let id_clone = self.get_id().clone();

            let props_cl = props.clone(); 

            // В Iced 0.14 конструктор принимает bool, а текст цепляется через .label()
            let mut w_check_box = checkbox(props.is_checked)
                .label(props.label.clone())
                .width(props.width)
                .size(props.flag_size) // Применяем динамический размер квадратика в пикселях
                .text_shaping(shaping_mode)
                .text_wrapping(wrapping_mode)
                .text_line_height(text::LineHeight::Relative(current_line_height))
                .spacing(props.spacing) // Передаем отступ до текста
                //.text_color(prop.text_color)
                .font(current_text_font)
                .style(move |theme: &Theme, status: checkbox::Status| {
                    // Получаем ПОЛНЫЙ системный стиль Iced для текущего состояния (цвета темы, рамки и т.д.)
                    let mut base_style = checkbox::primary(theme, status);

                    // Получить дефолтные значения
                    let def = CheckboxProps::default();

                    // Применяем только действительный цвет
                    // Если указан прозрачный цвет - оставляем дефолтный
                    if base_style.text_color != Some(Color::TRANSPARENT) {
                        base_style.text_color = Some(props_cl.text_color);
                    }
                    if base_style.border.color != Color::TRANSPARENT {
                        base_style.border.color = props_cl.border_color;
                    }
                    if props_cl.border_width != def.border_width {
                        base_style.border.width = props_cl.border_width;
                    }
                    if props_cl.border_radius != def.border_radius {
                        base_style.border.radius = props_cl.border_radius;
                    }

                    // Меняем ТОЛЬКО то, что нам нужно, не ломая остальные дефолтные параметры
                    match status {
                        // Если чекбокс нажат (стоит галочка), красим фон и птичку в свои цвета
                        checkbox::Status::Active  { is_checked: true } | 
                        checkbox::Status::Hovered { is_checked: true } => {
                            // Цвет бэкграунда Transparent не применяем, используем системный
                            if props_cl.bg_color != Color::TRANSPARENT {
                                base_style.background = iced::Background::Color(props_cl.bg_color);
                            }
                        }
                        _ => {}
                    }

                    // Возвращаем измененный стиль обратно в Iced
                    base_style
                });

            // Fix: размер текста применяем только если он больше нуля
            if props.text_size.0 > 0.0 {
                w_check_box = w_check_box.text_size(props.text_size);
            }

            if factory.is_design_mode() {
                // -------------------------------------------------------------
                // РЕЖИМ КОНСТРУКТОРА: Пассивный режим чекбокса
                // -------------------------------------------------------------
                // Оборачиваем в mouse_area и навешиваем на оба элемента событие выделения при клике
                mouse_area(w_check_box
                    .on_toggle(move |_new_state: bool| Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    )
                    .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    .into()
            } else {
                // -------------------------------------------------------------
                // РЕЖИМ РАБОТЫ: Интерактивный режим чекбокса Iced 0.14
                // -------------------------------------------------------------
                // Системная команда UpdateProperty для простого апдейта состояния флага check_box
                w_check_box.on_toggle(move |new_state: bool| Message::UpdateProperty {
                        widget_id: id_clone.clone(),
                        property_key: PROP_IS_CHECKED,
                        value: PropertyValue::Boolean(new_state),
                    })
                    .into()
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

    /*
        fn handle_event(
            &mut self,
            widget_action: &crate::blueprints::message_bp::WidgetAction,
            app:           &mut crate::app::App,
        ) -> iced::Task<crate::core::Message> {

            match widget_action {
                crate::blueprints::WidgetAction::CheckboxToggled { widget_id, is_checked } => {
                    if widget_id == &self.get_id() {
                        // Разрываем время жизни аргумента копируя его в локальную переменную
                        let is_checked = is_checked.clone();
                        app.get_factory().set(&self.get_id(), PROP_IS_CHECKED, is_checked);

                      /*
                        // 1. Считываем старый текст напрямую
                        let old_text = self.content.text();

                        // 2. Аппаратно мутируем наш вечный буфер!
                        // Iced сам сдвинет курсор, обновит выделение и буквы прямо по этому адресу!
                        self.content.perform(text_editor_action.clone());

                        // 3. Считываем измененный текст
                        let new_text = self.content.text();

                        // 4. Если изменились именно буквы — пишем в VTable фабрики
                        if old_text != new_text {
                            app.get_factory().set(&self.get_id(), PROP_VALUE, new_text);
                        }
                      */
                    }
                }
                _ => {}
            }

            iced::Task::none()
        }
    */

    // Функция возвращает динамический список имен свойств для экспорта
    // Возвращаются только имена свойств с недефолтныи значениями, которые нужно сохранить в JSON
    // Свойства с дефолтными значениями отсекаются
    fn get_exportable_property_names(&self, factory: &Factory) -> Vec<PropertyKey> {
        let mut prop_names = Vec::new();

        // Получаем текущие свойства виджета из фабрики
        let current = self.parse_props(factory);
        
        // Получаем дефолтные свойства чекбокса для сравнения
        let default = CheckboxProps::default();

        // Попунктно сравниваем каждое поле структуры
        if current.action != default.action {
            prop_names.push(PROP_ACTION);
        }
        if current.is_checked != default.is_checked {
            prop_names.push(PROP_IS_CHECKED);
        }
        if current.label != default.label {
            prop_names.push(PROP_LABEL);
        }
        if current.flag_size != default.flag_size {
            prop_names.push(PROP_FLAG_SIZE);
        }
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.spacing != default.spacing {
            prop_names.push(PROP_SPACING);
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
        if current.wrapping != default.wrapping {
            prop_names.push(PROP_WRAPPING);
        }
        if current.shaping != default.shaping {
            prop_names.push(PROP_SHAPING);
        }
        if current.line_height != default.line_height {
            prop_names.push(PROP_LINE_HEIGHT);
        }
        if current.text_color != default.text_color {
            prop_names.push(PROP_TEXT_COLOR);
        }
        if current.bg_color != default.bg_color {
            prop_names.push(PROP_BG_COLOR);
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
