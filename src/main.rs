use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
            VertexFormat,
        },
    },
};
use bevy_shader_utils::ShaderUtilsPlugin;

// A full rotation (360deg) in radians
use std::f32::consts::TAU;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .add_system(animate)
        .run();
}

// A "high" random id should be used for custom attributes to ensure consistent sorting and avoid collisions with other attributes.
// See the MeshVertexAttribute docs for more info.
const ATTRIBUTE_BLEND_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Icosphere {
        radius: 0.5,
        subdivisions: 50,
    });
    mesh.insert_attribute(
        ATTRIBUTE_BLEND_COLOR,
        // // ~~The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each~~
        // I'm now using .count_vertices() to get the number of vertices automatically.
        vec![[1.0, 0.0, 0.0, 1.0]; mesh.count_vertices()],
    );

    // cube
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: materials.add(CustomMaterial {
                color: Color::WHITE,
            }),
            ..default()
        })
        .insert(Animated {});

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn animate(mut animated: Query<(&Animated, &mut Transform)>, time: Res<Time>) {
    for (_item, mut transform) in &mut animated {
        transform.rotate_y(0.05 * TAU * time.delta_seconds());
    }
}

#[derive(Component)]
pub struct Animated;

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    // #[uniform(1)]
    // center: Vec3,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_vertex_attribute.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/custom_vertex_attribute.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
