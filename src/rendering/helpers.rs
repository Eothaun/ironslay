use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy_mod_raycast::Intersection;
use crate::math_helpers::vec3_all_eq;

// Temp fix to obtain vertex indices. In an ideal world, this would be supplied by bevy_mod_raycast's Intersection directly.
pub fn calculate_vertex_indices_from_intersection(
    intersection: &Intersection,
    mesh: &Mesh,
    mesh_to_world: Mat4,
) -> [u32; 3] {
    let positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        Some(VertexAttributeValues::Float3(positions)) => positions,
        _ => panic!("Mesh is expected to have float3 positions!"),
    };

    let tri = intersection.world_triangle().unwrap();

    let tri_idx_v0 = positions
        .iter()
        .position(|pos| {
            let world_pos = mesh_to_world.transform_point3((*pos).into());
            vec3_all_eq(world_pos, tri.v0, 0.01)
        })
        .unwrap();
    let tri_idx_v1 = positions
        .iter()
        .position(|pos| {
            let world_pos = mesh_to_world.transform_point3((*pos).into());
            vec3_all_eq(world_pos, tri.v1, 0.01)
        })
        .unwrap();
    let tri_idx_v2 = positions
        .iter()
        .position(|pos| {
            let world_pos = mesh_to_world.transform_point3((*pos).into());
            vec3_all_eq(world_pos, tri.v2, 0.01)
        })
        .unwrap();

    [tri_idx_v0 as u32, tri_idx_v1 as u32, tri_idx_v2 as u32]
}