use bevy::core::FromBytes;
use bevy::prelude::*;
use bevy::render::{
    render_graph::{RenderGraph, base, AssetRenderResourcesNode},
    shader::{ShaderSource, ShaderStage, ShaderStages},
    pipeline::PipelineDescriptor,
};
use crate::IronSlayGlobalResources;
use crate::components::*;
use super::components::*;


const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {
    v_Uv = Vertex_Uv;
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
"#;

// Unused, as we use the rust-gpu shader. But briefly kept for set/binding reference
const _FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
/*layout(set = 2, binding = 0) uniform MyMaterial_color {
    vec4 color;
};*/
void main() {
    o_Target = vec4(0.0, 0.0, 1.0, 1.0);
}
"#;


pub fn setup(
    mut ironslay_resources: ResMut<IronSlayGlobalResources>,
    asset_server: ResMut<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    asset_server.watch_for_changes().unwrap();

    // Create a new shader pipeline
    //let hex_shader_spriv = include_bytes!(env!("hex_shader.spv"));
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(asset_server.load::<Shader, _>("glsl_shaders/hex_shader.frag"))
        // fragment: Some(shaders.add(Shader::new(
        //     ShaderStage::Fragment,
        //     ShaderSource::spirv_from_bytes(hex_shader_spriv),
        // ))),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterial resources to our shader
    render_graph.add_system_node(
        "hex_material",
        AssetRenderResourcesNode::<HexMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge("hex_material", base::node::MAIN_PASS)
        .unwrap();
    
    ironslay_resources.hex_render_pipeline = pipeline_handle;
}

pub fn update_map_texture(grid_positions: Query<(&GridPosition, &TerrainType)>,
    hex_grid: Res<HexGrid>,
    mut textures: ResMut<Assets<Texture>>,
    hex_materials: Res<Assets<HexMaterial>>,
) {
    // TODO: Assumption that there is only 1 planet & hex_material
    let hex_material_handle = hex_materials.ids().next().expect("An already created HexMaterial should have been created at init");
    let hex_material = hex_materials.get(hex_material_handle).unwrap();

    let texture = textures.get_mut(&hex_material.map_state).unwrap();
    assert!(texture.size.width as i32 == hex_grid.width && texture.size.height as i32 == hex_grid.height, 
        "The texture size differs from the grid size, did it just get resized?");
    
    let mut map_buffer: Vec<u32> = vec![0; (hex_grid.width * hex_grid.height) as usize];
    for (coord, terrain_type) in grid_positions.iter() {
        map_buffer[hex_grid.coord_to_index(coord.position)] = match terrain_type {
            TerrainType::Land => 0,
            TerrainType::Water => 1,
        };
    }
    texture.data = Vec::from_bytes(bytemuck::cast_slice(map_buffer.as_slice()));
}