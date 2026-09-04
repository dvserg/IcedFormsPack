// -----------------------------------------------------------------------------
// Виджет 'mouse_area'
// Область мыши — Обертка над любым виджетом, позволяющая перехватывать
// детальные события мыши (клик, наведение, уход курсора, правый клик).
// -----------------------------------------------------------------------------
//use std::cell::{RefCell};
use iced::mouse::Interaction;
use iced::widget::mouse_area;
use iced::{Element, Length, Theme};

use crate::core::*;
use crate::core::{MenuAction, Message};

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
// Передаем функцию без круглых скобок (как указатель)
inventory::submit! {
    AutoRegisteredWidget {
        name: MouseAreaBlueprint::WIDGET_TYPE, //"mouse_area",
        category: CAT_CONTAIN,
        constructor: create_mouse_area_creator,
    }
}

// Функция-помощник для создания Arc (вызовется автоматически внутри Factory::default)
fn create_mouse_area_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация '{}'", "MouseArea");
    Box::new(MouseAreaCreator)
}

// Конструктор blueprint для виджета
#[derive(Debug, Clone)]
pub struct MouseAreaCreator;

impl WidgetCreator for MouseAreaCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(MouseAreaBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

// Cтруктура для хранения распарсенных свойств
#[derive(Debug, Clone)]
pub struct MouseareaProps {
    pub action:      String,
    pub cursor_type: String,
}

#[derive(Debug, Clone)]
pub struct MouseAreaBlueprint {
    pub meta:  CommonWidgetMeta,
}

impl HasCommonMeta for MouseAreaBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta {
        &self.meta
    }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta {
        &mut self.meta
    }
}

impl MouseAreaBlueprint {
    // Присваиваем константное имя этого блюпринта
    const WIDGET_TYPE: &'static str = "mouse_area";

    pub fn new(id: String) -> Self {
        Self {
            meta:  CommonWidgetMeta::new(id),
        }
    }

    // Парсинг свойств с использованием хелперов Factory
    pub fn parse_props(&self, factory: &Factory) -> MouseareaProps {
        // Получить ID виджета
        let widget_id = self.get_id();

        // Получаем дефолтные свойства
        let def = MouseareaProps::default();

        // Текстовая метка событий
        // Должна идентифицировать во всех обрабатываемых событиях этот элемент или определенный вид собьытий
        let action: String = factory.get_or_set(&widget_id, PROP_ACTION, def.action);

        // Тип курсора
        let cursor_type: String = factory.get_or_set(&widget_id, PROP_CURSOR_TYPE, def.cursor_type);

        MouseareaProps {
            action,
            cursor_type,
        }
    }
}

impl Default for MouseareaProps {
    fn default() -> MouseareaProps {
        MouseareaProps {
            action:      "".to_string(),
            cursor_type: "idle".to_string(),
        }
    }
}

//#[typetag::serde]
impl WidgetBlueprint for MouseAreaBlueprint {
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    // Функция возвращает статус "Принимает ли детей"
    fn can_accept_child(&self, factory: &Factory) -> bool {
        let widget_id = self.get_id();
        let can_accept = factory.get_blueprints_by_parent(&widget_id).is_empty();
        can_accept // Может принять детей (одного), если их список пустой (True)
    }

