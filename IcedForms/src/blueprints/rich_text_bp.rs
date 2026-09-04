// -----------------------------------------------------------------------------
// Виджет 'rich_text'
// Форматированный текст — Продвинутый вывод текста, где отдельные слова или 
// буквы могут иметь разные стили, цвета, шрифты или начертания в рамках одного абзаца.
// -----------------------------------------------------------------------------
use std::cell::{UnsafeCell};
use iced::widget::text::{self, Span};
use iced::widget::{rich_text, mouse_area, container, column, row, button, scrollable, responsive};
use iced::{Element, Length, Pixels, Color, Theme};
use iced::alignment::{Horizontal, Vertical};

use crate::ui::UIStyle;
use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
inventory::submit! {
    AutoRegisteredWidget {
        name: RichTextBlueprint::WIDGET_TYPE, // "rich_text"
        category: CAT_BASE,
        constructor: create_rich_text_creator, 
    }
}

// Функция-помощник для регистрации блюпринта
fn create_rich_text_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация RichText");
    Box::new(RichTextCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct RichTextCreator;

impl WidgetCreator for RichTextCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(RichTextBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct RichTextProps {

    pub content: String,

    // Адаптивные размеры    
    pub width:   Length,
    pub height:  Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,

    // Основная Типографика подписи
    pub font_family: String,
    pub text_size:   Pixels,
    pub font_weight: bool,
    pub font_style:  bool,

    pub line_height: f32,
    pub wrapping:    bool,      // Перенос по строкам

    // Визуальный стиль (Цвет)
    pub text_color: Color,

}

// Структура представляет данные для формирования Span
#[derive(Debug, Clone, Default)]
pub struct TextSpanData {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub color: Option<iced::Color>,
}

// Блюпринт
#[derive(Debug)]
pub struct RichTextBlueprint {
    pub meta:  CommonWidgetMeta,

    // Хранилище распарсеного контента
    pub parsed_pieces: UnsafeCell<Vec<TextSpanData>>,
    pub content:       UnsafeCell<String>,

    // Хранилище контента редактора
    pub edit_pieces:   UnsafeCell<Vec<TextSpanData>>,
    pub edit_content:  UnsafeCell<String>,
}

impl HasCommonMeta for RichTextBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta { &self.meta }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta { &mut self.meta }
}

impl RichTextBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "rich_text";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),

            // Инициализируем states блюпринта пустым вектором строк
            parsed_pieces: UnsafeCell::new(Vec::new()),
            content:       UnsafeCell::new(String::from("")),

            // Инициализируем states редактора пустым вектором строк
            edit_pieces:   UnsafeCell::new(Vec::new()),
            edit_content:  UnsafeCell::new(String::from("")),
        }
    }

    // Парсинг свойств с использованием хелперов Factory
    pub fn parse_props(&self, factory: &Factory) -> RichTextProps {
        let widget_id = self.get_id();

        let def = RichTextProps::default();

        let content: String     = factory.get_or_set(&widget_id, PROP_TEXT_CONTENT, def.content);

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

        // Визуальный стиль (Цвет)
        let text_color: Color = factory.get_or_set(&widget_id, PROP_TEXT_COLOR, def.text_color);

        RichTextProps {
            content,
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
            text_color,
        }
    }
}

// Значения по умолчанию
impl Default for RichTextProps {
    fn default() -> Self {
        Self {
            content:      String::from(r#"{\rtf1\ansi\deff0
{\fonttbl{\f0\fnil\fcharset204 Arial;}}
{\colortbl ;\red255\green0\blue0;\red0\green128\blue0;}
\viewkind4\uc1\pard\lang1049\f0\fs28
Это обычный текст. \par
\b Это жирный текст.\b0 \par
\i Это курсив.\i0 \par
\b\i Это жирный курсив.\b0\i0 \par
А это слово \cf1 КРАСНОЕ\cf0 , а это слово \cf2 ЗЕЛЕНОЕ\cf0 .
}"#),
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
            text_color:   Color::TRANSPARENT,
        }
    }
}

impl TextSpanData {

    // Генерирует объект Span из структуры данных
    pub fn build_span(&self) -> Span<'static, std::convert::Infallible, Font> {

