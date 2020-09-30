use iced_graphics::{
    widget::canvas::{Canvas, Program},
    Renderer,
};
use iced_native::{
    layout::{self, Layout, Node},
    overlay::{self, Overlay},
    widget::Widget,
    Clipboard, Element, Event, Hasher, Length, Point, Size,
};

pub struct CanvasBackground<'a, Message, Renderer, P>
where
    Renderer: iced_native::Renderer,
    P: Program<Message>,
{
    canvas: Canvas<Message, P>,
    foreground: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer, P> CanvasBackground<'a, Message, Renderer, P>
where
    Renderer: iced_native::Renderer,
    P: Program<Message>,
{
    pub fn new<E>(program: P, foreground: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        let foreground = foreground.into();
        Self {
            canvas: Canvas::new(program)
                .width(foreground.width())
                .height(foreground.height()),
            foreground,
        }
    }
}

impl<'a, Message, B, P> Widget<Message, Renderer<B>>
    for CanvasBackground<'a, Message, Renderer<B>, P>
where
    B: iced_graphics::Backend,
    P: Program<Message>,
{
    fn width(&self) -> Length {
        self.foreground.width()
    }

    fn height(&self) -> Length {
        self.foreground.height()
    }

    fn layout(&self, renderer: &Renderer<B>, limits: &layout::Limits) -> layout::Node {
        self.foreground.layout(renderer, limits)
    }

    fn draw(
        &self,
        renderer: &mut Renderer<B>,
        defaults: &<Renderer<B> as iced_native::Renderer>::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> <Renderer<B> as iced_native::Renderer>::Output {
        Widget::<Message, Renderer<B>>::draw(
            &self.canvas,
            renderer,
            defaults,
            layout,
            cursor_position,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.foreground.hash_layout(state)
    }

    fn overlay(
        &mut self,
        layout: Layout<'_>,
    ) -> Option<overlay::Element<'_, Message, Renderer<B>>> {
        Some(overlay::Element::new(
            layout.position(),
            Box::new(Foreground {
                inner: &mut self.foreground,
            }),
        ))
    }
}

impl<'a, Message, B, P> Into<Element<'a, Message, Renderer<B>>>
    for CanvasBackground<'a, Message, Renderer<B>, P>
where
    B: iced_graphics::Backend + 'a,
    P: Program<Message> + 'a,
    Message: 'a,
{
    fn into(self) -> Element<'a, Message, Renderer<B>> {
        Element::new(self)
    }
}

struct Foreground<'a, 'b, Message, Renderer> {
    inner: &'a mut Element<'b, Message, Renderer>,
}

impl<'a, 'b, Message, Renderer> Overlay<Message, Renderer> for Foreground<'a, 'b, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    fn layout(&self, renderer: &Renderer, bounds: Size, position: Point) -> Node {
        let mut node = self
            .inner
            .layout(renderer, &layout::Limits::new(Size::ZERO, dbg!(bounds)));
        node.move_to(position);
        node
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        self.inner.draw(renderer, defaults, layout, cursor_position)
    }

    fn hash_layout(&self, state: &mut Hasher, _position: Point) {
        self.inner.hash_layout(state)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>,
    ) {
        self.inner.on_event(
            event,
            layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        )
    }
}
