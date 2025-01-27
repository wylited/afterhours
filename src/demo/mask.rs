use bevy::{
    ecs::world::Command,
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    asset_tracking::LoadResource,
    demo::{
        animation::PlayerAnimation,
        movement::{MovementController, ScreenWrap},
    },
    screens::Screen,
    AppSet,
};

#[derive(Debug)]
pub struct SpawnMask {
    
}

impl Command for SpawnMask {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached_with(spawn_mask, self);
    }
}

fn spawn_mask(
    In(config): In<SpawnMask>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shapes = [
        meshes.add(Rectangle::new(500.0, 1000.0)),
    ];

    for (_, shape) in shapes.into_iter().enumerate() {
        let color = Color::srgb(1.0, 0.0, 0.0);

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                100.0,
                10.0,
                0.0,
            ),
        ));

    }


}