use std::marker::PhantomData;

use crate::diagram::model;
use iced_graphics::{backend, Backend, Defaults, Primitive, Renderer};
use iced_native::{
    layout, mouse, Background, Color, Element, Hasher, Layout, Length, Point, Rectangle, Size,
    Widget,
};

pub struct Item<'a, B>
where
    B: Backend + backend::Text,
{
    model: model::Item,
    widget: Element<'a, Message, Renderer<B>>,
    phantom: PhantomData<B>,
}

#[derive(Clone, Debug)]
pub struct Message;

impl<'a, B> Item<'a, B>
where
    B: Backend + backend::Text + 'a,
{
    pub fn new(model: model::Item) -> Self {
        Self {
            model,
            widget: Self::create_widget().into(),
            phantom: PhantomData,
        }
    }

    pub fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;

        self.model.name.hash(state);
    }

    fn create_widget() -> Element<'a, Message, Renderer<B>> {
        iced_graphics::Text::new("Krumpli Salat")
            .horizontal_alignment(iced::HorizontalAlignment::Center)
            .into()
    }
}

impl<'a, B> Widget<Message, Renderer<B>> for Item<'a, B>
where
    B: Backend + backend::Text,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer<B>, limits: &layout::Limits) -> layout::Node {
        //layout::Node::new(Size::new(
        //    limits.fill().width,
        //    limits.resolve(Size::new(100.0, 100.0)).height,
        //))
        //self.widget.layout(renderer, limits);
        layout::Node::new(Size::new(100.0, 100.0))
    }

    fn hash_layout(&self, state: &mut Hasher) {
        (self as &Item<B>).hash_layout(state);
    }

    fn draw(
        &self,
        renderer: &mut Renderer<B>,
        defaults: &Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> (Primitive, mouse::Interaction) {
        (
            Primitive::Group {
                primitives: vec![
                    Primitive::Quad {
                        bounds: layout.bounds(),
                        background: Background::Color(Color::TRANSPARENT),
                        border_radius: 0.0,
                        border_width: 1.0,
                        border_color: Color::BLACK,
                    },
                    Primitive::Text {
                        content: self.model.name.clone(),
                        bounds: Rectangle {
                            x: layout.bounds().center_x(),
                            //y: layout.bounds().center_y(),
                            ..layout.bounds()
                        },
                        color: Color::from_rgb(1.0, 0.0, 0.0),
                        size: 14.0,
                        font: iced::Font::default(),
                        horizontal_alignment: iced::HorizontalAlignment::Center,
                        vertical_alignment: iced::VerticalAlignment::Top,
                    },
                    self.widget
                        .draw(renderer, defaults, layout, cursor_position, viewport)
                        .0,
                ],
            },
            mouse::Interaction::default(),
        )
    }
}

impl<'a, B> Into<Element<'a, Message, Renderer<B>>> for Item<'a, B>
where
    B: Backend + backend::Text + 'a,
{
    fn into(self) -> Element<'a, Message, Renderer<B>> {
        Element::new(self)
    }
}
