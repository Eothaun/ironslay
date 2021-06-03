use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;

// Custom material for our custom shader
#[derive(RenderResources, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
pub struct MyMaterial {
    pub color: Color,
    pub highlighted_id: Vec2,
    pub selected_id: Vec2,
    pub background_texture: Handle<Texture>,
}
impl Default for MyMaterial {
    fn default() -> Self {
        Self { 
            color: Color::WHITE, 
            highlighted_id: Vec2::new(5.0, 5.0),
            selected_id: Vec2::new(10.0, 10.0),
            background_texture: Default::default(),
        }
    }
}