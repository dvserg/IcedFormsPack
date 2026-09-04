// -----------------------------------------------------------------------------
// Виджет 'image'
// Отображение растровых картинок в интерфейсе приложения
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::border::Radius;
use iced::widget::image::FilterMethod;
use iced::widget::{image, mouse_area};
use iced::{ContentFit, Degrees, Element, Length, Rotation, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        ImageBlueprint::WIDGET_TYPE,
        category:    CAT_INPUTS,
        constructor: create_image_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_image_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Image");
    Box::new(ImageCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct ImageCreator;

impl WidgetCreator for ImageCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        // Создаем чертеж виджета
        Rc::new(ImageBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Структура для распарсенных свойств картинки
#[derive(Debug, Clone)]
pub struct ImageProps {
    pub path:          String,
    pub width:         Length,
    pub height:        Length,
    pub content_fit:   String,
    pub scale:         f32,
    pub rotation:      f32, //Rotation,
    pub opacity:       f32,
    pub border_radius: Radius,
    pub filter_method: bool,
}

#[derive(Debug, Clone /*, serde::Serialize, serde::Deserialize*/)]
pub struct ImageBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<ImageProps>,
}

impl HasCommonMeta for ImageBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl ImageBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "image";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: ImageProps::default().into(),
        }
    }

    fn parse_props<'a>(&self, factory: &'a Factory) -> ImageProps {
        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = ImageProps::default();

        // Путь к файлу изображения
        let path: String = factory.get_or_set(&widget_id, PROP_PATH, "".to_string());

        // Адаптивные размеры
        let width:  Length = factory.get_or_set(&widget_id, PROP_WIDTH,  def.width);
        let height: Length = factory.get_or_set(&widget_id, PROP_HEIGHT, def.height);

        // Порядок размещения контента в контейнере
        let content_fit: String = factory.get_or_set(&widget_id, PROP_CONTENT_FIT, def.content_fit);

        // Масштаб: [-5..+5; 0.2]; Уменьшение - [0..1; 0.2], увеличение - [1..5; 0.2],
        // Числа меньше нуля отзеркаливают изображение
        let scale: f32 = factory.get_or_set(&widget_id, PROP_SCALE, def.scale);

        // Прозрачность: [1..0; 0.1] 1 - непрозрачный, 0 - прозрачный
        let opacity_raw: f32 = factory.get_or_set(&widget_id, PROP_OPACITY, def.opacity);
        let opacity = opacity_raw.clamp(0.0, 1.0);

        // Поворот изображения в градусах
        let rotation: f32 = factory.get_or_set(&widget_id, PROP_ROTATION, def.rotation);
        //let rotation = (Rotation::Floating(Degrees(rotation_raw).into()));

        // Скругление углов изображения
        let border_radius: Radius = factory.get_or_set(&widget_id, PROP_BORDER_RADIUS, def.border_radius);

        // Фильтрация при масштабировании
        let filter_method: bool = factory.get_or_set(&widget_id, PROP_FILTER_METHOD, def.filter_method);

        ImageProps {
            path,
            width,
            height,

            content_fit,
            scale,
            rotation,
            opacity,
            border_radius,
            filter_method,
        }
    }
}

impl Default for ImageProps {
    // Присваиваем дефолтные значения для контроля пропущенных значений и значений по умолчанию в инспекторе
    fn default() -> ImageProps {
        ImageProps {
            path:           "".to_string(),
            width:          Length::Shrink,
            height:         Length::Shrink,
            content_fit:    String::from("contain"),
            scale:          1.0_f32,
            rotation:       0.0_f32, //Rotation::Floating(Degrees(0.0).into()),
            opacity:        1.0_f32,
            border_radius:  Radius::from(0.0),
            filter_method:  false,
        }
    }
}

impl WidgetBlueprint for ImageBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_PATH,
            PROP_WIDTH,
            PROP_HEIGHT,
            PROP_CONTENT_FIT,
            PROP_SCALE,
            PROP_ROTATION,
            PROP_OPACITY,
            PROP_BORDER_RADIUS,
            PROP_FILTER_METHOD,
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        // Получаем чистые типизированные свойства картинки
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);

        // Прозрачность
        let final_opacity = props.opacity.clamp(0.0, 1.0);

        // Политика масштабирования ContentFit::{Contain, Cover, Fill, None}
        //  Contain - масштабируется пропорционально по максимальному размеру области;
        //            если размеры не совпадают - остаются пустые места
        //  Cover   - масштабируется пропорционально по минимальному размеру области;
        //            если размеры не совпадают - излишки обрезаются
        //  Fill    - растягиваются под размеры области; пропорции не сораняются
        //  None    - картинка выводится в оригинальном размере без масштабирования
        let current_content_fit = match props.content_fit.to_lowercase().as_str() {
            "none"  => ContentFit::None,
            "fill"  => ContentFit::Fill,
            "cover" => ContentFit::Cover,
            "contain" | _ => ContentFit::Contain,
        };

        // Фильтрация при масштабировании изображения
        // FilterMethod::Linear  - Линейная фильтрация / Размытие
        // FilterMethod::Nearest - Пикселизация
        let curren_filter_method = if props.filter_method {
            FilterMethod::Nearest
        } else {
            FilterMethod::Linear
        };

        // Создаем базовый нативный виджет картинки Iced 0.14 [1.35]
        let mut base_image = image(&props.path)
            .width(props.width)
            .height(props.height)
            .content_fit(current_content_fit)
            .scale(props.scale)
            // Градусы переводим в радианы
            .rotation(Rotation::Floating(Degrees(props.rotation).into()))
            .opacity(final_opacity)
            .border_radius(props.border_radius)
            .filter_method(curren_filter_method);

        // Формируем внутренний интерактивный элемент для конструктора
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // Защита от схлопывания: если размеры shrink, в конструкторе даем 100x100 для видимости блока
            let visual_width = if let Length::Shrink = props.width {
                Length::Fixed(100.0)
            } else {
                props.width
            };
            let visual_height = if let Length::Shrink = props.height {
                Length::Fixed(100.0)
            } else {
                props.height
            };

            // Применяем при пустом 'path'
            if props.path == "".to_string() {
                base_image = base_image
                    .width(visual_width)
                    .height(visual_height);
            }

            mouse_area(base_image)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()            
 
        } else {
            base_image.into()
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

        // Получаем текущие свойства картинки из фабрики
        let current = self.parse_props(factory);
        
        // Получаем дефолтные свойства для сравнения
        let default = ImageProps::default();

        // Сравниваем каждое поле структуры
        // Отличные от дефолтного пушим в вектор для экспорта
        if current.path != default.path {
            prop_names.push(PROP_PATH);
        }
        if current.width != default.width {
            prop_names.push(PROP_WIDTH);
        }
        if current.height != default.height {
            prop_names.push(PROP_HEIGHT);
        }
        if current.content_fit != default.content_fit {
            prop_names.push(PROP_CONTENT_FIT);
        }
        if current.scale != default.scale {
            prop_names.push(PROP_SCALE);
        }
        if current.rotation != default.rotation {
            prop_names.push(PROP_ROTATION);
        }
        if current.opacity != default.opacity {
            prop_names.push(PROP_OPACITY);
        }
        if current.border_radius != default.border_radius {
            prop_names.push(PROP_BORDER_RADIUS);
        }
        if current.filter_method != default.filter_method {
            prop_names.push(PROP_FILTER_METHOD);
        }

        prop_names
    }    
}
