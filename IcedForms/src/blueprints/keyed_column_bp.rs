// -----------------------------------------------------------------------------
// Виджет 'keyed_column'
// Индексированный стек — Оптимизированная версия `column` для динамических
// списков, где каждый элемент привязан к уникальному ключу для быстрой перерисовки.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::widget::{container, keyed_column, column, opaque, mouse_area};
use iced::{Alignment, Element, Length, Padding, Pixels, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        KeyedColumnBlueprint::WIDGET_TYPE, //"keyed_column",
        category:    CAT_CONTAIN,
        constructor: create_keyed_column_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_keyed_column_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "KeyedColumn");
    Box::new(KeyedColumnCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct KeyedColumnCreator;

impl WidgetCreator for KeyedColumnCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(KeyedColumnBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct KeyedColumnProps {
    // !!!
    pub pending_placeholder: String,

    pub width: Length,
    pub height: Length,
    pub max_width: Pixels,
    pub padding: Padding,
    pub spacing: Pixels,
    pub align_items: Alignment,
}

#[derive(Debug, Clone /*serde::Serialize, serde::Deserialize*/)]
pub struct KeyedColumnBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<KeyedColumnProps>,
}

impl HasCommonMeta for KeyedColumnBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl KeyedColumnProps {
    // Присваиваем дефолтные значения для контроля пропущенных значений и значений по умолчанию в инспекторе
    pub fn default() -> KeyedColumnProps {
        KeyedColumnProps {
            pending_placeholder: "".to_string(),
            width:     Length::Shrink,
            height:    Length::Shrink,
            max_width: Pixels(0.0), //Pixels(f32::INFINITY),    // По умолчанию 0,0 используется как "неограничено" (для инспектора)
            padding:   Padding::from(0.0),
            spacing:   Pixels(0.0),
            align_items: Alignment::Start,
        }
    }    
}

impl KeyedColumnBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "keyed_column";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: KeyedColumnProps::default().into(),
        }
    }
/*
    pub fn default_props() -> KeyedColumnProps {
        KeyedColumnProps {
            pending_placeholder: "".to_string(),
            width:  Length::Shrink,
            height: Length::Shrink,
            max_width: Pixels(0.0), //Pixels(f32::INFINITY),    // По умолчанию 0,0 используется как "неограничено" (для инспектора)
            padding: Padding::from(0.0),
            spacing: Pixels(0.0),
            align_items: Alignment::Start,
        }
    }
*/
    pub fn parse_props(&self, factory: &Factory) -> KeyedColumnProps {
        // Получить ID виджета
        let widget_id = self.get_id();

        // Получаем дефолтные свойства
        let def = KeyedColumnProps::default(); //Self::default_props();

        let pending_placeholder: String = factory.get_or_set(&widget_id, PROP_PLACEHOLDER, "".to_string());

        // Адаптивные размеры
        let width: Length = factory.get_or_set(&widget_id, PROP_WIDTH, Length::Fill);
        let height: Length = factory.get_or_set(&widget_id, PROP_HEIGHT, def.height);
        let max_width: Pixels = factory.get_or_set(&widget_id, PROP_MAX_WIDTH, def.max_width);

        // Шаг и Внутренний отступ
        let padding: Padding = factory.get_or_set(&widget_id, PROP_PADDING, def.padding);
        let spacing: Pixels = factory.get_or_set(&widget_id, PROP_SPACING, def.spacing);

        // Горизонтальное выравнивание содержимого в колонке
        let align_items = match factory.get_or_set(&widget_id, PROP_ALIGN_ITEMS, def.align_items) {
            Alignment::Start => Alignment::Start,
            Alignment::Center => Alignment::Center,
            Alignment::End => Alignment::End,
        };

        KeyedColumnProps {
            pending_placeholder,

            // Адаптивные размеры
            width,
            height,
            max_width,

            // Шаг и Внутренний отступ
            padding,
            spacing,

            // Выравнивание содержимого
            align_items,
        }
    }
}

//#[typetag::serde]
impl WidgetBlueprint for KeyedColumnBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Функция возвращает статус "Принимает ли детей"
    fn can_accept_child(&self, _factory: &Factory) -> bool {
        true
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_WIDTH,
            PROP_HEIGHT,
            PROP_MAX_WIDTH,
            PROP_PADDING,
            PROP_SPACING,
            PROP_ALIGN_ITEMS,
        ]
    }