        let mut native_span = Span::new(self.text.clone());
        let mut current_font = Font::default();

        if self.bold {
            current_font.weight = font::Weight::Bold; // Используем правильный enum веса
        }

        if self.italic {
            current_font.style = font::Style::Italic; // Используем правильный enum стиля
        }

        // Применяем настроенную гарнитуру к спану через официальный метод .font()
        native_span = native_span.font(current_font);

        // Настраиваем линии (эти поля в Iced 0.14 публичные, пишем в них напрямую)
        if self.underline {
            native_span.underline = true;
        }
        if self.strikethrough {
            native_span.strikethrough = true;
        }
        // Накатываем цвет текста, если он был задан в Option
        if let Some(custom_color) = self.color {
            native_span = native_span.color(custom_color);
        }

        // Возвращаем полностью собранный графический элемент
        native_span
    }
}

// Реализуем контракт интерфейса
impl WidgetBlueprint for RichTextBlueprint {

    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // RichText может принимать неограниченное количество дочерних спанов (текстовых фрагментов)
    // Но в нашем случае это исключено и добавляем спаны только при импорте текста в редакторе
    fn can_accept_child(&self, _factory: &Factory) -> bool {
        false
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_TEXT_CONTENT,
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
            PROP_TEXT_COLOR,
        ]
    }

    fn build_element<'a>(
        &'a self, 
        factory: &'a Factory, 
        selected_id: Option<&str>
    ) -> Element<'a, Message, Theme> {
        let widget_id = self.get_id();
        let props = self.parse_props(factory);

        // Собираем все дочерние элементы-спаны, привязанные к этому родителю.
        // Каждый дочерний blueprint типа "span" должен предоставлять метод, 
        // возвращающий iced::widget::text::Span, а не абстрактный Element.
        //let mut spans = Vec::new();
        //let mut spans: Vec<iced::widget::text::Span<'a, u32, iced::Theme>> = Vec::new();
        //let mut spans: Vec<iced::widget::text::Span<'a, u32, iced::Font>> = Vec::new();
        

        let content: &String = unsafe {& *self.content.get()};

        // Если контент VTable изменился, то делаем апдейт локального state
        if *content != props.content {
            let content: & mut String = unsafe {&mut *self.content.get()};
            let parsed_pieces: &mut Vec<TextSpanData> = unsafe { &mut *self.parsed_pieces.get() };

            let edit_pieces: &mut Vec<TextSpanData> = unsafe { &mut *self.edit_pieces.get() };


            if props.content != "" {
                *parsed_pieces = rtf_to_span_data(&props.content);
                *content = props.content;

                *edit_pieces = parsed_pieces.clone();
            } else {
                *parsed_pieces = Vec::new();
                *content = "".to_string();
            }
        }

        let pieces_ref = unsafe { &*self.parsed_pieces.get() };
        let iced_spans: Vec<_> = pieces_ref
            .iter()
            .map(|piece| piece.build_span()) // Превращаем DTO в нативные спаны Iced 0.14
            .collect();

        // Подготавливаем финальный виджет
        let element: Element<'a, Message, Theme> = if iced_spans.is_empty() {
            // Если внутри пусто, выводим стандартный placeholder-заглушку
            let placeholder = create_empty_placeholder(
                &widget_id, 
                &self.widget_type(), 
                Length::Shrink, 
                Length::Shrink
            );

            if factory.is_design_mode() {
                mouse_area(placeholder)
                    .interaction(iced::mouse::Interaction::Pointer)
                    .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    .into()
            } else {
                placeholder
            }
        } else {

            // Собираем Font
            let current_text_font = utils_bp::create_iced_font(
                &props.font_family.as_str(),
                props.font_weight,
                props.font_style,
            );

            // Собираем нативный RichText из подготовленного вектора спанов.
            // Применяем .on_link_click() для маршрутизации сообщений от кликабельных ссылок.
            let mut rich_ui = rich_text(iced_spans)
                .width(props.width)
                .height(props.height)
                .align_x(props.align_x)
                .align_y(props.align_y)
                .font(current_text_font) // Передаем чистый, собранный по шагам шрифт
                .line_height(text::LineHeight::Relative(props.line_height));

                // Применяем следующие параметры если они заданы, иначе оставляем системные
                if props.text_color != Color::TRANSPARENT {
                    rich_ui = rich_ui.color(props.text_color);
                }
                if props.wrapping {
                    rich_ui = rich_ui.wrapping(text::Wrapping::Word);
                }

                // Применяем размер шрифта больше 0.0, иначе автоматически используется
                // системный размер шрифта по умолчанию ( 16.0 )
                if props.text_size.0 > 0.0 {
                    rich_ui = rich_ui.size(props.text_size);
                }

                //.on_link_click(|link_id: u32| {
                //    Message::NoOp
                //    //Message::WidgetEvent(WidgetAction::LinkWithIdClicked(link_id))
                //});

            if factory.is_design_mode() {
                // В режиме дизайна оборачиваем в mouse_area, чтобы по клику вне ссылок 
                // выделялся сам контейнер RichText в дереве инспектора
                mouse_area(rich_ui)
                    .interaction(iced::mouse::Interaction::Pointer)
                    .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    .into()
            } else {
                rich_ui.into()
            }
        };

        // В самом конце применяем магию подсветки из трейта в режиме конструктора
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
    fn get_exportable_property_names(&self, _factory: &Factory) -> Vec<PropertyKey> {
        self.editable_properties()
    }

    // Встроенный редактор для rich_text
    fn build_editor_content<'a>(&'a self, _factory: &'a Factory) -> Element<'a, Message, Theme> {

        let ui_style = UIStyle::default();

        // Получение идентификатора текущего виджета и актуального текста из фабрики
        let widget_id = self.get_id();
        
        // Панель управления (закрытие окна)
        let action_bar = row![
            iced::widget::text(format!("Редактирование RichText: {}", widget_id.clone()))
                .size(18)
                .width(Length::Fill),
            button("Применить").on_press(Message::WidgetEvent(
                widget_id.clone(),
                WidgetAction::RichText{
                    widget_id: widget_id.clone(), 
                    action:    RichTextEdit::ApplyChanges
                }
            )),
            button("Закрыть").on_press(Message::OverlayEvent(
                OverlayAction::CloseOverlay
            )), 
        ]
        .spacing(12)
        .align_y(Vertical::Center);
        
        // Панель toolbar
        let toolbar_row = row![
            // Вставить текст RTF из буфера
            crate::ui::toolbar_small_button( crate::ui::ICON_RICH_CLIP, "", "Вставить из буфера обмена RTF или текст",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::RichText{
                        widget_id: widget_id.clone(), 
                        action:    RichTextEdit::InsertClipboardRTF
                    }
                ),
                ui_style
            ),
            // Очистить текст в редакторе
            crate::ui::toolbar_small_button( crate::ui::ICON_RICH_CLEAR, "", "Очистить",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::RichText{
                        widget_id: widget_id.clone(), 
                        action:    RichTextEdit::Clear
                    }
                ),
                ui_style
            ),
        ];
        
        // Исправлено замыкание для явной инициализации структуры container::Style
        let border_style = |theme: &iced::Theme| {
            let palette = theme.palette();
            iced::widget::container::Style {
                border: iced::Border {
                    color: palette.text, 
                    width: 1.0,
                    radius: 4.0.into(), 
                },
                ..Default::default()
            }
        };

        // Многострочное текстовое окно
        let editor_side = column![
            //format_toolbar,
            toolbar_row,
            
            container(
                responsive(move |size| {
                    // Определяем размер блока для динамического
                    // задания минимального размера text_editor
                    let height = size.height;
                    let width  = size.width;
                    //let widget_id_cl = widget_id.clone();

                    let pieces_ref = unsafe { &*self.edit_pieces.get() };
                    let iced_spans: Vec<_> = pieces_ref
                        .iter()
                        .map(|piece| piece.build_span()) // Превращаем DTO в нативные спаны Iced 0.14
                        .collect();

                    scrollable (
                        //space()
                        rich_text(iced_spans)
                            .height(height)
                            .width(width)
                    )
                    .height(Length::Fill)
                    .into()
                })                    
            )
            .style(border_style) 
            .padding(8)
            .height(Length::Fill)
            .width(Length::Fill)
        ]
        .spacing(10)
        .width(Length::FillPortion(1))
        .height(Length::Fill);
        
        // Корневой контейнер модального оверлея
        container(
            column![
                action_bar,
                iced::widget::rule::horizontal(1), 
                editor_side,
            ]
            .spacing(16)
        )
        .padding(20)      
        .style(container::rounded_box) 
        .into()
    }

    fn handle_event(
        &mut self,
        widget_action: &crate::core::message_bp::WidgetAction,
        app: &mut crate::app::App,
    ) -> iced::Task<crate::core::message::Message> {    

        log::trace!("RichText::handle_event: widget_action = {:?}", widget_action);

        match widget_action {
            // Сообщения RichText для редактора RichText
            WidgetAction::RichText { widget_id, action }
                if widget_id == &self.get_id() => {

                    match action {
                        // Вставляем RFT текст из буфера
                        RichTextEdit::InsertClipboardRTF => {
                            match get_rtf_from_clipboard() {
                                ClipboardContent::Rtf(rtf_text) => {
                                    log::trace!("RichText::handle_event: Получен из буфнра обмена RTF текст: \n{:?}.", rtf_text);

                                    let edit_content: &mut String = unsafe {&mut *self.edit_content.get()};
                                    let edit_pieces:  &mut Vec<TextSpanData> = unsafe { &mut *self.edit_pieces.get() };

                                    *edit_pieces  = rtf_to_span_data(&rtf_text);
                                    *edit_content = rtf_text;
                                }
                                ClipboardContent::PlainText(plain_text) => {
                                    log::trace!("RichText::handle_event: Получен из буфнра обмена простой текст: \n{:?}.", plain_text);
                                    let edit_content: &mut String = unsafe {&mut *self.edit_content.get()};
                                    let edit_pieces:  &mut Vec<TextSpanData> = unsafe { &mut *self.edit_pieces.get() };

                                    *edit_pieces  = 
                                        vec![TextSpanData {
                                            text: plain_text.clone(),
                                            ..Default::default()
                                        }];

                                    *edit_content = plain_text;
                                }
                                ClipboardContent::None => {
                                    log::warn!("RichText::handle_event: В буфере обмена нет данных или произошла ошибка.");
                                }
                            }
                        }

                        RichTextEdit::ApplyChanges => {

                            // Копируем контент редактора в виджет
                            let edit_content: &String = unsafe {&mut *self.edit_content.get()};
                            let edit_pieces:  &Vec<TextSpanData> = unsafe { &mut *self.edit_pieces.get() };
                            let content:       &mut String = unsafe {&mut *self.content.get()};
                            let parsed_pieces: &mut Vec<TextSpanData> = unsafe { &mut *self.parsed_pieces.get() };

                            *content = edit_content.clone();
                            *parsed_pieces = edit_pieces.clone();

                            app.get_factory().set::<String>(&widget_id, PROP_TEXT_CONTENT, edit_content.clone());

                            return iced::Task::done(Message::OverlayEvent(
                                OverlayAction::CloseOverlay
                            ));
                        }
                        RichTextEdit::Clear => {
                            // Очистить RichText
                            let edit_content: &mut String = unsafe {&mut *self.edit_content.get()};
                            let edit_pieces:  &mut Vec<TextSpanData> = unsafe { &mut *self.edit_pieces.get() };

                            edit_content.clear();
                            edit_pieces.clear();
                        }
                    }
                }

            // Пропускаем провие возможные сообщения,
            // которых не должно быть (могут быть указаны по ошибке)
            _ => {}
        }
        iced::Task::none()
    }

}


