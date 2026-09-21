// -----------------------------------------------------------------------------
// Модуль macros
// Содержит реализацию макросов приложения
// -----------------------------------------------------------------------------


#[macro_export]
macro_rules! declare_properties {
    (
        $( $const_name:ident => $string_name:expr, $type:ty ; )*
    ) => {
        // ГЕНЕРИРУЕМ ПУБЛИЧНЫЕ КОНСТАНТЫ ЧЕРЕЗ КОНСТРУКТОР DECLARE
        $(
            // Вызываем каноничный PropertyToken::declare(), чтобы не трогать приватные поля!
            pub const $const_name: $crate::core::PropertyKey = $crate::core::PropertyKey::declare($string_name);
        )*

        // ФУНКЦИЯ ИНИЦИАЛИЗАЦИИ БАЗОВЫХ СВОЙСТВ
        pub fn init_builtin_properties() {
            //if let Ok(mut guard) = $crate::core::ALL_PROPERTY_TOKENS.write() {
            $crate::core::ALL_PROPERTY_TOKENS.with(|tokens| {
                // Наносекундный захват мутабельной ссылки (полная замена mut write_guard)
                let mut guard = tokens.borrow_mut();

                if !guard.is_empty() { return; }

                $(
                    guard.push($crate::core::TokenMetadata {
                        hash: $crate::core::fnv1a_hash_64($string_name),
                        name: $string_name,

                        type_name: stringify!($type),
                        type_hash: $crate::core::fnv1a_hash_64(stringify!($type)),
                    });

                    log::info!("declare_properties: Добавлен {} <{}>: {}", $string_name, stringify!($type), $crate::core::fnv1a_hash_64(stringify!($type)),);

                )*

                log::info!("declare_properties: Всего {}", guard.len());

            });
        }
    };
}

#[macro_export]
macro_rules! impl_refresh_props {
    ($struct_name:ty, $props_type:ty) => {
        fn refresh_internal_props(&self, factory: &Factory) {
            // Извлекаем свежие пропсы с помощью метода, который есть у структуры
            let fresh_props: $props_type = self.parse_props(factory); 
            
            // UnsafeCell
            unsafe {
                // Получаем мутабельную ссылку на внутреннюю структуру пропсов
                let props_mut_ref: &mut $props_type = &mut *self.props.get();
                    
                // Перезаписываем старые свойства новыми
                *props_mut_ref = fresh_props;
            }
        
            log::trace!("refresh_internal_props: Обновление собственных свойств блюпринта <{}> виджета '{}' из VTable.", self.widget_type(), self.get_id());
        }
    };
}