// -----------------------------------------------------------------------------
// Модуль inspector_panel_edit
// Содержит реализацию панели редактирования свойств
// -----------------------------------------------------------------------------
// Коллекция редакторов свойств инспектора
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, container, text};
use iced::{Alignment, Color, Element, Length, Padding, Pixels, Theme, border::Radius};

use crate::app::App;
use crate::core::*;
use crate::ui::{UiPalette, hierarchy, inspector, inspector_prop_editors, render_style};
//use crate::ui::inspector_prop_editors::*;

// ====================================================================
/// ФУНКЦИЯ СБОРКИ НИЖНЕЙ ЗОНЫ РЕДАКТОРА СВОЙСТВ
pub fn build_bottom_editor_zone<'a>(
    //widget_id: &'a str,                         // ID текущего выделенного виджета
    //selected_property_name: Option<String>,     // Активное выбранное свойство (если есть)
    //factory: &'a Factory,                       // Ссылка на фабрику параметров
    //is_dark: bool,
    app: &'a App,
) -> Element<'a, Message, Theme> {
    // Получаем параметры и состояние приложения
    let app_state = app.get_state();
    let factory = app.get_factory();
    let is_dark = app.is_dark_theme();

    // ВАЖНО: widget_id изначально является Option
    let widget_id: &str = app_state.selected_widget_id.as_deref().unwrap();

    // Получаем текущую палитру приложения
    let palette = UiPalette::get_palette(is_dark);

    // Собираем базовую колонку панели редактора
    let mut editor_panel = column![].spacing(0);

    // Добавляем шапку редактора
    editor_panel = editor_panel.push(render_style::render_header(
        "Редактор свойств",
        app.get_ui_style()
    ));

    // Диспетчеризация контента: выбрано ли свойство в таблице?
    if let Some(active_prop) = &app_state.selected_property_key {
        // Создаем токен для типа String на лету, передавая в него имя свойства
        let prop_key = active_prop.clone(); //PropertyKey::from_dynamic(&active_prop);

        // Вызываем изолированную функцию подбора нужного редактора
        let active_editor = build_active_property_editor(
            widget_id, prop_key, //&active_prop.name,
            factory, is_dark,
        );

        // Оборачиваем весь динамический редактор в красивую карточку-плашку темы
        // (Охватывает только сам редактор без заголовка)
        let bounded_editor = inspector::property_card(active_editor, is_dark);
        editor_panel = editor_panel.push(bounded_editor);
    } else {
        // Если свойство не выбрано — выводим заглушку
        editor_panel = editor_panel.push(
            container(
                text("Выберите свойство в таблице выше")
                    .size(12)
                    .font(iced::Font {
                        style: iced::font::Style::Italic,
                        ..Default::default()
                    }),
            )
            .padding(10),
        );
    }

    // Финальная обертка всей зоны в контейнер с адаптивной разделительной рамкой сверху
    container(editor_panel)
        .width(Length::Fill)
        .padding(6)
        .style(move |_theme| container::Style {
            // Берем bg_element из палитры — фон станет сочным и адаптивным!
            background: Some(iced::Background::Color(palette.bg_element)),

            // Привязываем рамку к border_element палитры
            border: iced::Border {
                color: palette.border_element, // Тонкая рамка вокруг плашки [1.1]
                width: 1.0,
                radius: 4.0.into(), // Аккуратные закругленные углы [1.1]
            },
            ..Default::default()
        })
        .into()
}

