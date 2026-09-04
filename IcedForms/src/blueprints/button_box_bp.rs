// -----------------------------------------------------------------------------
// Виджет 'button_box'
// Может содержать в себе контент в виде любого одиночного виджета
// Ведет себя как стандартная кнопка-контейнер
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::Theme;
use iced::border::Radius;
use iced::widget::button;
use iced::{Border, Color, Element, Length, Padding};
use std::rc::Rc;

use crate::core::*;

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:  ButtonBoxBlueprint::WIDGET_TYPE, //"button_box",
        category: CAT_CONTAIN,
        constructor: create_button_box_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_button_box_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "ButtonBox");
    Box::new(ButtonBoxCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug)]
pub struct ButtonBoxCreator;

impl WidgetCreator for ButtonBoxCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(ButtonBoxBlueprint::new(id, "create_widget".to_string()))
    }
}
// -----------------------------------------------------------------------------

// Свойства виджета
#[derive(Debug, Clone)]
pub struct ButtonBoxProps {
    //pub content:     String,
    pub action: String,

    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub bg_color: Color,
    pub border_radius: Radius,
    pub border_color: Color,
    pub border_width: f32,
}

//#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[derive(Debug, Clone)]
pub struct ButtonBoxBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<ButtonBoxProps>,
}

impl HasCommonMeta for ButtonBoxBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl ButtonBoxBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "button_box";

    pub fn new(id: String, _action: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: ButtonBoxProps::default().into(),
        }
    }

    // ВЫНЕСЕННАЯ ФУНКЦИЯ ПАРСИНГА СВОЙСТВ
    fn parse_props<'a>(&self, factory: &'a Factory) -> ButtonBoxProps {
        // Идентификатор виджета
        let widget_id = self.get_id();

        // Получаем дефолтные свойства
        let def = ButtonBoxProps::default();

        // Событие
        let action: String = factory.get(&widget_id, PROP_ACTION).unwrap_or_default();

        // Адаптивные размеры (по умолчанию сжиматься по тексту — Shrink)
        let width:   Length  = factory.get_or_set(&widget_id, PROP_WIDTH,   def.width);
        let height:  Length  = factory.get_or_set(&widget_id, PROP_HEIGHT,  def.height);
        let padding: Padding = factory.get_or_set(&widget_id, PROP_PADDING, def.padding);

        // Стиль контейнера
        let bg_color:      Color  = factory.get_or_set(&widget_id, PROP_BG_COLOR,      def.bg_color);
        let border_radius: Radius = factory.get_or_set(&widget_id, PROP_BORDER_RADIUS, def.border_radius);
        let border_width:  f32    = factory.get_or_set(&widget_id, PROP_BORDER_WIDTH,  def.border_width);
        let border_color:  Color  = factory.get_or_set(&widget_id, PROP_BORDER_COLOR,  def.border_color);

        ButtonBoxProps {
            action,

            width,
            height,
            padding,

            bg_color,
            border_radius,
            border_color,
            border_width,
        }
    }
}

impl Default for ButtonBoxProps {
    // Присваиваем дефолтные значения Iced для контроля пропущенных значений и значений по умолчанию в инспекторе
    fn default() -> ButtonBoxProps {
        ButtonBoxProps {
            action:         "".to_string(),
            width:          Length::Shrink,
            height:         Length::Shrink,
            padding:        Padding::from([4.0, 8.0]),
            bg_color:       Color::TRANSPARENT,
            border_radius:  Radius::from(2.0_f32),
            border_color:   Color::TRANSPARENT,
            border_width:   0.0_f32,
        }
    }    
}

// -----------------------------------------------------------------------------
// Реализация Трейта WidgetBlueprint
// -----------------------------------------------------------------------------

