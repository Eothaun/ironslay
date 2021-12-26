use bevy::prelude::*;

pub mod components;
mod setup;
mod systems;
mod types;

pub struct UIPlugins;

impl Plugin for UIPlugins {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(systems::update_units.system())
            .add_system(systems::update_resources.system())
            .add_system(systems::update_turns.system())
            .add_system(systems::button_system.system())
            .add_system(systems::update_tile.system())
            .init_resource::<types::ButtonMaterials>()
            .add_startup_system(setup::setup.system());
    }
}
