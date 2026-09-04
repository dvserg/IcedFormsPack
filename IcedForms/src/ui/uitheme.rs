// -----------------------------------------------------------------------------
// Модуль uitheme
// Содержит реализацию тем оформления приложения
// -----------------------------------------------------------------------------
use iced::Font;
use iced::widget::*;
use iced::{Background, Border, Color, Shadow, Pixels, Padding};
use iced_aw::menu;
//use iced::Theme;

use crate::ui::*;



/// Указатель на шрифт для виджетов.
/// Имя должно строго совпадать с внутренним именем семейства TTF-файла.
//   pub const ICON_FONT: Font = Font::with_name("Font Awesome 6 Free Solid");

/// Байты шрифта, которые внедрим в бинарник программы.
/// Они понадобятся только при инициализации приложения.
//   pub const ICON_FONT_BYTES: &[u8] = include_bytes!("../../assets/fa-solid-900.ttf");

pub const FONT_SERIF: Font = Font::with_name("Serif");
pub const FONT_SERIF_BYTES: &[u8] = include_bytes!("../../assets/fonts/Serif.ttf");

pub const FONT_MATERIAL: Font = Font::with_name("Material Icons");
pub const FONT_MATERIAL_BYTES: &[u8] = include_bytes!("../../assets/fonts/MaterialIcons-Regular.ttf");

// ====================================================================
// СИСТЕМНЫЕ И ГЛОБАЛЬНЫЕ ИНСТРУМЕНТЫ (Файлы, Проект, Удаление)
// ====================================================================
pub const ICON_WIDGET_DEFAULT: &str = "\u{f12e}"; // Иконка пазла/кубика
pub const ICON_TRASH: &str          = "\u{f1f8}"; // Иконка мусорной корзины

// ====================================================================
// ГЛАВНОЕ МЕНЮ И ТУЛБАР
// ====================================================================
pub const ICON_NEW:      &str = "\u{e24d}";
pub const ICON_OPEN:     &str = "\u{e89c}";
pub const ICON_SAVE:     &str = "\u{e161}";
pub const ICON_CLEAR:    &str = "\u{e872}";
pub const ICON_DELETE:   &str = "\u{e5c9}";
pub const ICON_EXPORT:   &str = "\u{e2c6}";
pub const ICON_CODE:     &str = "\u{e86f}";
pub const ICON_SETTINGS: &str = "\u{e8b8}";
pub const ICON_ABOUT:    &str = "\u{e88e}";
pub const ICON_EXIT:     &str = "\u{eb4f}";
pub const ICON_SUN:      &str = "\u{e81a}";
pub const ICON_MOON:     &str = "\u{e51c}";
//pub const ICON_ROCKET:   &str = "🚀";
//pub const ICON_DESIGN:   &str = "🛠️";
pub const ICON_EYE:      &str = "\u{e417}";         // Глаз e8f4
pub const ICON_DESIGN:   &str = "\u{ea3c}";         // Дизайн (инструменты) (ea49)

// ====================================================================
// СТРУКТУРНЫЕ КОНТЕЙНЕРЫ (Разметка, Сетка, Слои)
// ====================================================================
pub const ICON_CUBE: &str           = "\u{e9fe}"; // Кубик для Column/Container
pub const ICON_COLUMN: &str         = "\u{e8f3}"; // Горизонтальные стрелки (Row/В стороны)
pub const ICON_ROW: &str            = "\u{e8f2}"; // Горизонтальные стрелки (Row/В стороны)
pub const ICON_STACK: &str          = "\u{e53b}"; // Слои друг на друге (Layers)
pub const ICON_SCROLLABLE: &str     = "\u{e5d7}"; // Стрелочки вверх-вниз
pub const ICON_CONTAINER: &str      = "\u{e9fe}"; // (Примечание: у вас в коде Column/Container делят ICON_CUBE)