    // Декларация свойств для инспектора
    // Порядок следования свойств соответствует списку
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_ACTION, 
            PROP_CURSOR_TYPE
        ]
    }

    fn build_element<'a>(
        &'a self,
        factory: &'a Factory,
        selected_id: Option<&str>,
    ) -> Element<'a, Message, Theme> {
        let widget_id = self.get_id();
        //let props = self.props.borrow(); //self.parse_props(factory);
        let props = self.parse_props(factory);
        //let is_selected = selected_id == Some(widget_id.as_str());

        // Собираем внутреннее содержимое (дочерний элемент)
        let mut child_element: Option<Element<'a, Message, Theme>> = None;
        for (child_id, child_blueprint) in factory.blueprints_iter() {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            if parent_id == widget_id {
                child_element = Some(child_blueprint.build_element(factory, selected_id));
                break;
            }
        }

        // Вычисляем типы Length (размеры), которые мы передадим в рамку оверлея.
        // Поскольку mouse_area — это функциональный «прокси», её рамка выделения
        // в режиме дизайна должна полностью повторять размеры своего внутреннего контента.
        let (_overlay_width, _overlay_height) = if child_element.as_ref().is_some() {
            // Если ребенок есть — считываем из фабрики ЕГО No-Code стратегию размеров
            // (для этого находим ID ребенка повторным сканированием или сохраняем на шаге 1)

            let w: Length = factory.get(&widget_id, PROP_WIDTH).unwrap_or(Length::Shrink);
            let h: Length = factory.get(&widget_id, PROP_HEIGHT).unwrap_or(Length::Shrink);

            (w, h)
        } else {
            // Если внутри пусто и там заглушка — оверлей сожмется строго по размерам заглушки (Shrink)
            (Length::Shrink, Length::Shrink)
        };

        let inner_content = child_element.unwrap_or_else(|| {
            create_empty_placeholder(
                &widget_id,
                &self.widget_type(),
                Length::Shrink,
                Length::Shrink,
            )
        });

        // Конвертирует строковое название курсора из JSON в системный тип Iced 0.14
        let current_interaction: Interaction =
            match props.cursor_type.to_lowercase().replace(' ', "").as_str() {
                "none"        => Interaction::None,
                "hidden"      => Interaction::Hidden,
                "idle"        => Interaction::Idle,
                "contextmenu" => Interaction::ContextMenu,
                "help"        => Interaction::Help,
                "pointer"     => Interaction::Pointer,
                "progress"    => Interaction::Progress,
                "wait"        => Interaction::Wait,
                "cell"        => Interaction::Cell,
                "crosshair"   => Interaction::Crosshair,
                "text"        => Interaction::Text,
                "alias"       => Interaction::Alias,
                "copy"        => Interaction::Copy,
                "move"        => Interaction::Move,
                "nodrop"      => Interaction::NoDrop,
                "notalowwed"  => Interaction::NotAllowed,
                "grab"        => Interaction::Grab,
                "grabbing"    => Interaction::Grabbing,

                "resizinghorizontally"   => Interaction::ResizingHorizontally,
                "resizingvertically"     => Interaction::ResizingVertically,
                "resizingdiagonallyup"   => Interaction::ResizingDiagonallyUp,
                "resizingdiagonallydown" => Interaction::ResizingDiagonallyDown,
                "resizingcolumn"         => Interaction::ResizingColumn,
                "resizingrow"            => Interaction::ResizingRow,

                "allscroll"              => Interaction::AllScroll,
                "zoomin"                 => Interaction::ZoomIn,
                "zoomout"                => Interaction::ZoomOut,
                // По умолчанию
                _ => Interaction::Idle,  
            };

        // НАСТОЯЩАЯ СБОРКА MOUSE_AREA ДЛЯ ОБОИХ РЕЖИМОВ
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // === РЕЖИМ ДИЗАЙНЕРА ===
            // Оборачиваем в mouse_area, которая шлет команду ВЫДЕЛЕНИЯ в инспектор свойств

            mouse_area(inner_content)
                .interaction(iced::mouse::Interaction::Pointer) // Дизайнеру удобно видеть палец
                //.on_press(Message::SelectWidget{widget_id: widget_id.clone()})  // Ваше общее сообщение выделения
                .on_press(Message::MenuEvent(MenuAction::SelectWidget(self.get_id())))
                .into()
        } else {
            // === РЕЖИМ ПОЛЬЗОВАТЕЛЯ (РЕНДЕР) ===

            // Размеры у mouse_area зависят от размеров контента
            // Оборачиваем внутренний контенет
            mouse_area(inner_content)
                .interaction(current_interaction)
                .into()
        };

        // Дизайн-оверлей (нарисует красную или синюю рамку вокруг нашей области)
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

        // Извлекаем текущие свойства mouse_area из фабрики
        let current = self.parse_props(factory);
        
        // Получаем чистые дефолтные свойства для сравнения
        let default = MouseareaProps::default();

        if current.cursor_type != default.cursor_type {
            prop_names.push(PROP_CURSOR_TYPE);
        }

        prop_names
    }    
}
