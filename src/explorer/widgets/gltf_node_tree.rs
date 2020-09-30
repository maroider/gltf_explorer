use iced_graphics::{canvas, Rectangle};
use iced_native::{
    mouse,
    widget::{
        scrollable::{self, Scrollable},
        Text,
    },
};

use super::{
    canvas_background::CanvasBackground,
    tree::{Tree, TreeTraverser},
};

pub fn tree<'a, Message, B>(
    document: &'a gltf::Document,
    state: &'a mut State,
) -> Scrollable<'a, Message, iced_graphics::Renderer<B>>
where
    B: iced_graphics::Backend + iced_graphics::backend::Text + 'a,
    Message: 'a,
{
    Scrollable::new(&mut state.scrollable).push(CanvasBackground::new(
        CanvasProgram::new(),
        Tree::new(GltfTraverser::new(document), |node_info| {
            Text::new(node_info.name.unwrap_or("<unnamed node>"))
        }),
    ))
}

struct CanvasProgram {
    //
}

impl CanvasProgram {
    fn new() -> Self {
        Self {}
    }
}

impl<Message> canvas::Program<Message> for CanvasProgram {
    fn draw(&self, _bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        vec![]
    }

    fn mouse_interaction(&self, _bounds: Rectangle, _cursor: canvas::Cursor) -> mouse::Interaction {
        mouse::Interaction::Idle
    }

    fn update(
        &mut self,
        _event: canvas::Event,
        _bounds: Rectangle,
        _cursor: canvas::Cursor,
    ) -> Option<Message> {
        None
    }
}

#[derive(Debug)]
pub struct GltfTraverser<'a> {
    default_scene: Option<gltf::Scene<'a>>,
    scenes: (IterState, gltf::iter::Scenes<'a>),
    scene_nodes: Option<gltf::scene::iter::Nodes<'a>>,
    node_stack: Vec<(IterState, gltf::scene::iter::Children<'a>)>,
}

impl<'a> GltfTraverser<'a> {
    pub fn new(document: &'a gltf::Document) -> Self {
        Self {
            default_scene: document.default_scene(),
            scenes: (IterState::Initial, document.scenes()),
            scene_nodes: None,
            node_stack: Vec::with_capacity(8),
        }
    }
}

impl<'a> TreeTraverser for GltfTraverser<'a> {
    type Item = NodeInfo<'a>;

    fn first_child(&mut self) -> Option<Self::Item> {
        if let Some((iter_state, children)) = self.node_stack.last_mut() {
            if *iter_state == IterState::Initial {
                *iter_state = IterState::Used;
                if let Some(node) = children.next() {
                    self.node_stack.push((IterState::Initial, node.children()));
                    return Some(NodeInfo::from_node(node));
                }
            }
        } else if let Some(nodes) = self.scene_nodes.as_mut() {
            if let Some(node) = nodes.next() {
                self.node_stack.push((IterState::Initial, node.children()));
                return Some(NodeInfo::from_node(node));
            }
        } else {
            let (iter_state, scenes) = &mut self.scenes;
            if *iter_state == IterState::Initial {
                *iter_state = IterState::Used;
                if let Some(scene) = scenes.next() {
                    self.scene_nodes = Some(scene.nodes());
                    return Some(NodeInfo::from_scene(scene, self.default_scene.as_ref()));
                }
            }
        }

        None
    }

    fn next_sibling(&mut self) -> Option<Self::Item> {
        let node_stack_len = self.node_stack.len();
        if node_stack_len >= 2 {
            if let Some((_, children)) = self.node_stack.get_mut(node_stack_len - 2) {
                if let Some(child) = children.next() {
                    self.node_stack.pop();
                    self.node_stack.push((IterState::Initial, child.children()));
                    return Some(NodeInfo::from_node(child));
                }
            }
        } else if node_stack_len == 1 {
            if let Some(scene_nodes) = self.scene_nodes.as_mut() {
                if let Some(node) = scene_nodes.next() {
                    self.node_stack.pop();
                    self.node_stack.push((IterState::Initial, node.children()));
                    return Some(NodeInfo::from_node(node));
                }
            }
        } else {
            let (iter_state, scenes) = &mut self.scenes;
            if *iter_state == IterState::Used {
                if let Some(scene) = scenes.next() {
                    self.scene_nodes = Some(scene.nodes());
                    return Some(NodeInfo::from_scene(scene, self.default_scene.as_ref()));
                }
            }
        }

        None
    }

    fn next_uncle(&mut self) -> Option<(Self::Item, usize)> {
        let mut levels = 0;
        loop {
            if !self.node_stack.is_empty() {
                levels += 1;
                self.node_stack.pop();
                if let Some(node) = self.next_sibling() {
                    break Some((node, levels));
                }
            } else {
                break self.next_sibling().map(|node| (node, levels));
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IterState {
    Initial,
    Used,
}

pub struct NodeInfo<'a> {
    pub index: usize,
    pub name: Option<&'a str>,
    pub kind_info: NodeKind,
}

pub enum NodeKind {
    Scene { is_default: bool },
    Node,
}

impl<'a> NodeInfo<'a> {
    fn from_scene(scene: gltf::Scene<'a>, default_scene: Option<&gltf::Scene<'a>>) -> Self {
        Self {
            index: scene.index(),
            name: scene.name(),
            kind_info: NodeKind::Scene {
                is_default: Some(scene.index()) == default_scene.map(|scene| scene.index()),
            },
        }
    }

    fn from_node(node: gltf::Node<'a>) -> Self {
        Self {
            index: node.index(),
            name: node.name(),
            kind_info: NodeKind::Node,
        }
    }
}

#[derive(Clone, Default)]
pub struct State {
    scrollable: scrollable::State,
}