// ====================================================================
// ЭЛЕМЕНТЫ РАЗДЕЛЕНИЯ И ОТСТУПОВ (Линии, Разделители, Пространство)
// ====================================================================
pub const ICON_SPACE: &str          = "\u{ead1}"; // Четыре стрелки в углы (Expand)
pub const ICON_H_RULE: &str         = "\u{e41c}"; // Горизонтальные стрелки (Влево-вправо H_rule)
pub const ICON_V_RULE: &str         = "\u{f548}"; // Вертикальные стрелки (Вверх-вниз для V_rule)
//pub const ICON_MINUS:           &str = "\u{f141}";  // Три горизонтальные точки '...'

// ====================================================================
// ИНТЕРАКТИВНЫЕ ЭЛЕМЕНТЫ ВВОДА (Кнопки, Переключатели, Формы)
// ====================================================================
pub const ICON_BUTTON: &str         = "\u{e913}"; // Шеврон вправо '>' (Chevron Right)
pub const ICON_INPUT: &str          = "\u{e312}"; // Клавиатура / Ввод текста
pub const ICON_CHECKBOX: &str       = "\u{e834}"; // Просто чистая галочка
pub const ICON_RADIO: &str          = "\u{e837}"; // Круг с точкой (Dot circle)
pub const ICON_TOGGLER: &str        = "\u{e9f5}"; // Переключатель On (Toggle on)
pub const ICON_SLIDER: &str         = "\u{e429}"; // Ползунки (Sliders)
pub const ICON_SLIDERS: &str        = "\u{f1de}"; // Ползунки для Counter
pub const ICON_PICK_LIST: &str      = "\u{f274}"; // Стрелочка вниз (Индикатор раскрытия списка)

// ====================================================================
// ИНФОРМАЦИОННЫЕ И ЦИФРОВЫЕ ВИДЖЕТЫ (Счетчики, Прогресс)
// ====================================================================
pub const ICON_COUNTER: &str        = "\u{e3f6}"; // Калькулятор
pub const ICON_PROGRESS_BAR: &str   = "\u{f242}"; // Три горизонтальные полосы
pub const ICON_TEXT: &str           = "\u{e23c}"; // Текстовое поле (Text width)
pub const ICON_TEXT_WIDGET: &str    = "\u{e165}"; // Жирная буква «А» с чертой под ней

// ====================================================================
// МЕДИА, ГРАФИКА И СЛОЖНЫЕ КОМПОНЕНТЫ (Вектор, Растр, Клики)
// ====================================================================
pub const ICON_IMAGE: &str          = "\u{e1bc}"; // Иконка "Файл-изображение"
pub const ICON_SVG: &str            = "\u{ef6e}"; // Свиток / Чертеж (Drafting compass)
pub const ICON_QRCODE: &str         = "\u{e00a}"; // Сетка из квадратов
pub const ICON_MOUSEAREA: &str      = "\u{f245}"; // Курсор мыши (Mouse pointer)
pub const ICON_TABS: &str           = "\u{ebbd}"; // Клонирование / Вкладки (Clone)

// ====================================================================
// Оверлей диалоги
// ====================================================================
pub const ICON_OVERLAY_CLOSE:  &str = "\u{e5cd}"; // Иконка "Крестик 45*"

pub const ICON_TEXT_BOLD:   &str    = "\u{e238}"; // Иконка "Полужирный текст"
pub const ICON_TEXT_ITALIC: &str    = "\u{e23f}"; // Иконка "Наклонный текст"
pub const ICON_TEXT_STRIKE: &str    = "\u{e257}"; // Иконка "Зачеркнутый текст"

pub const ICON_TEXT_H1:     &str    = "\u{e400}"; // Иконка "Заголовок 1"
pub const ICON_TEXT_H2:     &str    = "\u{e401}"; // Иконка "Заголовок 2"
pub const ICON_TEXT_H3:     &str    = "\u{e3fb}"; // Иконка "Заголовок 3"

pub const ICON_TEXT_CODE:   &str    = "\u{e86f}"; // Иконка "Код"
pub const ICON_TEXT_QUOTE:  &str    = "\u{e244}"; // Иконка "Цитата"
pub const ICON_TEXT_LIST:   &str    = "\u{e241}"; // Иконка "Список"
pub const ICON_TEXT_NLIST:  &str    = "\u{e242}"; // Иконка "Нумерованый список"

