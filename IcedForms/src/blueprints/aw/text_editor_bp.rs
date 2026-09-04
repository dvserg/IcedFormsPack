// -----------------------------------------------------------------------------
// Библиотека AW
// Виджет 'text_editor'
// Представляет собой простой многострочный редактор текста типа 'Блокнот'
// -----------------------------------------------------------------------------
use std::cell::UnsafeCell;
use std::rc::Rc;
use iced::widget::{button, text_editor};
use iced::{Color, Element, Length, Padding, Pixels, Theme, border::Radius};
use log::info;


use crate::core::*;

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: TextEditorBlueprint::WIDGET_TYPE, //"text_editor",
        category: CAT_INPUTS,
        constructor: create_text_editor_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_text_editor_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    info!("Авторегистрация '{}'", "TextEditor");
    Box::new(TextEditorCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug)]
pub struct TextEditorCreator;

impl WidgetCreator for TextEditorCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(TextEditorBlueprint::new(id, ""))
    }
}
// -----------------------------------------------------------------------------

// Структура для распарсенных свойств
#[derive(Debug)]
pub struct TextEditorProps {
    pub value: String,

    pub placeholder: String,
    pub action: String,
    pub content_width: Pixels,
    pub height: Length,
    pub text_size: Pixels,
    pub padding: Padding,
    pub font_family: String,
    pub font_weight: bool,
    pub font_style: bool,

    pub text_color: Color,
    pub placeholder_color: Color,
    pub selection_color: Color,

    pub bg_color: Color,
    pub border_radius: Radius,
    pub border_width: f32,
    pub border_color: Color,
}

// ОБЪЯВЛЯЕМ СТРУКТУРУ БЛЮПРИНТА
#[derive(Debug)]
pub struct TextEditorBlueprint {
    pub meta: CommonWidgetMeta,
    initial_text: String,

    pub content: UnsafeCell<text_editor::Content>,
}

impl HasCommonMeta for TextEditorBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl TextEditorBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "text_editor";

    pub fn new(id: String, initial_text: &str) -> Self {
        Self {
            meta: CommonWidgetMeta::new(id),
            initial_text: initial_text.to_string(),
            content: UnsafeCell::new(text_editor::Content::with_text(initial_text)),
        }
    }

    fn parse_props(&self, factory: &Factory) -> TextEditorProps {
        let widget_id = self.get_id();

        let value: String = factory.get_or_set(&self.get_id(), PROP_VALUE, "".to_string());
        let action = "".to_string();
        let placeholder: String = factory.get_or_set(
            &self.get_id(),
            PROP_PLACEHOLDER,
            "Введите многострочный текст...".to_string(),
        );
        let height: Length = factory.get_or_set(&widget_id, PROP_HEIGHT, Length::Fill);
        let padding: Padding = factory.get_or_set(&widget_id, PROP_PADDING, Padding::new(8.0));
        let content_width: Pixels =
            factory.get_or_set(&widget_id, PROP_CONTENT_WIDTH, Pixels(350.0));

        let font_family: String =
            factory.get_or_set(&widget_id, PROP_FONT_FAMILY, String::from("System"));
        let text_size:   Pixels = factory.get_or_set(&widget_id, PROP_TEXT_SIZE,   Pixels(16.0));
        let font_weight: bool   = factory.get_or_set(&widget_id, PROP_FONT_WEIGHT, false);
        let font_style:  bool   = factory.get_or_set(&widget_id, PROP_FONT_STYLE,  false);

        //let line_height = 1.3;

        // Стиль контейнера
        let bg_color: Color =
            factory.get_or_set(&widget_id, PROP_BG_COLOR, Color::from_rgb(0.9, 0.9, 0.9)); // Мягкий светло-серый фон
        let border_radius: Radius =
            factory.get_or_set(&widget_id, PROP_BORDER_RADIUS, Radius::from(4.0));
        let border_width: f32 = factory.get_or_set(&widget_id, PROP_BORDER_WIDTH, 1.0);
        let border_color: Color = factory.get_or_set(
            &widget_id,
            PROP_BORDER_COLOR,
            Color::from_rgb(0.7, 0.7, 0.7),
        );

        let text_color: Color = factory.get_or_set(
            &widget_id,
            PROP_TEXT_COLOR,
            Color::from_rgb(0.118, 0.118, 0.118),
        ); // Мягкий черный / темно-серый
        let placeholder_color: Color = factory.get_or_set(
            &widget_id,
            PROP_PLACEHOLDER_COLOR,
            Color::from_rgb(0.118, 0.118, 0.118),
        ); // Мягкий черный / темно-серый
        let selection_color: Color = factory.get_or_set(
            &widget_id,
            PROP_SELECTION_COLOR,
            Color::from_rgb(0.80, 0.90, 1.0),
        ); // Постельно-голубой цвет #CCE5FF

        TextEditorProps {
            value,
            placeholder,
            action,

            content_width,
            height,
            padding,
            text_size,
            font_family,
            font_weight,
            font_style,

            text_color,
            placeholder_color,
            selection_color,

            bg_color,
            border_radius,
            border_width,
            border_color,
        }
    }
}

