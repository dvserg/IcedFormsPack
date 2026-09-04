// -----------------------------------------------------------------------------
// Виджет 'grid'
// Контейнер, распределяющий дочерние элементы в виде адаптивной сетки
// -----------------------------------------------------------------------------
use iced::widget::{Grid, mouse_area};
use iced::{Element, Length, Pixels, Theme};

use crate::core::*;


// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        GridBlueprint::WIDGET_TYPE,
        category:    CAT_BASE,
        constructor: create_grid_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_grid_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Grid");
    Box::new(GridCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct GridCreator;

impl WidgetCreator for GridCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(GridBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct GridProps {    
    pub columns:      usize,    // Количество колонок
    pub fluid:        Pixels,   // Максимальная ширина одной ячейки.
                                // Отменяет количество колонок, делает сетку 'резиновой': 
                                // адаптивно подгоняет количество колонок под ширину.
    pub width:        Length,   // Ширина Grid. Если не задан, заполняет Fill родителя    
    pub height:       Length,   // Высота Grid, если передан в height как Sizing::EvenlyDistribute    
    pub aspect_ratio: f32,      // Задает соотношение ширины и высоты ячейки, если передан в height как Sizing::AspectRatio
                                // Поэтому в обычном случае чтобы сработал размер Length, задаем 'aspect_ratio = 0'
    pub spacing:      Pixels,   // Расстояние между ячейками
}

#[derive(Debug, Clone)]
pub struct GridBlueprint {
    pub meta:  CommonWidgetMeta,
}

impl HasCommonMeta for GridBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}


impl GridBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "grid";

    pub fn new(id: String) -> Self {
        Self {
            meta: CommonWidgetMeta::new(id),
        }
    }

    pub fn parse_props(&self, factory: &Factory) -> GridProps {
        let widget_id = self.get_id();
        let def = GridProps::default();

        let columns: usize  = factory.get_or_set(&widget_id, PROP_COLUMNS, def.columns);
        let fluid:   Pixels = factory.get_or_set(&widget_id, PROP_FLUID,   def.fluid);

        let aspect_ratio: f32 = factory.get_or_set(&widget_id, PROP_ASPECT_RATIO, def.aspect_ratio);

        // Адаптивные размеры
        let width:   Length = factory.get_or_set(&widget_id, PROP_WIDTH,  Length::Fill);
        let height:  Length = factory.get_or_set(&widget_id, PROP_HEIGHT, def.height);

        // Шаг и Внутренний отступ
        let spacing: Pixels = factory.get_or_set(&widget_id, PROP_SPACING, def.spacing);

        GridProps {
            columns,
            fluid,
            width,
            height,
            spacing,
            aspect_ratio,
        }
    }
}

impl Default for GridProps {
    // Присваиваем дефолтные значения для контроля пропущенных значений и значений по умолчанию в инспекторе
    fn default() -> GridProps {
        GridProps {
            columns: 3_usize,
            fluid:   Pixels(0.0),
            width:   Length::Fill,
            height:  Length::Fill,
            spacing: Pixels(10.0),
            aspect_ratio: 0.0_f32,
        }
    }
}


impl WidgetBlueprint for GridBlueprint {

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
            PROP_COLUMNS,
            PROP_FLUID,
            PROP_ASPECT_RATIO,
            PROP_WIDTH,
            PROP_HEIGHT,
            PROP_SPACING,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        use iced::widget::grid::Sizing;

        // Получаем чистые свойства через вынесенную функцию
        let props = self.parse_props(factory);


        // Создаем мутабельный grid
        let mut w_grid = Grid::new()
            .columns(props.columns)
            .spacing(props.spacing);

        // Приводим значение width Length к Pixels: все, что не Fixed > 0.0 будет равно 0.0 (соответствует Fill)
        let w_pixels: Pixels = Pixels(if let iced::Length::Fixed(w) = props.width { w } else { 0.0 });

        // Установить ширину
        if w_pixels.0 > 0.0 {
            w_grid = w_grid.width(w_pixels);
        }

        // Установить высоту
        if props.aspect_ratio > 0.0 {
            w_grid = w_grid.height(Sizing::AspectRatio(props.aspect_ratio));
        } 
        else if let iced::Length::Fixed(h) = props.height {
            if h > 0.0 {
                w_grid = w_grid.height(Sizing::from(props.height));
            }
        };

        // Устанавливаем мксимальную ширину ячейки для адаптивного числа колонок
        if props.fluid.0 > 0.0 {
            w_grid = w_grid.fluid(props.fluid);
        }

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
            w_grid = w_grid.push(utils_bp::create_empty_placeholder(
                &self.get_id(),
                &self.widget_type(),
                props.width,
                props.height,
            ));
        } else {
            // Вливаем весь готовый вектор за один раз через нативный Extend
            w_grid = w_grid.extend(children_elements);
        }

        // Обертка в интерактивную кнопку для Design Mode
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // -------------------------------------------------------------
            // РЕЖИМ КОНСТРУКТОРА: Событие выделения виджета
            // -------------------------------------------------------------
            mouse_area(w_grid)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()                

        } else {
            // -------------------------------------------------------------
            // РЕЖИМ РАБОТЫ
            // -------------------------------------------------------------
            w_grid.into()
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
    fn get_exportable_property_names(&self, _factory: &Factory) -> Vec<PropertyKey> {
        Vec::new()
    }
}