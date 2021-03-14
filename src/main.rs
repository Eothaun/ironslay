use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::{
	mesh::shape,
	pipeline::{PipelineDescriptor, RenderPipeline},
    render_graph::{base, AssetRenderResourcesNode, RenderGraph},
    renderer::RenderResources,
    shader::{ShaderStage, ShaderStages, ShaderSource},
};


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_asset::<MyMaterial>()
        .add_startup_system(setup.system())
        .run();
}

// Custom material for our custom shader
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
struct MyMaterial {
    //pub color: Color,
}

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {
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
                translation: Vec3::new(0.0, 0.0, 1.5),
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
                translation: Vec3::new(0.0, 0.0, 0.0),
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
                translation: Vec3::new(0.0, 0.0, -1.5),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
                ..Default::default()
            },
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        // cube with custom shader
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(my_material)
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}