use iced::widget::{button, column, container, row, text};
use iced::{Element, Alignment, Background, Border, Color, Length, Padding, Theme};
use iced::advanced::renderer;
use iced_aw::style::status;
use iced_aw::menu;

use crate::core::*;
use crate::ui::{UIStyle, RenderStyle, UiPalette, UIListTileStyle};



// -----------------------------------------------------------------------------
// Формирование заголовочного элементв
// -----------------------------------------------------------------------------
pub fn render_header<'a, Message>(
    title: &'static str,
    ui_style: UIStyle,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    let title = String::from(title);
    match ui_style.render_style {
        RenderStyle::Blender => render_header_blender(title, ui_style.is_dark_theme),
        RenderStyle::VSCode  => render_header_vscode (title, ui_style.is_dark_theme),
        RenderStyle::Figma   => render_header_figma  (title, ui_style.is_dark_theme),
    }
}

pub fn render_header_vscode<'a, Message>(
    title:    String,
    _is_dark: bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    // Получаем палитру приложения
    let palette = UiPalette::get_palette_vscode(_is_dark);

    row![
        // Полоса-маркер (акцентная вертикальная черта слева от текста)
        container(iced::widget::space::horizontal().width(Length::Fixed(3.0)))
            .height(Length::Fixed(14.0)),
        // Текст заголовка
        text(title).size(13).font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        })
    ]
    .spacing(8)
    .padding(iced::Padding {
        top: 3.0,
        bottom: 3.0,
        left: 2.0,
        right: 2.0,
    })
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}

pub fn render_header_blender<'a, Message>(
    title:   String,
    is_dark: bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    let palette = UiPalette::get_palette_blender(is_dark);

    column![
        // Главная плашка заголовка в стиле Blender
        container(
            row![
                // Стрелочка раскрытия группы (Используем text_muted для приглушения)
                // text("▼ ")
                //     .size(10)
                //     .style(move |_| iced::widget::text::Style {
                //         color: Some(palette.text_muted)
                //     }),
                // Текст заголовка (Используем text_main — станет белым на темной теме!)
                text(title)
                    .size(12)
                    .font(iced::Font {
                        weight: iced::font::Weight::Semibold,
                        ..Default::default()
                    })
                    .style(move |_| iced::widget::text::Style {
                        color: Some(palette.text_main)
                    }),
            ]
            .align_y(iced::Alignment::Center)
        )
        .width(Length::Fill)
        .padding(iced::Padding {
            top: 6.0,
            bottom: 6.0,
            left: 8.0,
            right: 8.0
        })
        .style(move |_theme| container::Style {
            background: Some(iced::Background::Color(palette.bg_element)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 2.0.into()
            },
            ..Default::default()
        }),
        // Тонкая разделительная линия снизу блока
        container(iced::widget::space::horizontal().width(Length::Fill))
            .height(Length::Fixed(1.0))
            .style(move |_| container::Style {
                background: Some(palette.border_element.into()),
                ..Default::default()
            })
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

pub fn render_header_figma<'a, Message>(
    title:    String,
    _is_dark: bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    //let _palette = UiPalette::get_palette(is_dark);

    column![
        // Текст заголовка панели (всегда в верхнем регистре по канонам Figma)
        text(title)
            .size(12)
            .font(iced::Font {
                weight: iced::font::Weight::Semibold,
                ..Default::default()
            })
            /*.style(move |_| iced::widget::text::Style {
                // ИСПРАВЛЕНИЕ: Берем text_muted из палитры для идеального Figma-контраста
                color: Some(palette.text_muted), 
            })*/
            ,
            
        // Нативный аккуратный вертикальный отступ под текстом
        iced::widget::space::vertical().height(6), 
        
        // Тонкая разделительная черта снизу блока
        container(iced::widget::space::horizontal().width(Length::Fill))
            .height(Length::Fixed(1.0))
            /*.style(move |_| container::Style {
                // ИСПРАВЛЕНИЕ: Линию красим в цвет общих границ темы border_element
                background: Some(palette.border_element.into()),
                ..Default::default()
            })*/
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

// -----------------------------------------------------------------------------
// Формирование заголовка группового элемента
// -----------------------------------------------------------------------------
pub fn render_group_header<'a, Message>(
    label:    &'static str, 
    ui_style: UIStyle,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    let label = String::from(label);
    match ui_style.render_style {
        RenderStyle::Blender => render_group_header_blender(label, ui_style.is_dark_theme),
        RenderStyle::VSCode  => render_group_header_vscode (label, ui_style.is_dark_theme),
        RenderStyle::Figma   => render_group_header_figma  (label, ui_style.is_dark_theme),
    }
}

pub fn render_group_header_blender<'a, Message>(
    label: String, 
    _is_dark: bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    text(label).size(13).width(Length::Fill).into()
}