impl WidgetBlueprint for ButtonBoxBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Функция возвращает статус "Принимает ли детей"
    fn can_accept_child(&self, factory: &Factory) -> bool {
        let widget_id = self.get_id();       
        let can_accept = factory.get_blueprints_by_parent(&widget_id).is_empty();
        can_accept // Может принять детей ( True ), если их список пустой
    }


    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_WIDTH,
            PROP_HEIGHT,
            PROP_PADDING,
            PROP_BG_COLOR,
            PROP_BORDER_RADIUS,
            PROP_BORDER_COLOR,
            PROP_BORDER_WIDTH,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые типизированные свойства
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Ищем и рендерим вложенный контент контейнера (ребенка), если он есть
        let mut child_element: Option<Element<'a, Message, Theme>> = None;
        for (child_id, child_blueprint) in factory.blueprints_iter() {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            if parent_id == self.get_id() {
                child_element = Some(child_blueprint.build_element(factory, selected_id));
                // У баттон-контейнера может быть только один прямой потомок
                break;
            }
        }

        // Собираем контент для кнопки-контейнера
        let content_view = child_element.unwrap_or_else(|| {
            // Если ребенка нет, создаем заглушку, чтобы контейнер не схлопнулся
            create_empty_placeholder(
                &self.get_id(),
                &self.widget_type(),
                props.width,
                props.height,
            )
        });

        // Виджет 'button'
        let mut w_button = button(content_view)
            .width(props.width)
            .height(props.height)
            .padding(props.padding)
            .style(move |_theme, _status| button::Style {
                background: Some(iced::Background::Color(props.bg_color)),
                border: Border {
                    color: props.border_color,
                    width: props.border_width,
                    radius: props.border_radius,
                },
                ..button::Style::default()
            });

        // Назначить события для 'button' в режимах конструктора и пользователя
        if factory.is_design_mode() {
            // -------------------------------------------------------------
            // РЕЖИМ КОНСТРУКТОРА: Событие выделения виджета
            // -------------------------------------------------------------

            w_button =
                w_button.on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())));
        } else {
            // -------------------------------------------------------------
            // РЕЖИМ РАБОТЫ: Интерактивное событие нажатия
            // -------------------------------------------------------------
            //let action_string = props.action.to_string();

            //w_button = w_button.on_press(Message::ValueChanged {
            //    id: self.get_id(),
            //    new_value: action_string,
            //})
        }

        /*
          let element: Element<'a, Message, Theme> = if factory.is_design_mode {
              // -------------------------------------------------------------
              // РЕЖИМ КОНСТРУКТОРА: Отрисовка пассивного макета кнопки
              // -------------------------------------------------------------
              let bg_val = props.bg_color;
              let radius_val = props.border_radius;

              let b_width = props.border_width;
              let b_color = props.border_color;

              // Рисуем сам макет кнопки. Он растягивается на Fill внутри кликера
              let button_mockup = container(
                  content_view
              )
              .width(Length::Fill)  // Теперь макет слушается размеров внешнего контейнера
              .height(Length::Fill) // Теперь макет слушается размеров внешнего контейнера
              .padding(props.padding)
              .align_x(Alignment::Center)
              .align_y(Alignment::Center)
              .style(move |_theme| container::Style {
                  background: Some(iced::Background::Color(bg_val)),
                  border: Border {
                      // Если в конструкторе толщина 0, покажем тонкую дефолтную рамку, чтобы кнопку было видно
                      color: if b_width > 0.0 { b_color } else { Color::from_rgb(0.8, 0.8, 0.8) },
                      width: if b_width > 0.0 { b_width } else { 1.0 },
                      radius: radius_val.into(),
                  },
                  ..Default::default()
              });

              // Кнопка-обертка для клика мышкой. Она пассивно занимает 100% места
              button(button_mockup)
                  .width(Length::Fill)
                  .height(Length::Fill)
                  .padding(0)
                  .style(|_, _| button::Style { background: None, border: Border::default(), ..Default::default() })
                  .on_press(Message::SelectWidget { widget_id: self.get_id().clone() })
                  .into()

          } else {
              // -------------------------------------------------------------
              // РЕЖИМ РАБОТЫ: Настоящая интерактивная кнопка Iced 0.14
              // -------------------------------------------------------------
              let id_clone = self.get_id().clone();
              let action_string = "".to_string(); // props.action_name.to_string();

              let bg_val = props.bg_color;
              let radius_val = props.border_radius;

              let b_width = props.border_width;
              let b_color = props.border_color;

              button(content_view)
                  .width(props.width)
                  .height(props.height)
                  .padding(props.padding)
                  .style(move |_theme, _status| button::Style {
                      background: Some(iced::Background::Color(bg_val)),
                      border: Border {
                          color:  b_color,
                          width:  b_width, // Настоящая рамка в готовом приложении!
                          radius: radius_val.into(),
                      },
                  //border: Border {
                  //    color: Color::TRANSPARENT,
                  //    width: 0.0,
                  //    radius: radius_val.into(),
                  //},
                      ..button::Style::default()
                  })
                  .on_press(Message::ValueChanged {
                      id: id_clone,
                      new_value: action_string,
                  })
                  .into()
          };
        */

        // Приводим виджет к тип 'Element'
        let element: Element<'a, Message, Theme> = w_button.into();

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
    fn get_exportable_property_names(&self, factory: &Factory) -> Vec<PropertyKey> {
        // По умолчанию возвращаем пустой вектор
        let mut prop_names = Vec::new();

        // Получаем текущие свойства виджета
        let current = self.parse_props(factory);
        
        // Получаем дефолтные свойства для сравнения
        let default = ButtonBoxProps::default();

        // Сравниваем каждое поле структуры
        // Отличные от дефолтного пушим в вектор для экспорта
        if current.action != default.action {
            prop_names.push(PROP_ACTION);
        }
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.height != default.height {
            prop_names.push(PROP_HEIGHT);
        }
        if current.padding != default.padding {
            prop_names.push(PROP_PADDING);
        }
        if current.bg_color != default.bg_color {
            prop_names.push(PROP_BG_COLOR);
        }
        if current.border_radius != default.border_radius {
            prop_names.push(PROP_BORDER_RADIUS);
        }
        if current.border_color != default.border_color {
            prop_names.push(PROP_BORDER_COLOR);
        }
        if current.border_width != default.border_width {
            prop_names.push(PROP_BORDER_WIDTH);
        }

        prop_names
    }
}
