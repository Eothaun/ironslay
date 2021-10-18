use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;

// Custom material for our custom shader
#[derive(RenderResources, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
pub struct HexMaterial {
    pub color: Color,
    pub highlighted_coord: Vec2,
    pub selected_coord: Vec2,
    pub background_texture: Handle<Texture>,
    pub map_state: Handle<Texture>
}
impl Default for HexMaterial {
    fn default() -> Self {
        Self { 
            color: Color::WHITE, 
            highlighted_coord: Vec2::new(5.0, 5.0),
            selected_coord: Vec2::new(10.0, 10.0),
            background_texture: Default::default(),
            map_state: Default::default(),
        }
    }
}