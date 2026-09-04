// -----------------------------------------------------------------------------
// Виджет 'radio'
// Радиокнопка — Компонент, который позволяет пользователю выбирать один вариант из группы.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::widget::{radio, mouse_area};
use iced::{Element, Length, Pixels, Color, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: RadioBlueprint::WIDGET_TYPE, //"radio",
        category: CAT_INPUTS,
        constructor: create_radio_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_radio_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Radio");
    Box::new(RadioCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct RadioCreator;
impl WidgetCreator for RadioCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        // По умолчанию создаем радиокнопку с текстом опции и значением "Option"
        Rc::new(RadioBlueprint::new(id, "Вариант выбора".to_string()))
    }
}
// -----------------------------------------------------------------------------

// Структура для распарсенных свойств радиокнопки
#[derive(Debug, Clone)]
pub struct RadioProps {
    // Логика связи и значения (Бизнес-логика)
    pub label:        String,
    pub value:        String,       // Собственное уникальное значение конкретно этой радиокнопки
    pub selected:     String,       // Текущая выбранная СТРОКА внутри этой группы (например, "Option1"),

    // поле не принадлежит данному виджету
    pub group:        String,       // Имя ключа состояния группы (например, "group_1_state")

    // Геометрия и размеры виджета
    pub flag_size:    Pixels,       // Размер индикатора
    pub spacing:      Pixels,
    pub width:        Length,

    // Основная Типографика подписи
    pub font_family:  String,
    pub text_size:    Pixels,
    pub font_weight:  bool,
    pub font_style:   bool,

    // Форматирование и Рендеринг текста
    pub wrapping:     bool,         // Перенос по строкам
    pub shaping:      bool,         // Продвинутая отрисовка текста

    pub line_height:  f32,
    pub text_color:   iced::Color,  // Кастомный цвет надписи

    pub dot_color:    Color,
    pub bg_color:     Color,
    pub border_width: f32,
    pub border_color: Color,
}

#[derive(Debug, Clone /*, serde::Serialize, serde::Deserialize*/)]
pub struct RadioBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<RadioProps>,
}

impl HasCommonMeta for RadioBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl RadioBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "radio";

    pub fn new(id: String, _label: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: RadioProps::default().into(),
        }
    }

    // Парсинг свойств из вашей String-driven базы фабрики
    fn parse_props<'a>(&self, factory: &'a Factory) -> RadioProps {
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = RadioProps::default();

        // Логика связи и значения (Бизнес-логика)
        let label: String = factory.get_or_set(&widget_id, PROP_LABEL, widget_id.clone());                      // Текстовая метка   
        let value: String = factory.get_or_set(&widget_id, PROP_VALUE, format!("Flag_{}", widget_id.clone()));  // Значение текущего элемента 'radio'

        // Логика выбора 'radio':
        // Виджет относится к группе 'group', эти данные достаем из 'value' виджета
        // Значение выбора группы хранится в единой 'state' группы ['group', PROP_SELECTED]
        // Если в виджет передаем одинаковые 'value' и 'selected' - статус виджета становится выбранным
        let group:    String = factory.get_or_set(&widget_id,     PROP_GROUP,    String::from("group_main"));   // Группы 'radio', не должна быть пустой по дефолту        
        let selected: String = factory.get_or_set(&group.clone(), PROP_SELECTED, String::from(""));             // Выбранное значение данной группы, берем из общего State группы

        // Записываем актуальный статус группы в состояние виджета для отображения в инспекторе
        // FIX: set(...) здесь вызывает падение в рандомное время 
        // при экспорте из-за повторного borrow_mut
        // !!! Подумать как убрать set, нужно перезаписывать или апдейтить
        factory.set(&widget_id, PROP_SELECTED, selected.to_string());

        // Геометрия и размеры виджета
        let flag_size: Pixels = factory.get_or_set(&widget_id, PROP_FLAG_SIZE, def.flag_size);
        let spacing:   Pixels = factory.get_or_set(&widget_id, PROP_SPACING,   def.spacing);
        let width:     Length = factory.get_or_set(&widget_id, PROP_WIDTH,     def.width);

        // Основная Типографика подписи
        let font_family: String = factory.get_or_set(&widget_id, PROP_FONT_FAMILY, def.font_family);
        let text_size:   Pixels = factory.get_or_set(&widget_id, PROP_TEXT_SIZE,   def.text_size);
        let font_weight: bool   = factory.get_or_set(&widget_id, PROP_FONT_WEIGHT, def.font_weight);
        let font_style:  bool   = factory.get_or_set(&widget_id, PROP_FONT_STYLE,  def.font_style);

        // Форматирование и Рендеринг текста
        let line_height: f32  = factory.get_or_set(&widget_id, PROP_LINE_HEIGHT, def.line_height);
        let wrapping:    bool = factory.get_or_set(&widget_id, PROP_WRAPPING,    def.wrapping);
        let shaping:     bool = factory.get_or_set(&widget_id, PROP_SHAPING,     def.shaping);

        let text_color: Color = factory.get_or_set(&widget_id, PROP_TEXT_COLOR,   def.text_color);
        let dot_color:  Color = factory.get_or_set(&widget_id, PROP_ACTIVE_COLOR, def.dot_color);

        // Стиль контейнера
        let bg_color:      Color  = factory.get_or_set(&widget_id, PROP_BG_COLOR,     def.bg_color);
        let border_width:  f32    = factory.get_or_set(&widget_id, PROP_BORDER_WIDTH, def.border_width);
        let border_color:  Color  = factory.get_or_set(&widget_id, PROP_BORDER_COLOR, def.border_color);

        RadioProps {
            label,
            value,
            selected,
            group,

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
            text_color,  // Кастомный цвет надписи

            dot_color,
            bg_color,
            border_width,
            border_color,
        }
    }
}

