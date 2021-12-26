use bevy::prelude::*;

pub mod components;
pub mod helpers;
pub mod systems;

use bevy_mod_raycast::RaycastSystem;

pub struct GamePlayPlugins;

impl Plugin for GamePlayPlugins {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            systems::update_raycast_with_cursor
                .system()
                .before(RaycastSystem::BuildRays),
        )
        .add_system(systems::update_mouse_hovering_and_selected.system())
        .add_system(
            systems::deselection_system
                .system()
                .label("deselection_system"),
        )
        .add_system(
            systems::selection_system
                .system()
                .after("deselection_system"),
        )
        .add_state(components::GameState::default());
    }
}
