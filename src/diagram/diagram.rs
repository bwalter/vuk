use iced_graphics::{backend, Backend, Defaults, Primitive, Renderer};
use iced_native::{
    layout::{self, Limits},
    mouse, Element, Hasher, Layout, Length, Point, Rectangle, Size, Widget,
};
use std::hash::Hash;

use crate::diagram::Item;

pub struct Diagram<'a, B>
where
    B: Backend + backend::Text,
{
    children: Vec<Item<'a, B>>,
}

impl<'a, B> Diagram<'a, B>
where
    B: Backend + backend::Text,
{
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    pub fn with_children(children: Vec<Item<'a, B>>) -> Self {
        Self { children }
    }

    pub fn push(mut self, child: Item<'a, B>) -> Self {
        self.children.push(child);
        self
    }
}

impl<'a, Message, B> Widget<Message, Renderer<B>> for Diagram<'a, B>
where
    B: Backend + backend::Text,
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Fill
    }

    // TODO
    fn layout(&self, renderer: &Renderer<B>, limits: &layout::Limits) -> layout::Node {
        // TODO
        let size = Size::new(640.0, 480.0);

        let mut current_y = 10.0;

        let nodes = self
            .children
            .iter()
            .map(|child| {
                let child_limits = Limits::new(Size::ZERO, Size::new(200.0, 200.0));
                let mut node = child.layout(renderer, &child_limits);

                let child_size = Size::new(100.0, 100.0); //node.size();
                let child_position = Point::new(50.0, current_y);
                current_y += child_size.height + 20.0;

                node.move_to(child_position);
                node
            })
            .collect();

        layout::Node::with_children(size, nodes)
    }

    // TODO
    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        for child in &self.children {
            child.hash_layout(state);
        }
    }

    // TODO
    fn draw(
        &self,
        renderer: &mut Renderer<B>,
        defaults: &Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> (Primitive, mouse::Interaction) {
        let mut mouse_interaction = mouse::Interaction::default();

        (
            Primitive::Group {
                primitives: self
                    .children
                    .iter()
                    .zip(layout.children())
                    .map(|(child, layout)| {
                        let (primitive, new_mouse_interaction) =
                            child.draw(renderer, defaults, layout, cursor_position, viewport);

                        if new_mouse_interaction > mouse_interaction {
                            mouse_interaction = new_mouse_interaction;
                        }

                        primitive
                    })
                    .collect(),
            },
            mouse_interaction,
        )
    }
}

impl<'a, Message, B> Into<Element<'a, Message, Renderer<B>>> for Diagram<'a, B>
where
    B: 'a + Backend + backend::Text,
    Message: 'a,
{
    fn into(self) -> Element<'a, Message, Renderer<B>> {
        Element::new(self)
    }
}
