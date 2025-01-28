use bevy::{
    ecs::world::Command,
    prelude::*,
    render::{
        mesh::{MeshVertexBufferLayout, MeshVertexBufferLayoutRef},
        render_resource::*,
        texture::*,
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
};

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<CircleMaskMaterial>::default())
        .add_systems(
            Update,
            (
                toggle_mask.in_set(AppSet::RecordInput),
                update_mask_material,
            ),
        );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[derive(Default)]
pub struct Mask {
    state: bool,
}

#[derive(Debug)]
pub struct SpawnMask {}

impl Command for SpawnMask {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached_with(spawn_mask, self);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CircleMaskMaterial {
    #[uniform(0)] // This matches @binding(0) in the shader
    pub color: Vec4,
    #[uniform(1)] // Add this to match the shader's struct layout
    pub radius: f32,
    #[uniform(2)] // Add this to match the shader's struct layout
    pub screen_size: Vec2,
    #[uniform(3)] // Add this to match the shader's struct layout
    pub enabled: f32,
}

impl Material2d for CircleMaskMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circle_mask.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // Enable alpha blending
        if let Some(fragment) = &mut descriptor.fragment {
            if let Some(target) = &mut fragment.targets[0] {
                target.blend = Some(BlendState::ALPHA_BLENDING);
            }
        }
        Ok(())
    }
}

#[derive(Component)]
pub struct CircleMaskMaterialHandle(Handle<CircleMaskMaterial>);

fn spawn_mask(
    In(_config): In<SpawnMask>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<CircleMaskMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();
    let shape = meshes.add(Rectangle::new(
        window.resolution.width(),
        window.resolution.height(),
    ));

    let material = materials.add(CircleMaskMaterial {
        color: Vec4::new(1.0, 0.0, 0.0, 1.0),
        radius: 1000.0,
        screen_size: Vec2::new(window.resolution.width(), window.resolution.height()),
        enabled: 1.0,
    });

    let material_handle = CircleMaskMaterialHandle(material.clone());

    commands.spawn((
        Mask { state: false },
        material_handle,
        Mesh2d(shape),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 999.0),
    ));
}

fn update_mask_material(
    mask_query: Query<(&Mask, &CircleMaskMaterialHandle)>,
    mut materials: ResMut<Assets<CircleMaskMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();

    for (mask, material_handle) in mask_query.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.enabled = if mask.state { 1.0 } else { 0.0 };
            material.screen_size = Vec2::new(window.resolution.width(), window.resolution.height());
        }
    }
}

fn toggle_mask(input: Res<ButtonInput<KeyCode>>, mut mask_query: Query<&mut Mask>) {
    if input.just_pressed(KeyCode::Space) {
        if let Ok(mut mask) = mask_query.get_single_mut() {
            mask.state = !mask.state;
        }
    }
}

// fn toggle_mask(
//     input: Res<ButtonInput<KeyCode>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut controller_query: Query<(&mut Mask, &MeshMaterial2d<ColorMaterial>)>,
// ) {
//     if input.just_pressed(KeyCode::Space) {
//         if let Ok((mut mask, material_handle)) = controller_query.get_single_mut() {
//             if let Some(material) = materials.get_mut(&material_handle.0) {
//                 // Toggle between transparent and semi-transparent
//                 if !mask.state {
//                     material.color = Color::srgba(0.0, 0.0, 0.0, 1.0);
//                 } else {
//                     material.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
//                 }

//                 mask.state = !mask.state;
//             }
//         }
//     }
// }
