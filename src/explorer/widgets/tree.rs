use std::hash::Hash;

use iced_native::{
    layout, overlay, Align, Clipboard, Element, Event, Hasher, Layout, Length, Point, Widget,
};

/// A tree widget.
pub struct Tree<'a, Message, Renderer> {
    spacing: u16,
    padding: u16,
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    align_items: Align,
    elements: Vec<Element<'a, Message, Renderer>>,
    levels: Vec<usize>,
}

impl<'a, Message, Renderer> Tree<'a, Message, Renderer>
where
    Renderer: self::Renderer + 'a,
    Message: 'a,
{
    pub fn new<T, F>(mut traverser: T, mut f: F) -> Self
    where
        T: TreeTraverser,
        T::Item: 'a,
        F: FnMut(&T::Item) -> Element<'a, Message, Renderer>,
    {
        let (elements, levels) = {
            let mut elements = Vec::with_capacity(16);
            let mut levels: Vec<usize> = Vec::with_capacity(16);

            let tree_item = |elem, level: usize| {
                use iced_native::widget::{Row, Space};
                Row::new()
                    .push(Space::new(
                        Length::Units(level.saturating_mul(20) as u16),
                        Length::Shrink,
                    ))
                    .push(elem)
                    .into()
            };

            loop {
                let mut level = 0;
                if let Some(first_child) = traverser.first_child() {
                    levels.push(
                        levels
                            .last()
                            .map(|l| {
                                level = l.saturating_add(1);
                                level
                            })
                            .unwrap_or(0),
                    );
                    elements.push(tree_item(f(&first_child), level));
                } else if let Some(next_sibling) = traverser.next_sibling() {
                    levels.push(
                        levels
                            .last()
                            .map(|l| {
                                level = *l;
                                level
                            })
                            .unwrap_or(0),
                    );
                    elements.push(tree_item(f(&next_sibling), level));
                } else if let Some((next_uncle, levels_up)) = traverser.next_uncle() {
                    levels.push(
                        levels
                            .last()
                            .map(|l| {
                                level = l.saturating_sub(levels_up);
                                level
                            })
                            .unwrap_or(0),
                    );
                    elements.push(tree_item(f(&next_uncle), level));
                } else {
                    break;
                }
            }
            (elements, levels)
        };

        Self {
            spacing: 0,
            padding: 0,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            align_items: Align::Start,
            elements,
            levels,
        }
    }

    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn align_items(mut self, align: Align) -> Self {
        self.align_items = align;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Tree<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits
            .max_width(self.max_width)
            .max_height(self.max_height)
            .width(self.width)
            .height(self.height);

        layout::flex::resolve::<Message, Renderer>(
            layout::flex::Axis::Vertical,
            renderer,
            &limits,
            self.padding as f32,
            self.spacing as f32,
            self.align_items,
            &self.elements,
        )
    }

    fn on_event(
        &mut self,
        _event: Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) {
        // todo!()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        self::Renderer::draw::<Message>(
            renderer,
            defaults,
            &self.elements,
            &self.levels,
            layout,
            cursor_position,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
    }

    fn overlay(&mut self, _layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
        None
    }
}

pub trait Renderer:
    iced_native::Renderer
    + iced_native::widget::row::Renderer
    + iced_native::widget::space::Renderer
    + iced_native::widget::text::Renderer
    + Sized
{
    fn draw<'a, Message>(
        &mut self,
        defaults: &Self::Defaults,
        elements: &[Element<'a, Message, Self>],
        levels: &[usize],
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: iced_graphics::Backend + iced_graphics::backend::Text,
{
    fn draw<'a, Message>(
        &mut self,
        defaults: &Self::Defaults,
        elements: &[Element<'a, Message, Self>],
        levels: &[usize],
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Self::Output {
        use iced_graphics::Primitive;
        use iced_native::mouse;

        let mut mouse_interaction = mouse::Interaction::default();

        (
            Primitive::Group {
                primitives: elements
                    .iter()
                    .zip(levels.iter())
                    .zip(layout.children())
                    .map(|((element, _level), layout)| {
                        let (primitive, new_mouse_interaction) =
                            element.draw(self, defaults, layout, cursor_position);

                        if new_mouse_interaction > mouse_interaction {
                            mouse_interaction = new_mouse_interaction;
                        }

                        primitive
                    })
                    .collect(),
            },
            mouse::Interaction::Idle,
        )
    }
}

impl<'a, Message, Renderer> From<Tree<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: self::Renderer + 'a,
{
    fn from(from: Tree<'a, Message, Renderer>) -> Self {
        Element::new(from)
    }
}

#[derive(Clone, Default)]
pub struct State {
    //
}

/// An interface for traversing trees.
///
/// A traverser is an object which allows the `Tree` widget to interact with
/// a variety of tree implementation and other tree-like data-structures.
///
///
/// The traverser cannot traverse any further once no more siblings or uncles
/// can be traversed.
pub trait TreeTraverser {
    /// The thing which the traverser returns.
    ///
    /// Should be an intermediary type which gives your widget transform
    /// function just enough information to draw itself.
    type Item;

    /// Move the traverser to the first child of the current node, if any, and
    /// return its `Element`.
    ///
    /// Consider the following tree:
    /// ```text
    /// root
    /// |-- node1
    /// |-- node2
    /// └-- node3
    ///     └-- node4
    /// ```
    ///
    /// If the current node is `node1`, this function should return `None` and
    /// the traverser should stay put.
    ///
    /// If the current node is `node3`, this function should return
    /// `Some(node4)` and the traverser should move to `node4`.
    fn first_child(&mut self) -> Option<Self::Item>;

    /// Move the traverser to the next sibling of the current node, if any, and
    /// return its `Element`.
    ///
    /// Consider the following tree:
    /// ```text
    /// root
    /// |-- node1
    /// |-- node2
    /// └-- node3
    ///     └-- node4
    ///         |-- node5
    ///         └-- node6
    /// ```
    ///
    /// If the current node is `node1`, this function should return
    /// `Some(node2)` and the traverser should move to `node2`.
    ///
    /// If the current node is `node3`, this function should return `None` and
    /// the traverser should stay put.
    fn next_sibling(&mut self) -> Option<Self::Item>;

    /// Move the traverser to the next "uncle" of the current node, if any, and
    /// return its `Element` and how many levels up the tree the traverser had
    /// go to find an uncle. An "uncle" is the sibling of a node's parent or
    /// grandparent or great grandparent and so on.
    ///
    /// ```text
    /// root
    /// |-- node1
    /// |-- node2
    /// |-- node3
    /// |   └-- node4
    /// |       |-- node5
    /// |       └-- node6
    /// └-- node7
    /// ```
    ///
    /// If the current node is any of `node1`, `node2`, `node3` or `node7`,
    /// this function should return `None` and the traverser should stay put.
    ///
    /// If the current node is `node3`, this function should return
    /// `Some((node7, 1))` and the traverser should move to `node7`.
    ///
    /// If the current node is `node4`, this function should return
    /// `Some((node7, 2))` and the traverser should move to `node7`.
    ///
    /// If the current node is `node5` or `node6`, this function should return
    /// `Some((node7, 3))` and the traverser should move to `node7`.
    fn next_uncle(&mut self) -> Option<(Self::Item, usize)>;
}
