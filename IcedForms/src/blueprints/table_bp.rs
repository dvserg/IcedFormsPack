// -----------------------------------------------------------------------------
// Виджет 'table'
// Сожержит в себе контент: виджет 'text'.
// Ведет себя как стандартная кнопка
// -----------------------------------------------------------------------------
use iced::widget::table::{Table};
use iced::widget::{table, text};
use iced::{Element, Length, Padding, Pixels, Color, Theme};

use crate::core::*;
use crate::core::Message;


// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        TableBlueprint::WIDGET_TYPE,
        category:    CAT_BASE,
        constructor: create_table_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_table_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Table");
    Box::new(TableCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct TableCreator;

impl WidgetCreator for TableCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(TableBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct TableProps {
    pub width:        Length,
    pub padding:      Padding,
    pub border_width: f32,
    pub border_color: Color,
}

#[derive(Debug, Clone)]
pub struct TableBlueprint {
    pub meta:  CommonWidgetMeta,

    // Локальные states виджета
    headers:     Vec<String>,           // Названия колонок    
    matrix_rows: Vec<Vec<String>>,      // Двумерный массив данных
}

impl HasCommonMeta for TableBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl TableBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "table";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),

            headers:     Vec::new(),
            matrix_rows: Vec::new(),
        }
    }

    // Парсинг свойств
    fn parse_props<'a>(&self, factory: &'a Factory) -> TableProps {
        let widget_id = self.get_id();
        let def = TableProps::default();

        // АДАПТИВНЫЕ РАЗМЕРЫ
        let width:   Length  = factory.get_or_set(&widget_id, PROP_WIDTH,   def.width);
        let padding: Padding = factory.get_or_set(&widget_id, PROP_PADDING, def.padding);

        // Бордюр, сепаратор
        let border_width:  f32    = factory.get_or_set(&widget_id, PROP_BORDER_WIDTH,  def.border_width);
        let border_color:  Color  = factory.get_or_set(&widget_id, PROP_BORDER_COLOR,  def.border_color);

        TableProps {
            width,
            padding,
            border_width,
            border_color,
        }
    }
}


impl Default for TableProps {
    // Присваиваем дефолтные значения для контроля пропущенных значений и значений по умолчанию в инспекторе
    fn default() -> TableProps {
        //let set = iced::Settings::default();

        TableProps {
            width:   Length::Fill,
            padding: Padding::from([0.0, 0.0]),
            border_width: 0.0_f32,
            border_color: Color::TRANSPARENT,
        }
    }
}


impl WidgetBlueprint for TableBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_WIDTH,
            PROP_PADDING,
            PROP_BORDER_WIDTH,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем все свойства виджета через вынесенную функцию парсинга
        let props = self.parse_props(factory);

        // Динамически создаем вектор колонок на основе вектора .headers
        let mut columns = Vec::new();

        let header = if self.headers.is_empty() {
            vec![String::from("column 1")]
        } else {
            self.headers.clone()
        };

        for (index, header_name) in header.iter().enumerate() {
            let col_index = index;
            // Создаем колонку, привязывая её к индексу массива данных
            let col = table::column(
                text(header_name.clone()),
                move |row_data: &Vec<String>| {
                    // Извлекаем текст по индексу колонки и размещаем в виджете 'text'
                    let cell_text = row_data.get(col_index).cloned().unwrap_or_default();
                    text(cell_text)
                }
            );
        
            columns.push(col);
        }

        // Передаем динамический набор колонок и двумерный вектор строк
        let element: Element<'a, Message, Theme> = {
            Table::new(columns, &self.matrix_rows)
                .width(props.width)
                .padding_x(props.padding.left)
                .padding_y(props.padding.top)
                .separator(Pixels::from(props.border_width))     
                .into()
        };

        // В самом конце применяем магию подсветки из трейта в одну строчку!
        apply_design_overlay(
            element,
            factory.is_design_mode(),
            selected_id,
            &self.get_id(),
        )
    }

    fn get_exportable_property_names(&self, _factory: &Factory) -> Vec<PropertyKey> {
        Vec::new()
    }

}