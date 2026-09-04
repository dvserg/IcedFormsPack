// -----------------------------------------------------------------------------
// Виджет 'lazy'
// Ленивое обновление — Кэширует граф дочерних виджетов. Перерисовывает внутренности 
// только в том случае, если изменились отслеживаемые зависимости 
// (мощный инструмент оптимизации).
// -----------------------------------------------------------------------------
use iced::{Element, Length, Theme};
use iced::widget::{mouse_area, lazy};

use crate::core::*;
use crate::core::{Message, MenuAction};
use crate::blueprints::*;

// -----------------------------------------------------------------------------
// Автоматически регистрируем blueprint в реестре
// -----------------------------------------------------------------------------
inventory::submit! {
    AutoRegisteredWidget {
        name:        LazyBlueprint::WIDGET_TYPE, // "lazy"
        category:    CAT_CONTAIN,
        constructor: create_lazy_creator, 
    }
}

fn create_lazy_creator() -> Box<dyn WidgetCreator + Send + Sync> {
    log::info!("Авторегистрация Lazy");
    Box::new(LazyCreator)
}

#[derive(Debug, Clone)]
pub struct LazyCreator;

impl WidgetCreator for LazyCreator {
    fn create_blueprint(&self, id: String) -> Rc<dyn WidgetBlueprint> {
        Rc::new(LazyBlueprint::new(id))
    }
}
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct LazyProps {
    pub width:        Length,
    pub height:       Length,    
    //pub trigger_name: String, // Имя переменной состояния для отслеживания изменений
}

#[derive(Debug, Clone)]
pub struct LazyBlueprint {
    pub meta: CommonWidgetMeta,
}

impl HasCommonMeta for LazyBlueprint {
    fn get_meta(&self) -> &CommonWidgetMeta { &self.meta }
    fn get_meta_mut(&mut self) -> &mut CommonWidgetMeta { &mut self.meta }
}

impl LazyBlueprint {
    const WIDGET_TYPE: &'static str = "lazy";

    pub fn new(id: String) -> Self {
        Self {
            meta: CommonWidgetMeta::new(id), 
        }
    }

    pub fn parse_props(&self, factory: &Factory) -> LazyProps {
        let widget_id = self.get_id();

        let width:        Length = factory.get_or_set(&widget_id, PROP_WIDTH,   Length::Shrink);
        let height:       Length = factory.get_or_set(&widget_id, PROP_HEIGHT,  Length::Shrink);
        //let trigger_name: String = factory.get_or_set(&widget_id, PROP_VAL_U32, 0_u32);

        LazyProps {
            width,
            height,
            //trigger_name,
        }
    }
}


impl WidgetBlueprint for LazyBlueprint {
  
    fn widget_type(&self) -> &'static str {
        Self::WIDGET_TYPE
    }

    /*
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    */

    // Lazy принимает ровно одного ребенка, граф которого будет кэшироваться
    fn can_accept_child(&self, factory: &Factory) -> bool {
        let is_occupied = factory.blueprints.keys().any(|child_id| {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();
            parent_id == self.get_id()
        });

        // True если контейнер пустой, False если ребенок уже добавлен
        !is_occupied 
    }

    // Экспонируем настройки размеров и имени триггера в инспектор свойств
    fn editable_properties(&self) -> Vec<PropertyKey> {
        vec![
            PROP_WIDTH, 
            PROP_HEIGHT,
            //PropertyKey::from(PROP_VAL_U32),
        ]
    }

    fn build_element<'a>(
        &'a self, 
        factory: &'a Factory, 
        selected_id: Option<&str>
    ) -> Element<'a, Message, Theme> {
        let widget_id = self.get_id();
        let props = self.parse_props(factory);
        let current_version_value: u64 = 0_u64;

        // Находим и собираем блупринт единственного дочернего элемента
        //let mut child_element: Option<Element<'a, Message, Theme>> = None;
        //let mut child_blueprint_opt: Option<&Rc<dyn WidgetBlueprint>> = None;
        let mut inner_child_id = "";
        for (child_id, child_blueprint) in &factory.blueprints {
            let parent_id: String = factory.get(child_id, PROP_PARENT).unwrap_or_default();

            if parent_id == widget_id {
                //child_element = Some(child_blueprint.build_element(factory, selected_id));
                //child_blueprint_opt = Some(child_blueprint);
                inner_child_id = child_id;
                break;
            }
        }

        let bp_opt = if inner_child_id != "" {
            factory.get_blueprint(&inner_child_id)
        } else {
            None
        };


        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // Логика для режима ДИЗАЙНЕРА
            let inner: Element<'a, Message, Theme> = if let Some(bp) = bp_opt {
                    bp.build_element(factory, selected_id).into()
                } else {
                    create_empty_placeholder( &self.get_id(), &self.widget_type(), props.width, props.height ).into()
                };
            mouse_area(inner)
                .interaction(iced::mouse::Interaction::Pointer)
                .on_press(Message::MenuEvent(
                    MenuAction::SelectWidget(self.get_id())
                ))
                .into()
        } else {
            if (inner_child_id == "") {
                lazy(current_version_value, move |_version| {
                    Element::from(create_empty_placeholder( &self.get_id(), &self.widget_type(), props.width, props.height ))
                })
                .into()                
            } else {
                lazy(current_version_value, move |_version| {
                    // Используем Element::from, чтобы жестко аннотировать типы и избежать ошибки E0283
                    //Element::from(blueprint.build_element(factory, selected_id))
                    //inner_content
                    Element::from(create_empty_placeholder( &self.get_id(), &self.widget_type(), props.width, props.height ))
                })
                .into()
            }
        };



        // Собираем нативный виджет container из Iced 0.14
        //let inner_content = child_element.unwrap_or_else(|| {
            // Если ребенка нет, создаем невидимую заглушку для режима конструктора
        //    create_empty_placeholder( &self.get_id(), &self.widget_type(), props.width, props.height )
        //});
        /*
        let element: Element<'a, Message, Theme> =  lazy(current_version_value, move |_version| {
            if inner_child_id == "" {
                Element::from(create_empty_placeholder( &self.get_id(), &self.widget_type(), props.width, props.height )).into()
            } else 
            if let Some(bp) = factory.get_blueprint(&inner_child_id) {
                Element::from(bp.build_element(factory, selected_id)).into()
            } else {
                Element::from(create_empty_placeholder( &self.get_id(), &self.widget_type(), props.width, props.height )).into()
            }
        }).into();*/
