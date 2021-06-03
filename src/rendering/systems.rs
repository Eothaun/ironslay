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
    o_Target = vec4(0.0, 1.0, 0.0, 1.0);
}
"#;


pub fn setup(
    mut ironslay_resources: ResMut<IronSlayGlobalResources>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Create a new shader pipeline
    let hex_shader_spriv = include_bytes!(env!("hex_shader.spv"));
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::new(
            ShaderStage::Fragment,
            ShaderSource::spirv_from_bytes(hex_shader_spriv),
        ))),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterial resources to our shader
    render_graph.add_system_node(
        "my_material",
        AssetRenderResourcesNode::<MyMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge("my_material", base::node::MAIN_PASS)
        .unwrap();
    
    ironslay_resources.hex_render_pipeline = pipeline_handle;
}