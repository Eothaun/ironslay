// Internal
mod orbit_camera;
use orbit_camera::*;

// External
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::{
	mesh::shape,
	pipeline::{PipelineDescriptor, RenderPipeline},
    render_graph::{base, AssetRenderResourcesNode, RenderGraph},
    renderer::RenderResources,
    shader::{ShaderStage, ShaderStages, ShaderSource},
};
use bevy_skybox::{SkyboxPlugin, SkyboxCamera};
use bevy_mod_raycast::*;

use std::env;


fn main() {
    let path = env::current_dir().unwrap();
    println!("The current working directory is {}", path.display());

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(OrbitCameraPlugin)
        .add_system_to_stage(stage::POST_UPDATE, update_raycast::<HexRaycastLayer>.system())
        .add_system_to_stage(stage::POST_UPDATE, update_debug_cursor::<HexRaycastLayer>.system())
        .add_asset::<MyMaterial>()
        .add_startup_system(setup.system())
        .add_plugin(SkyboxPlugin::from_image_file("sky1.png"))
        .run();
}

// Custom material for our custom shader
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
struct MyMaterial {
    //pub color: Color,
}

struct HexRaycastLayer;

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform Camera {
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

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut my_materials: ResMut<Assets<MyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Create a new shader pipeline
    let hex_shader_spriv = include_bytes!(env!("hex_shader.spv"));
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::new(ShaderStage::Fragment, ShaderSource::spirv_from_bytes(hex_shader_spriv)))),
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

    // load a texture and retrieve its aspect ratio
    let texture_handle = asset_server.load("branding/bevy_logo_dark_big.png");
    let aspect = 0.25;

    // Load hexagon cap model
    let hexagon_cap = asset_server.load("models/HexagonCap.gltf#Mesh0/Primitive0");

    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 8.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * aspect,
    ))));

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        albedo_texture: Some(texture_handle.clone()),
        shaded: false,
        ..Default::default()
    });

    // this material modulates the texture to make it red (and slightly transparent)
    let red_material_handle = materials.add(StandardMaterial {
        albedo: Color::rgba(1.0, 0.0, 0.0, 0.5),
        albedo_texture: Some(texture_handle.clone()),
        shaded: false,
    });

    // and lets make this one blue! (and also slightly transparent)
    let blue_material_handle = materials.add(StandardMaterial {
        albedo: Color::rgba(0.0, 0.0, 1.0, 0.5),
        albedo_texture: Some(texture_handle),
        shaded: false,
    });

    // Create a new custom material
    let my_material = my_materials.add(MyMaterial {
        // color: Color::rgb(0.0, 0.8, 0.0),
    });

    // add entities to the world
    commands
        // textured quad - normal
        .spawn(PbrBundle {
            mesh: quad_handle.clone(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(-8.0, 0.0, 1.5),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
                ..Default::default()
            },
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        // textured quad - modulated
        .spawn(PbrBundle {
            mesh: quad_handle.clone(),
            material: red_material_handle,
            transform: Transform {
                translation: Vec3::new(-8.0, 0.0, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
                ..Default::default()
            },
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        // textured quad - modulated
        .spawn(PbrBundle {
            mesh: quad_handle,
            material: blue_material_handle,
            transform: Transform {
                translation: Vec3::new(-8.0, 0.0, -1.5),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
                ..Default::default()
            },
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        // plane with custom shader
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 2.0 })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle.clone(),
            )]),
            transform: Transform::from_translation(Vec3::new(5.0, -1.0, 0.0)),
            ..Default::default()
        })
        .with(my_material.clone())
        .with(RayCastMesh::<HexRaycastLayer>::default())
        // custom mesh
        .spawn(MeshBundle {
            mesh: hexagon_cap,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(my_material)
        .with(RayCastMesh::<HexRaycastLayer>::default())
        // light
        .spawn(LightBundle {
                    transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
                    ..Default::default()
                })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(OrbitCamera::default())
        .with(SkyboxCamera)
        .with(OrbitCamera::default())
        .with(RayCastSource::<HexRaycastLayer>::new(
            RayCastMethod::CameraCursor(UpdateOn::EveryFrame(Vec2::zero()), EventReader::default())
        ))
        ;
}