pub const ICON_RICH_CLIP:   &str    = "\u{e14f}"; // Иконка "Планшет-клипборд"
pub const ICON_RICH_CLEAR:  &str    = "\u{e5cd}"; // Иконка "Крестик 45*"
pub const ICON_INFO:        &str    = "\u{e88f}"; // Иконка "i"
pub const ICON_WARN:        &str    = "\u{e002}"; // Иконка "/!\"

// -----------------------------------------------------------------------------
// Стиль отрисовки интерфейса
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderStyle {
    #[default]
    Blender,
    Figma,
    VSCode,
}

// -----------------------------------------------------------------------------
// Стиль отображения интерфейса
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)] 
pub struct UIStyle {
    pub render_style:  RenderStyle, 
    pub is_dark_theme: bool,
}

// -----------------------------------------------------------------------------
// Стиль отображения интерфейса
// -----------------------------------------------------------------------------
// Стиль плитки списка

pub struct UIListTileStyle {
    pub item_text_size: f32,
    pub item_icon_size: f32,
    pub item_spacing:   Pixels,
    pub item_padding:   Padding,
    pub menu_radius:    f32,
}

impl Default for UIListTileStyle {
    fn default() -> UIListTileStyle {
        let text_size = 12.0;
        let icon_size = text_size + 2.0;

        UIListTileStyle {
            item_text_size: text_size,
            item_icon_size: icon_size,
            item_spacing:   Pixels(6.0),
            item_padding:   Padding::from([0.0, 4.0]),
            menu_radius:    4.0,
        }
    }
}
// -----------------------------------------------------------------------------
// Палитра
// -----------------------------------------------------------------------------

/// Цветовая палитра для элементов интерфейса в стиле Figma/VS Code
#[derive(Debug, Clone, Copy)]
pub struct UiPalette {
    pub bg_color:       iced::Color,    // Глобальный фон
    pub bg_panel:       iced::Color,    // Фон самого сайдбара (светло-серый)
    pub bg_header:      iced::Color,    // Фон заголовка
    pub bg_element:     iced::Color,    // Фон обычных кнопок в покое
    pub hv_element:     iced::Color,    // Hover  - цвет обычных кнопок при активации
    pub bg_active:      iced::Color,    // Active - цвет активного элемента при активации
    pub btn_active:     iced::Color,    // Active - цвет активной кнопки при активации
    pub border_element: iced::Color,    // Цвет рамок кнопок и разделителей
    pub text_main:      iced::Color,    // Цвет основного текста виджетов
    pub text_muted:     iced::Color,    // Цвет подписей и неактивного текста

    // Деструктивные элементы (Очистить холст)
    pub bg_danger_hover:     iced::Color,
    pub border_danger_hover: iced::Color,
}

// Палитра по умолчанию
impl Default for UiPalette {
    fn default() -> Self {
        Self {
            bg_color:   iced::Color::from_rgb(0.96, 0.96, 0.96),
            bg_panel:   iced::Color::from_rgb(0.96, 0.96, 0.96),
            bg_header:  iced::Color::from_rgb(0.94, 0.94, 0.94),
            bg_element: iced::Color::from_rgb(0.94, 0.94, 0.94),
            hv_element: iced::Color::from_rgb(0.94, 0.94, 0.94),
            bg_active:  iced::Color::from_rgb(0.94, 0.94, 0.94),
            btn_active: iced::Color::from_rgb(0.94, 0.94, 0.94),
            border_element: iced::Color::from_rgb(0.88, 0.88, 0.88),

            text_main:  iced::Color::from_rgb(0.15, 0.15, 0.15),
            text_muted: iced::Color::from_rgb(0.4, 0.4, 0.4),

            bg_danger_hover: iced::Color::from_rgb(0.92, 0.25, 0.25),
            border_danger_hover: iced::Color::from_rgb(0.8, 0.2, 0.2),
        }
    }
}

impl UiPalette {

    // Возвращает базовую палитру приложения
    pub fn get_palette(is_dark: bool) -> Self {
        if is_dark { Self::dark() } else { Self::light() }
    }

