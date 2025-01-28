use bevy::{
    ecs::world::Command,
    prelude::*,
};


use bevy_light_2d::prelude::*;

use crate::AppSet;
use super::player::Player; 


pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Light2dPlugin)
        .add_systems(
            Update,
        
            (toggle_mask.in_set(AppSet::RecordInput), update_light_position),
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
    query: Query<Entity, With<Camera2d>>, 
) {

    if let Ok(camera_entity) = query.get_single() {
        commands.entity(camera_entity).despawn();
    }

    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 1.;

    commands.spawn((
        Camera2d,
        projection,
        AmbientLight2d {
            brightness: 0.0,
            ..default()
        },
        IsDefaultUiCamera
    ));

    commands.spawn((
        Mask {state:true},
        Transform::from_xyz(0.0, 4.0, 1.),
        PointLight2d {
        intensity: 2.0,
        radius: 200.0,
        ..default()
    },
   
    ));
}


fn toggle_mask(input: Res<ButtonInput<KeyCode>>, mut mask_query: Query<(&mut Mask, &mut PointLight2d)>) {
    if input.just_pressed(KeyCode::Space) {
        if let Ok((mut mask, mut PointLight2d)) = mask_query.get_single_mut() {
            mask.state = !mask.state;

            if mask.state {
                PointLight2d.intensity = 3.;
            }
            else {
                PointLight2d.intensity = 0.;
            }
        }
    }
}


fn update_light_position(
    player_query: Query<&Transform, With<Player>>, // Query the player's position
    mut light_query: Query<&mut Transform, (With<Mask>, Without<Player>)>, // Query the light's position
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut light_transform) = light_query.get_single_mut() {
            light_transform.translation = player_transform.translation;
        }
    }
}