// -----------------------------------------------------------------------------
use std::convert::TryFrom;
use iced::{Font, font};
use rtf_parser::RtfDocument;


/// Хелпер: Принимает сырую RTF-строку и возвращает готовый для рендеринга Element.
/// Параметр `Message` — это тип ваших сообщений в приложении Iced.
pub fn rtf_to_element<'a, Message>(rtf_text: &str) -> Vec<Span<'_>>
where
    Message: 'a + Clone,
{
    let mut spans = Vec::new();
    let mut current_font = Font::default();

    // Парсим RTF силами готовой библиотеки rtf-parser
    if let Ok(doc) = RtfDocument::try_from(rtf_text) {
        for block in doc.body {
            // Создаем нативный Span из текста блока
            let mut s: Span = Span::new(block.text.clone());

            // Переносим стили оформления из RTF
            if block.painter.bold { current_font.weight = font::Weight::Bold; }
            if block.painter.italic { current_font.style = font::Style::Italic; }

            s = s.font(current_font);

            if block.painter.underline {
                s.underline = true;
            }
            if block.painter.strike {
                s.strikethrough = true;
            }
            //if block.painter.strike { s.strikethrough; }
            //if block.painter.underline { s.underline; }

            // Вытаскиваем RGB цвет из таблицы цветов RTF документа
            let color_idx = block.painter.color_ref;

            if color_idx > 0 {
                // Приводим к usize, так как векторы в Rust индексируются типом usize
                if let Some(rtf_color) = doc.header.color_table.get(&color_idx) {
                    s = s.color(iced::Color::from_rgb8(
                        rtf_color.red, 
                        rtf_color.green, 
                        rtf_color.blue
                    ));
                }
            }

            //if let Some(color_idx) = block.painter.color_ref {
            //    if let Some(rtf_color) = doc.header.color_table.get(color_idx) {
            //        s = s.color(Color::from_rgb8(rtf_color.red, rtf_color.green, rtf_color.blue));
            //    }
            //}
            spans.push(s);
        }
    } else {
        // Если RTF битый или пустой — аккуратно выводим ошибку красным текстом
        spans.push(Span::new("Ошибка чтения формата RTF").color(Color::from_rgb(1.0, 0.0, 0.0)));
    }

    // Собираем rich_text виджет и сразу превращаем его в абстрактный Element интерфейса
    spans
}