// РЕАЛИЗУЕМ ТРЕЙТ ФАБРИКИ
//#[typetag::serde]
impl WidgetBlueprint for TextEditorBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            //PROP_CONTENT,
            PROP_VALUE,
            PROP_PLACEHOLDER,
            PROP_CONTENT_WIDTH,
            PROP_HEIGHT,
            PROP_PADDING,
            PROP_FONT_FAMILY,
            PROP_FONT_WEIGHT,
            PROP_FONT_STYLE,
            PROP_TEXT_COLOR,
            PROP_PLACEHOLDER_COLOR,
            PROP_SELECTION_COLOR,
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
        let props = self.parse_props(factory);

        let widget_id_cloned = self.get_id().clone();
        let is_selected = selected_id == Some(self.get_id().as_str());

        // Собираем Font
        let current_text_font = utils_bp::create_iced_font(
            &props.font_family.as_str(),
            props.font_weight,
            props.font_style,
        ); 

        // Вскрываем ячейку с сохраненным контентом на ЧТЕНИЕ
        let content_str_ref = unsafe { &*self.content.get() };

        // Переводим строки в вечный формат только для плейсхолдера Iced
        let current_placeholder_string = props.placeholder.clone();
        let leak_placeholder_str: &'static str =
            Box::leak(current_placeholder_string.into_boxed_str());

        let mut base_text_editor = text_editor(content_str_ref)
            .placeholder(leak_placeholder_str)
            .width(props.content_width)
            .height(props.height)
            .padding(props.padding)
            .font(current_text_font)
            .style(move |theme, status| {
                // Берем стандартный стиль темы для текстового редактора как основу,
                // чтобы не потерять цвета фона и рамок по умолчанию
                let base_style = iced::widget::text_editor::default(theme, status);

                iced::widget::text_editor::Style {
                    value: props.text_color,              // Применили цвет вводимого текста!
                    placeholder: props.placeholder_color, // Применили цвет подсказки!
                    selection: props.selection_color,     // Применили цвет выделения букв мышью!

                    // Все остальные свойства (фон, рамки) бесшовно заимствуем из базовой темы
                    background: base_style.background,
                    border: iced::Border {
                        color: props.border_color,          // Применили цвет рамки!
                        width: props.border_width,          // Применили толщину рамки!
                        radius: props.border_radius.into(), // Применили радиус скругления!
                    },
                }
            });

        if !factory.is_design_mode() {
            base_text_editor = base_text_editor.on_action(move |action| {
                Message::WidgetEvent(
                    self.get_id(),
                    WidgetAction::TextChanged {
                            widget_id: widget_id_cloned.clone(),
                            text_editor_action: action,
                    }
                )
            });
        };

        // ФОРМИРУЕМ ЭЛЕМЕНТ ДЛЯ ХОЛСТА РЕДАКТОРА
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {

            // В режиме дизайна накрываем прозрачной кнопкой, чтобы гасить внутренний ввод
            // и позволять дизайнеру свободно выделять TextEditor на холсте кликом мыши
            button(base_text_editor)
                .padding(0)
                .style(move |_theme, _status| button::Style {
                    background: None,
                    border: iced::Border {
                        color: if is_selected {
                            iced::Color::from_rgb(0.12, 0.53, 0.9) // Синяя рамка выделения
                        } else {
                            iced::Color::TRANSPARENT
                        },
                        width: 1.5,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                })
                // Выделяем элемент пр нажатии на него в режиме дизайнера
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            //base_text_editor = base_text_editor
            //.on_action(move |action| Message::TextEditorAction {
            //    widget_id: widget_id_cloned.clone(),
            //    action,
            //});

            base_text_editor.into()
        };

        // В самом конце применяем магию подсветки из трейта в режиме конструктора
        apply_design_overlay(
            element,
            factory.is_design_mode(),
            selected_id,
            &self.get_id(),
        )
    }

    fn handle_event(
        &mut self,
        widget_action: &crate::core::message_bp::WidgetAction,
        app: &mut crate::app::App,
    ) -> iced::Task<crate::core::Message> {
        match widget_action {
            crate::core::WidgetAction::TextChanged {
                widget_id,
                text_editor_action,
            } => {
                if widget_id == &self.get_id() {
                    // Апдейтим локальный state text_editor
                    let (old_text, new_text) = {
                        let content_mut: &mut text_editor::Content = unsafe { &mut *self.content.get() };
                        let old_text = content_mut.text();
                        content_mut.perform(text_editor_action.clone());
                        let new_text = content_mut.text();
                        (old_text, new_text)
                    };

                    if old_text != new_text {
                        app.get_factory().set(&self.get_id(), PROP_VALUE, new_text);
                    }
                }
            }
            _ => {}
        }

        iced::Task::none()
    }
}
