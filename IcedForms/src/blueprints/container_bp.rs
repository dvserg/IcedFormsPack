// -----------------------------------------------------------------------------
// Виджет 'container'
// Универсальный контейнер — Ограничивает один дочерний элемент. Используется для
// задания внутренних отступов (`padding`), выравнивания, фонового цвета, рамок и теней.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::{Color, Element, Length, Padding, Pixels, Theme};
use iced::widget::{container, mouse_area};
use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: ContainerBlueprint::WIDGET_TYPE, //"container",
        category: CAT_CONTAIN,
        constructor: create_container_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_container_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Container");
    Box::new(ContainerCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct ContainerCreator;

impl WidgetCreator for ContainerCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(ContainerBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Структура для распарсенных свойств контейнера
#[derive(Debug, Clone)]
pub struct ContainerProps {
    pub width:          Length,
    pub height:         Length,    
    pub max_width:      Pixels,
    pub max_height:     Pixels,
    pub padding:        Padding,
    pub bg_color:       Color,
    pub border_radius:  Radius,
    pub border_width:   f32,
    pub border_color:   Color,
    pub align_x:        Horizontal,
    pub align_y:        Vertical,
    // pub clip:        bool,
}

#[derive(Debug, Clone)]
pub struct ContainerBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<ContainerProps>,
}

impl HasCommonMeta for ContainerBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl ContainerBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "container";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: ContainerProps::default().into(),
        }
    }

    // Парсинг свойств с использованием хелперов Factory
    fn parse_props(&self, factory: &Factory) -> ContainerProps {
        // Получить ID виджета
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = ContainerProps::default();

        // (*) Некоторые исходные размеры устанавливаем отличными от дефолтных для работы конструктора

        // Адаптивные размеры
        let width:  Length = factory.get_or_set(&widget_id, PROP_WIDTH,  Length::Fill);                     // Ширина, исходное значение Length::Fill
        let height: Length = factory.get_or_set(&widget_id, PROP_HEIGHT, def.height);

        let max_width:  Pixels = factory.get_or_set(&widget_id, PROP_MAX_WIDTH,  def.max_width);
        let max_height: Pixels = factory.get_or_set(&widget_id, PROP_MAX_HEIGHT, def.max_height);

        // Шаг и Внутренние отступы
        let padding: Padding = factory.get_or_set(&widget_id, PROP_PADDING, def.padding);

        // Выравнивание для осей X и Y
        let align_x: Horizontal = factory.get_or_set(&widget_id, PROP_ALIGN_X, def.align_x);
        let align_y: Vertical   = factory.get_or_set(&widget_id, PROP_ALIGN_Y, def.align_y);

        // Параметры границ и скруглений
        let bg_color:      Color  = factory.get_or_set(&widget_id, PROP_BG_COLOR,      def.bg_color);
        let border_radius: Radius = factory.get_or_set(&widget_id, PROP_BORDER_RADIUS, def.border_radius);
        let border_width:  f32    = factory.get_or_set(&widget_id, PROP_BORDER_WIDTH,  def.border_width);
        let border_color:  Color  = factory.get_or_set(&widget_id, PROP_BORDER_COLOR,  def.border_color);

        ContainerProps {
            // Адаптивные размеры
            width,
            height,
            max_width,
            max_height,

            // Шаг и Внутренние отступы
            padding,

            // Выравнивание для осей X и Y
            align_x,
            align_y,

            // Параметры границ и скруглений
            bg_color,
            border_radius,
            border_width,
            border_color,
        }
    }
}

impl Default for ContainerProps {
    // Присваиваем дефолтные значения для контроля пропущенных значений и значений по умолчанию в инспекторе
    fn default() -> ContainerProps {
        ContainerProps {
            width:          Length::Shrink,
            height:         Length::Shrink,
            max_width:      Pixels(0.0),
            max_height:     Pixels(0.0),
            padding:        Padding::ZERO,

            align_x:        Horizontal::Left,
            align_y:        Vertical::Top,

            bg_color:       Color::TRANSPARENT,
            border_radius:  0.0.into(),
            border_width:   0.0_f32,
            border_color:   Color::TRANSPARENT,
        }
    }
}