pub fn rtf_to_span_data(rtf_text: &str) -> Vec<TextSpanData> {
    let mut pieces = Vec::new();

    let cleaned_rtf = rtf_text.trim_matches(|c| c == '\0' || c == '\r' || c == '\n' || c == ' ').trim();

    // Парсим RTF силами готовой библиотеки rtf-parser
    if let Ok(doc) = RtfDocument::try_from(cleaned_rtf) {
        for block in doc.body {
            
            // Извлекаем цвет из таблицы цветов документа
            let mut custom_color = None;
            let color_idx = block.painter.color_ref;
            
            if color_idx > 0 {
                if let Some(rtf_color) = doc.header.color_table.get(&color_idx) {
                    custom_color = Some(Color::from_rgb8(
                        rtf_color.red, 
                        rtf_color.green, 
                        rtf_color.blue
                    ));
                }
            }

            // Создаем структуру данных TextSpanData
            let piece = TextSpanData {
                text: block.text.clone(),
                bold: block.painter.bold,
                italic: block.painter.italic,
                underline: block.painter.underline,
                strikethrough: block.painter.strike, // rtf-parser использует поле .strike
                color: custom_color,
            };

            pieces.push(piece);
        }
    } else {
        // Если при парсинге RTF ошибка — возвращаем одну структуру с исходным текстом
        pieces.push(TextSpanData {            
            text: rtf_text.to_string(),
            //color: Some(Color::from_rgb(1.0, 0.0, 0.0)),
            ..Default::default()
        });
    }

    pieces
}


