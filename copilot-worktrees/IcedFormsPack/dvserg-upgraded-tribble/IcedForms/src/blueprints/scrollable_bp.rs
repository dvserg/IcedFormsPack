// -----------------------------------------------------------------------------
// Виджет 'scrollable'
// Обёртка, добавляющая вертикальную и/или горизонтальную
// прокрутку (скроллбары) для контента, который не помещается на экране.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::widget::scrollable;
use iced::{Color, Element, Length, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: ScrollableBlueprint::WIDGET_TYPE, //"scrollable",
        category: CAT_CONTAIN,
        constructor: create_scrollable_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_scrollable_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Scrollable");
    Box::new(ScrollableCreator)
}

// Конструктор blueprint для виджета

#[derive(Debug, Clone)]
pub struct ScrollableCreator;

impl WidgetCreator for ScrollableCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(ScrollableBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct ScrollableProps {
    pub width:            Length,
    pub height:           Length,
    pub direction:        String,
    pub scroller_width:   f32,
    pub scrollbar_width:  f32,
    pub scrollbar_margin: f32,
    pub track_color:      Color,
    pub thumb_color:      Color,
}

#[derive(Debug, Clone /*, serde::Serialize, serde::Deserialize*/)]
pub struct ScrollableBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<ScrollableProps>,
}

impl HasCommonMeta for ScrollableBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}


impl ScrollableBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "scrollable";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: ScrollableProps::default().into(),
        }
    }

    pub fn parse_props(&self, factory: &Factory) -> ScrollableProps {
        // Получить ID виджета
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = ScrollableProps::default();

        // Адаптивные размеры
        let width:  Length = factory.get_or_set(&widget_id, PROP_WIDTH,  def.width);
        let height: Length = factory.get_or_set(&widget_id, PROP_HEIGHT, def.height);

        let direction: String = factory.get_or_set(&widget_id, PROP_DIRECTION, def.direction);

        let scroller_width:   f32 = factory.get_or_set(&widget_id, PROP_SCROLLER_WIDTH,   def.scroller_width);       // Ширина ползунка
        let scrollbar_margin: f32 = factory.get_or_set(&widget_id, PROP_SCROLLBAR_MARGIN, def.scrollbar_margin);     // Отступ от контента
        let scrollbar_width:  f32 = factory.get_or_set(&widget_id, PROP_SCROLLBAR_WIDTH,  def.scrollbar_width);      // Ширина трека

        let track_color: Color = factory.get_or_set(&widget_id, PROP_TRACK_COLOR, def.track_color);
        let thumb_color: Color = factory.get_or_set(&widget_id, PROP_THUMB_COLOR, def.thumb_color);

        ScrollableProps {
            // Адаптивные размеры
            width,
            height,

            direction,

            scroller_width,
            scrollbar_width,
            scrollbar_margin,
            track_color,
            thumb_color,
        }
    }
}

impl Default for ScrollableProps {
    fn default() -> Self {
        ScrollableProps {
            // ГАБАРИТЫ: Scrollable по умолчанию пытается занять всё доступное место
            width:            Length::Fill,
            height:           Length::Fill,

            // НАПРАВЛЕНИЕ: Стандартная вертикальная прокрутка
            // В Iced 0.14.2 это scrollable::Direction::Vertical
            direction:        String::from("vertical"), 

            // ГЕОМЕТРИЯ СКРОЛЛ-БАРА:
            // В Iced 0.14.2 стандартная ширина бегунка/полосы равна 10px, а маргин — 0px.
            scroller_width:   10.0,
            scrollbar_width:  10.0,
            scrollbar_margin: 0.0,

            // СТИЛЬ И ЦВЕТА: Цвета трека (дорожки) и самба (бегунка) берутся из темы.
            // Используем маркер TRANSPARENT для перегрузки в инспекторе.
            track_color:      Color::TRANSPARENT,
            thumb_color:      Color::TRANSPARENT,
        }
    }
}


