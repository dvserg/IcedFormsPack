// -----------------------------------------------------------------------------
// Модуль update_property
// Содержит реализацию обработки событий апдейта свойств VTable
// -----------------------------------------------------------------------------
use iced::Color;
//use log::{info, warn, error};

pub use crate::app::App;
pub use crate::core::*;
//pub use crate::core::os_dialogs::*;

// -----------------------------------------------------------------------------
// Диспетчер обработки сообщений апдейта свойств
// -----------------------------------------------------------------------------

pub fn handle_property_update(
    factory: &mut Factory,
    widget_id: String,
    property_key: PropertyKey,
    value: PropertyValue,
) {
    // Пишем логирование обработки
    log::info!(
        "handle_property_update: Обработка события PropertyValue. Обновление свойства: widget '{}:{}'",
        widget_id,
        property_key.name
    );

    let prop_hash = property_key.hash;

    match value {
        PropertyValue::Parent(parent_id) => {
            let widget_id_cl = widget_id.clone();
            log::info!(
                "Инспектор: Обновление <Parent> значение для '{}:{}' = \"{}\"",
                &widget_id_cl,
                property_key.name,
                parent_id
            );

            factory.set_blueprint_parent(&widget_id, &parent_id);
        }
        PropertyValue::Text(txt) => {
            log::info!(
                "Инспектор: Обновление <Text> значение для '{}:{}' = \"{}\"",
                widget_id,
                property_key.name,
                txt
            );

            factory.set_by_hash::<String>(&widget_id, prop_hash, txt);
        }
        PropertyValue::USize(num) => {
            log::info!(
                "Инспектор: Обновление <usize> значение для {}:{} -> \"{}\"",
                widget_id,
                property_key.name,
                num
            );
            factory.set_by_hash::<usize>(&widget_id, prop_hash, num);
        }
        PropertyValue::Float(num) => {
            log::info!(
                "Инспектор: Обновление <Float> значение для {}:{} -> \"{}\"",
                widget_id,
                property_key.name,
                num
            );
            factory.set_by_hash::<f32>(&widget_id, prop_hash, num);
        }
        PropertyValue::Integer(num) => {
            log::info!(
                "Инспектор: Обновление <Integer> значение для {}:{} -> \"{}\"",
                widget_id,
                property_key.name,
                num
            );
            factory.set_by_hash::<i32>(&widget_id, prop_hash, num);
        }
        PropertyValue::Boolean(flag) => {
            log::info!(
                "Инспектор: Обновление <Bool> значение для  {}:{} -> \"{}\"",
                widget_id,
                property_key.name,
                flag
            );
            factory.set_by_hash::<bool>(&widget_id, prop_hash, flag);
        }
        PropertyValue::Color(color) => {
            log::info!(
                "Инспектор: Обновление <Color> значение для  {}:{} -> \"{}\"",
                widget_id,
                property_key.name,
                color
            );
            factory.set_by_hash::<Color>(&widget_id, prop_hash, color);
        }
        //PropertyValue::Quad() => {
        //
        //}
        PropertyValue::Length(length, pixels) => {
            log::info!(
                "Инспектор: Обновление <Length> значение для  {}:{} -> \"{:?}\"",
                widget_id,
                property_key.name,
                length, //utils::cast_length_2_string(length)
            );
            let prop_key_pixels = format!("{}:pixels", property_key.name);
            let hash_key_pixels = fnv1a_hash_64(&prop_key_pixels);

            // Чтобы данные не терялись, сохраняем сразу 2 значения: length и pixels
            factory.set_by_hash::<f32>(&widget_id, hash_key_pixels, pixels);
            factory.set_by_hash::<iced::Length>(&widget_id, prop_hash, length);
        }
        //Size
        PropertyValue::Pixels(pixels) => {
            log::info!(
                "Инспектор: Обновление <Pixels> значение для  {}:{} -> \"{}\"",
                widget_id,
                property_key.name,
                utils::cast_pixels_2_string(pixels)
            );

            factory.set_by_hash::<iced::Pixels>(&widget_id, prop_hash, pixels);
        }
        PropertyValue::Padding(padding) => {
            log::info!(
                "Инспектор: Обновление <Padding> значение для  {}:{} -> \"{}\"",
                widget_id,
                property_key.name,
                utils::cast_padding_2_string(padding)
            );

            factory.set_by_hash::<iced::Padding>(&widget_id, prop_hash, padding);
        }
        PropertyValue::Radius(radius) => {
            log::info!(
                "Инспектор: Обновление <Radius> значение для  {}:{} -> \"{}\"",
                widget_id,
                property_key.name,
                utils::cast_radius_2_string(radius)
            );

            factory.set_by_hash::<iced::border::Radius>(&widget_id, prop_hash, radius);
        }
        PropertyValue::AlignItems(align) => {
            log::info!(
                "Инспектор: Обновление <AlignItems> значение для  {}:{} -> \"{}\"",
                widget_id,
                property_key.name,
                utils::cast_align_items_2_string(align)
            );

            factory.set_by_hash::<iced::Alignment>(&widget_id, prop_hash, align);
        }
        PropertyValue::AlignX(align) => {
            log::info!(
                "Инспектор: Обновление <AlignX> значение для  {}:{} -> \"{}\"",
                widget_id,
                property_key.name,
                utils::cast_align_x_2_string(align)
            );

            factory.set_by_hash::<iced::alignment::Horizontal>(&widget_id, prop_hash, align);
        }
        PropertyValue::AlignY(align) => {
            log::info!(
                "Инспектор: Обновление <Align_Y> значение для  {}:{} -> \"{}\"",
                widget_id,
                property_key.name,
                utils::cast_align_y_2_string(align)
            );

            factory.set_by_hash::<iced::alignment::Vertical>(&widget_id, prop_hash, align);
        }

        _ => log::warn!(
            "handle_property_update: Обработка данного типа свойства {:?} не реализована",
            value
        ),
    }
}