#[derive(Debug, Clone)]
pub enum ClipboardContent {
    // Обнаружено богатое форматирование из Word/WordPad
    Rtf(String),

    // Обнаружен обычный плоский текст
    PlainText(String),

    None
}

/*
pub fn get_rtf_from_clipboard() -> Option<String> {
    let _clip = Clipboard::new().ok()?;
    let rtf_format_id = raw::register_format("Rich Text Format")?;
    let rtf_getter = formats::RawData(rtf_format_id.get());
    let mut rtf_bytes = Vec::new();
   
    rtf_getter.read_clipboard(&mut rtf_bytes).ok()?;

    String::from_utf8(rtf_bytes).ok()
}
*/

// --- ВЕТКА ДЛЯ WINDOWS (Низкоуровневый Win32 API) ---
#[cfg(target_os = "windows")]
pub fn get_rtf_from_clipboard() -> ClipboardContent {
    use clipboard_win::{formats, Clipboard, raw};
    use clipboard_win::Getter;

    // Открываем системный буфер обмена Windows
    let _clip = match Clipboard::new() {
        Ok(c) => c,
        Err(_) => return ClipboardContent::None,
    };

    
    // Регистрируем ID для формата RTF в операционной системе
    let rtf_format_id = match raw::register_format("Rich Text Format") {
        Some(id) => id,
        None => return ClipboardContent::None,
    };
    let rtf_u32_id = rtf_format_id.get();

    // ПРОВЕРКА 1: Читаем RTF (Высший приоритет)
    if raw::is_format_avail(rtf_u32_id) {
        let rtf_getter = formats::RawData(rtf_u32_id);
        let mut rtf_bytes = Vec::new();
        
        if rtf_getter.read_clipboard(&mut rtf_bytes).is_ok() {
            // Очищаем системный концевой нулевой байт '\0', который ломал парсер
            if rtf_bytes.last() == Some(&0) {
                rtf_bytes.pop();
            }

            // Декодируем байты через кодировку Ворда (Windows-1251) в UTF-8 String
            let (decoded_string, _, had_errors) = encoding_rs::WINDOWS_1251.decode(&rtf_bytes);
            if !had_errors {
                let clean_rtf = decoded_string.into_owned().trim().to_string();
                if !clean_rtf.is_empty() {
                    // Возвращаем упакованный RTF-вариант
                    return ClipboardContent::Rtf(clean_rtf); 
                }
            }
        }
    }
   
    // ПРОВЕРКА 2: Если RTF нет, забираем обычный чистый текст (Низший приоритет)
    if raw::is_format_avail(formats::CF_UNICODETEXT) {
        let mut text_string = String::new();
        if formats::Unicode.read_clipboard(&mut text_string).is_ok() {
            let clean_text = text_string.trim_matches('\0').trim().to_string();
            if !clean_text.is_empty() {
                // Возвращаем упакованный текст
                return ClipboardContent::PlainText(clean_text); 
            }
        }
    }

    ClipboardContent::None
}