//#[typetag::serde]
impl WidgetBlueprint for ContainerBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Функция возвращает статус "Принимает ли детей"
    fn can_accept_child(&self, factory: &Factory) -> bool {
        let widget_id = self.get_id();
        let can_accept = factory.get_blueprints_by_parent(&widget_id).is_empty();
        can_accept // Может принять детей (одного), если их список пустой (True)
    }
    /*    
    fn can_accept_child(&self, factory: &Factory) -> bool {
        let is_occupied = factory.blueprints.keys().any(|child_id| {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            parent_id == self.get_id()
        });

        !is_occupied // True если пустой, False если уже есть ребенок
    }
    */

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_WIDTH,
            PROP_HEIGHT,
            PROP_MAX_WIDTH,
            PROP_MAX_HEIGHT,
            PROP_PADDING,
            PROP_ALIGN_X,
            PROP_ALIGN_Y,
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
        // Получаем обновленные свойства (включая width и height)
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Ищем и рендерим вложенный контент (ребенка) контейнера, если он есть
        let mut child_element: Option<Element<'a, Message, Theme>> = None;

        for (child_id, child_blueprint) in factory.blueprints_iter() {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            if parent_id == self.get_id() {
                child_element = Some(child_blueprint.build_element(factory, selected_id));
                break; // У обычного контейнера может быть только один прямой потомок
            }
        }

        // Собираем нативный виджет container из Iced 0.14
        let content_view = child_element.unwrap_or_else(|| {
            // Если ребенка нет, создаем невидимую заглушку для режима конструктора
            create_empty_placeholder(
                &self.get_id(),
                &self.widget_type(),
                Length::Shrink, //props.width,
                Length::Shrink, //props.height,
            )
        });

        let props_cl = props.clone();
        let mut w_container = container(content_view)
            .width(props.width)
            .height(props.height)
            .padding(props.padding)
            .align_x(props.align_x)
            .align_y(props.align_y)
            .style(move |_theme| {
                // Берем готовую тему 'transparent' и меняем только то, что нам нужно
                // Тема 'transparent' - это чистая дефолтная тема
                let mut base_style = container::transparent(_theme);

                // Получить дефолтные значения
                let def = ContainerProps::default();

                // Применяем параметры только если они отличаются от дефолтных
                // Если указан прозрачный цвет - оставляем дефолтный
                if props_cl.bg_color != Color::TRANSPARENT {
                    base_style.background = Some(iced::Background::Color(props_cl.bg_color));
                }
                if props_cl.border_color != Color::TRANSPARENT {
                    base_style.border.color = props_cl.border_color;
                }
                if props_cl.border_width != def.border_width {
                    base_style.border.width = props_cl.border_width;
                }
                if props_cl.border_radius != def.border_radius {
                    base_style.border.radius = props_cl.border_radius;
                }
                
                base_style






                /*
                container::Style {
                background: Some(iced::Background::Color(props.bg_color)),
                border: iced::Border {
                    // Если толщина рамки не задана, в режиме дизайна показываем тонкую дефолтную сетку
                    color: if props.border_color != Color::TRANSPARENT {
                        props.border_color
                    } else {
                        Color::from_rgb(0.8, 0.8, 0.8)
                    },
                    width: if props.border_width > 0.0 {
                        props.border_width
                    } else {
                        if factory.is_design_mode() { 1.0 } else { 0.0 }
                    },
                    radius: props.border_radius.into(),
                },
                ..Default::default()
                }
                */
            });

        // Получить дефолтные значения
        let def = ContainerProps::default();

        // Применяем только значение отличное от дефолтного
        // Значение Pixels(0.0) инспектора соответствует внутеннему Pixels(f32::INFINITY) Iced
        if props.max_width != def.max_width {
            w_container = w_container.max_width(props.max_width);
        }
        if props.max_height != def.max_height {
            w_container = w_container.max_height(props.max_height);
        }

        // Обертка для Design Mode (кнопка выбора по клику)
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            mouse_area(w_container)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()                
        } else {
            w_container.into()
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

        // Извлекаем текущие свойства контейнера из фабрики
        let current = self.parse_props(factory);
        
        // Получаем абсолютно чистые дефолтные свойства Iced для сравнения
        let default = ContainerProps::default();

        // Попунктно сравниваем свойства на неравенство с дефолтом
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.height != default.height {
            prop_names.push(PROP_HEIGHT);
        }
        if current.max_width != default.max_width {
            prop_names.push(PROP_MAX_WIDTH);
        }
        if current.max_height != default.max_height {
            prop_names.push(PROP_MAX_HEIGHT);
        }
        if current.padding != default.padding {
            prop_names.push(PROP_PADDING);
        }
        if current.align_x != default.align_x {
            prop_names.push(PROP_ALIGN_X);
        }
        if current.align_y != default.align_y {
            prop_names.push(PROP_ALIGN_Y);
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