    // Возвращает палитру для указанного стиля
    pub fn get_style_palette(ui_style: UIStyle) -> Self {
        match ui_style.render_style {
            RenderStyle::Blender => UiPalette::get_palette_blender(ui_style.is_dark_theme),
            RenderStyle::VSCode  => UiPalette::get_palette_vscode (ui_style.is_dark_theme),
            RenderStyle::Figma   => UiPalette::get_palette_figma  (ui_style.is_dark_theme),
        }
    }
    pub fn get_palette_blender(is_dark: bool) -> Self {
        if is_dark { Self::dark_blender() } else { Self::light_blender() }
    }
    pub fn get_palette_vscode(is_dark: bool) -> Self {
        if is_dark { Self::dark_vscode() } else { Self::light_vscode() }
    }
    pub fn get_palette_figma(is_dark: bool) -> Self {
        if is_dark { Self::dark_figma() } else { Self::light_figma() }
    }

    pub fn light() -> Self {
        Self {
            bg_color:   Color::from_rgb(0.950, 0.950, 0.950),
            bg_panel:   Color::from_rgb(0.950, 0.950, 0.950),   // Светло-серый фон панелей
            bg_header:  Color::from_rgb(0.880, 0.880, 0.880),   // Выделенные плашки
            bg_element: Color::from_rgb(0.880, 0.880, 0.880),   // Выделенные плашки
            hv_element: Color::from_rgb(0.910, 0.910, 0.910),
            bg_active:  Color::from_rgb(0.910, 0.910, 0.910),
            btn_active: Color::from_rgb(0.910, 0.910, 0.910),
            border_element: Color::from_rgb(0.8, 0.8, 0.8),     // Серые границы ячеек

            // Задаем для всех значков меню, иконок и шрифтов темно-графитовый CAD-цвет (0.28)
            text_main:  Color::from_rgb(0.280, 0.280, 0.280),
            text_muted: Color::from_rgb(0.500, 0.500, 0.500),   // Серые подписи полей

            ..Default::default()
        }
    }

    /// Тёмная тема в стиле VS Code / Figma Dark
    fn dark() -> Self {
        Self {
            bg_color:   iced::Color::from_rgb(0.120, 0.120, 0.120),     // Глубокий тёмно-серый (#1E1E1E)
            bg_panel:   iced::Color::from_rgb(0.120, 0.120, 0.120),     // Глубокий тёмно-серый (#1E1E1E)
            bg_header:  iced::Color::from_rgb(0.180, 0.180, 0.180),     // Чуть светлее для кнопок (#2D2D2D)
            bg_element: iced::Color::from_rgb(0.180, 0.180, 0.180),     // Чуть светлее для кнопок (#2D2D2D)
            hv_element: iced::Color::from_rgb(0.180, 0.180, 0.180),     // Чуть светлее для кнопок (#2D2D2D)
            bg_active:  iced::Color::from_rgb(0.180, 0.180, 0.180),       
            btn_active: iced::Color::from_rgb(0.180, 0.180, 0.180),       
            border_element: iced::Color::from_rgb(0.25, 0.25, 0.25),    // Границы кнопок (#404040)

            text_main:  iced::Color::from_rgb(0.850, 0.850, 0.850),     // Светло-серый читаемый текст
            text_muted: iced::Color::from_rgb(0.600, 0.600, 0.600),     // Приглушенный текст для неактивных состояний

            // В тёмной теме деструктивный красный должен быть чуть менее ядовитым
            bg_danger_hover:     iced::Color::from_rgb(0.750, 0.200, 0.200),
            border_danger_hover: iced::Color::from_rgb(0.600, 0.150, 0.15),
        }
    }