// --- ВЕТКА ДЛЯ MACOS И LINUX (Кроссплатформенный arboard) ---
#[cfg(not(target_os = "windows"))]
/*
pub fn get_rtf_from_clipboard() -> ClipboardContent {
    use arboard::Clipboard;

    let mut ctx = match Clipboard::new() {
        Ok(c) => c,
        Err(_) => return ClipboardContent::None,
    };

    // На Mac/Linux форматы буфера запрашиваются по текстовому имени
    if let Ok(rtf_bytes) = ctx.get_bytes("Rich Text Format") {
        // На других ОС буфер обмена работает в UTF-8, конвертация WINDOWS_1251 не нужна
        if let Ok(rtf_str) = String::from_utf8(rtf_bytes) {
            let clean_rtf = rtf_str.trim().to_string();
            if !clean_rtf.is_empty() {
                return ClipboardContent::Rtf(clean_rtf);
            }
        }
    }

    // Если RTF нет, забираем обычный текст
    if let Ok(text) = ctx.get_text() {
        let clean_text = text.trim().to_string();
        if !clean_text.is_empty() {
            return ClipboardContent::PlainText(clean_text);
        }
    }

    ClipboardContent::None
}
*/

// --- ВЕТКА ДЛЯ MACOS И LINUX (Кроссплатформенный clipboard-rs) ---
#[cfg(not(target_os = "windows"))]
pub fn get_rtf_from_clipboard() -> ClipboardContent {
    // ВАЖНО: Импортируем сам Трейт Clipboard, чтобы стали доступны его методы!
    use clipboard_rs::{Clipboard, ClipboardContext};

    // Создаем контекст буфера обмена
    let ctx = match ClipboardContext::new() {
        Ok(c) => c,
        Err(_) => return ClipboardContent::None,
    };

    // clipboard-rs сам знает MIME-типы для Linux/macOS и возвращает String
    if let Ok(rtf_str) = ctx.get_rich_text() {
        let clean_rtf = rtf_str.trim().to_string();
        if !clean_rtf.is_empty() {
            return ClipboardContent::Rtf(clean_rtf);
        }
    }

    // Если RTF нет или произошла ошибка, забираем обычный плоский текст
    if let Ok(text) = ctx.get_text() {
        let clean_text = text.trim().to_string();
        if !clean_text.is_empty() {
            return ClipboardContent::PlainText(clean_text);
        }
    }

    ClipboardContent::None
}

