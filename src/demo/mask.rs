use bevy::{
    ecs::world::Command,
    prelude::*,
};

use bevy_light_2d::prelude::*;

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Light2dPlugin)
        .add_systems(
            Update,
        
            toggle_mask.in_set(AppSet::RecordInput),
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


fn spawn_mask(
    In(_config): In<SpawnMask>,
    mut commands: Commands,
) {

    commands.spawn(PointLight2d {
        intensity: 3.0,
        radius: 100.0,
        ..default()
    });
}


fn toggle_mask(input: Res<ButtonInput<KeyCode>>, mut mask_query: Query<&mut Mask>) {
    if input.just_pressed(KeyCode::Space) {
        if let Ok(mut mask) = mask_query.get_single_mut() {
            mask.state = !mask.state;
        }
    }
}