    // -------------------------------------------------------------------------
    // Палитра Blender
    // -------------------------------------------------------------------------
    pub fn light_blender() -> Self {
        Self {
            bg_color:   Color::from_rgb(0.588, 0.588, 0.588),       // #969696 
            bg_panel:   Color::from_rgb(0.714, 0.714, 0.714),       // #B6B6B6
            bg_header:  Color::from_rgb(0.651, 0.651, 0.651),       // #A6A6A6
            bg_element: Color::from_rgb(0.800, 0.800, 0.800),       // #CCCCCC
            hv_element: Color::from_rgb(0.733, 0.733, 0.733),       // #BBBBBB
            //bg_active:  Color::from_rgb(0.200, 0.400, 0.600),       // #336699
            bg_active:  Color::from_rgb(0.169, 0.357, 0.569),       // #2B5B91
            btn_active: Color::from_rgb(0.169, 0.357, 0.569),       // #2B5B91
            border_element: Color::from_rgb(0.557, 0.557, 0.557),   // #8E8E8E

            // Задаем для всех значков меню, иконок и шрифтов темно-графитовый CAD-цвет (0.28)
            text_main:  Color::from_rgb(0.240, 0.240, 0.240),       // #3D3D3D
            text_muted: Color::from_rgb(0.459, 0.459, 0.459),       // Серые подписи полей

            ..Default::default()
        }
    }

    fn dark_blender() -> Self {
        Self {
            bg_color:       iced::Color::from_rgb(0.078, 0.078, 0.078),     // #141414
            bg_panel:       iced::Color::from_rgb(0.180, 0.180, 0.180),     // #2E2E2E
            //bg_panel:       iced::Color::from_rgb(0.239, 0.239, 0.239),   // #3D3D3D // #2E2E2E
            bg_header:      iced::Color::from_rgb(0.239, 0.239, 0.239),     // #3D3D3D
            bg_element:     iced::Color::from_rgb(0.329, 0.329, 0.329),     // #545454
            hv_element:     iced::Color::from_rgb(0.400, 0.400, 0.400),     // #666666
            bg_active:      iced::Color::from_rgb(0.278, 0.447, 0.702),     // #4772B3                      
            btn_active:     iced::Color::from_rgb(0.278, 0.447, 0.702),     // #4772B3                      
            border_element: iced::Color::from_rgb(0.231, 0.231, 0.231),     // #3B3B3B // Границы кнопок (#404040)

            text_main:  iced::Color::from_rgb(0.850, 0.850, 0.850),         // Светло-серый читаемый текст
            text_muted: iced::Color::from_rgb(0.600, 0.600, 0.600),         // Приглушенный текст для неактивных состояний

            // В тёмной теме деструктивный красный должен быть чуть менее ядовитым
            bg_danger_hover:     iced::Color::from_rgb(0.750, 0.200, 0.200),
            border_danger_hover: iced::Color::from_rgb(0.600, 0.150, 0.150),
        }
    }

    // -------------------------------------------------------------------------
    // Палитра VSCode
    // -------------------------------------------------------------------------
    pub fn light_vscode() -> Self {
        Self {            
            bg_color:   Color::from_rgb(1.000, 1.000, 1.000),       // #FFFFFF
            bg_panel:   Color::from_rgb(0.953, 0.953, 0.953),       // #F3F3F3
            bg_header:  Color::from_rgb(0.910, 0.910, 0.910),       // #E8E8E8
            bg_element: Color::from_rgb(1.000, 1.000, 1.000),       // #FFFFFF
            hv_element: Color::from_rgb(0.910, 0.910, 0.910),       // #E8E8E8
            bg_active:  Color::from_rgb(0.176, 0.392, 0.800),       // _ // #0066B3
            btn_active: Color::from_rgb(0.176, 0.392, 0.800),       // _ // #0066B3
            border_element: Color::from_rgb(0.894, 0.894, 0.898),   // #E4E4E5
            text_main:  Color::from_rgb(0.200, 0.200, 0.200),       // #333333
            text_muted: Color::from_rgb(0.380, 0.380, 0.380),       // #616161

            ..Default::default()
        }
    }    

