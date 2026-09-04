// -----------------------------------------------------------------------------
// Виджет 'qr_code'
// QR-код — Компонент, который берет строку данных и процедурно генерирует
// и рендерит растровый QR-код прямо на экране приложения.
// -----------------------------------------------------------------------------
use std::cell::{UnsafeCell};
use iced::widget::{qr_code, mouse_area};
use iced::{Color, Element, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        QRCodeBlueprint::WIDGET_TYPE, //"qr_code",
        category:    CAT_BASE,
        constructor: create_qr_code_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_qr_code_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "QRCode");
    Box::new(QRCodeCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct QRCodeCreator;

impl WidgetCreator for QRCodeCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(QRCodeBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct QRCodeProps {
    pub data:       String,
    pub cell_size:  f32,
    pub cell_color: Color,
    pub bg_color:   Color,
}

// Возвращаем структуру к потокобезопасному виду (String реализует Sync)
#[derive(Debug)]
pub struct QRCodeBlueprint {
    pub meta:     CommonWidgetMeta,

    // Содержит текущие данные строку QRCode
    pub data:     UnsafeCell<qr_code::Data>,
    pub data_str: UnsafeCell<String>,
    //pub props: RefCell<QRCodeProps>,
}

impl HasCommonMeta for QRCodeBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl QRCodeBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "qr_code";

    pub fn new(id: String) -> Self {
        let def:  QRCodeProps = QRCodeProps::default().into();
        let (data_str, data)  = Self::prepare_data(def.data);

        Self {
            meta: CommonWidgetMeta::new(id),
            data:     UnsafeCell::new(data),
            data_str: UnsafeCell::new(data_str),
            //props: QRCodeProps::default().into(),
        }
    }

    // Формирует из заданной строки пару - проверенную строку и соответствующйю qr_code::Data
    fn prepare_data (data: String) -> (String, qr_code::Data) {
        let mut data_str = data;
        let qr_data = qr_code::Data::new(&data_str).unwrap_or_else(|_| {
            data_str = "".to_string();
            qr_code::Data::new(&data_str).unwrap()
        });

        (data_str, qr_data)
    }

    // Парсинг свойств
    fn parse_props(&self, factory: &Factory) -> QRCodeProps {
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = QRCodeProps::default();

        // Данные и размер
        let data:       String = factory.get_or_set(&widget_id, PROP_DATA,       String::from("https://rust-lang.org"));
        let cell_size:  f32    = factory.get_or_set(&widget_id, PROP_CELL_SIZE,  def.cell_size);
        let bg_color:   Color  = factory.get_or_set(&widget_id, PROP_BG_COLOR,   def.bg_color);
        let cell_color: Color  = factory.get_or_set(&widget_id, PROP_CELL_COLOR, def.cell_color);

        QRCodeProps {
            data,
            cell_size,
            cell_color,
            bg_color,
        }
    }
}

impl Default for QRCodeProps {
    fn default() -> Self {
        QRCodeProps {
            // ДАННЫЕ: Изначально строка пустая
            data:       String::new(),

            // ГАБАРИТЫ: Размер одной точки (пикселя) QR-кода в Iced по умолчанию равен 3.0
            cell_size:  3.0_f32,

            // СТИЛЬ И ЦВЕТА: По умолчанию QR-код рисуется черным на прозрачном фоне.
            // Цвет TRANSPARENT соответствует цвету фона по умолчанию.
            cell_color: Color::BLACK,
            bg_color:   Color::TRANSPARENT,
        }
    }
}

impl WidgetBlueprint for QRCodeBlueprint {

    // Функция возвращает тип виджета
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_DATA, 
            PROP_CELL_SIZE, 
            PROP_BG_COLOR, 
            PROP_CELL_COLOR
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые свойства через вынесенную функцию
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Вскрываем ячейку с сохраненной qr_code строкой на ЧТЕНИЕ
        let current_str_ref = unsafe { &*self.data_str.get() };

        // Выполнение апдейта если данные изменились
        if current_str_ref != &props.data {
            let (new_data_str, new_data)  = Self::prepare_data(props.data);
            unsafe {
                 let data_mut_ref: &mut qr_code::Data = &mut *self.data.get();
                *data_mut_ref = new_data;
           
                let string_mut_ref: &mut String = &mut *self.data_str.get();
                *string_mut_ref = new_data_str;
            }
            log::info!("build_element: Данные '{}' изменились! Сгенерирован новый QR-код .", self.get_id());
        }        

        // Получаем ссылку на state QRCode
        let data_ref: &qr_code::Data = unsafe { &* self.data.get() };

        // Создаем элемент qr_code
        let base_qrcode = qr_code(data_ref)
            .cell_size(props.cell_size)
            .style(move |_theme| qr_code::Style {
                background: props.bg_color,
                cell: props.cell_color,
            });

        // Формируем элемент в зависимости от режима конструктора
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // -------------------------------------------------------------
            // РЕЖИМ КОНСТРУКТОРА: Пассивный режим text_input
            // -------------------------------------------------------------
            // Оборачиваем в mouse_area и навешиваем на оба элемента событие выделения при клике
            mouse_area(base_qrcode)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .on_release(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            base_qrcode.into()
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

        // Извлекаем текущие свойства QR-кода из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = QRCodeProps::default();

        // Сравниваем свойства строго по вашему списку editable_properties
        if current.data != default.data {
            prop_names.push(PROP_DATA);
        }
        if current.cell_size != default.cell_size {
            prop_names.push(PROP_CELL_SIZE);
        }
        if current.bg_color != default.bg_color {
            prop_names.push(PROP_BG_COLOR);
        }
        if current.cell_color != default.cell_color {
            prop_names.push(PROP_CELL_COLOR);
        }

        prop_names
    }

}