impl WidgetBlueprint for ScrollableBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Функция возвращает статус "Принимает ли детей"
    fn can_accept_child(&self, factory: &Factory) -> bool {
        let widget_id = self.get_id();
        let can_accept = factory.get_blueprints_by_parent(&widget_id).is_empty();
        can_accept // У scollable контейнера может быть только один прямой потомок
    }
    /*
    fn can_accept_child(&self, factory: &Factory) -> bool {
        // Ищем в базе фабрики, занято ли уже место
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
            PROP_DIRECTION,
            PROP_SCROLLBAR_WIDTH,
            PROP_SCROLLBAR_MARGIN,
            PROP_SCROLLER_WIDTH,
            PROP_TRACK_COLOR,
            PROP_THUMB_COLOR,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>, // Получаем выбранный ID для инспектора
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые типизированные свойства колонки
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Ищем и рендерим единственного ребенка, который вложен в этот container
        let mut child_element: Option<Element<'a, Message, Theme>> = None;

        for (child_id, child_blueprint) in factory.blueprints_iter() {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            if parent_id == self.get_id() {
                // ВАЖНО: Пробрасываем selected_id рекурсивно вниз к дочернему виджету!
                child_element = Some(child_blueprint.build_element(factory, selected_id));
                break;
            }
        }

        // Если дочернего элемента нет, создаем пустое пространство-заглушку
        let inner_content = child_element.unwrap_or_else(|| {
            create_empty_placeholder(
                &self.get_id(),
                &self.widget_type(),
                Length::Shrink,
                Length::Shrink,
            )
        });

        let native_scrollbar = scrollable::Scrollbar::default()
            .width(props.scrollbar_width)
            .margin(props.scrollbar_margin)
            .scroller_width(props.scroller_width);

        let native_direction = match props.direction.to_lowercase().as_str() {
            "horizontal" => scrollable::Direction::Horizontal(native_scrollbar),
            "both" => scrollable::Direction::Both {
                vertical: native_scrollbar,
                horizontal: native_scrollbar,
            },
            _ => scrollable::Direction::Vertical(native_scrollbar), // По умолчанию только вверх-вниз
        };

        // Оборачиваем внутреннее содержимое в прокручиваемый контейнер
        let scrollable_content = iced::widget::scrollable(inner_content)
            .width(props.width)
            .height(props.height)
            .direction(native_direction)
            .style(move |_theme, _status| {
                // Получаем готовый, валидный нативный стиль Iced из текущей темы.
                // В нём ВСЕ поля (включая auto_scroll и gap) уже заполнены фреймворком!
                let palette = _theme.extended_palette();

                let computed_radius = iced::border::Radius::from(f32::from(props.scrollbar_width / 2.0));

                // Общая конфигурация одной дорожки (Rail) и её ползунка (Scroller)
                let rail_style = scrollable::Rail {                    
                    // Цвет подложки-трека
                    background: 
                        if props.track_color != Color::TRANSPARENT {
                            Some(iced::Background::Color(props.track_color)) 
                        } else {
                            Some(palette.background.weak.color.into())
                        },
                    border: iced::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        radius: computed_radius, // Скругление дорожки
                    },
                    // Настройка самого бегунка (ползунка)
                    scroller: scrollable::Scroller {
                        background:
                            if props.thumb_color != Color::TRANSPARENT {
                                iced::Background::Color(props.thumb_color) 
                            } else {
                                palette.background.strongest.color.into()
                            },
                        border: iced::Border {
                            color: iced::Color::TRANSPARENT,
                            width: 0.0,
                            radius: computed_radius, // Скругление ползунка
                        },
                    },
                };

                scrollable::Style {
                    // Оставляем фон прокручиваемой области прозрачным
                    container: iced::widget::container::Style::default(),

                    // Применяем наши рельсы и бегунки к обеим осям
                    vertical_rail:   rail_style,
                    horizontal_rail: rail_style,

                    // Стык между вертикальным и горизонтальным скроллбаром (прозрачный по умолчанию)
                    gap: None,

                    // Настройка автоскролла: в Iced 0.14 это enum.
                    // Вариант AutoScroll::Disabled полностью отключает кастомный автоскролл,
                    // что возвращает стандартное системное поведение прокрутки Iced.
                    auto_scroll: scrollable::AutoScroll {
                        background: iced::Background::Color(iced::Color::TRANSPARENT),
                        border: iced::Border {
                            color: iced::Color::TRANSPARENT,
                            width: 0.0,
                            radius: 0.0.into(),
                        },
                        shadow: iced::Shadow::default(),
                        icon: iced::Color::TRANSPARENT,
                    },
                }
            });

        // ДЕЛАЕМ СКРОЛЛ КЛИКАБЕЛЬНЫМ В РЕЖИМЕ РЕДАКТОРА
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // Оборачиваем рабочий скролл в нативную mouse_area,
            // чтобы клик по свободному пространству скролла выделял его в инспекторе!
            iced::widget::mouse_area(scrollable_content)
                //.on_press(Message::SelectWidget { widget_id: id.clone() })
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            // В обычном пользовательском режиме отдаем чистый скролл без лишних оберток
            scrollable_content.into()
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

        // Извлекаем текущие свойства скролла из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = ScrollableProps::default();

        // Сравниваем свойства строго по вашему списку editable_properties
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.height != default.height {
            prop_names.push(PROP_HEIGHT);
        }
        if current.direction != default.direction {
            prop_names.push(PROP_DIRECTION);
        }
        if current.scroller_width != default.scroller_width {
            prop_names.push(PROP_SCROLLER_WIDTH);
        }
        if current.scrollbar_width != default.scrollbar_width {
            prop_names.push(PROP_SCROLLBAR_WIDTH);
        }
        if current.scrollbar_margin != default.scrollbar_margin {
            prop_names.push(PROP_SCROLLBAR_MARGIN);
        }
        if current.track_color != default.track_color {
            prop_names.push(PROP_TRACK_COLOR);
        }
        if current.thumb_color != default.thumb_color {
            prop_names.push(PROP_THUMB_COLOR);
        }

        prop_names
    }

}