pub fn render_group_header_figma<'a, Message>(
    label: String, 
    _is_dark: bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    text(label).size(13).width(Length::Fill).into()
}

pub fn render_group_header_vscode<'a, Message>(
    label:    String, 
    _is_dark: bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    let palette = UiPalette::get_palette_vscode(_is_dark);

    container(
        text(label)
            .size(12)
            .font(iced::Font {
                weight: iced::font::Weight::Semibold,
                ..Default::default()
            })
            .style(move |_| iced::widget::text::Style {
                color: Some(palette.text_main)
            })
    )
    .width(Length::Fill)
    .into()
}

// -----------------------------------------------------------------------------
// Формирование групповой панели
// ** Наверное удалю
// -----------------------------------------------------------------------------
pub fn render_group_panel<'a, Message>(
    content:  Element<'a, Message, Theme>,
    ui_style: UIStyle,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    match ui_style.render_style {
        RenderStyle::Blender => render_group_panel_blender(content, ui_style.is_dark_theme),
        RenderStyle::VSCode  => render_group_panel_vscode (content, ui_style.is_dark_theme),
        RenderStyle::Figma   => render_group_panel_figma  (content, ui_style.is_dark_theme),
    }
}

pub fn render_group_panel_blender<'a, Message>(
    content:  Element<'a, Message, Theme>,
    is_dark:  bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    content
}

pub fn render_group_panel_figma<'a, Message>(
    content:  Element<'a, Message, Theme>,
    is_dark:  bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    content
}

pub fn render_group_panel_vscode<'a, Message>(
    content:  Element<'a, Message, Theme>,
    is_dark:  bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    content
}

// -----------------------------------------------------------------------------
// Формирование стилей элементов sidebar, inspector, tree
// -----------------------------------------------------------------------------
pub fn style_item_button(
    bt_style:  &button::Style,
    bt_status: &button::Status,
    ui_style:  UIStyle,
    is_selected: bool,
) -> button::Style
{
    match ui_style.render_style {
        RenderStyle::Blender => style_item_button_blender(bt_style, bt_status, ui_style.is_dark_theme, is_selected),
        RenderStyle::VSCode  => style_item_button_vscode (bt_style, bt_status, ui_style.is_dark_theme, is_selected),
        RenderStyle::Figma   => style_item_button_figma  (bt_style, bt_status, ui_style.is_dark_theme, is_selected),
    }
}

pub fn style_item_button_blender(
    bt_style:  &button::Style, 
    bt_status: &button::Status,
    is_dark:   bool,
    is_selected: bool,
) -> button::Style
{
    let palette      = UiPalette::get_palette_blender(is_dark);

    let is_selected = is_selected;
    let is_hovered = matches!(bt_status, button::Status::Hovered | button::Status::Pressed);

    let bg_color = match (is_selected, is_hovered) {
        (true, _)     => palette.bg_active,  // Поведение активного элемента
        (false, true) => palette.hv_element, // Поведение кнопки под курсором
        _             => palette.bg_element, // Обычное базовое состояние
    };
    let text_color = match (is_selected, is_hovered) {
        (true, _) => Color::WHITE,           // На синем фоне текст ВСЕГДА белый!
        _         => palette.text_main,      // В обычных состояниях — графитовый серый
    };

    button::Style {
        text_color: text_color,
        background: Some(Background::Color(bg_color)),
        border:     Border {
            color:  bg_color,
            width:  1.0,
            radius: 3.0.into(),
        },
        ..*bt_style
    }
}