    fn dark_vscode() -> Self {
        Self {
            bg_color:        iced::Color::from_rgb(0.118, 0.118, 0.118),     // #1E1E1E
            bg_panel:        iced::Color::from_rgb(0.145, 0.145, 0.149),     // #252526
            bg_header:       iced::Color::from_rgb(0.176, 0.176, 0.176),     // #2D2D2D
            bg_element:      iced::Color::from_rgb(0.235, 0.235, 0.235),     // #3C3C3C
            hv_element:      iced::Color::from_rgb(0.165, 0.176, 0.180),     // #2A2D2E
            bg_active:       iced::Color::from_rgb(0.224, 0.484, 0.640),     // #007ACC
            btn_active:      iced::Color::from_rgb(0.224, 0.484, 0.640),     // #007ACC
            border_element:  iced::Color::from_rgb(0.243, 0.243, 0.247),     // #252526 или #3E3E3E

            text_main:       iced::Color::from_rgb(0.800, 0.800, 0.800),     // #CCCCCC
            text_muted:      iced::Color::from_rgb(0.549, 0.549, 0.549),     // #8C8C8C

            bg_danger_hover:     iced::Color::from_rgb(0.658, 0.180, 0.180), // #A82E2E
            border_danger_hover: iced::Color::from_rgb(0.545, 0.137, 0.137),
        }
    }

    // -------------------------------------------------------------------------
    // Палитра Figma
    // -------------------------------------------------------------------------
    pub fn light_figma() -> Self {
        Self {
            bg_color:   iced::Color::from_rgb(0.961, 0.961, 0.961),             // #F5F5F5
            bg_panel:   iced::Color::from_rgb(1.000, 1.000, 1.000),       	    // #FFFFFF
            bg_header:  iced::Color::from_rgb(1.000, 1.000, 1.000),             // #FFFFFF
            bg_element: iced::Color::from_rgb(1.000, 1.000, 1.000),             // #FFFFFF
            hv_element: iced::Color::from_rgb(0.902, 0.902, 0.902),             // #E6E6E6 // #F5F5F5 from_rgb(0.961, 0.961, 0.961)
            bg_active:  iced::Color::from_rgb(0.886, 0.933, 1.000),             // #E2EEFF _ // #4080FF // #0C8CE9
            btn_active: iced::Color::from_rgb(0.250, 0.596, 0.996),             // _ // #4080FF // #0C8CE9
            border_element: iced::Color::from_rgb(0.902, 0.902, 0.902),         // #E6E6E6

            text_main:  iced::Color::from_rgb(0.000, 0.000, 0.000),             // #000000
            text_muted: iced::Color::from_rgb(0.701, 0.701, 0.701),             // #B3B3B3

            bg_danger_hover:     iced::Color::from_rgb(0.949, 0.282, 0.133),    // #F24822
            border_danger_hover: iced::Color::from_rgb(0.902, 0.200, 0.100),
        }
    }

    fn dark_figma() -> Self {
        Self {
            bg_color:       iced::Color::from_rgb(0.118, 0.122, 0.133),         // #1E1F22
            bg_panel:       iced::Color::from_rgb(0.173, 0.173, 0.173),     	// #2C2C2C // #2C2C2E
            bg_header:      iced::Color::from_rgb(0.173, 0.173, 0.173),     	// #2C2C2C // #2C2C2E
            bg_element:     iced::Color::from_rgb(0.200, 0.204, 0.220),     	// #333438 //#3E3E42
            hv_element:     iced::Color::from_rgb(0.243, 0.243, 0.259),     	// #3E3E42 // #333438
            bg_active:      iced::Color::from_rgb(0.231, 0.259, 0.376),     	// #3B4260 // #242B3D _ // #4080FF // #0C8CE9
            btn_active:     iced::Color::from_rgb(0.250, 0.596, 0.996),     	// _ // #4080FF // #0C8CE9
            border_element: iced::Color::from_rgb(0.267, 0.267, 0.275),     	// #444446

            text_main:  iced::Color::from_rgb(1.000, 1.000, 1.000),             // #FFFFFF
            text_muted: iced::Color::from_rgb(0.701, 0.701, 0.701),             // #B3B3B3

            bg_danger_hover:     iced::Color::from_rgb(0.949, 0.282, 0.133),    // #F24822
            border_danger_hover: iced::Color::from_rgb(0.850, 0.200, 0.100),
        }
    }

}

