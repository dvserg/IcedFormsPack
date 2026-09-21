// -----------------------------------------------------------------------------
// Модуль design_proxy
// Содержит реализацию кастомной обертки для подсветки виджетов в Designt Mode
// -----------------------------------------------------------------------------
// Файл: src/core/design_proxy.rs
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::event::{Event};
use iced::mouse;
use iced::{Background, Border, Color, Element, Length, Rectangle, Size, Shadow};

pub struct DesignProxy<'a, Message, Theme, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    is_selected: bool,
    border_color: Color,
}

impl<'a, Message, Theme, Renderer> DesignProxy<'a, Message, Theme, Renderer> {
    pub fn new(content: Element<'a, Message, Theme, Renderer>, is_selected: bool) -> Self {
        Self {
            content,
            is_selected,
            border_color: if is_selected { Color::from_rgb(1.0, 0.0, 0.0) } else { Color::TRANSPARENT },
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for DesignProxy<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(&mut self, tree: &mut widget::Tree, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        // ИСПРАВЛЕНО: Передаем само дерево, так как iced 0.14 автоматически маппит детей
        self.content.as_widget_mut().layout(tree, renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        // ИСПРАВЛЕНО: Передаем дерево напрямую дочернему элементу
        self.content.as_widget_mut().update(
            tree,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        // ИСПРАВЛЕНО: Передаем дерево напрямую
        self.content.as_widget().draw(
            tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );

        if self.is_selected {
            let bounds = layout.bounds();

            renderer.fill_quad(
                renderer::Quad { 
                    bounds, 
                    border: Border::default(), 
                    shadow: Shadow::default(),
                    snap: true,
                },
                Background::Color(Color::from_rgba(0.0, 0.5, 1.0, 0.04)),
            );

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        color: self.border_color,
                        width: 2.0,
                        radius: 2.0.into(),
                    },
                    shadow: Shadow::default(),
                    snap: true,
                },
                Background::Color(Color::TRANSPARENT),
            );
        }
    }

    // Наследуем тег и состояние у ОРИГИНАЛЬНОГО виджета, чтобы не ломать Button/Toggler
    fn tag(&self) -> widget::tree::Tag { 
        self.content.as_widget().tag() 
    }

    fn state(&self) -> widget::tree::State { 
        self.content.as_widget().state() 
    }
    
    fn children(&self) -> Vec<widget::Tree> { 
        self.content.as_widget().children() 
    }
    
    fn diff(&self, tree: &mut widget::Tree) { 
        self.content.as_widget().diff(tree); 
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(tree, layout, cursor, viewport, renderer)
    }
}

pub fn design_proxy<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    is_selected: bool,
) -> Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Theme: 'a,
    Message: 'a,
{
    Element::new(DesignProxy::new(content.into(), is_selected))
}
