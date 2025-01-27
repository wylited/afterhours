use bevy::{ecs::world::Command, prelude::*};

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (toggle_mask.in_set(AppSet::RecordInput),));
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

fn spawn_mask(
    In(_config): In<SpawnMask>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();

    let shapes = [meshes.add(Rectangle::new(
        window.resolution.width(),
        window.resolution.height(),
    ))];

    for shape in shapes.into_iter() {
        let color = Color::srgba(0.0, 0.0, 0.0, 0.0);

        commands.spawn((
            Mask { state: false },
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(0.0, 0.0, 999.0),
        ));
    }
}

fn toggle_mask(
    input: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut controller_query: Query<(&mut Mask, &MeshMaterial2d<ColorMaterial>)>,
) {
    if input.just_pressed(KeyCode::Space) {
        if let Ok((mut mask, material_handle)) = controller_query.get_single_mut() {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                // Toggle between transparent and semi-transparent
                if !mask.state {
                    material.color = Color::srgba(0.0, 0.0, 0.0, 1.0);
                } else {
                    material.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
                }

                mask.state = !mask.state;
            }
        }
    }
}