// -----------------------------------------------------------------------------
// Хелперы
// -----------------------------------------------------------------------------

// Возвращает иконку для типа виджета (сайдбар, дерево виджетов)
pub fn get_widget_icon(w_type: &str) -> &'static str {
    match w_type {
        "column"       => ICON_COLUMN,
        "row"          => ICON_ROW,
        "container"    => ICON_CONTAINER,
        "scrollable"   => ICON_SCROLLABLE,
        "stack"        => ICON_STACK,
        "text"         => ICON_TEXT,
        "button"       => ICON_BUTTON,
        "button_box"   => ICON_BUTTON,
        "input"        => ICON_INPUT,
        "checkbox"     => ICON_CHECKBOX,
        "check_box"    => ICON_CHECKBOX,
        "radio"        => ICON_RADIO,
        "toggler"      => ICON_TOGGLER,
        "slider"       => ICON_SLIDER,
        "picklist"     => ICON_PICK_LIST,
        "pick_list"    => ICON_PICK_LIST,
        "image"        => ICON_IMAGE,
        "svg"          => ICON_SVG,
        "qrcode"       => ICON_QRCODE,
        "qr_code"      => ICON_QRCODE,
        "progressbar"  => ICON_PROGRESS_BAR,
        "progress_bar" => ICON_PROGRESS_BAR,
        "mousearea"    => ICON_MOUSEAREA,
        "mouse_area"   => ICON_MOUSEAREA,
        "space"        => ICON_SPACE,
        "h_rule"       => ICON_H_RULE,
        "v_rule"       => ICON_V_RULE,
        "counter"      => ICON_COUNTER,
        _ => ICON_WIDGET_DEFAULT,
    }
}

// -----------------------------------------------------------------------------
// Style
// -----------------------------------------------------------------------------
pub fn style_mainmenu_button(
    ui_style: UIStyle
) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> button::Style 
{
    move |_theme, status|  {
        let btn_style = button::Style::default();
        render_style::style_mainmenu_button(
            &btn_style,
            &status,
            ui_style
        )
    }
}

pub fn style_menu_container(
    ui_style: UIStyle
) -> impl Fn(&iced::Theme) -> container::Style 
{
    move |_theme| {    
        render_style::style_menu_container(
            container::transparent(_theme),     // Начинаем с прозрачного стиля контейнера 
            ui_style
        )
    }
}

pub fn style_dropdown_menu(
    ui_style: UIStyle
) -> impl Fn(&iced::Theme, iced_aw::style::Status) -> menu::Style
{
    move |theme: &Theme, status: iced_aw::style::Status| {
        let theme = menu::primary(theme, status);   // Используем базовый стиль выпадающего меню
        render_style::style_dropdown_menu(
            &theme,   
            &status,
            ui_style
        )
    }
}

pub fn style_toolbar_container(ui_style: UIStyle) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| {    
        render_style::style_toolbar_container(
            container::transparent(_theme),         // Начинаем с прозрачного стиля контейнера 
            ui_style
        )
    }
}

pub fn style_toolbar_button(
    ui_style: UIStyle
) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> button::Style 
{
    move |_theme, status|  {
        let btn_style = button::Style::default();
        render_style::style_toolbar_button(
            &btn_style,
            &status,
            ui_style
        )
    }
}

// Элемент списка sidebar, inspector_panel, tree_panel
pub fn style_item_button(
    ui_style:    UIStyle,
    is_selected: bool,
) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> button::Style 
{
    move |_theme, status|  {
        let btn_style = button::Style::default();
        render_style::style_item_button(
            &btn_style,
            &status,
            ui_style,
            is_selected,
        )
    }
}

/// Генератор стиля для панелей сайдбара и инспектора
pub fn container_panel_style(palette: UiPalette) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(palette.bg_panel)),
        text_color: Some(palette.text_main),
        border: Border::default(),
        shadow: Shadow::default(),

        ..Default::default()
    }
}

