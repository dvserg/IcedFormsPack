// -----------------------------------------------------------------------------
// Модуль prop_keys
// Содержит реализацию декларации ключей свойств для VTable
// -----------------------------------------------------------------------------
use crate::core::PropertyKey;
use crate::declare_properties;

// -----------------------------------------------------------------------------
// Объявление константных имен свойств
// -----------------------------------------------------------------------------

// ВНИМАНИЕ: ! Очень важно для создания hash-ключей!
// Типы не менять, расширения типов (iced:: и т.д.) не добавлять
// Соблюдать строчные/прописные буквы

declare_properties! {
    // Геометрия базовых контейнеров
    PROP_WIDTH            => "width",             Length;
    PROP_HEIGHT           => "height",            Length;
    PROP_MAX_WIDTH        => "max_width",         Pixels;
    PROP_MAX_HEIGHT       => "max_height",        Pixels;
    PROP_SCALE            => "scale",             f32;
    PROP_CONTENT_WIDTH    => "content_width",     Pixels;
    PROP_WIDTH_FLOAT      => "width_f32",         f32;
    PROP_PIN_X            => "pin_x",             f32;
    PROP_PIN_Y            => "pin_y",             f32;

    // Размер индикатора (флага) checkbox, radio, toggle
    PROP_FLAG_SIZE        => "flag_size",         Pixels;

    // Выравнивание контента
    PROP_ALIGN_ITEMS      => "align_items",       Alignment;
    PROP_ALIGN_X          => "align_x",           Horizontal;
    PROP_ALIGN_Y          => "align_y",           Vertical;

    // Размеры, отступы и интервалы
    PROP_SIZE             => "size",              f32;
    PROP_CELL_SIZE        => "cell_size",         f32;
    PROP_SIZE_PIXELS      => "size_px",           Pixels;
    PROP_TEXT_SIZE        => "text_size",         Pixels;
    PROP_PADDING          => "padding",           Padding;
    PROP_SPACING          => "spacing",           Pixels;
    PROP_LENGTH           => "length",            Length;
    PROP_FILL_PERCENT     => "fill_percent",      f32;
    PROP_MENU_HEIGHT      => "menu_height",       Length;
    PROP_RAIL_WIDTH       => "rail_width",        f32;

    // Состояния видимости, подписи и текстовый ввод
    PROP_VISIBLE          => "visible",           bool;
    PROP_LABEL            => "label",             String;
    PROP_TEXT             => "text",              String;
    PROP_TEXT_CONTENT     => "text_content",      String;
    //PROP_CONTENT          => "content",           String;
    PROP_PLACEHOLDER      => "placeholder",       String;
    PROP_GROUP            => "group",             String;
    PROP_SELECTED         => "selected",          String;

    // Типографика и перенос строк
    PROP_LINE_HEIGHT      => "line_height",       f32;
    PROP_WRAPPING         => "wrapping",          bool;
    PROP_SHAPING          => "shaping",           bool;

    // Логические флаги состояний (Инпуты, Чекбоксы, Ползунки)
    PROP_IS_CHECKED       => "is_checked",        bool;
    PROP_IS_ENABLED       => "is_enabled",        bool;
    PROP_IS_VERTICAL      => "is_vertical",       bool;
    PROP_SECURE           => "secure",            bool;
    PROP_SHOW_HANDLE      => "show_handle",       bool;
    PROP_CLIP             => "clip",              bool; // Специфика Stack контейнеров

    PROP_IS_HANDLE_RECTANGLE => "handle_rectangle", bool;

    // Границы диапазонов слайдеров и числовой ввод
    PROP_MIN              => "min",               f32;
    PROP_MAX              => "max",               f32;
    PROP_STEP             => "step",              f32;

    PROP_TEXT_LINE_HEIGHT => "text_line_height",  f32;

    // Выпадающие списки и значения данных
    PROP_OPTIONS          => "options",           String;
    PROP_VALUE            => "value",             String;
    PROP_VAL_F32          => "value_f32",         f32;
    PROP_VAL_U32          => "value_u32",         u32;
    PROP_GIRTH            => "girth",             f32;
    PROP_DATA             => "data",              String;
    PROP_PATH             => "path",              String;

    // Цветовая палитра элементов интерфейса
    PROP_COLOR            => "color",             Color;
    PROP_TEXT_COLOR       => "text_color",        Color;
    PROP_CELL_COLOR       => "cell_color",        Color;
    PROP_BAR_COLOR        => "bar_color",         Color;
    PROP_FG_COLOR         => "fg_color",          Color;
    PROP_BG_COLOR         => "bg_color",          Color;
    PROP_ACTIVE_COLOR     => "active_color",      Color;
    PROP_TRACK_COLOR      => "track_color",       Color; // Специфика Scrollable
    PROP_THUMB_COLOR      => "thumb_color",       Color; // Специфика Scrollable

    // Скругления и параметры рамок
    PROP_BORDER_RADIUS    => "border_radius",     Radius;
    PROP_BORDER_WIDTH     => "border_width",      f32;
    PROP_BORDER_COLOR     => "border_color",      Color;

    // Настройки системных шрифтов
    PROP_FONT_FAMILY      => "font_family",       String;
    PROP_FONT_WEIGHT      => "font_weight",       bool;
    PROP_FONT_STYLE       => "font_style",        bool;

    // Параметры трансформации (Векторная графика SVG)
    PROP_ROTATION         => "rotation",          f32;
    PROP_CONTENT_FIT      => "content_fit",       String;
    PROP_OPACITY          => "opacity",           f32;
    PROP_FILTER_METHOD    => "filter_method",     bool;

    // Разделительные линии (HRule, VRule)
    PROP_THICKNESS        => "thickness",         Pixels;

    // Параметры конфигурации контейнеров с прокруткой (Scrollable)
    PROP_DIRECTION        => "direction",         String;
    PROP_SCROLLER_WIDTH   => "scroller_width",    f32;
    PROP_SCROLLBAR_WIDTH  => "scrollbar_width",   f32;
    PROP_SCROLLBAR_HEIGHT => "scrollbar_height",  f32;
    PROP_SCROLLBAR_MARGIN => "scrollbar_margin",  f32;

    PROP_CURSOR_TYPE      => "cursor_type",       String;

    // Бизнес-команды движка, иерархия и прочее несортированное
    PROP_ACTION           => "action",            String;   // Команда экшена (кнопки, инпуты, скролл)
    PROP_PARENT           => "parent",            String;

    PROP_PLACEHOLDER_COLOR => "placeholder_color", Color;
    PROP_SELECTION_COLOR   => "selection_color",   Color;

    PROP_IGNORE_SCROLL     => "ignore_scroll",     bool;
    PROP_IGNORE_BUTTONS    => "ignore_buttons",    bool;

    PROP_COLUMNS           => "columns",           usize;
    PROP_FLUID             => "fluid",             Pixels;
    PROP_ASPECT_RATIO      => "aspect_ratio",      f32;
}