/*
    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые типизированные свойства ключевой колонки
        let props = self.parse_props(factory);
        let def   = Self::default_props();

        // Инициализируем макрос keyed_column
        let mut w_column = keyed_column(Vec::<(u64, Element<'a, Message, Theme>)>::new());

        // Применяем свойства только если они отличаются от дефолтных
        if props.width != def.width {
            w_column = w_column.width(props.width);
        }
        if props.height != def.height {
            w_column = w_column.height(props.height);
        }
        if props.spacing != def.spacing {
            w_column = w_column.spacing(props.spacing);
        }
        if props.padding != def.padding {
            w_column = w_column.padding(props.padding);
        }
        if props.align_items != def.align_items {
            w_column = w_column.align_items(props.align_items);
        }
        if props.max_width != def.max_width {
            w_column = w_column.max_width(props.max_width);
        }

        // Собираем детей, у которых parent == self.get_id()
        let mut children_elements: Vec<(u64, Element<'a, Message, Theme>)> = Vec::new();
        for (child_id, child_blueprint) in &factory.blueprints {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            if parent_id == self.get_id() {
                let key = utils::runtime_hash_64(&child_id);
                println!("KeyedColumn: child_id = {}, key = {}", child_id, key);
                //let ch   = child_blueprint.build_element(factory, selected_id);
                //let cont = container(ch);

                // Оборачиваем каждого ребёнка в container, а затем в opaque — внешний тип для каждого ключа будет стабилен
                children_elements.push((key, opaque(container(child_blueprint.build_element(factory, selected_id))).into()));
            }
        }

        // Если он пуст — пушим заглушку, иначе — вливаем всех детей разом
        //if !is_has_childs {
        //    w_column = w_column.push(0, utils_bp::create_empty_placeholder(&self.get_id(), &self.widget_type(), props.width, props.height));
        //}

        if children_elements.is_empty() {
            if props.pending_placeholder != "visible" {
                // Ставим признак отображения плейсхолдера и пропускаем кадр,
                // даем время внутреннему кэшу keyed_column очиститься
                factory.set(&self.get_id(), PROP_PLACEHOLDER, "visible".to_string());
            } else {    
                let key = utils::runtime_hash_64("KeyedColumn placeholder");
                w_column = w_column.push(
                    key,
                    opaque(container(utils_bp::create_empty_placeholder(
                        &self.get_id(),
                        &self.widget_type(),
                        props.width,
                        props.height,
                    ))),
                );
            }
        } else {
            //w_column = w_column.push(0, utils_bp::create_empty_placeholder(&self.get_id(), &self.widget_type(), props.width, props.height));
            //w_column = w_column.extend(children_elements);
            //w_column = keyed_column(children_elements);

            for (idx, el) in children_elements {
                let id = idx;

                //w_column = w_column.push(id, button(text("123")));
                w_column = w_column.push(id, el);
            }
            // Триггер пропуска кадра
            factory.set(&self.get_id(), PROP_PLACEHOLDER, "".to_string());
        }

        // Обертка в интерактивную область для Design Mode
        // Используем mouse_area вместо Button, чтобы не вкладывать Button внутри Button
        // (вложенные интерактивные виджеты с on_press приводят к паникам в Iced).
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            mouse_area(w_column)
                .interaction(iced::mouse::Interaction::Pointer)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            w_column.into()
        };

        // В самом конце применяем магию подсветки из трейта в одну строчку!
        apply_design_overlay(
            element,
            factory,
            selected_id,
            &self.get_id(),
            props.width,
            props.height,
            4.0.into(),
            false, // Для этого элемента рендерим рамку всегда /* Fix */
        )
    }
*/
    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые типизированные свойства ключевой колонки
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);
        let def   = KeyedColumnProps::default();

        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // Инициализируем макрос column
            let mut w_column = column![]
                .width(props.width)
                .height(props.height)
                .spacing(props.spacing)
                .padding(props.padding)
                .align_x(props.align_items);

            // Применяем только значение отличное от дефолтного
            // Значение Pixels(0.0) инспектора соответствует внутеннему Pixels(f32::INFINITY) Iced
            if props.max_width != def.max_width {
                w_column = w_column.max_width(props.max_width);
            }

            // Собираем детей, у которых parent == self.get_id()
            let mut children_elements: Vec<Element<'a, Message, Theme>> = Vec::new();
            for (child_id, child_blueprint) in factory.blueprints_iter() {
                let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

                if parent_id == self.get_id() {
                    // Оборачиваем каждого ребёнка в container, а затем в opaque — внешний тип для каждого ключа будет стабилен
                    children_elements.push(child_blueprint.build_element(factory, selected_id).into());
                }
            }

            if children_elements.is_empty() {
                w_column = w_column.push(
                    utils_bp::create_empty_placeholder(
                        &self.get_id(),
                        &self.widget_type(),
                        props.width,
                        props.height,
                    ),
                );            
            } else {
                w_column = w_column.extend(children_elements);
            }

            // Обертка в интерактивную область для Design Mode
            // Используем mouse_area вместо Button, чтобы не вкладывать Button внутри Button
            // (вложенные интерактивные виджеты с on_press приводят к паникам в Iced).
            let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
                mouse_area(w_column)
                    .interaction(iced::mouse::Interaction::Pointer)
                    .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                    .into()
            } else {
                w_column.into()
            };

            // В самом конце применяем магию подсветки из трейта в одну строчку!
        apply_design_overlay(
            element,
            factory.is_design_mode(),
            selected_id,
            &self.get_id(),
        )
        } else {
            // Инициализируем макрос keyed_column
            let mut w_column = keyed_column(Vec::<(u64, Element<'a, Message, Theme>)>::new())
                .width(props.width)
                .height(props.height)
                .spacing(props.spacing)
                .padding(props.padding)
                .align_items(props.align_items);

            // Применяем только значение отличное от дефолтного
            // Значение Pixels(0.0) инспектора соответствует внутеннему Pixels(f32::INFINITY) Iced
            if props.max_width != def.max_width {
                w_column = w_column.max_width(props.max_width);
            }

            // Собираем детей, у которых parent == self.get_id()
            let mut children_elements: Vec<(u64, Element<'a, Message, Theme>)> = Vec::new();
            for (child_id, child_blueprint) in factory.blueprints_iter() {
                let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

                if parent_id == self.get_id() {
                    let key = utils::runtime_hash_64(&child_id);

                    // Оборачиваем каждого ребёнка в container, а затем в opaque — внешний тип для каждого ключа будет стабилен
                    children_elements.push((key, opaque(container(child_blueprint.build_element(factory, selected_id))).into()));
                }
            }

            if children_elements.is_empty() {
                let key = utils::runtime_hash_64("KeyedColumn placeholder");
                w_column = w_column.push(
                    key,
                    opaque(container(utils_bp::create_empty_placeholder(
                        &self.get_id(),
                        &self.widget_type(),
                        props.width,
                        props.height,
                    ))),
                );
            } else {
                w_column = w_column.extend(children_elements);
            }

            w_column.into()
        };

        element
    }
}