impl Default for RadioProps {
    fn default() -> Self {
        RadioProps {
            // БИЗНЕС-ЛОГИКА И СВЯЗИ: Строки-заглушки
            label:          String::new(),
            value:          String::new(),
            selected:       String::new(),
            group:          String::new(),

            // ГЕОМЕТРИЯ: В Iced 0.14.2 размер кружка радиокнопки равен 14px,
            // а стандартный отступ от кружка до текста подписи равен 8px.
            flag_size:      Pixels(16.0),
            spacing:        Pixels(8.0),
            width:          Length::Shrink, // Сжимается под размер контента

            // ТИПОГРАФИКА: Системный шрифт, размер 16px и множитель высоты строки 1.0
            font_family:    "System".to_string(),
            text_size:      Pixels(16.0),
            font_weight:    false,
            font_style:     false,

            // ФОРМАТИРОВАНИЕ: Базовый рендеринг без переносов
            line_height:    1.0_f32,
            wrapping:       false,
            shaping:        false,

            text_color:     Color::TRANSPARENT,  // Кастомный цвет надписи

            dot_color:      Color::TRANSPARENT,
            bg_color:       Color::TRANSPARENT,
            border_width:   1.0_f32,
            border_color:   Color::TRANSPARENT,
        }
    }
}


