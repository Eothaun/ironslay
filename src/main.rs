// Internal
mod orbit_camera;
use orbit_camera::*;

// External
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::{
    mesh::shape,
    mesh::VertexAttributeValues,
    pipeline::{PipelineDescriptor, RenderPipeline},
    render_graph::{base, AssetRenderResourcesNode, RenderGraph},
    renderer::RenderResources,
    shader::{ShaderSource, ShaderStage, ShaderStages},
};
use bevy_mod_raycast::*;
use bevy_skybox::{SkyboxCamera, SkyboxPlugin};

use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::InspectorPlugin;
use std::env;

#[derive(Inspectable, Default)]
struct Data {
    should_render: bool,
    text: String,
    #[inspectable(min = 42.0, max = 100.0)]
    size: f32,
}

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current working directory is {}", path.display());

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(OrbitCameraPlugin)
        .add_plugin(InspectorPlugin::<Data>::new())
        .add_system_to_stage(
            stage::POST_UPDATE,
            update_raycast::<HexRaycastLayer>.system(),
        )
        .add_system_to_stage(
            stage::POST_UPDATE,
            update_debug_cursor::<HexRaycastLayer>.system(),
        )
        .add_asset::<MyMaterial>()
        .add_system(update_hex_selection.system())
        .add_startup_system(setup.system())
        .add_plugin(SkyboxPlugin::from_image_file("sky1.png"))
        .run();
}

// Custom material for our custom shader
#[derive(RenderResources, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
struct MyMaterial {
    pub color: Color,
    pub highlighted_id: Vec2,
    pub background_texture: Handle<Texture>,
}
impl Default for MyMaterial {
    fn default() -> Self {
        Self { 
            color: Color::WHITE, 
            highlighted_id: Vec2::new(5.0, 5.0),
            background_texture: Default::default(),
        }
    }
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

fn vec3_all_eq(a: Vec3, b: Vec3, epsilon: f32) -> bool {
    (a.x - b.x).abs() <= epsilon && (a.y - b.y).abs() <= epsilon && (a.z - b.z).abs() <= epsilon
}

// Temp fix to obtain vertex indices. In an ideal world, this would be supplied by bevy_mod_raycast's Intersection directly.
fn calculate_vertex_indices_from_intersection(
    intersection: &Intersection,
    mesh: &Mesh,
    mesh_to_world: Mat4,
) -> [u32; 3] {
    let positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        Some(VertexAttributeValues::Float3(positions)) => positions,
        _ => panic!("Mesh is expected to have float3 positions!"),
    };

    let tri = intersection.world_triangle();

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

fn update_hex_selection(
    raycast_source_query: Query<&RayCastSource<HexRaycastLayer>>,
    raycast_mesh_query: Query<(
        &RayCastMesh<HexRaycastLayer>,
        &Handle<MyMaterial>,
        &Handle<Mesh>,
        &GlobalTransform,
    )>,
    meshes: Res<Assets<Mesh>>,
    mut my_materials: ResMut<Assets<MyMaterial>>,
) {
    for raycast_source in raycast_source_query.iter() {
        if let Some((entity, intersection)) = raycast_source.intersect_top() {
            let tri = intersection.world_triangle();
            let pos = intersection.position();

            // From https://gamedev.stackexchange.com/questions/23743/whats-the-most-efficient-way-to-find-barycentric-coordinates
            let v0: Vec3 = tri.v1 - tri.v0;
            let v1: Vec3 = tri.v2 - tri.v0;
            let v2: Vec3 = pos - tri.v0;
            let d00 = v0.dot(v0);
            let d01 = v0.dot(v1);
            let d11 = v1.dot(v1);
            let d20 = v2.dot(v0);
            let d21 = v2.dot(v1);
            let denom = d00 * d11 - d01 * d01;

            let v = (d11 * d20 - d01 * d21) / denom;
            let w = (d00 * d21 - d01 * d20) / denom;
            let u = 1.0 - v - w;

            // Sanity test
            let reconstructed_pos: Vec3 = tri.v0 * u + tri.v1 * v + tri.v2 * w;
            assert!(vec3_all_eq(reconstructed_pos, pos, 0.01));

            // Calculate uv and update the material with it
            if let Ok((_raycast_mesh, material_handle, mesh_handle, transform)) = raycast_mesh_query.get(entity) {
                if let Some(mesh) = meshes.get(mesh_handle.clone()) {
                    let triangle_indices = calculate_vertex_indices_from_intersection(
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

                        let interpolated_uv = uv_v0 * u + uv_v1 * v + uv_v2 * w;

                        if let Some(material) = my_materials.get_mut(material_handle) {
                            material.highlighted_id = interpolated_uv;
                        }
                    }
                }
            }
        }
    }
}

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

    // load a texture and retrieve its aspect ratio
    let texture_handle = asset_server.load("branding/bevy_logo_dark_big.png");
    let background_handle = asset_server.load("textures/paper_tileable.jpg");
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
    let my_material = my_materials.add(MyMaterial{ 
        color: Color::SEA_GREEN, 
        highlighted_id: Vec2::new(5.0, 5.0), 
        background_texture: background_handle,   
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
            RayCastMethod::CameraCursor(UpdateOn::EveryFrame(Vec2::zero()), EventReader::default()),
        ));
}