/*
let element: Element<'a, Message, Theme> = lazy(current_version_value, move |_version| -> Element<'a, Message, Theme> {
    if inner_child_id != "" {
        if let Some(bp) = factory.get_blueprint(&inner_child_id) {
            return bp.build_element(factory, selected_id).into();
        }
    }
    
    // Единая точка создания плейсхолдера для пустой строки или отсутствующего blueprint
    create_empty_placeholder(&self.get_id(), &self.widget_type(), props.width, props.height).into()
}).into();
*/
        /*
        let element: Element<'a, Message, Theme> = if factory.is_design_mode() {
            // Логика для режима ДИЗАЙНЕРА
            mouse_area(inner_content)
                .interaction(iced::mouse::Interaction::Pointer)
                .on_press(Message::MenuEvent(
                    MenuAction::SelectWidget(self.get_id())
                ))
                .into()
        } else {
            lazy(current_version_value, move |_version| {
                // Используем Element::from, чтобы жестко аннотировать типы и избежать ошибки E0283
                //Element::from(blueprint.build_element(factory, selected_id))
                //inner_content
                Element::from(create_empty_placeholder( &self.get_id(), &self.widget_type(), props.width, props.height ))
            })
            .into()
        };
        */

        /*
        if factory.is_design_mode() {
            // В режиме дизайна ленивое кэширование принудительно ОТКЛЮЧАЕТСЯ.
            // Проектировщик должен видеть изменения на холсте мгновенно, без задержек кэша.
            // Также оборачиваем в mouse_area, чтобы элемент можно было выделить кликом.
            //let inner_content = match child_blueprint_opt {
            //    Some(blueprint) => blueprint.build_element(factory, selected_id),
            //    None => create_empty_placeholder(&widget_id, &self.widget_type(), Length::Shrink, Length::Shrink),
            //};

            let element: Element<'a, Message, Theme> = mouse_area(inner_content)
                .interaction(iced::mouse::Interaction::Pointer)
                .on_press(Message::MenuEvent(
                    MenuAction::SelectWidget(self.get_id())
                ))
                .into();

            return apply_design_overlay(
                element,
                factory,
                selected_id,
                &widget_id,
                props.width,
                props.height,
                0.0.into(),
                false,                
            );
        }
        */


        // 3. Логика для ПОЛЬЗОВАТЕЛЬСКОГО РЕЖИМА (РАНТАЙМ)
        // Получаем текущее числовое значение версии/хэша триггера из глобального стейта фабрики.
        // Если переменная не найдена, используем 0 в качестве базового состояния кэша.
        /*
        let current_version_value: u64 = 0_u64; //factory.get_global_state_value(&props.trigger_name).unwrap_or(0_u64);

        let element: Element<'a, Message, Theme> = match child_blueprint_opt {
            Some(blueprint) => {
                // Вызываем нативный виджет lazy из Iced 0.14.2.
                // Первым аргументом передается переменная, за изменением которой следит кэш.
                // Второе поле — замыкание, которое выполнится ТОЛЬКО при изменении этой переменной.
                lazy(current_version_value, move |_version| {
                    // Используем Element::from, чтобы жестко аннотировать типы и избежать ошибки E0283
                    Element::from(blueprint.build_element(factory, selected_id))
                })
                .into()
            }
            None => {
                // Если пользователь не добавил ребенка внутрь lazy, рендерим плейсхолдер
                utils_bp::create_empty_placeholder(&self.get_id(), &self.widget_type(), props.width, props.height)
            }
        };
        */
        // В рантайме возвращаем стабильный ленивый элемент, упакованный в стандартный оверлей
        apply_design_overlay(
            element,
            factory,
            selected_id,
            &widget_id,
            props.width,
            props.height,
            0.0.into(),
            false,
        )
    }
}