// -----------------------------------------------------------------------------
// Хелперы
// -----------------------------------------------------------------------------

/// Возвращает текстовое название (описание) свойства по его ключу
pub fn friendly_label(prop_key: PropertyKey) -> String {
    String::from(match prop_key {
        PROP_BORDER_COLOR       => "Цвет бордюра",
        PROP_ACTIVE_COLOR       => "Цвет активного элемента",
        PROP_BG_COLOR           => "Цвет фона",
        PROP_FG_COLOR           => "Цвет основного элемента",
        PROP_TEXT_COLOR         => "Цвет текста",
        PROP_COLOR              => "Цвет",
        PROP_BAR_COLOR          => "Цвет заполнения",
        PROP_PLACEHOLDER_COLOR  => "Цвет подсказки",
        PROP_SELECTION_COLOR    => "Подсветка выделенного",
        PROP_CELL_COLOR         => "Цвет кода",

        PROP_FONT_FAMILY        => "Шрифт",
        PROP_FONT_WEIGHT        => "Полужирный текст",
        PROP_FONT_STYLE         => "Курсив",

        PROP_LINE_HEIGHT        => "Межстрочный интервал",
        PROP_TEXT_LINE_HEIGHT   => "Межстрочный интервал",
        //PROP_TEXT_ALIGNMENT => "Выравнивание текста",
        PROP_TEXT_SIZE          => "Размер текста",

        PROP_HEIGHT             => "Высота",
        PROP_WIDTH              => "Ширина",
        PROP_MAX_WIDTH          => "Максимальная ширина",
        PROP_MAX_HEIGHT         => "Максимальная высота",
        PROP_WIDTH_FLOAT        => "Ширина (f32)",
        PROP_CONTENT_WIDTH      => "Ширина текста",
        PROP_SIZE               => "Размер",
        PROP_CELL_SIZE          => "Размер",
        PROP_FLAG_SIZE          => "Размер флага",
        PROP_THICKNESS          => "Толщина",
        PROP_LENGTH             => "Длина (размер)",
        PROP_FILL_PERCENT       => "Заполнение, %",
        PROP_SIZE_PIXELS        => "Размер (px)",
        PROP_PIN_X              => "Позиция X",
        PROP_PIN_Y              => "Позиция Y",
        PROP_MENU_HEIGHT        => "Высота выпадающего меню",
        PROP_RAIL_WIDTH         => "Ширина полосы",

        PROP_MIN                => "Минимальное значение",
        PROP_MAX                => "Максимальное значение",
        PROP_STEP               => "Шаг изменения",
        PROP_VAL_F32            => "Значение, F32",
        //PROP_RADIUS         => "Cкругление углов",
        PROP_BORDER_RADIUS      => "Cкругление углов",
        PROP_BORDER_WIDTH       => "Толщина бордюра",
        PROP_GIRTH              => "Толщина полосы",

        PROP_SCALE              => "Масштаб",
        PROP_PADDING            => "Внутренние отступы",
        PROP_SPACING            => "Отступ между элементами",

        PROP_ALIGN_ITEMS        => "Выравнивание элементов",
        PROP_ALIGN_X            => "Выравнивание по горизонтали",
        PROP_ALIGN_Y            => "Выравнивание по вертикали",

        PROP_GROUP              => "Группа",
        PROP_IS_CHECKED         => "Статус флага",
        PROP_SELECTED           => "Статус флага",
        //PROP_SELECTED_VALUE => "Выбранное значение",
        PROP_SHOW_HANDLE        => "Отображать индикатор",
        PROP_SECURE             => "Скрытый ввод",
        PROP_CLIP               => "Обрезать контент",

        PROP_IS_HANDLE_RECTANGLE => "Квадратный ползунок",

        //PROP_TEXT_WRAPPING  => "Перенос текста",
        PROP_WRAPPING           => "Перенос по словам",
        //PROP_TEXT_SHAPING   => "Кернинг текста",
        PROP_SHAPING            => "Расширенная обработка Unicode",
        PROP_IS_VERTICAL        => "Вертикальный режим",

        PROP_TEXT               => "Текст",
        PROP_LABEL              => "Текстовая метка",
        PROP_PLACEHOLDER        => "Подсказка",
        PROP_TEXT_CONTENT       => "Содержание",
        //PROP_CONTENT        => "Содержание",
        PROP_VALUE              => "Значение",
        //PROP_OPTION_VALUE   => "Значение опции (ключ)",
        PROP_OPTIONS            => "Варианты значений",

        PROP_CURSOR_TYPE        => "Тип курсора",
        PROP_DATA               => "Данные",
        PROP_PARENT             => "Родительский элемент",

        PROP_SCROLLER_WIDTH     => "Ширина ползунка",
        PROP_SCROLLBAR_WIDTH    => "Ширина скроллбара",
        PROP_SCROLLBAR_MARGIN   => "Отступ ползунка",
        PROP_TRACK_COLOR        => "Цвет трека",
        PROP_THUMB_COLOR        => "Цвет ползунка",

        PROP_CONTENT_FIT        => "Режим заполнения",
        PROP_OPACITY            => "Прозрачность",
        PROP_ROTATION           => "Поворот",
        PROP_FILTER_METHOD      => "Не сглаживать при масштабировании",

        PROP_DIRECTION          => "Направление прокрутки",

        PROP_PATH               => "Путь к файлу",
        PROP_ACTION             => "Идентификатор события",
        PROP_IGNORE_SCROLL      => "Отключить скроллинг мышью",
        PROP_IGNORE_BUTTONS     => "Отключить кнопки",

        PROP_COLUMNS            => "Количество колонок",
        PROP_FLUID              => "Макс. ширина ячейки",
        PROP_ASPECT_RATIO       => "Отношение сторон ячейки",

        _ => "Свойство",
    })
}