pub fn style_item_button_vscode(
    bt_style:  &button::Style, 
    bt_status: &button::Status,
    is_dark:   bool,
    is_selected: bool,
) -> button::Style
{
    let palette = UiPalette::get_palette_vscode(is_dark);

    let is_selected = is_selected;
    let is_hovered  = matches!(bt_status, button::Status::Hovered | button::Status::Pressed);

    let bg_color = match (is_selected, is_hovered) {
        (true, _)     => palette.bg_active,  // Поведение активного элемента
        (false, true) => palette.hv_element, // Поведение кнопки под курсором
        //_             => palette.bg_element, // Обычное базовое состояние
        _             => Color::TRANSPARENT, // Обычное базовое состояние
    };
    let text_color = match (is_selected, is_hovered) {
        (true, _) => Color::WHITE,           // На синем фоне текст ВСЕГДА белый!
        _         => palette.text_main,      // В обычных состояниях — графитовый серый
    };
    button::Style {
        text_color: text_color,
        background: Some(Background::Color(bg_color)),
        border:     Border {
            color:  bg_color,
            width:  1.0,
            radius: 3.0.into(),
        },
        ..*bt_style
    }
}

pub fn style_item_button_figma(
    bt_style:  &button::Style, 
    bt_status: &button::Status,
    is_dark:   bool,
    is_selected: bool,
) -> button::Style
{
    let palette = UiPalette::get_palette_figma(is_dark);

    let is_selected = is_selected;
    let is_hovered = matches!(bt_status, button::Status::Hovered | button::Status::Pressed);

    let bg_color = match (is_selected, is_hovered) {
        (true, _)     => palette.bg_active,  // Поведение активного элемента
        (false, true) => palette.hv_element, // Поведение кнопки под курсором
        _             => Color::TRANSPARENT, // Обычное базовое состояние
    };
    let text_color = match (is_selected, is_hovered) {
        (true, _) => palette.text_main,      // На синем фоне текст ВСЕГДА белый!
        _         => palette.text_main,      // В обычных состояниях — графитовый серый
    };
    button::Style {
        text_color: text_color,
        background: Some(Background::Color(bg_color)),
        border:     Border {
            color:  bg_color,
            width:  1.0,
            radius: 3.0.into(),
        },
        ..*bt_style
    }
}

// -----------------------------------------------------------------------------
// Render panel frame
// -----------------------------------------------------------------------------
pub fn render_panel_frame<'a, Message> (
    _content: Element<'a, Message, Theme>, 
    ui_style: UIStyle
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    match ui_style.render_style {
        RenderStyle::Blender => render_panel_frame_blender(_content, ui_style.is_dark_theme),
        RenderStyle::VSCode  => render_panel_frame_vscode (_content, ui_style.is_dark_theme),
        RenderStyle::Figma   => render_panel_frame_figma  (_content, ui_style.is_dark_theme),
    }
}

pub fn render_panel_frame_blender<'a, Message>(
    _content: Element<'a, Message, Theme>, 
    _is_dark: bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    //_content
    
    // Получаем палитру приложения
    let palette = UiPalette::get_palette_blender(_is_dark);

    let panel_color  = palette.bg_panel;//bg_color;//bg_panel;
    let border_color = palette.border_element;

    // Контейнер, отвечающий ИСКЛЮЧИТЕЛЬНО за скругленную рамку
    let inner_container = container(_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([2.0, 2.0])            // Резервируем место для бордюра
        .style(move |_theme: &Theme| {
            let mut base_style = container::transparent(_theme);
            //base_style.background    = Some(Background::Color(Color::TRANSPARENT));
            base_style.background    = Some(Background::Color(panel_color));
            base_style.border.color  = border_color;
            base_style.border.width  = 1.0;
            base_style.border.radius = 6.0.into();
            base_style
        });
        
    let outer_container = container(inner_container)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([1.0, 1.0])            // Отступы между панелями
        .style(move |_theme: &Theme| {
            container::transparent(_theme)
        });

    outer_container.into()
}

