// -----------------------------------------------------------------------------
// Виджет 'row'
// Ряд — Компонент, который позволяет размещать другие виджеты горизонтально.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::alignment::{Vertical};
use iced::widget::{row, mouse_area};
use iced::{Element, Length, Padding, Pixels, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: RowBlueprint::WIDGET_TYPE, //"row",
        category: CAT_CONTAIN,
        constructor: create_row_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_row_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Row");
    Box::new(RowCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct RowCreator;

impl WidgetCreator for RowCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(RowBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct RowProps {
    pub width:   Length,
    pub height:  Length,
    pub padding: Padding,
    pub spacing: Pixels,
    pub align_y: Vertical,
}

#[derive(Debug, Clone /*, serde::Serialize, serde::Deserialize*/)]
pub struct RowBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<RowProps>,
}

impl HasCommonMeta for RowBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl RowBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "row";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: RowProps::default().into(),
        }
    }

    pub fn parse_props(&self, factory: &Factory) -> RowProps {
        // Получить ID виджета
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = RowProps::default();

        // Адаптивные размеры
        let width:  Length = factory.get_or_set(&widget_id, PROP_WIDTH,  Length::Fill);
        let height: Length = factory.get_or_set(&widget_id, PROP_HEIGHT, def.height);

        // Шаг и Внутренние отступы
        let padding: Padding = factory.get_or_set(&widget_id, PROP_PADDING, def.padding);
        let spacing: Pixels  = factory.get_or_set(&widget_id, PROP_SPACING, def.spacing);

        // Вертикальное выравнивание по оси Y (Top, Center, Bottom)
        let align_y: Vertical = factory.get_or_set(&widget_id, PROP_ALIGN_Y, def.align_y);

        RowProps {
            // Адаптивные размеры
            width,
            height,

            // Шаг и Внутренние отступы
            padding,
            spacing,

            // Вертикальное выравнивание по оси Y (Top, Center, Bottom)
            align_y,
        }
    }
}

impl Default for RowProps {
    fn default() -> Self {
        RowProps {
            // ГАБАРИТЫ: Строка по умолчанию сжимается под размеры вложенных элементов
            width:   Length::Shrink,
            height:  Length::Shrink,

            // ОТСТУПЫ: Полностью отсутствуют (все нули)
            padding: Padding::ZERO, // или 0.0.into()
            spacing: Pixels(8.0),   // Элементы прижаты друг к другу без зазоров

            // ВЫРАВНИВАНИЕ: По умолчанию элементы внутри выравниваются по верхнему краю
            align_y: Vertical::Top,
        }
    }
}

impl WidgetBlueprint for RowBlueprint {
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
            PROP_PADDING,
            PROP_SPACING,
            PROP_ALIGN_Y,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получить ID виджета
        let widget_id = self.get_id();

        // Получаем чистые типизированные свойства колонки
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Собираем базовый макрос
        let mut w_row = row![]
            .width(props.width)
            .height(props.height)
            .padding(props.padding)
            .spacing(props.spacing)
            .align_y(props.align_y);

        // Заполняем вектор детьми
        let mut children_elements = Vec::new();
        for (child_id, child_blueprint) in factory.blueprints_iter() {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            if parent_id == widget_id {
                children_elements.push(child_blueprint.build_element(factory, selected_id));
            }
        }

        // Если он пуст — пушим заглушку, иначе — вливаем всех детей разом
        if children_elements.is_empty() {
            w_row = w_row.push(create_empty_placeholder(
                &widget_id,
                &self.widget_type(),
                props.width,
                props.height,
            ));
        } else {
            // Вливаем весь готовый вектор за один присест через нативный Extend
            w_row = w_row.extend(children_elements);
        }

        // Обертка в кнопку выбора для Design Mode
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // -------------------------------------------------------------
            // РЕЖИМ КОНСТРУКТОРА: Событие выделения виджета
            // -------------------------------------------------------------
            // Оборачиваем в mouse_area и навешиваем на оба элемента событие выделения при клике
            mouse_area(w_row)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            // -------------------------------------------------------------
            // РЕЖИМ РАБОТЫ: Интерактивное событие нажатия
            // -------------------------------------------------------------
            w_row.into()
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

        // Извлекаем текущие свойства строки из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = RowProps::default();

        // Сравниваем свойства строго по вашему списку editable_properties
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.height != default.height {
            prop_names.push(PROP_HEIGHT);
        }
        if current.padding != default.padding {
            prop_names.push(PROP_PADDING);
        }
        if current.spacing != default.spacing {
            prop_names.push(PROP_SPACING);
        }
        if current.align_y != default.align_y {
            prop_names.push(PROP_ALIGN_Y);
        }

        prop_names
    }

}