// ====================================================================
/// ФУНКЦИЯ ДИНАМИЧЕСКОГО ПОДБОРA РЕДАКТОРА СВОЙСТВА
pub fn build_active_property_editor<'a>(
    widget_id: &'a str,           // Идентификатор редактируемого виджета
    active_prop_key: PropertyKey, // Текущее активное свойство (например, "text_size")
    //current_value: String,            // Живое значение свойства из базы данных field_values
    factory: &'a Factory, // Ссылка на фабрику (нужна для извлечения связей в parent)
    is_dark: bool,        // Флаг текущей темы оформления
) -> Element<'a, Message, Theme> {
    // *** Здесь логеры не нужны, т.к. генерируются логи при кадом обновлении кадра ***
    // Логирование обработки
    //log::info!("build_active_property_editor: Создание динамического редактора свойств для widget '{}' > property '{}'. ", widget_id, active_prop_key.name);

    let active_key = active_prop_key.clone();

    // Приводим строки к нужному виду для проброса в аллокации замыканий редакторов
    let widget_id_cl = widget_id.to_string();

    match active_key {
        PROP_TEXT_CONTENT => {
            let current_value: String = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            let ieditor = inspector_prop_editors::overlay_text_editor(
                widget_id_cl,
                active_key,
                current_value,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Дефолтная заглушка 'text edit', если свойство неизвестно
        // Редактор 'color'
        PROP_COLOR
        | PROP_TEXT_COLOR
        | PROP_BG_COLOR
        | PROP_FG_COLOR
        | PROP_ACTIVE_COLOR
        | PROP_BORDER_COLOR
        | PROP_BAR_COLOR
        | PROP_TRACK_COLOR
        | PROP_THUMB_COLOR
        | PROP_PLACEHOLDER_COLOR
        | PROP_SELECTION_COLOR
        | PROP_CELL_COLOR => {
            let current_value: Color = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            let ieditor = inspector_prop_editors::color_picker_editor(
                widget_id_cl,
                active_key,
                current_value,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'scale'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_COLUMNS => {
            let current_value: usize = factory.get(&widget_id_cl, active_key).unwrap_or(0);
            let ieditor = inspector_prop_editors::usize_counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::columns_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }
        
        // Редактор 'counter' для 'scale'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_SCALE => {
            let current_value: f32 = factory.get(&widget_id_cl, active_key).unwrap_or(1.0);
            let ieditor = inspector_prop_editors::counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::scale_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'aspect' [0..10; 0.1]
        PROP_ASPECT_RATIO => {
            let current_value: f32 = factory.get(&widget_id_cl, active_key).unwrap_or(1.0);
            let ieditor = inspector_prop_editors::counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::aspect_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'max_width' и 'max_height'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_MAX_WIDTH 
        | PROP_MAX_HEIGHT 
        | PROP_FLUID => {
            let current_value: Pixels = factory
                .get(&widget_id_cl, active_key)
                .unwrap_or(Pixels(0.0));
            let ieditor = inspector_prop_editors::pixel_counter_editor(
                widget_id_cl,
                active_key,
                current_value, //utils::cast_pixels_2_f32(current_value),
                &options::max_size_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'text_size'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_TEXT_SIZE => {
            let current_value: Pixels = factory
                .get(&widget_id_cl, active_key)
                .unwrap_or(Pixels(16.0));
            let ieditor = inspector_prop_editors::pixel_counter_editor(
                widget_id_cl,
                active_key,
                current_value, //utils::cast_pixels_2_f32(current_value),
                &options::text_size_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'checked' для булевых свойств
        PROP_IS_CHECKED 
        | PROP_SHOW_HANDLE 
        | PROP_SECURE 
        | PROP_CLIP 
        | PROP_WRAPPING
        | PROP_SHAPING 
        | PROP_IS_VERTICAL 
        | PROP_FILTER_METHOD 
        | PROP_IGNORE_SCROLL
        | PROP_IGNORE_BUTTONS 
        | PROP_IS_HANDLE_RECTANGLE => {
            let current_value: bool = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            let ieditor =
                inspector_prop_editors::checkbox_editor(widget_id_cl, active_key, current_value);
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'thickness'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_THICKNESS => {
            let current_value: Pixels = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            //let current_value = utils::cast_pixels_2_f32(current_value_raw);
            let ieditor = inspector_prop_editors::pixel_counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::thickness_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'scrollbar_width'
        // В 'counter' отличаются настройки (min, max, step): [1.0, 100.0, 1.0]
        PROP_SCROLLBAR_WIDTH
        | PROP_SCROLLER_WIDTH        
        | PROP_RAIL_WIDTH => {
            let current_value: f32 = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            let ieditor = inspector_prop_editors::counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::scrollbar_width_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'scrollbar_margin'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_SCROLLBAR_MARGIN => {
            let current_value: f32 = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            let ieditor = inspector_prop_editors::counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::scrollbar_margin_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'duo_editor' для 'padding'
        // Меняет отступы попарно - [верх_низ, лево_право]
        // В 'counter' отличаются настройки (min, max, step)
        PROP_PADDING => {
            // Test
            //println!("PROP_PADDING => {}.{}", widget_id_cl, active_key.name);

            // Quad и Duo редакторы хранят свои данные в <Padding>
            // Чтение <Padding> > Преобразование в <f32> > Передача в редактор > Update одного индекса в массиве >
            // Сохранение <Padding>
            let current_value_raw: Padding =
                factory.get(&widget_id_cl, active_key).unwrap_or_default();
            let current_value = utils::cast_padding_2_vecf32(current_value_raw);
            let ieditor = inspector_prop_editors::duo_editor(
                widget_id_cl,
                active_key,
                current_value,
                String::from("Вертикаль, Горизонталь"),
                &options::padding_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'spacing'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_SPACING => {
            let current_value: Pixels = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            // Преобразуем параметр в формат для counter
            //let current_value = utils::cast_pixels_2_f32(current_value_raw);
            let ieditor = inspector_prop_editors::pixel_counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::spacing_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для группы свойств 'Pixels'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_SIZE_PIXELS | PROP_FLAG_SIZE => {
            let current_value: Pixels = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            // Преобразуем параметр в формат для counter
            //let current_value = utils::cast_pixels_2_f32(current_value_raw);
            let ieditor = inspector_prop_editors::pixel_counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::counter_options(8.0, 100.0, 2.0),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для группы свойств 'f32'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_SIZE | PROP_CELL_SIZE | PROP_WIDTH_FLOAT | PROP_MIN | PROP_MAX | PROP_STEP | 
        PROP_PIN_X | PROP_PIN_Y | PROP_VAL_F32 | PROP_GIRTH => {
            let current_value: f32 = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            let ieditor = inspector_prop_editors::counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::counter_options(0.0, 9999.0, 1.0),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'border_radius'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_BORDER_RADIUS => {
            let current_value: Radius = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            let ieditor = inspector_prop_editors::radius_counter_editor(
                widget_id_cl,
                active_key,
                current_value, //current_value.top_left,         // все скругления Radius равнозначны
                &options::radius_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор выпадающий список выбора (PickList)
        PROP_FONT_FAMILY => {
            let current_value: String = factory.get(&widget_id_cl, active_key).unwrap_or_default();

            //let type_h1 = get_prop_type_hash(active_key);
            //let type_h2 = get_prop_type_hash(PROP_OPTIONS);
            //let test: String = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            //println!("build_active_property_editor: Проверка что прочитано в действительности: Прочитано '{widget_id}:{}' -> {} Тип: <String>", active_key.name, test);
            //println!("build_active_property_editor: Тип хэша: {:?} vs {:?}", type_h1, type_h2);

            let font_options = options::font_family_options();
            let ieditor = inspector_prop_editors::select_editor(
                widget_id_cl,
                active_key,
                current_value,
                font_options,
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'checked' для булевых свойств
        PROP_FONT_WEIGHT | PROP_FONT_STYLE => {
            let current_value: bool = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            let ieditor =
                inspector_prop_editors::checkbox_editor(widget_id_cl, active_key, current_value);
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        /*
        // Редактор 'checked' для булевых свойств
        PROP_FONT_STYLE => {
            let current_value: bool = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            let ieditor = inspector_prop_editors::checkbox_editor(
                widget_id_cl,
                active_key,
                current_value,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }
        */
        // Редактор выпадающий список выбора (PickList)
        // Выравнивание по горизонтали (col)
        PROP_ALIGN_ITEMS => {
            let options = options::align_items_options();
            let current_value: Alignment = factory
                .get(&widget_id_cl, active_key)
                .unwrap_or(Alignment::Start);
            let ieditor = inspector_prop_editors::align_items_select_editor(
                widget_id_cl,
                active_key,
                current_value,
                options,
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор выпадающий список выбора (PickList)
        // Выравнивание по горизонтали (col)
        PROP_ALIGN_X => {
            let options = options::col_align_items_options();
            let current_value: Horizontal = factory
                .get(&widget_id_cl, active_key)
                .unwrap_or(Horizontal::Left);
            let ieditor = inspector_prop_editors::align_x_select_editor(
                widget_id_cl,
                active_key,
                current_value,
                options,
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор выпадающий список выбора (PickList)
        // Выравнивание по вертикали (row)
        PROP_ALIGN_Y => {
            //let current_value_raw: Vertical = factory.get(&widget_id_cl, active_key).unwrap_or(Vertical::Top);
            //let current_value = utils::cast_align_y_2_string(current_value_raw);
            let options = options::row_align_items_options();
            //let ieditor = inspector_prop_editors::select_editor(
            //    widget_id_cl,
            //    active_key,
            //    current_value,
            //    options,
            //    is_dark
            //);
            let current_value: Vertical = factory
                .get(&widget_id_cl, active_key)
                .unwrap_or(Vertical::Top);
            let ieditor = inspector_prop_editors::align_y_select_editor(
                widget_id_cl,
                active_key,
                current_value,
                options,
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'line_height'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_LINE_HEIGHT | PROP_TEXT_LINE_HEIGHT => {
            let current_value: f32 = factory.get(&widget_id_cl, active_key).unwrap_or(16.0);
            let config = options::line_height_options();
            let ieditor = inspector_prop_editors::counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &config,
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'fill_percent'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_FILL_PERCENT => {
            let current_value: f32 = factory.get(&widget_id_cl, active_key).unwrap_or(16.0);
            let config  = options::rule_fill_percent_options();
            let ieditor = inspector_prop_editors::counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &config,
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор комбинированный для типа 'iced::Lenght'
        // Содержит (pick_list + counter)
        PROP_WIDTH | PROP_HEIGHT | PROP_LENGTH | PROP_MENU_HEIGHT => {
            // !!!
            // Парсим служебную проперть "pixels" для обновления значения в редакторе
            // так как в рабочем поле могут быть (Fill, Shrink) цифровое Fixed значение теряется
            // для его сохранения используется поле "pixels"
            // (Больше не умничать, все уже настроено)
            // !!!
            let pixels_key_str = format!("{}:pixels", active_key.name);
            let pixels_key = PropertyKey::from_dynamic(&pixels_key_str);

            // В поле "pixels" только значение {f32}
            let current_pixels: f32 = factory.get(&widget_id_cl, pixels_key).unwrap_or_default();
            let current_value: Length = factory
                .get(&widget_id_cl, active_key)
                .unwrap_or(iced::Length::Shrink);

            let ieditor = inspector_prop_editors::size_mode_editor(
                widget_id_cl,
                active_key,     // "width", "height", ..
                current_value,  // Сама строка из базы (например, "Fixed:150")
                current_pixels, // Резервная информация {f32} из "pixels"
                &options::size_options(),
                is_dark,
            )
            .into();

            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }
        PROP_CONTENT_WIDTH => {
            let current_value: Pixels = factory
                .get(&widget_id_cl, active_key)
                .unwrap_or(Pixels(350.0));
            let config = options::content_width_options();
            let ieditor = inspector_prop_editors::pixel_counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &config,
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор 'counter' для 'bordero_width'
        // В 'counter' отличаются настройки (min, max, step): [0.0, 20.0, 1.0]
        PROP_BORDER_WIDTH => {
            let current_value: f32 = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            let ieditor = inspector_prop_editors::counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::border_width_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор выпадающий список выбора (PickList)
        // Свойство смены родителя(владельца) виджета
        PROP_PARENT => {
            // Получить текущее значение
            let current_value: String = factory.get(&widget_id_cl, active_key).unwrap_or_default();

            // Получаем безопасные ID виджетов в виде ссылок &str без угрозы циклической вложенности
            // !!!
            let safe_ids = hierarchy::get_safe_hierarchy(factory, Some(&widget_id_cl));

            // Инициализируем вектор строк и сразу кладем туда опцию "root" на первое место
            let mut parent_options: Vec<String> = vec!["root".to_string()];

            // Дописываем остальные безопасные виджеты, переводя их в String
            for id in safe_ids {
                parent_options.push(id.to_string());
            }

            // Создаем выпадающий список для безопасной миграции элемента по дереву иерархии
            let ieditor = inspector_prop_editors::parent_select_editor(
                widget_id_cl,
                active_key,
                current_value,
                parent_options,
                is_dark,
            );

            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор выпадающий список выбора (PickList)
        // Указатель мыши
        PROP_CURSOR_TYPE => {
            // Получить текущее значение
            let current_value: String = factory.get(&widget_id_cl, active_key).unwrap_or_default();

            let cursor_options = options::mouse_area_cursor_options();
            let ieditor = inspector_prop_editors::select_editor(
                widget_id_cl,
                active_key,
                current_value,
                cursor_options,
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор выпадающий список выбора (PickList)
        // Направление скроллинга
        PROP_DIRECTION => {
            // Получить текущее значение
            let current_value: String = factory.get(&widget_id_cl, active_key).unwrap_or_default();

            let scroll_options = options::scroll_options();
            let ieditor = inspector_prop_editors::select_editor(
                widget_id_cl,
                active_key,
                current_value,
                scroll_options,
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Редактор выпадающий список выбора (PickList)
        // Способ размещения контента
        PROP_CONTENT_FIT => {
            // Получить текущее значение
            let current_value: String = factory.get(&widget_id_cl, active_key).unwrap_or_default();

            let svg_fit_options = options::svg_content_fit_options();
            let ieditor = inspector_prop_editors::select_editor(
                widget_id_cl,
                active_key,
                current_value,
                svg_fit_options,
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Прозрачность
        // Редактор 'counter' для 'line_height'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_OPACITY => {
            let current_value: f32 = factory.get(&widget_id_cl, active_key).unwrap_or(1.0);
            let ieditor = inspector_prop_editors::counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::svg_opacity_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Вращение
        // Редактор 'counter' для 'line_height'
        // В 'counter' отличаются настройки (min, max, step)
        PROP_ROTATION => {
            let current_value: f32 = factory.get(&widget_id_cl, active_key).unwrap_or(1.0);
            let ieditor = inspector_prop_editors::counter_editor(
                widget_id_cl,
                active_key,
                current_value,
                &options::rotation_options(),
                is_dark,
            );
            let label = friendly_label(active_key);
            inspector::inspector_row(label, ieditor, is_dark)
        }

        // Информационная плашка для свойств не имеющих своих редакторов
        // Специальное поле отображает выбор группы
        PROP_SELECTED => {
            let group: String = factory.get(&widget_id_cl, PROP_GROUP).unwrap_or_default();
            let current_value: String = factory
                .get(&group, active_key)
                .unwrap_or(String::from("Не выбран"));

            let fallback_label = friendly_label(active_key);
            let fallback_text = iced::widget::text(current_value).size(12);
            //inspector::inspector_row(prop_label(fallback_label, active_prop), fallback_text.into(), is_dark)
            inspector::inspector_row(fallback_label, fallback_text.into(), is_dark)
        }

        _ => {
            //log::info!("build_active_property_editor: По условию '_' выбран редактор 'text_editor'");

            let current_value: String = factory.get(&widget_id_cl, active_key).unwrap_or_default();
            //let ieditor = inspector_prop_editors::text_editor(widget_id_string, active_prop.to_string(), current_value);
            let ieditor =
                inspector_prop_editors::text_editor(widget_id_cl, active_key, current_value);

            let label = friendly_label(active_key);
            inspector::inspector_col(label.to_string(), ieditor, is_dark)
        }
    }
}