impl WidgetBlueprint for RadioBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_LABEL,
            PROP_VALUE,
            PROP_SELECTED,          // Специальное поле отображает выбор группы
            PROP_GROUP,
            //PROP_FLAG_SIZE,
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
            PROP_ACTIVE_COLOR,
            PROP_BG_COLOR,
            PROP_BORDER_WIDTH,
            PROP_BORDER_COLOR,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Кнопка активна, если выбранное значение совпадает с собственным значением этой кнопки [1.1]
        //let is_selected = props.value == props.selected;

        // Собираем Font
        let current_font = utils_bp::create_iced_font(
            &props.font_family.as_str(),
            props.font_weight,
            props.font_style,
        );

        let wrapping_mode = if props.wrapping {
            iced::widget::text::Wrapping::Word // Если true -> перенос по словам
        } else {
            iced::widget::text::Wrapping::None // Если false -> не переносить текст
        };

        let shaping_mode = if props.shaping {
            iced::widget::text::Shaping::Advanced // Если true -> HarfBuzz (лигатуры, кернинг)
        } else {
            iced::widget::text::Shaping::Basic // Если false -> обычный быстрый вывод
        };

        let element: Element<'a, Message, Theme> = {
            //let id_clone = self.get_id().clone();
            let value_cl = props.value.to_string();
            let group_cl = props.group.to_string();

            let current_option: &str = &props.value; // Собственное значение кнопки
            let current_selected: &str = &props.selected; // Текущий выбор группы

            // Если они оба опции совпадут - кнопка загорится выбранной.
            let mut w_radio = radio(
                &props.label,
                current_option,         // Собственное значение кнопки
                Some(current_selected), // Выбранное значение группы
                move |_| {
                    if factory.is_design_mode() {
                        Message::MenuEvent(MenuAction::SelectWidget(self.get_id()))
                    } else {
                        Message::UpdateProperty {
                        // При клике в рабочем режиме реактивно обновляем опцию PRO_SELECTED
                        // в разделе с именем группы, к которой относится кнопка
                        // Передаем пару PROP_SELECTED=props.value текущего виджета
                        widget_id: group_cl.clone(),
                        property_key: PROP_SELECTED,
                        value: PropertyValue::Text(value_cl.clone()),
                        }
                    }
                },
            )
            .size(props.flag_size)
            .spacing(props.spacing)                 // Передаем отступ до текста
            .font(current_font)                     // Устанавливаем кастомный шрифт (семейство, наклон, жирность)
//            .text_size(props.text_size)             // Устанавливаем размер шрифта
            .text_line_height(props.line_height)
            .text_shaping(shaping_mode)
            .text_wrapping(wrapping_mode)
            .width(props.width);                    // Устанавливаем ширину нативного виджета радиокнопки

            // Применяем размер шрифта больше 0.0, иначе автоматически используется
            // системный размер шрифта по умолчанию ( 16.0 )
            if props.text_size.0 > 0.0 {
                w_radio = w_radio.text_size(props.text_size);
            }

            // Применяем кастомный стиль
            w_radio = w_radio.style(move |theme: &Theme, status: radio::Status| {
                let mut base_style = radio::default(theme, status);

                base_style.border_width  = props.border_width;

                if props.text_color != Color::TRANSPARENT {
                    base_style.text_color = Some(props.text_color);
                }
                if props.bg_color != Color::TRANSPARENT {
                    base_style.background = iced::Background::Color(props.bg_color);
                }
                if props.border_color != Color::TRANSPARENT {
                    base_style.border_color  = props.border_color;
                }

                // Если нужно кастомизировать цвет самого кружка (флага):
                use iced::widget::radio::Status;
                match status {
                    // Когда радиокнопка выбрана и активна
                    Status::Active { is_selected: _ } => {
                        if props.dot_color != Color::TRANSPARENT {
                            base_style.dot_color = props.dot_color;
                        }
                    }
                    _ => {}
                }
                
                base_style
            });

            if factory.is_design_mode() {
                // -------------------------------------------------------------
                // РЕЖИМ КОНСТРУКТОРА: Пассивный режим text_input
                // -------------------------------------------------------------
                // Оборачиваем в mouse_area и навешиваем на оба элемента событие выделения при клике
                mouse_area(w_radio)
                    .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    .on_release(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    .into()

            } else {
                w_radio.into()
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

        // Извлекаем текущие свойства радиокнопки из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = RadioProps::default();

        // Сравниваем свойства строго по вашему списку editable_properties
        if current.label != default.label {
            prop_names.push(PROP_LABEL);
        }
        if current.value != default.value {
            prop_names.push(PROP_VALUE);
        }
        if current.selected != default.selected {
            prop_names.push(PROP_SELECTED);
        }
        if current.group != default.group {
            prop_names.push(PROP_GROUP);
        }
        //if current.flag_size != default.flag_size {
        //    prop_names.push(PROP_FLAG_SIZE);
        //}
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
        if current.dot_color != default.dot_color {
            prop_names.push(PROP_ACTIVE_COLOR);
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

        prop_names
    }    
}
