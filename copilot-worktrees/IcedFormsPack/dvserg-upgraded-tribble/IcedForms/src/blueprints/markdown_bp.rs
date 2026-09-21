// -----------------------------------------------------------------------------
// Виджет 'markdown'
// Рендерер разметки — Парсит и визуализирует стандартный Markdown-текст,
// автоматически преобразуя заголовки, списки и жирный шрифт в UI-элементы.
// -----------------------------------------------------------------------------
use std::rc::Rc;
use std::cell::{UnsafeCell};
use iced::widget::{button, column, container, markdown, mouse_area, row, scrollable, text, text_editor, responsive};
use iced::{Element, Length, Padding, Pixels, Theme};
use iced::alignment::{Vertical};

//use crate::blueprints::message_bp::WidgetAction;
use crate::ui::UIStyle;
use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в глобальном реестре
// -----------------------------------------------------------------------------
inventory::submit! {
    AutoRegisteredWidget {
        name:        MarkdownBlueprint::WIDGET_TYPE,    // "markdown"
        category:    CAT_INPUTS,                        // Используем категорию отображения данных
        constructor: create_markdown_creator,
    }
}

// Функция-помощник для автоматической регистрации указателя фабрики
fn create_markdown_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Markdown");
    Box::new(MarkdownCreator)
}

// Фабричный конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct MarkdownCreator;

impl WidgetCreator for MarkdownCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(MarkdownBlueprint::new(id))
    }
}

/*
struct CacheMarkdown {
    content_key: String,
    items: *const [markdown::Item],
}
*/

// -----------------------------------------------------------------------------
// Свойства виджета Markdown, доступные для инспектора редактора
// -----------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct MarkdownProps {
    pub content: String,
    pub width: Length,
    pub height: Length,
    pub max_width: Pixels,
    pub padding: Padding,
}

// -----------------------------------------------------------------------------
// Основная Blueprint-структура метаданных виджета
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct MarkdownBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<MarkdownProps>,

    pub content: UnsafeCell<Vec<markdown::Item>>,
    pub editor:  UnsafeCell<text_editor::Content>,

}

impl HasCommonMeta for MarkdownBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl MarkdownBlueprint {
    // Константный уникальный строковый тип виджета
    const WIDGET_TYPE: &'static str = "markdown";

    pub fn new(id: String) -> Self {
        // При инициализации экземпляра виджета считываем свойства из VTable
        //let props = self.parse_props(factory);
        let def = MarkdownProps::default();

        let initial_content: Vec<markdown::Item> = markdown::parse(&def.content).collect();
        let initial_editor = text_editor::Content::with_text(&def.content);

        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: MarkdownProps::default().into(),

            content: UnsafeCell::new(initial_content),
            editor:  UnsafeCell::new(initial_editor),
        }
    }

    // Парсинг динамических адаптивных свойств из хранилища Factory
    pub fn parse_props(&self, factory: &Factory) -> MarkdownProps {
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = MarkdownProps::default();

        // Читаем или инициализируем сырой Markdown текст
        let content: String = factory.get_or_set(&widget_id, PROP_TEXT_CONTENT, def.content);

        // Размеры и геометрия
        let width:     Length  = factory.get_or_set(&widget_id, PROP_WIDTH,     def.width);
        let height:    Length  = factory.get_or_set(&widget_id, PROP_HEIGHT,    def.height);
        let max_width: Pixels  = factory.get_or_set(&widget_id, PROP_MAX_WIDTH, def.max_width);
        let padding:   Padding = factory.get_or_set(&widget_id, PROP_PADDING,   def.padding);

        // Апдейтим локальные unsafe буфферы
        //let content_mut_ref: &mut Vec<markdown::Item>  = unsafe { &mut *self.content.get() };
        //let editor_mut_ref:  &mut text_editor::Content = unsafe { &mut *self.editor.get() };

        //*content_mut_ref = markdown::parse(&content).collect();
        //*editor_mut_ref  = text_editor::Content::with_text(&content);

        MarkdownProps {
            content,
            width,
            height,
            max_width,
            padding,
        }
    }
}


