use bevy::prelude::*;
use bevy_mod_raycast::RayCastSource;
use crate::components::*;


fn update_gpu_buffer() {

}

pub fn update_mouse_hovering_and_selected(
    raycast_source_query: Query<&HexRaycastSource>,
    raycast_mesh_query: Query<(
        &HexRaycastTarget,
        &Handle<Mesh>,
        &GlobalTransform,
    )>,
    grid_query: Query<&GridPosition>,
    meshes: Res<Assets<Mesh>>,
) {

}

fn move_player_unit(
    mut commands: &mut Commands,
    mut selected_query: Query<(Entity, &mut GridPosition), (With<SelectedTag>, With<MoveableTag>, Without<MovedTag>)>,
    hover_query: Query<&GridPosition, (With<HoverTag>,)>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    let (entity, mut selected_pos) = match selected_query.iter_mut().next() {
        Some(pos) => pos,
        None => return,
    };
    let hover_pos = match hover_query.iter().next() {
        Some(pos) => pos,
        None => return,
    };

    if mouse_button_input.just_pressed(MouseButton::Left) {
        selected_pos.position = hover_pos.position;
        commands.insert_one(entity, MovedTag);
    }
}