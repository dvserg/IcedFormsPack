use std::rc::Rc;
use std::cell::{UnsafeCell};
use iced::widget::{combo_box, text, text::Shaping};
use iced::{Element, Length, Padding, Pixels, Theme};
//use log::{info, warn};


use crate::core::*;

// -----------------------------------------------------------------------------
// Автоматическая Регистрация на Фабрике через Inventory
// -----------------------------------------------------------------------------
inventory::submit! {
    AutoRegisteredWidget {
        name: ComboBoxBlueprint::WIDGET_TYPE, //"combo_box",
        category: CAT_INPUTS,
        constructor: create_combobox_creator,
    }
}

fn create_combobox_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Combobox");
    Box::new(ComboBoxCreator)
}

#[derive(Debug)]
pub struct ComboBoxCreator;

impl WidgetCreator for ComboBoxCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        // Создаем дефолтный набор опций для нового комбобокса на холсте
        Rc::new(ComboBoxBlueprint::new(id))
    }
}

// -----------------------------------------------------------------------------

// Структура для распарсенных свойств контейнера
#[derive(Debug)]
pub struct ComboBoxProps {
    pub options:     Vec<String>,
    pub value:       Option<String>,
    pub placeholder: String,

    pub width:       Length,    // Ширина виджета   
    //pub size:        Pixels,  // Размер ширина
    pub menu_height: Length,    // Высота выпадающего списка
    pub padding:     Padding,   // Отступ текста от края рамки

    // Основная Типографика подписи
    pub font_family: String,
    pub text_size:   Pixels,
    pub font_weight: bool,
    pub font_style:  bool,

    pub line_height: f32,
    pub shaping:     bool,      // Продвинутая отрисовка текста
}

// -----------------------------------------------------------------------------
// Структура ComboBox
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct ComboBoxBlueprint {
    pub meta: CommonWidgetMeta,

    // Стейт combobox
    pub combo_state: std::cell::UnsafeCell<iced::widget::combo_box::State<String>>,

    // Внутренний массив ВСЕХ доступных опций, из которых пользователь выбирает
    //pub options: Vec<String>,
}

impl ComboBoxBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "combo_box";

    pub fn new(id: String) -> Self {

        // Получить дефолтные значения
        //let def = ComboBoxProps::default();

        // Создаем стандартный стейт комбобокса Iced
        let internal_state = combo_box::State::new(vec![]);

        Self {
            meta:        CommonWidgetMeta::new(id),
            combo_state: UnsafeCell::new(internal_state),
            //options,
        }
    }

    fn parse_props<'a>(&self, factory: &'a Factory) -> ComboBoxProps {
        // Состояние ComboBox
        //let combo_state:    combo_box::State<String>,

        // Получить ID виджета
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = ComboBoxProps::default();

        // Текстовые свойства
        //let action:      String = factory.get_or_set(&widget_id, PROP_ACTION,      "".to_string());
        let placeholder: String = factory.get_or_set(&widget_id, PROP_PLACEHOLDER, "Выберите значение..".to_string());

        // Парсинг опций (строка через запятую -> Вектор String)
        let options_str: String = factory.get_or_set(&widget_id, PROP_OPTIONS, "Опция 1,Опция 2,Опция 3".to_string());
        let options = utils_bp::parse_comma_separated(&options_str);

        // Текущее выбранное значение
        let value_raw: String = factory.get_or_set(&widget_id, PROP_VALUE, "Select..".to_string());
        let value: Option<String> = if value_raw.is_empty() {
            None
        } else {
            Some(value_raw.to_string())
        };

        // Адаптивные размеры
        let width:       Length = factory.get_or_set(&widget_id, PROP_WIDTH,       Length::Fill);
        //let size:        Pixels = factory.get_or_set(&widget_id, PROP_SIZE,        def.size);
        let menu_height: Length = factory.get_or_set(&widget_id, PROP_MENU_HEIGHT, def.menu_height);

        // Шаг и Внутренние отступы
        let padding: Padding = factory.get_or_set(&widget_id, PROP_PADDING, def.padding);

        // Основная Типографика (Шрифт и размер)
        let font_family: String = factory.get_or_set(&widget_id, PROP_FONT_FAMILY, def.font_family);
        let text_size:   Pixels = factory.get_or_set(&widget_id, PROP_TEXT_SIZE,   def.text_size);
        let font_weight: bool   = factory.get_or_set(&widget_id, PROP_FONT_WEIGHT, def.font_weight);
        let font_style:  bool   = factory.get_or_set(&widget_id, PROP_FONT_STYLE,  def.font_style);

        // Дополнительное форматирование текста
        let line_height: f32  = factory.get_or_set(&widget_id, PROP_LINE_HEIGHT, def.line_height);
        let shaping:     bool = factory.get_or_set(&widget_id, PROP_SHAPING,     def.shaping);

        ComboBoxProps {
            options,
            value,
            placeholder,
            width,
            //size,
            menu_height,
            padding,

            font_family,
            text_size,
            font_weight,
            font_style,

            line_height,
            shaping,
        }
    }
}

