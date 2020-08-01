use iced_native::widget::{
    scrollable::{self, Scrollable},
    Text,
};

pub fn stats<'a, Message, Renderer>(
    document: &'a gltf::Document,
    state: &'a mut State,
) -> Scrollable<'a, Message, Renderer>
where
    Renderer: iced_native::widget::scrollable::Renderer + iced_native::widget::text::Renderer + 'a,
    Message: 'a,
{
    Scrollable::new(&mut state.scrollable)
        .push(Text::new(format!(
            "Accessors: {}",
            document.accessors().count()
        )))
        .push(Text::new(format!(
            "Animations: {}",
            document.animations().count()
        )))
        .push(Text::new(format!(
            "Buffers: {}",
            document.buffers().count()
        )))
        .push(Text::new(format!(
            "Cameras: {}",
            document.cameras().count()
        )))
        .push(Text::new(format!(
            "Has default scene: {}",
            document.default_scene().is_some()
        )))
        .push(Text::new(format!(
            "Extensions used: {}",
            document.extensions_used().count()
        )))
        .push(Text::new(format!(
            "Extensions required: {}",
            document.extensions_required().count()
        )))
        .push(Text::new(format!("Images: {}", document.images().count())))
        .push(Text::new(format!(
            "Materials: {}",
            document.materials().count()
        )))
        .push(Text::new(format!("Meshes: {}", document.meshes().count())))
        .push(Text::new(format!(
            "Samplers: {}",
            document.samplers().count()
        )))
        .push(Text::new(format!("Skins: {}", document.skins().count())))
        .push(Text::new(format!(
            "Textures: {}",
            document.textures().count()
        )))
        .push(Text::new(format!(
            "Buffer views: {}",
            document.views().count()
        )))
}

#[derive(Clone, Default)]
pub struct State {
    scrollable: scrollable::State,
}