/// Генератор стиля для обычных интерактивных кнопок
pub fn button_element_style(
    palette: UiPalette,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let mut s = button::Style {
            // Кнопка имеет аккуратный серый/графитовый фон из палитры элементов
            background: Some(Background::Color(palette.bg_element)),
            // Основной контрастный цвет шрифта
            text_color: palette.text_main,
            border: Border {
                color: palette.border_element,
                width: 1.0,
                radius: 4.0.into(), // Задаем скругление углов
            },
            shadow: Shadow::default(),

            ..Default::default()
        };

        // Интерактивный отклик при наведении мыши (Hover)
        if status == button::Status::Hovered {
            // Приподнимаем контраст, меняя фон на цвет границ
            s.background = Some(Background::Color(palette.border_element));
            s.border.color = palette.text_main;
        }

        s
    }
}

// !! Сейчас не используется !!
/// Генератор стиля для выпадающих списков
pub fn pick_list_style(
    palette: UiPalette,
) -> impl Fn(&iced::Theme, pick_list::Status) -> pick_list::Style {
    move |_theme, status| {
        // Вычисляем цвет обводки рамки в зависимости от наведения мыши
        let border_color = if status == pick_list::Status::Hovered {
            palette.text_main // Подсвечиваем цветом текста при наведении
        } else {
            palette.border_element // Спокойный цвет границ в обычном состоянии
        };

        // ИСПРАВЛЕНИЕ: Явно перечисляем абсолютно ВСЕ поля структуры,
        // полностью исключая вызов ..Default::default()!
        pick_list::Style {
            background: Background::Color(palette.bg_element),
            text_color: palette.text_main,
            placeholder_color: palette.text_muted,
            handle_color: palette.text_main, // Цвет стрелочки раскрытия списка справа
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 4.0.into(),
            },
        }
    }
}

/// Глобальный генератор стиля для всплывающих подсказок (Tooltip) под Iced 0.14
pub fn tooltip_style(palette: UiPalette) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| {
        container::Style {
            // Применяем вычисленный плотный и непрозрачный фон
            background: Some(iced::Background::Color(palette.bg_panel)),

            // Задаем контрастный цвет букв
            text_color: Some(palette.text_main),

            // Настраиваем аккуратную рамку вокруг подсказки
            border: iced::Border {
                color: palette.border_element,
                width: 1.0,
                radius: 4.0.into(), // Задаем CAD-скругление углов
            },
            shadow: iced::Shadow::default(),

            ..Default::default()
        }
    }
}

// Генератор стиля для холста (canvas)
pub fn canvas_rule_style() -> impl Fn(&iced::Theme) -> rule::Style {
    // Фиксируем СВЕТЛУЮ палитру для холста!
    //let palette = UiPalette::light();

    move |_theme| rule::Style {
        // Линия всегда будет аккуратной светло-серой
        color: iced::Color::TRANSPARENT,
        //width: 1,
        radius: 0.0.into(),
        fill_mode: rule::FillMode::Full,
        snap: true,
    }
}

// -----------------------------------------------------------------------------
// Данные для элементов управления Дизайн(Design/View) и Тема (Light/Dark)
// -----------------------------------------------------------------------------
pub fn get_ui_theme_toggle_data(is_dark: bool) -> (&'static str, &'static str, &'static str,) {
    // Переключение темы
    let (theme_icon, theme_text, theme_hint) = match is_dark {
        true =>  (ICON_SUN,  "Включить светлую тему", "Переключить на светлую тему"),
        false => (ICON_MOON, "Включить тёмную тему",  "Переключить на тёмную тему"),
    };

    (
        theme_icon,
        theme_text,
        theme_hint,
    )
}

pub fn get_ui_mode_toggle_data(is_design_mode: bool) -> (&'static str, &'static str, &'static str,) {
    // Переключение темы
    let (mode_icon, mode_text, mode_hint) = match is_design_mode {
        true =>  (ICON_EYE,    "Включить режим просмотра", "Переключить в режим просмотра"),
        false => (ICON_DESIGN, "Включить режим дизайна",   "Переключить в режим дизайна"),
    };

    (
        mode_icon,
        mode_text,
        mode_hint,
    )
}    