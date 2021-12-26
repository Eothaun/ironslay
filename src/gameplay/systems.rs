use super::components::*;
use crate::math_helpers;
use crate::rendering;
use crate::rendering::components::*;
use crate::wrapped_shader_functions;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy_mod_raycast::RayCastMethod;
use bevy_mod_raycast::RayCastSource;

pub fn update_mouse_hovering_and_selected(
    raycast_source_query: Query<&RayCastSource<HexRaycastLayer>>,
    raycast_mesh_query: Query<(
        &HexRaycastTarget,
        &Handle<HexMaterial>,
        &Handle<Mesh>,
        &GlobalTransform,
    )>,
    mouse_button_input: Res<Input<MouseButton>>,
    meshes: Res<Assets<Mesh>>,
    mut current_selection: ResMut<Selection>,
    mut my_materials: ResMut<Assets<HexMaterial>>,
) {
    for raycast_source in raycast_source_query.iter() {
        if let Some((entity, intersection)) = raycast_source.intersect_top() {
            if let Ok((_raycast_mesh, material_handle, mesh_handle, transform)) =
                raycast_mesh_query.get(entity)
            {
                if let Some(mesh) = meshes.get(mesh_handle.clone()) {
                    let triangle_indices =
                        rendering::helpers::calculate_vertex_indices_from_intersection(
                            &intersection,
                            mesh,
                            transform.compute_matrix(),
                        );

                    if let Some(VertexAttributeValues::Float2(uvs)) =
                        mesh.attribute(Mesh::ATTRIBUTE_UV_0)
                    {
                        let uv_v0 = Vec2::from(uvs[triangle_indices[0] as usize]);
                        let uv_v1 = Vec2::from(uvs[triangle_indices[1] as usize]);
                        let uv_v2 = Vec2::from(uvs[triangle_indices[2] as usize]);

                        let tri = intersection.world_triangle().unwrap();
                        let pos = intersection.position();
                        let barycentric_coords =
                            math_helpers::calculate_barycentric_coords(tri.v0, tri.v1, tri.v2, pos);
                        let mut interpolated_uv: Vec2 = uv_v0 * barycentric_coords.x
                            + uv_v1 * barycentric_coords.y
                            + uv_v2 * barycentric_coords.z;
                        // Temp thing in the shader, to get more hexes on the mesh
                        interpolated_uv *= 5.0;

                        if let Some(material) = my_materials.get_mut(material_handle) {
                            let hex_id = interpolated_uv
                                - wrapped_shader_functions::hex_relative_uv(interpolated_uv);
                            let hex_coord = wrapped_shader_functions::hex_grid_coord(hex_id);

                            // TODO: This should probably set some world state. And then we should translate it into the material
                            material.highlighted_coord = hex_coord;
                            if mouse_button_input.just_pressed(MouseButton::Left) {
                                material.selected_coord = hex_coord;
                                current_selection.coords = IVec2::new(
                                    hex_coord.x.round().trunc() as i32,
                                    hex_coord.y.round().trunc() as i32,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn deselection_system(
    mut commands: Commands,
    mut positions: Query<(Entity, &mut GridPosition), With<SelectedTag>>,
    current_selection: Res<Selection>,
    grid: Res<HexGrid>,
) {
    for (entity, pos) in positions.iter_mut() {
        if grid.coord_to_index(pos.position) != grid.coord_to_index(current_selection.coords) {
            commands.entity(entity).remove::<SelectedTag>();
        }
    }
}

pub fn selection_system(
    mut commands: Commands,
    mut positions: Query<(Entity, &mut GridPosition), Without<SelectedTag>>,
    current_selection: Res<Selection>,
    grid: Res<HexGrid>,
) {
    for (entity, pos) in positions.iter_mut() {
        if grid.coord_to_index(pos.position) == grid.coord_to_index(current_selection.coords) {
            commands.entity(entity).insert(SelectedTag);
        }
    }
}

pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut HexRaycastSource>,
) {
    for mut pick_source in &mut query.iter_mut() {
        if let Some(cursor_latest) = cursor.iter().last() {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
        }
    }
}
