// -----------------------------------------------------------------------------
// Виджет 'text'
// Текст — Простой вывод строки текста на экран с возможностью изменения шрифта и размера.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{text, text::Shaping, mouse_area};
use iced::{Color, Element, Length, Pixels, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: TextBlueprint::WIDGET_TYPE, //"text",
        category: CAT_BASE,
        constructor: create_text_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_text_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Text");
    Box::new(TextCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct TextCreator;

impl WidgetCreator for TextCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(TextBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct TextProps {
    // Логика связи и значения
    pub text:  String,

    // Геометрия и размеры виджета
    pub width:   Length,
    pub height:  Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,
    //pub text_alignment: iced::alignment::Horizontal,

    // Основная Типографика подписи
    pub font_family: String,
    pub text_size:   Pixels,
    pub font_weight: bool,
    pub font_style:  bool,

    // Форматирование и Рендеринг текста
    pub line_height: f32,
    pub wrapping:    bool,  // Перенос по строкам
    pub shaping:     bool,  // Продвинутая отрисовка текста

    // --- Визуальный стиль (Цвет) ---
    pub text_color: Color,
}

#[derive(Debug, Clone /*, serde::Serialize, serde::Deserialize*/)]
pub struct TextBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<TextProps>,
}

impl HasCommonMeta for TextBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl TextBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "text";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: TextProps::default().into(),
        }
    }

    // Финальная атомарная функция парсинга свойств текста
    fn parse_props<'a>(&self, factory: &'a Factory) -> TextProps {
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = TextProps::default();

        // Текст по ссылке &str
        let text:    String = factory.get_or_set(&widget_id, PROP_TEXT, "Виджет 'text'".to_string());

        // Размеры и Расположение (Геометрия)
        let width:   Length     = factory.get_or_set(&widget_id, PROP_WIDTH,   def.width);
        let height:  Length     = factory.get_or_set(&widget_id, PROP_HEIGHT,  def.height);
        let align_x: Horizontal = factory.get_or_set(&widget_id, PROP_ALIGN_X, def.align_x);
        let align_y: Vertical   = factory.get_or_set(&widget_id, PROP_ALIGN_Y, def.align_y);

        // Основная Типографика (Шрифт и размер)
        let font_family: String = factory.get_or_set(&widget_id, PROP_FONT_FAMILY, def.font_family);
        let text_size:   Pixels = factory.get_or_set(&widget_id, PROP_TEXT_SIZE,   def.text_size);
        let font_weight: bool   = factory.get_or_set(&widget_id, PROP_FONT_WEIGHT, def.font_weight);
        let font_style:  bool   = factory.get_or_set(&widget_id, PROP_FONT_STYLE,  def.font_style);

        // Дополнительное форматирование текста
        let line_height: f32  = factory.get_or_set(&widget_id, PROP_LINE_HEIGHT, def.line_height);
        let wrapping:    bool = factory.get_or_set(&widget_id, PROP_WRAPPING,    def.wrapping);
        let shaping:     bool = factory.get_or_set(&widget_id, PROP_SHAPING,     def.shaping);

        // Визуальный стиль (Цвет)
        let text_color: Color = factory.get_or_set(&widget_id, PROP_TEXT_COLOR, def.text_color);

        TextProps {
            text,
            width,
            height,
            text_size,
            align_x,
            align_y,
            font_family,
            font_weight,
            font_style,
            line_height,
            wrapping,
            shaping,
            text_color,
        }
    }
}

// Значения по умолчанию
impl Default for TextProps {
    fn default() -> Self {
        Self {
            text:         String::new(),
            width:        Length::Shrink,
            height:       Length::Shrink,
            align_x:      Horizontal::Left,      // Стандартное выравнивание по левому краю
            align_y:      Vertical::Top,         // Стандартное выравнивание по верхнему краю
            font_family:  "System".to_string(),  // Системный шрифт по умолчанию
            text_size:    Pixels(16.0),          // Базовый размер текста в Iced
            font_weight:  false,                 // false = Regular (обычный)
            font_style:   false,                 // false = Normal (прямой)
            line_height:  1.0,                   // Стандартный межстрочный интервал
            wrapping:     true,                  // Обычно перенос текста включен по умолчанию
            shaping:      false,                 // Продвинутый шейпинг выключен для производительности
            text_color:   Color::TRANSPARENT,
        }
    }
}

impl WidgetBlueprint for TextBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_TEXT,
            PROP_WIDTH,
            PROP_HEIGHT,
            PROP_ALIGN_X,
            PROP_ALIGN_Y,
            PROP_FONT_FAMILY,
            PROP_TEXT_SIZE,
            PROP_FONT_WEIGHT,
            PROP_FONT_STYLE,
            PROP_LINE_HEIGHT,
            PROP_WRAPPING,
            PROP_SHAPING,
            PROP_TEXT_COLOR,
        ]
    }

    // Реализация апдейта собственных свойств блюпринта из VTable
    //crate::impl_refresh_props!(TextBlueprint, TextProps);


    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Собираем Font
        let current_text_font = utils_bp::create_iced_font(
            &props.font_family.as_str(),
            props.font_weight,
            props.font_style,
        );

        // Формируем виджет текста, объединяя семейство шрифта и его жирность
        let mut w_text = text(props.text.clone())
            .width(props.width)
            .height(props.height)
            .align_x(props.align_x)
            .align_y(props.align_y)
            .font(current_text_font) // Передаем чистый, собранный по шагам шрифт
            .line_height(text::LineHeight::Relative(props.line_height));

        // Применяем следующие параметры если они заданы, иначе оставляем системные
        if props.text_color != Color::TRANSPARENT {
            w_text = w_text.color(props.text_color);
        }
        if props.wrapping {
            w_text = w_text.wrapping(text::Wrapping::Word);
        }
        if props.shaping {
            w_text = w_text.shaping(Shaping::Advanced);
        }

        // Применяем размер шрифта больше 0.0, иначе автоматически используется
        // системный размер шрифта по умолчанию ( 16.0 )
        if props.text_size.0 > 0.0 {
            w_text = w_text.size(props.text_size);
        }

        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // -------------------------------------------------------------
            // РЕЖИМ КОНСТРУКТОРА: Оборачиваем в mouse_area для выделения
            // -------------------------------------------------------------
            mouse_area(w_text)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            // -------------------------------------------------------------
            // РЕЖИМ РАБОТЫ: Чистый текст в приложении
            // -------------------------------------------------------------
            w_text.into()
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

        // Извлекаем текущие свойства текста из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = TextProps::default();

        // Сравниваем текущие значения со значениями по умолчанию
        if current.text != default.text {
            prop_names.push(PROP_TEXT);
        }
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.height != default.height {
            prop_names.push(PROP_HEIGHT);
        }
        if current.align_x != default.align_x {
            prop_names.push(PROP_ALIGN_X);
        }
        if current.align_y != default.align_y {
            prop_names.push(PROP_ALIGN_Y);
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
        if current.wrapping != default.wrapping {
            prop_names.push(PROP_WRAPPING);
        }
        if current.shaping != default.shaping {
            prop_names.push(PROP_SHAPING);
        }
        if current.text_color != default.text_color {
            prop_names.push(PROP_TEXT_COLOR);
        }

        prop_names
    }    
}