impl Default for ComboBoxProps {
    fn default() -> Self {
        ComboBoxProps {
            options:     vec![],
            value:       None,
            placeholder: String::from(""),

            width:       Length::Shrink,
            //size:        Pixels(16.0),  // Размер ширина
            padding:     Padding::from([5.0, 5.0]),  // Отступ текста от края рамки
            menu_height: Length::Shrink,

            font_family:  "System".to_string(),  // Системный шрифт по умолчанию
            text_size:    Pixels(16.0),          // Базовый размер текста в Iced
            font_weight:  false,                 // false = Regular (обычный)
            font_style:   false,                 // false = Normal (прямой)
            
            line_height:  1.0,                   // Стандартный межстрочный интервал
            shaping:      false,                 // Продвинутый шейпинг выключен для производительности
        }
    }
}

impl HasCommonMeta for ComboBoxBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

//#[typetag::serde]
impl WidgetBlueprint for ComboBoxBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_PLACEHOLDER,
            PROP_OPTIONS,

            PROP_WIDTH,
            //PROP_SIZE,
            PROP_MENU_HEIGHT,
            PROP_PADDING,

            PROP_FONT_FAMILY,
            PROP_TEXT_SIZE,
            PROP_FONT_WEIGHT,
            PROP_FONT_STYLE,
            PROP_LINE_HEIGHT,
            PROP_SHAPING,

        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем обновленные свойства (включая width и height)
        let props = self.parse_props(factory);

        // Получаем ID текущего виджета
        let widget_id = self.get_id();

        // Собираем Font
        let current_text_font = utils_bp::create_iced_font(
            &props.font_family.as_str(),
            props.font_weight,
            props.font_style,
        );

        let current_selection: String = factory.get_or_set(&widget_id, PROP_VALUE, "".to_string());

        // Превращаем String выбора в Option<&String> для сигнатуры Iced
        let selection_option = if current_selection.is_empty() {
            None
        } else {
            Some(&current_selection)
        };

        // Апдейтим выпадающий список 
        let combo_state_mut: &mut combo_box::State<String> = unsafe {&mut *self.combo_state.get()};
        *combo_state_mut = combo_box::State::new(props.options);

        // Собираем базовый нативный ComboBox
        let mut base_combo = combo_box(
            combo_state_mut,            
            &props.placeholder,
            selection_option,
            move |_selected_item: String| Message::NoOp,
        )
        .width(props.width)
        .padding(props.padding)
        .menu_height(props.menu_height)
        .font(current_text_font)
        .line_height(text::LineHeight::Relative(props.line_height));

        // Применяем следующие параметры если они заданы, иначе оставляем системные
        if props.shaping {
            base_combo = base_combo.text_shaping(Shaping::Advanced);
        }

        // Применяем размер шрифта больше 0.0, иначе автоматически используется
        // системный размер шрифта по умолчанию ( 16.0 )
        if props.text_size.0 > 0.0 {
            base_combo = base_combo.size(props.text_size);
        }

        // ФОРМИРУЕМ ЭЛЕМЕНТ В ЗАВИСИМОСТИ ОТ РЕЖИМА
        /*let element: Element<'a, Message, Theme> = if factory.is_design_mode {
            // В РЕЖИМЕ ДИЗАЙНЕРА: накрываем невидимой кнопкой для перехвата клика инспектором
            button(base_combo)
                .padding(0) // Чтобы кнопка не раздувала комбобокс
                .style(move |_theme, _status| button::Style {
                    background: None,
                    border: iced::Border {
                        // Если виджет сейчас выделен — рисуем рамку фокуса редактора
                        color: if selected_id == Some(self.get_id().as_str()) {
                            iced::Color::from_rgb(0.12, 0.53, 0.9)
                        } else {
                            iced::Color::TRANSPARENT
                        },
                        width: 1.5,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(Message::SelectWidget { widget_id: self.get_id().clone() })
                .into()
        } else {
            // В РЕЖИМЕ РАБОТЫ: отдаем чистый продуктовый комбобокс пользователю
            base_combo.into()
        };
        */


        let element: Element<'a, Message, Theme> = base_combo.into();

        // В самом конце применяем магию подсветки из трейта в одну строчку!
        apply_design_overlay(
            element,
            factory.is_design_mode(),
            selected_id,
            &self.get_id(),
        )        
    }
}

// -----------------------------------------------------------------------------
// Создаем прозрачную обертку (Newtype паттерн)
#[derive(Debug)]
pub struct SyncComboState(pub combo_box::State<String>);

// Снимаем ограничения компилятора по потокобезопасности
unsafe impl Send for SyncComboState {}
unsafe impl Sync for SyncComboState {}

// Реализуем ручной Clone для поддержки авто-копирования Блюпринта
impl Clone for SyncComboState {
    fn clone(&self) -> Self {
        // При клонировании чертежа мы просто создаем для новой копии
        // свежий чистый стейт, в который пока не забито рантайм-выделение элементов
        SyncComboState(combo_box::State::new(Vec::new()))
    }
}
// -----------------------------------------------------------------------------