impl Default for MarkdownProps {
    // Присваиваем дефолтные значения для контроля пропущенных значений и значений по умолчанию в инспекторе
    // Дефолтные свойства инициализации виджета
    fn default() -> MarkdownProps {
        MarkdownProps {
            content: String::from(
                "## Привет, Markdown!\nЭто встроенный текст по умолчанию. **Жирный**, *курсив* или `код`.",
            ),
            width: Length::Fill,
            height: Length::Shrink,
            max_width: Pixels(0.0), // 0.0 используется в качестве "неограничено" для инспектора
            padding: Padding::from(5.0),
        }
    }
}
// -----------------------------------------------------------------------------
// Реализация основного трейта построения элемента разметки Iced
// -----------------------------------------------------------------------------
impl WidgetBlueprint for MarkdownBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_TEXT_CONTENT, // Поле для ввода сырого Markdown текста
            PROP_WIDTH,
            PROP_HEIGHT,
            PROP_MAX_WIDTH,
            PROP_PADDING,
        ]
    }

    // Инициализация из VTable
    fn from_vtab(&self, _factory: &Factory) {
        let props = self.parse_props(_factory);
        let initial_content: Vec<markdown::Item> = markdown::parse(&props.content).collect();
        let initial_editor = text_editor::Content::with_text(&props.content);

        // Обход ограничений мутабельности через unsafecell
        let content_mut_ref: &mut Vec<markdown::Item>  = unsafe { &mut *self.content.get() };
        let editor_mut_ref:  &mut text_editor::Content = unsafe { &mut *self.editor.get() };

        *content_mut_ref = initial_content;
        *editor_mut_ref  = initial_editor;

        log::trace!("from_vtab: Инициализация свойств блюпринта 
            <{}> виджета '{}' из VTable.", 
            self.widget_type(), 
            self.get_id()
        );

    }
    // Внимание: сигнатура принимает &self. Это легитимно для Rc!
    /*
    fn refresh_internal_props(&self, factory: &Factory) {
        // Функция читает свежие свойства из VTable и возвращает MarkdownProps
        let fresh_props: MarkdownProps = self.parse_props(factory); 

        // Вскрываем RefCell на запись. 
        // Метод borrow_mut() даёт нам временную мутабельную ссылку &mut MarkdownProps
        //let mut current_props = self.props.borrow_mut();

        // ПРИСВАИВАЕМ прочитанную структуру. Старая заменяется новой!
        //*current_props = fresh_props;

        let mut content = self.content.borrow_mut();

        content = fresh_props.content;
     
        log::trace!("refresh_internal_props: Обновление собственных свойств блюпринта <{}> виджета '{}' из VTable.", self.widget_type(), self.get_id());
    }
    */
    

    /*
    fn build_editor_content<'a>(&'a self, factory: &'a Factory) -> Element<'a, Message, Theme> {
        let props = self.parse_props(factory);
        let widget_id = self.get_id();
        let widget_id_for_action = widget_id.clone();

        let cached_content = factory.get_or_create_text_editor_content(
            &widget_id,
            &props.content,
            Some(&props.content),
        );

        let editor = text_editor(cached_content)
            .height(Length::Fill)
            .padding(10)
            .on_action(move |action| Message::WidgetEvent(WidgetAction::TextChanged {
                widget_id: widget_id_for_action.clone(),
                text_editor_action: action,
            }));

        let parsed_items = factory.get_or_create_markdown_items(&widget_id, &props.content);
        let preview = markdown::view(parsed_items.iter(), iced::Theme::Dark).map(|_uri| Message::NoOp);

        let editor_column = column![
            text("Markdown content").size(16),
            container(scrollable(editor).width(Length::Fill).height(Length::Fill)).padding(8),
        ]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::Fill);

        let preview_column = column![
            text("Preview").size(16),
            container(scrollable(preview).width(Length::Fill).height(Length::Fill)).padding(8),
        ]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::Fill);

        let action_row = row![
            button(text("Закрыть"))
                .on_press(Message::OverlayEvent(OverlayAction::CloseOverlay))
                .padding(10),
            button(text("Сохранить"))
                .on_press(Message::OverlayEvent(OverlayAction::CloseOverlay))
                .padding(10),
        ]
        .spacing(12);

        container(
            column![
                text("Редактор Markdown").size(20),
                text("Измените содержимое и сразу увидите его отображение ниже.")
                    .size(13),
                action_row,
                row![editor_column, preview_column]
                    .spacing(12)
                    .width(Length::Fill)
                    .height(Length::Fill),
            ]
            .spacing(14)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(12),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
    */
    */
    /*
    fn build_editor_content<'a>(&'a self, factory: &'a Factory) -> Element<'a, Message, Theme> {
        
        // Поля данных
        let widget_title = &self.get_id();//meta.name; 
        let markdown_text = ""; 
        let rendered_preview: Element<'a, Message, Theme> = text("Здесь будет отрендеренный Markdown...").into();

        // 1. Верхняя панель
        let action_bar = row![
            text(format!("Редактирование: {}", widget_title))
                .size(18)
                .width(Length::Fill),
            button("Закрыть"), 
            button("Сохранить"), 
        ]
        .spacing(12)
        .align_y(Vertical::Center);

        // 2. Тулбар форматирования
        let format_toolbar = row![
            button("Ж"), 
            button("К"),
            button("H1"),
            button("H2"),
            button("Код"),
            button("Список"),
        ]
        .spacing(6);

        // Функция-помощник для создания рамки
        let border_style = |theme: &Theme| {
            let palette = theme.palette();
            container::Style::default()
                .border(Border {
                    color: palette.text, 
                    width: 1.0,
                    radius: 4.0.into(), 
                })
        };

        // 3. Левая колонка (Редактор)
        let editor_side = column![
            text("Исходный код Markdown").size(13),
            format_toolbar,
            container(
                scrollable(
                    text_input("Введите Markdown текст здесь...", markdown_text)
                        .size(14)
                )
            )
            .style(border_style) 
            .padding(8)
            .height(Length::Fill)
            .width(Length::Fill)
        ]
        .spacing(10)
        .width(Length::FillPortion(1));

        // 4. Правая колонка (Предпросмотр)
        let preview_side = column![
            text("Предпросмотр").size(13),
            container(text("")).height(30), 
            container(
                scrollable(rendered_preview)
            )
            .style(border_style) 
            .padding(12)
            .height(Length::Fill)
            .width(Length::Fill)
        ]
        .spacing(10)
        .width(Length::FillPortion(1));

        // 5. Разделение сплит-экрана вертикальной линией
        let main_body = row![
            editor_side,
            rule::vertical(1), 
            preview_side,
        ]
        .spacing(16)
        .height(Length::Fill);

        // 6. Итоговый контейнер всего модального окна
        container(
            column![
                action_bar,
                rule::horizontal(1), 
                main_body,
            ]
            .spacing(16)
        )
        .max_width(950)   
        .max_height(650)  
        .padding(20)      
        .style(container::rounded_box) 
        .into()
    }
    */


    // Встроенный редактор Markdown для демонстрации его возможностей
    fn build_editor_content<'a>(&'a self, _factory: &'a Factory) -> Element<'a, Message, Theme> {

        let ui_style = UIStyle::default();

        // Получение идентификатора текущего виджета и актуального текста из фабрики
        let widget_id = self.get_id();
        //let props     = self.props.borrow(); //self.props.borrow();
        //let props = self.parse_props(factory);

        // Обход ограничений времени жизни локального vectors кадра отрисовки
        let content_ref: &'a Vec<markdown::Item> = unsafe { &mut *self.content.get() };

        let rendered_preview: Element<'a, Message, Theme> = iced::widget::markdown::view(
            content_ref,
            iced::Theme::Dark,
        )
        .map(|_| Message::NoOp)     
        .into();

        // Панель управления (закрытие окна)
        let action_bar = row![
            text(format!("Редактирование Markdown: {}", widget_id.clone()))
                .size(18)
                .width(Length::Fill),
            button("Закрыть").on_press(Message::OverlayEvent(OverlayAction::CloseOverlay)), 
        ]
        .spacing(12)
        .align_y(Vertical::Center); // Исправлен метод и тип выравнивания

        // Панель быстрого форматирования синтаксиса
        let toolbar_row = row![
            crate::ui::toolbar_small_button( crate::ui::ICON_TEXT_BOLD, "", "Полужирный",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::Markdown{
                        widget_id: widget_id.clone(), 
                        action:    MarkdownEdit::FormatBold
                    }
                ),
                ui_style
            ),
            crate::ui::toolbar_small_button( crate::ui::ICON_TEXT_ITALIC, "", "Курсив",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::Markdown{
                        widget_id: widget_id.clone(), 
                        action:    MarkdownEdit::FormatItalic
                    }
                ),
                ui_style
            ),
            crate::ui::toolbar_small_button( crate::ui::ICON_TEXT_STRIKE, "", "Зачеркнутый",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::Markdown{
                        widget_id: widget_id.clone(), 
                        action:    MarkdownEdit::FormatStrikethrough
                    }
                ),
                ui_style
            ),
            crate::ui::toolbar_small_button( crate::ui::ICON_TEXT_H1, "", "Заголовок",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::Markdown{
                        widget_id: widget_id.clone(), 
                        action:    MarkdownEdit::FormatH1
                    }
                ),
                ui_style
            ),
            crate::ui::toolbar_small_button( crate::ui::ICON_TEXT_H2, "", "Раздел",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::Markdown{
                        widget_id: widget_id.clone(), 
                        action:    MarkdownEdit::FormatH2
                    }
                ),
                ui_style
            ),
            crate::ui::toolbar_small_button( crate::ui::ICON_TEXT_H3, "", "Подраздел",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::Markdown{
                        widget_id: widget_id.clone(), 
                        action:    MarkdownEdit::FormatH3
                    }
                ),
                ui_style
            ),
            crate::ui::toolbar_small_button( crate::ui::ICON_TEXT_CODE, "", "Код",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::Markdown{
                        widget_id: widget_id.clone(), 
                        action:    MarkdownEdit::FormatCode
                    }
                ),
                ui_style
            ),
            crate::ui::toolbar_small_button( crate::ui::ICON_TEXT_QUOTE, "", "Цитата",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::Markdown{
                        widget_id: widget_id.clone(), 
                        action:    MarkdownEdit::FormatBlockquote
                    }
                ),
                ui_style
            ),
            crate::ui::toolbar_small_button( crate::ui::ICON_TEXT_LIST, "", "Список",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::Markdown{
                        widget_id: widget_id.clone(), 
                        action:    MarkdownEdit::FormatList
                    }
                ),
                ui_style
            ),
            crate::ui::toolbar_small_button( crate::ui::ICON_TEXT_NLIST, "", "Нумерованый список",
                Message::WidgetEvent(
                    widget_id.clone(),
                    WidgetAction::Markdown{
                        widget_id: widget_id.clone(), 
                        action:    MarkdownEdit::FormatOrderedList
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

        // Извлекаем вечную ссылку &'a на контент напрямую из UnsafeCell
        let editor_ref: &'a text_editor::Content = unsafe {
            let ptr = self.editor.get();
            &*ptr // Разыменовываем и берем ссылку, Rust проливает лайфтайм 'a
        };

        // Левая область: Многострочный текстовый редактор кода
        let editor_side = column![
            text("Исходный код Markdown").size(14),
            //format_toolbar,
            toolbar_row,
            container(
                responsive(move |size| {
                    // Определяем размер блока для динамического
                    // задания минимального размера text_editor
                    let height = size.height;
                    let widget_id_cl = widget_id.clone();

                    scrollable (
                        iced::widget::text_editor(editor_ref)
                            .placeholder("Пишите ваш Markdown текст здесь...")
                            .on_action(move |action| {
                                // Трансляция внутренних событий редактора в архитектуру WidgetAction проекта
                                Message::WidgetEvent( 
                                    widget_id_cl.clone(),
                                    WidgetAction::TextChanged {
                                        widget_id: widget_id_cl.to_string(),
                                        text_editor_action: action,
                                    }
                                )
                            })
                            // Динамически подгоняем минимальный размер text_editor
                            // под изменяющийся размер формы
                            .min_height(height)
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

        // Правая область: Скомпилированный интерактивный предпросмотр
        let preview_side = column![
            text("Предпросмотр").size(14),
            container(text("")).height(28), // Выравнивание по высоте относительно левой панели с тулбаром
            container(
                scrollable(rendered_preview)
            )
            .style(border_style) 
            .padding(12)
            .height(Length::Fill)
            .width(Length::Fill)
        ]
        .spacing(10)
        .width(Length::FillPortion(1));

        // Вертикальное разделение интерфейса
        let main_body = row![
            editor_side,
            iced::widget::rule::vertical(1), 
            preview_side,
        ]
        .spacing(16)
        .height(Length::Fill);

        // Корневой контейнер модального оверлея
        container(
            column![
                action_bar,
                iced::widget::rule::horizontal(1), 
                main_body,
            ]
            .spacing(16)
        )
        //.max_width(950)   
        //.max_height(650)  
        .padding(20)      
        .style(container::rounded_box) 
        .into()
    }
   

    fn handle_event(
        &mut self,
        widget_action: &crate::core::message_bp::WidgetAction,
        app: &mut crate::app::App,
    ) -> iced::Task<crate::core::message::Message> {
        
        log::trace!("Markdown::handle_event: widget_action = {:?}", widget_action);

        match widget_action {
            // Сообщения TextChanged для TextEdit в редакторе Murkdown
            WidgetAction::TextChanged { widget_id, text_editor_action }
                if widget_id == &self.get_id() => {

                    log::trace!("MarkdownBlueprint::handle_event: WidgetAction::TextChanged for widget_id = {}, action = {:?}", widget_id, text_editor_action);     

                    // Применяем экшен изменения текста напрямую к локальному буферу блюпринта
                    // Вынимаем мутабельную ссылку из <UnsafeCell>
                    let editor_mut_ref:  &mut text_editor::Content = unsafe { &mut *self.editor.get() };
                    let content_mut_ref: &mut Vec<markdown::Item>  = unsafe { &mut *self.content.get() };

                    // Применяем изменения редактора
                    editor_mut_ref.perform(text_editor_action.clone());

                    // Извлекаем новую строку
                    let new_text = editor_mut_ref.text();

                    // Апдейтим буфер макрдауна
                    *content_mut_ref = markdown::parse(&new_text).collect();

                    // Синхронизируем VTable фабрики со свежим текстом.
                    app.get_factory_mut().set(&self.get_id(), PROP_TEXT_CONTENT, new_text);
                }

            WidgetAction::Markdown { widget_id, action }
                if widget_id == &self.get_id() => {
                    use iced::widget::text_editor::{Action as EditorAction, Edit};

                    log::trace!("MarkdownBlueprint::handle_event: WidgetAction::Markdown action for widget_id = {}, action = {:?}", widget_id, action);

                    // Определяем маркдаун-теги
                    let (open_tag, close_tag, _is_wrapping) = match action {
                        MarkdownEdit::FormatBold          => ("**",     "**", true),
                        MarkdownEdit::FormatItalic        => ("*",      "*",  true),
                        MarkdownEdit::FormatH1            => ("\n# ",   "",   false),    // Одиночный префикс
                        MarkdownEdit::FormatH2            => ("\n## ",  "",   false),
                        MarkdownEdit::FormatH3            => ("\n### ", "",   false),
                        MarkdownEdit::FormatCode          => ("`",      "`",  true),
                        MarkdownEdit::FormatStrikethrough => ("~~",     "~~", true),     // Зачеркнутый текст
                        MarkdownEdit::FormatBlockquote    => ("\n> ",   "",   false),    // Цитата
                        MarkdownEdit::FormatList          => ("\n- ",   "",   false),
                        MarkdownEdit::FormatOrderedList   => ("\n1. ",  "",   false),
                    };

                    // Получаем доступ к буферам виджета для text_edit и markdown
                    let editor_mut_ref:  &mut text_editor::Content = unsafe { &mut *self.editor.get() };
                    let content_mut_ref: &mut Vec<markdown::Item>  = unsafe { &mut *self.content.get() };

                    //let current_text = editor_mut_ref.text();
                    let selection    = editor_mut_ref.selection();  // Возвращает Range<usize> или None

                    // Оборачиваем тэгом
                    let current_text = if let Some(text) = selection.clone() {
                        // Заменяем выделенный диапазон на обернутый текст: [open][выделение][close]
                        format!("{}{}{}", open_tag, text, close_tag)
                    } else {                            
                        // Постим пустые тэги
                        format!("{}{}", open_tag, close_tag)
                    };

                    // Апдейтим изменения в text_editor
                    // Курсор автоматически окажется ПОСЛЕ закрывающего тега.
                    let paste_text = std::sync::Arc::new(current_text);
                    editor_mut_ref.perform(EditorAction::Edit(Edit::Paste (paste_text)));

                    // При пустом selected смещаем курсор назад на величину закрывающего тэга
                    // Для префиксных тэгов курсор останется на месте
                    if selection.is_none() && close_tag.len() > 0 {
                        for _ in 0..close_tag.len() {
                            editor_mut_ref.perform(EditorAction::Move(
                                iced::widget::text_editor::Motion::Left
                            ));
                        }
                    }

                    // Извлекаем новую строку
                    let new_text = editor_mut_ref.text();

                    // Апдейтим буфер маркдауна
                    *content_mut_ref = markdown::parse(&new_text).collect();

                    // Синхронизируем VTable фабрики со свежим текстом.
                    app.get_factory_mut().set(&self.get_id(), PROP_TEXT_CONTENT, new_text);
                }

            // Пропускаем провие возможные сообщения,
            // которых не должно быть (могут быть указаны по ошибке)            
            _ => {}
        }

        iced::Task::none()
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые свойства из фабрики
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);
        let def   = MarkdownProps::default();

        let content_ref: &'a Vec<markdown::Item> = unsafe {
            let ptr = self.content.get();
            &*ptr // Разыменовываем *mut Vec и берем чистейшую ссылку &Vec
        };

        // Создаем базовый виджет markdown::view, передаём итератор по срезу
        //let w_markdown = markdown::view(&self.content/*parsed_items.iter()*/, iced::Theme::Dark)
        let w_markdown = markdown::view(content_ref, iced::Theme::Dark)
            // Внутренние ссылки мапим в Message вашей системы
            .map(|_uri| Message::NoOp);

        // Оборачиваем во внешний контейнер для соблюдения геометрии и отступов
        let mut w_container = container(w_markdown)
            .width(props.width)
            .height(props.height)
            .padding(props.padding);

        // Применяем ограничение максимальной ширины
        if props.max_width != def.max_width {
            w_container = w_container.max_width(props.max_width);
        }

        // ПРИМЕНЕНИЕ РЕЖИМА ДИЗАЙНА:
        // Оборачиваем контейнер в безопасную mouse_area для кликабельности в дизайн-моде
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            mouse_area(w_container)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            w_container.into()
        };

        // Отрисовываем дизайн-рамку (выделение активного элемента в окне инспектора)
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

        // Извлекаем текущие свойства Markdown из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = MarkdownProps::default();

        // Сравниваем свойства строго по списку editable_properties
        if current.content != default.content {
            prop_names.push(PROP_TEXT_CONTENT);
        }
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.height != default.height {
            prop_names.push(PROP_HEIGHT);
        }
        if current.max_width != default.max_width {
            prop_names.push(PROP_MAX_WIDTH);
        }
        if current.padding != default.padding {
            prop_names.push(PROP_PADDING);
        }

        prop_names
    }

}