pub fn render_panel_frame_figma<'a, Message>(
    _content: Element<'a, Message, Theme>, 
    _is_dark: bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    // Получаем палитру приложения
    let palette = UiPalette::get_palette_figma(_is_dark);

    // В стиле Figma делаем панели под цвет palette.bg_panel
    // а бордюр в цвет элемента
    let panel_color  = palette.bg_panel;
    let border_color = palette.border_element;

    let frame_container = container(_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([1.0, 1.0])            // Резервируем место для бордюра
        .style(move |_theme: &Theme| {
            let mut base_style = container::transparent(_theme);
            base_style.background    = Some(Background::Color(panel_color));
            base_style.border.color  = border_color;
            base_style.border.width  = 1.0;
            base_style
        });

    frame_container.into()
}

pub fn render_panel_frame_vscode<'a, Message>(
    _content: Element<'a, Message, Theme>, 
    _is_dark: bool,
) -> iced::Element<'a, Message, Theme>
where
    Message: 'a,
{
    // Получаем палитру приложения
    let palette = UiPalette::get_palette_vscode(_is_dark);

    let border_color = palette.border_element;

    // Контейнер, отвечающий ИСКЛЮЧИТЕЛЬНО за скругленную рамку
    let inner_container = container(_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([4.0, 4.0])            // Резервируем место для бордюра
        .style(move |_theme: &Theme| {
            let mut base_style = container::transparent(_theme);
            base_style.border.color  = border_color;
            base_style.border.width  = 1.0;
            base_style.border.radius = 6.0.into();
            base_style
        });
        
    let outer_container = container(inner_container)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([2.0, 2.0])            // Отступы между панелями
        .style(move |_theme: &Theme| {
            container::transparent(_theme)
        });

    outer_container.into()
}

// -----------------------------------------------------------------------------
// Style toolbar container
// -----------------------------------------------------------------------------

pub fn style_toolbar_container(
    ct_style: container::Style,
    ui_style: UIStyle,
) -> container::Style
{
    match ui_style.render_style {
        RenderStyle::Blender => style_toolbar_container_blender(ct_style, ui_style.is_dark_theme),
        RenderStyle::VSCode  => style_toolbar_container_vscode (ct_style, ui_style.is_dark_theme),
        RenderStyle::Figma   => style_toolbar_container_figma  (ct_style, ui_style.is_dark_theme),
    }
}

pub fn style_toolbar_container_blender(
    ct_style: container::Style, 
    is_dark:  bool,
) -> container::Style
{
    let palette = UiPalette::get_palette_blender(is_dark);

    container::Style {
        background: Some(Background::Color(palette.bg_color)),
        text_color: Some(palette.text_main),    
        ..ct_style 
    }
}

pub fn style_toolbar_container_vscode(
    ct_style: container::Style, 
    is_dark:  bool,
) -> container::Style
{
    let palette = UiPalette::get_palette_vscode(is_dark);
    container::Style {
        background: Some(Background::Color(palette.bg_color)),
        text_color: Some(palette.text_main),    
        ..ct_style 
    }
}

pub fn style_toolbar_container_figma(
    ct_style:   container::Style, 
    is_dark:    bool,
) -> container::Style
{
    let palette = UiPalette::get_palette_figma(is_dark);
    container::Style {
        background: Some(Background::Color(palette.bg_color)),
        text_color: Some(palette.text_main),    
        ..ct_style 
    }
}
// -----------------------------------------------------------------------------
// Style mainmenu button
// -----------------------------------------------------------------------------
pub fn style_mainmenu_button(
    bt_style:  &button::Style,
    bt_status: &button::Status,
    ui_style:  UIStyle,
) -> button::Style
{
    match ui_style.render_style {
        RenderStyle::Blender => style_mainmenu_button_blender(bt_style, bt_status, ui_style.is_dark_theme),
        RenderStyle::VSCode  => style_mainmenu_button_vscode (bt_style, bt_status, ui_style.is_dark_theme),
        RenderStyle::Figma   => style_mainmenu_button_figma  (bt_style, bt_status, ui_style.is_dark_theme),
    }
}

pub fn style_mainmenu_button_blender(
    bt_style:  &button::Style, 
    bt_status: &button::Status,
    is_dark:   bool,
) -> button::Style
{
    let palette      = UiPalette::get_palette_blender(is_dark);
    let palette_dark = UiPalette::get_palette_blender(true);

    // Поведение кнопки под курсором
    if matches!(bt_status, button::Status::Hovered | button::Status::Pressed)  {
        button::Style {
            text_color: palette_dark.text_main,
            // Для темной темы бэкграунд - панель, для светлой - active
            background: Some(Background::Color( if is_dark { palette.bg_panel } else { palette.bg_active } )),
            border:     Border{
                color:  palette_dark.border_element,
                width:  1.0,
                radius: UIListTileStyle::default().menu_radius.into(), //3.0.into(),
            },
            ..*bt_style
        }
    } else {
        button::Style {
            text_color: palette.text_main,
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..*bt_style
        }
    }
}

pub fn style_mainmenu_button_vscode(
    bt_style:  &button::Style, 
    bt_status: &button::Status,
    is_dark:   bool,
) -> button::Style
{
    let palette = UiPalette::get_palette_vscode(is_dark);

    // Поведение кнопки под курсором
    if matches!(bt_status, button::Status::Hovered | button::Status::Pressed)  {
        button::Style {        
            text_color: palette.text_main,
            background: Some(Background::Color(palette.border_element)),
            border:     Border {
                color:  palette.border_element,
                width:  1.0,
                radius: UIListTileStyle::default().menu_radius.into(), //3.0.into(),
            },
            ..*bt_style
        }
    } else {
        button::Style {
            text_color: palette.text_main,
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..*bt_style
        }
    }
}

pub fn style_mainmenu_button_figma(
    bt_style:  &button::Style, 
    bt_status: &button::Status,
    is_dark:   bool,
) -> button::Style
{
    let palette = UiPalette::get_palette_figma(is_dark);

    // Поведение кнопки под курсором
    if matches!(bt_status, button::Status::Hovered | button::Status::Pressed)  {
        let btn_color = palette.btn_active;

        button::Style {        
            text_color: palette.text_main,
            background: Some(Background::Color(btn_color/*palette.hv_element*/)),
            border:     Border {
                color:  btn_color, /*palette.hv_element,*/
                width:  1.0,
                radius: UIListTileStyle::default().menu_radius.into(), //3.0.into(),
            },
            ..*bt_style
        }
    } else {
        button::Style {        
            text_color: palette.text_main,
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..*bt_style
        }
    }
}
// -----------------------------------------------------------------------------
// Style menu container
// -----------------------------------------------------------------------------

pub fn style_menu_container(
    ct_style:   container::Style,
    ui_style:   UIStyle,
) -> container::Style
{
    match ui_style.render_style {
        RenderStyle::Blender => style_menu_container_blender(ct_style, ui_style.is_dark_theme),
        RenderStyle::VSCode  => style_menu_container_vscode (ct_style, ui_style.is_dark_theme),
        RenderStyle::Figma   => style_menu_container_figma  (ct_style, ui_style.is_dark_theme),
    }
}

pub fn style_menu_container_blender(
    ct_style:   container::Style, 
    is_dark:    bool,
) -> container::Style
{
    let palette = UiPalette::get_palette_blender(is_dark);
    container::Style {
        background: Some(Background::Color(palette.bg_color)),
        border: Border {
            color:  palette.border_element,
            width:  1.0,
            radius: 0.0.into(),
        },
        ..ct_style
    }
}

pub fn style_menu_container_vscode(
    ct_style:   container::Style, 
    is_dark:    bool,
) -> container::Style
{
    let palette = UiPalette::get_palette_vscode(is_dark);
    container::Style {
        background: Some(Background::Color(palette.bg_color)),
        border: Border {
            color:  palette.border_element,
            width:  1.0,
            radius: 0.0.into(),
        },
        ..ct_style
    }
}

pub fn style_menu_container_figma(
    ct_style:   container::Style, 
    is_dark:    bool,
) -> container::Style
{
    ct_style
}

// -----------------------------------------------------------------------------
// Style dropdown menu
// -----------------------------------------------------------------------------

pub fn style_dropdown_menu(
    m_style:  &menu::Style,
    m_status: &iced_aw::style::Status,
    ui_style: UIStyle,
) -> menu::Style
{
    match ui_style.render_style {
        RenderStyle::Blender => style_dropdown_menu_blender(m_style, m_status, ui_style.is_dark_theme),
        RenderStyle::VSCode  => style_dropdown_menu_vscode (m_style, m_status, ui_style.is_dark_theme),
        RenderStyle::Figma   => style_dropdown_menu_figma  (m_style, m_status, ui_style.is_dark_theme),
    }
}

pub fn style_dropdown_menu_blender(
    m_style:  &menu::Style,
    m_status: &iced_aw::style::Status,
    is_dark: bool,
) -> menu::Style
{
    let palette     = UiPalette::get_palette_blender(is_dark);
    let palette_inv = UiPalette::get_palette_blender(!is_dark);

    let mut bg_color     = palette.bg_color;
    let mut border_color = palette.border_element;
    let mut bar_color    = Color::TRANSPARENT;
    let mut path_color   = match is_dark {
        true  => palette.bg_panel,
        false => palette.bg_panel, //palette.btn_active,
    };

    if matches!(&m_status, iced_aw::style::Status::Hovered | iced_aw::style::Status::Pressed | iced_aw::style::Status::Focused | iced_aw::style::Status::Selected)  {
        bg_color     = palette_inv.bg_color;
        border_color = palette_inv.border_element;
        bar_color    = palette_inv.bg_panel;
    } else {
        //bar_color    = Color::TRANSPARENT;
    }

    menu::Style {
        // Стилизуем выпадающее меню
        menu_background: iced::Background::Color(bg_color),
        menu_border: Border {
            color:   border_color,
            width:   1.0,
            radius:  UIListTileStyle::default().menu_radius.into(), //4.0.into(),
            ..Default::default()
        },

        // Верхний бар меню делаем полностью прозрачным, чтобы он сливался с шапкой
        bar_background: iced::Background::Color(Color::TRANSPARENT),
        bar_border: Border {
            radius: UIListTileStyle::default().menu_radius.into(), //3.0.into(),
            ..Default::default()
        },

        // Цвет верхнего бара при активном выпадающем меню
        path:  iced::Background::Color(path_color),
        path_border: Border {
            color:  path_color,
            width:  1.0,
            radius: UIListTileStyle::default().menu_radius.into(),
        },

        //..Default::default()
        ..*m_style
    }
}

pub fn style_dropdown_menu_vscode(
    m_style:  &menu::Style,
    m_status: &iced_aw::style::Status,
    is_dark:  bool,
) -> menu::Style
{
    let palette     = UiPalette::get_palette_vscode(is_dark);
    let palette_inv = UiPalette::get_palette_vscode(!is_dark);

    //let bg_color     = palette.bg_panel;
    //let border_color = palette.border_element;
    let mut bg_color     = palette.bg_panel;
    let mut border_color = Color::TRANSPARENT;
    let mut bar_color    = Color::TRANSPARENT;
    let mut path_color   = palette.btn_active;

    if matches!(&m_status, iced_aw::style::Status::Hovered | iced_aw::style::Status::Pressed | iced_aw::style::Status::Focused | iced_aw::style::Status::Selected)  {
        bg_color     = palette_inv.bg_color;
        border_color = palette_inv.border_element;
        bar_color    = palette_inv.bg_panel;
    }

    menu::Style {
        // Стилизуем выпадающее меню
        menu_background: iced::Background::Color(bg_color),
        menu_border: Border {
            color:   border_color,
            width:   1.0,
            radius:  UIListTileStyle::default().menu_radius.into(), //3.0.into(),
            ..Default::default()
        },

        // Верхний бар меню делаем полностью прозрачным, чтобы он сливался с шапкой
        bar_background: iced::Background::Color(Color::TRANSPARENT),
        bar_border: Border {
            radius: UIListTileStyle::default().menu_radius.into(), //3.0.into(),
            ..Default::default()
        },

        // Цвет верхнего бара при активном выпадающем меню
        path:  iced::Background::Color(path_color),
        path_border: Border {
            color:  path_color,
            width:  1.0,
            radius: UIListTileStyle::default().menu_radius.into(),
        },

        ..*m_style
    }
}

pub fn style_dropdown_menu_figma(
    m_style:  &menu::Style,
    m_status: &iced_aw::style::Status,
    is_dark:  bool,
) -> menu::Style
{
    let palette     = UiPalette::get_palette_figma(is_dark);
    let palette_inv = UiPalette::get_palette_figma(!is_dark);

    // let bg_color     = palette.bg_panel;
    // let border_color = palette. border_element;

    let mut bg_color     = palette.bg_panel;
    let mut border_color = palette.border_element;
    let mut bar_color    = Color::TRANSPARENT;
    let mut path_color   = palette.btn_active;

    if matches!(&m_status, iced_aw::style::Status::Hovered | iced_aw::style::Status::Pressed | iced_aw::style::Status::Focused | iced_aw::style::Status::Selected)  {
        bg_color     = palette_inv.bg_color;
        border_color = palette_inv.border_element;
        bar_color    = palette_inv.bg_panel;
    }

    menu::Style {
        // Стилизуем выпадающее меню
        menu_background: iced::Background::Color(bg_color),
        menu_border: Border {
            color:   border_color,
            width:   1.0,
            radius:  UIListTileStyle::default().menu_radius.into(), //4.0.into(),
            ..Default::default()
        },

        // Верхний бар меню делаем полностью прозрачным, чтобы он сливался с шапкой
        bar_background: iced::Background::Color(Color::TRANSPARENT),
        bar_border: Border {
            radius: UIListTileStyle::default().menu_radius.into(), //3.0.into(),
            ..Default::default()
        },

        // Цвет верхнего бара при активном выпадающем меню
        path:  iced::Background::Color(path_color),
        path_border: Border {
            color:  path_color,
            width:  1.0,
            radius: UIListTileStyle::default().menu_radius.into(),
        },

        ..*m_style
    }
}


// -----------------------------------------------------------------------------
// Style toolbar button
// -----------------------------------------------------------------------------
pub fn style_toolbar_button(
    bt_style:  &button::Style,
    bt_status: &button::Status,
    ui_style:  UIStyle,
) -> button::Style
{
    match ui_style.render_style {
        RenderStyle::Blender => style_toolbar_button_blender(bt_style, bt_status, ui_style.is_dark_theme),
        RenderStyle::VSCode  => style_toolbar_button_vscode (bt_style, bt_status, ui_style.is_dark_theme),
        RenderStyle::Figma   => style_toolbar_button_figma  (bt_style, bt_status, ui_style.is_dark_theme),
    }
}

pub fn style_toolbar_button_blender(
    bt_style:  &button::Style, 
    bt_status: &button::Status,
    is_dark:   bool,
) -> button::Style
{
    let palette = UiPalette::get_palette_blender(is_dark);

    // Поведение кнопки под курсором
    if matches!(bt_status, button::Status::Hovered | button::Status::Pressed)  {
        button::Style {
            background: Some(Background::Color(palette.hv_element)),
            border:     Border{
                color:  palette.hv_element,
                width:  1.0,
                radius: 3.0.into(),
            },
            ..*bt_style
        }
    } else {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..*bt_style
        }
    }
}

pub fn style_toolbar_button_vscode(
    bt_style:  &button::Style, 
    bt_status: &button::Status,
    is_dark:   bool,
) -> button::Style
{
    let palette = UiPalette::get_palette_figma(is_dark);

    // Поведение кнопки под курсором
    if matches!(bt_status, button::Status::Hovered | button::Status::Pressed)  {
        button::Style {        
            background: Some(Background::Color(palette.border_element)),
            border:     Border {
                color:  palette.border_element,
                width:  1.0,
                radius: 3.0.into(),
            },
            ..*bt_style
        }
    } else {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..*bt_style
        }
    }
}

pub fn style_toolbar_button_figma(
    bt_style:  &button::Style, 
    bt_status: &button::Status,
    is_dark:   bool,
) -> button::Style
{
    let palette = UiPalette::get_palette_figma(is_dark);

    // Поведение кнопки под курсором
    if matches!(bt_status, button::Status::Hovered | button::Status::Pressed)  {
        button::Style {        
            background: Some(Background::Color(palette.hv_element)),
            border:     Border {
                color:  palette.hv_element,
                width:  1.0,
                radius: 3.0.into(),
            },
            ..*bt_style
        }
    } else {
        button::Style {        
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..*bt_style
        }
    }
}

