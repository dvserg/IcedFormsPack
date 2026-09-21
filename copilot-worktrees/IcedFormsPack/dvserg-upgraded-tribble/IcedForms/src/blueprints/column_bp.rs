// -----------------------------------------------------------------------------
// Виджет 'column'
// Вертикальный стек — Размещает дочерние элементы друг под другом.
// Управляет вертикальным шагом (`spacing`) и выравниванием по горизонтали.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::alignment::Horizontal;
use iced::widget::{column, mouse_area};
use iced::{Element, Length, Padding, Pixels, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        ColumnBlueprint::WIDGET_TYPE, //"column",
        category:    CAT_CONTAIN,
        constructor: create_column_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_column_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Column");
    Box::new(ColumnCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct ColumnCreator;

impl WidgetCreator for ColumnCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(ColumnBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Структура для распарсенных свойств колонки
#[derive(Debug, Clone)]
pub struct ColumnProps {
    pub width:     Length,
    pub height:    Length,
    pub max_width: Pixels,
    pub padding:   Padding,
    pub spacing:   Pixels,
    pub align_x:   Horizontal, // В колонке это ось X (горизонтальное выравнивание)
}

#[derive(Debug, Clone)]
pub struct ColumnBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<ColumnProps>,
}

impl HasCommonMeta for ColumnBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl ColumnBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "column";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: ColumnProps::default().into(),
        }
    }

    // Парсинг свойств с использованием хелперов Factory
    pub fn parse_props(&self, factory: &Factory) -> ColumnProps {
        // Получить ID виджета
        let widget_id = self.get_id();
        
        let def = ColumnProps::default();

        // (*) Некоторые исходные размеры устанавливаем отличными от дефолтных для работы конструктора

        // Адаптивные размеры
        let width:     Length = factory.get_or_set(&widget_id, PROP_WIDTH,     Length::Fill);
        let height:    Length = factory.get_or_set(&widget_id, PROP_HEIGHT,    def.height);
        let max_width: Pixels = factory.get_or_set(&widget_id, PROP_MAX_WIDTH, def.max_width);

        // Шаг и Внутренний отступ (Числа парсятся автоматически через get_prop_parsed)
        let padding: Padding = factory.get_or_set(&widget_id, PROP_PADDING, def.padding);
        let spacing: Pixels  = factory.get_or_set(&widget_id, PROP_SPACING, def.spacing);

        // Горизонтальное выравнивание по оси X (Left, Center, Right)
        let align_x: Horizontal = factory.get_or_set(&widget_id, PROP_ALIGN_X, def.align_x);

        ColumnProps {
            // Адаптивные размеры
            width,
            height,
            max_width,

            // Шаг и Внутренний отступ
            padding,
            spacing,

            // Горизонтальное выравнивание по оси X (Left, Center, Right)
            align_x,
        }
    }
}

impl Default for ColumnProps {
    // Присваиваем дефолтные значения для контроля пропущенных значений и значений по умолчанию в инспекторе
    fn default() -> ColumnProps {
        ColumnProps {
            width:     Length::Shrink,
            height:    Length::Shrink,
            // По умолчанию (0.0) используется как "неограничено" (для инспектора)
            max_width: Pixels(0.0),     //Pixels(f32::INFINITY),
            padding:   Padding::ZERO,
            spacing:   Pixels(8.0),
            align_x:   Horizontal::Left,
        }
    }
}

//#[typetag::serde]
impl WidgetBlueprint for ColumnBlueprint {
    
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
            PROP_ALIGN_X,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые типизированные свойства колонки
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);
        let def   = ColumnProps::default();

        // Инициализируем макрос column и применяем свойства Iced 0.14
        let mut w_column = column![]
            .width(props.width)
            .height(props.height)
            .padding(props.padding)
            .spacing(props.spacing)
            .align_x(props.align_x);

        // Применяем только значение отличное от дефолтного
        // Значение Pixels(0.0) инспектора соответствует внутеннему Pixels(f32::INFINITY) Iced
        if props.max_width != def.max_width {
            w_column = w_column.max_width(props.max_width);
        }

        /*
          // Ищем и рендерим всех детей, у которых parent == self.get_id()
          let mut children_elements = Vec::new();
          for (child_id, child_blueprint) in &factory.blueprints {
              let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

              if parent_id == self.get_id() {
                  children_elements.push(child_blueprint.build_element(factory, selected_id));
              }
          }

          let has_children = !children_elements.is_empty();

          // Собираем детей, при их отсутствии используем заглушку
          if has_children {
              for (_idx, child) in children_elements.into_iter().enumerate() {
                  w_column = w_column.push(child);
              }
          } else {
              w_column = w_column.push(
                  create_empty_placeholder( &self.get_id(), &self.widget_type(), props.width, props.height )
              );
          }
        */

        // Заполняем вектор детьми
        let mut children_elements = Vec::new();
        for (child_id, child_blueprint) in factory.blueprints_iter() {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            if parent_id == self.get_id() {
                children_elements.push(child_blueprint.build_element(factory, selected_id));
            }
        }

        // Если он пуст — пушим заглушку, иначе — вливаем всех детей разом
        if children_elements.is_empty() {
            w_column = w_column.push(utils_bp::create_empty_placeholder(
                &self.get_id(),
                &self.widget_type(),
                props.width,
                props.height,
            ));
        } else {
            // Вливаем весь готовый вектор за один присест через нативный Extend
            w_column = w_column.extend(children_elements);
        }

        // Обертка в интерактивную кнопку для Design Mode
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // -------------------------------------------------------------
            // РЕЖИМ КОНСТРУКТОРА: Событие выделения виджета
            // -------------------------------------------------------------
            mouse_area(w_column)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()                

        } else {
            // -------------------------------------------------------------
            // РЕЖИМ РАБОТЫ: Интерактивное событие нажатия
            // -------------------------------------------------------------
            w_column.into()
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

        // 1. Извлекаем текущие свойства колонки из фабрики
        let current = self.parse_props(factory);
        
        // 2. Получаем дефолтные свойства для сравнения
        let default = ColumnProps::default();

        // 3. Сравниваем свойства и пушим только те, которые изменил пользователь
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.height != default.height {
            prop_names.push(PROP_HEIGHT);
        }
        
        // Умный фильтр для max_width:
        // Экспортируем значение в JSON только если пользователь явно ограничил ширину
        // (значение отличается от дефолтного Pixels(0.0))
        if current.max_width != default.max_width {
            prop_names.push(PROP_MAX_WIDTH);
        }
        
        if current.padding != default.padding {
            prop_names.push(PROP_PADDING);
        }
        if current.spacing != default.spacing {
            prop_names.push(PROP_SPACING);
        }
        if current.align_x != default.align_x {
            prop_names.push(PROP_ALIGN_X);
        }

        prop_names
    }

}
