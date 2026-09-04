// -----------------------------------------------------------------------------
// Виджет 'svg'
// SVG — Компонент для отображения векторной графики.
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::widget::{svg, stack};
use iced::{Color, ContentFit, Element, Length, Rotation, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name:        SvgBlueprint::WIDGET_TYPE,
        category:    CAT_BASE,
        constructor: create_svg_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_svg_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "Svg");
    Box::new(SvgCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct SvgCreator;

impl WidgetCreator for SvgCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        // Создаем чертеж SVG-виджета
        Rc::new(SvgBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct SvgProps {
    pub path:        String,
    pub width:       Length,
    pub height:      Length,
    pub color:       Color,         // Option позволяет оставлять родные цвета SVG
    pub content_fit: String,        // Тип из библиотеки iced
    pub opacity:     f32,           // Прозрачность [0, 1, 0.1]
    pub rotation:    f32,           // В Iced поворот задается f32 [0.0 .. 360.0 ]
}

#[derive(Debug, Clone)]
pub struct SvgBlueprint {
    pub meta:  CommonWidgetMeta,
    //pub props: RefCell<SvgProps>,
}

impl HasCommonMeta for SvgBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl SvgBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "svg";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
            //props: SvgProps::default().into(),
        }
    }

    // ВЫНЕСЕННАЯ ФУНКЦИЯ ПАРСИНГА СВОЙСТВ
    fn parse_props<'a>(&self, factory: &'a Factory) -> SvgProps {
        //use iced::{Degrees, Rotation};

        let widget_id = self.get_id();

        // Получить дефолтные значения
        let def = SvgProps::default();

        let path: String = factory.get_or_set(&widget_id, PROP_PATH, def.path);

        // Геометрия
        let width:  Length = factory.get_or_set(&widget_id, PROP_WIDTH,  def.width);
        let height: Length = factory.get_or_set(&widget_id, PROP_HEIGHT, def.height);

        // Цвет заливки векторной графики (по умолчанию черный, если не задан)
        let color: Color = factory.get_or_set(&widget_id, PROP_COLOR, def.color);

        // Порядок размещения контента в контейнере
        let content_fit: String = factory.get_or_set(&widget_id, PROP_CONTENT_FIT, def.content_fit);
        // Прозрачность
        let opacity_raw: f32 = factory.get_or_set(&widget_id, PROP_OPACITY, def.opacity);
        // Поворот изображения в градусах
        let rotation: f32 = factory.get_or_set(&widget_id, PROP_ROTATION, def.rotation);

        // Коррекция значения в рамках диапазона
        let opacity = opacity_raw.clamp(0.0, 1.0);

        // Конвертация в радианы
        //let rotation = Rotation::Floating(Degrees(rotation_raw).into());

        SvgProps {
            path,
            width,
            height,
            color,
            content_fit,
            opacity,
            rotation,
        }
    }
}

// Значения по умолчанию
impl Default for SvgProps {
    fn default() -> Self {
        Self {
            path:        String::new(),
            width:       Length::Shrink,
            height:      Length::Shrink,
            color:       Color::TRANSPARENT,
            content_fit: String::from("contain"),
            opacity:     1.0_f32,                       // По умолчанию SVG полностью непрозрачный
            rotation:    0.0_f32,                       // Обычно это 0 градусов / без поворота
        }
    }
}

impl WidgetBlueprint for SvgBlueprint {
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
            PROP_COLOR,
            PROP_OPACITY,
            PROP_ROTATION,
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
            "none" => ContentFit::None,
            "fill" => ContentFit::Fill,
            "cover" => ContentFit::Cover,
            "contain" | _ => ContentFit::Contain,
        };

        // Создаем базовый виджет SVG Iced 0.14 [1.35]
        let mut base_svg = svg(&props.path)
            .width(props.width)
            .height(props.height)
            .content_fit(current_content_fit)   // Применяем ContentFit
            .opacity(final_opacity)             // Применяем прозрачность
            .rotation(Rotation::Floating(iced::Degrees(props.rotation).into())); // Применяем поворот

        // МАГИЯ ВЕКТОРОВ: Если цвет задан как transparent — не красим (оставляем оригинал),
        // во всех остальных случаях динамически перекрашиваем контур иконки!

        let props_cl = props.clone();
        if props.color != Color::TRANSPARENT {
            base_svg = base_svg.style(move |_theme, _status| {
                iced::widget::svg::Style {
                    // Принудительно задаем цвет для заливки векторных контуров
                    color: Some(props_cl.color),
                }
            });
        }

        // Формируем элемент в зависимости от режима конструктора
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            
            let visual_width = if let Length::Shrink = props.width {
                Length::Fixed(48.0)
            } else {
                props.width
            };
            let visual_height = if let Length::Shrink = props.height {
                Length::Fixed(48.0)
            } else {
                props.height
            };
            
            // Применяем при пустом 'path'
            if props.path == "".to_string() {
                base_svg = base_svg
                    .width(visual_width)
                    .height(visual_height);
            }

            let mut inner = stack![base_svg];

            // Вставляем плейсхолдер
            if props.path == "".to_string() {
                inner = inner.push(
                    create_empty_placeholder(&self.get_id(), &self.widget_type(), Length::Fill, Length::Fill)
                );
            }

            iced::widget::mouse_area(inner)
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            base_svg.into()
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

        // Извлекаем текущие свойства SVG из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = SvgProps::default();

        // Сравниваем текущие значения со значениями по умолчанию
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
        if current.color != default.color {
            prop_names.push(PROP_COLOR);
        }
        // Для f32 (opacity) прямое сравнение != может быть неточным из-за специфики float,
        // но для точных дефолтных шагов UI [0.0, 1.0, 0.1] оно обычно допустимо.
        if current.opacity != default.opacity {
            prop_names.push(PROP_OPACITY);
        }
        if current.rotation != default.rotation {
            prop_names.push(PROP_ROTATION);
        }

        prop_names
    }    
}
