//! Player sprite animation.
use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::{
    audio::SoundEffect,
    demo::{movement::MovementController, player::PlayerAssets},
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAnimation>().add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSet::TickTimers),
            (
                update_animation_movement,
                update_animation_atlas,
                trigger_step_sound_effect,
            )
                .chain()
                .run_if(resource_exists::<PlayerAssets>)
                .in_set(AppSet::Update),
        ),
    );
}

#[derive(Reflect, PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Right = 0,
    Up = 1,
    Left = 2,
    Down = 3,
}

#[derive(Reflect, PartialEq, Clone, Debug)]
pub enum AnimationType {
    Idle(Direction),
    Walk(Direction),
    Sleep,
}

impl AnimationType {
    fn get_config(&self) -> AnimationConfig {
        match self {
            AnimationType::Idle(_) => AnimationConfig {
                frames: 6,
                row_offset: 56,
                frame_duration: Duration::from_millis(500),
            },
            AnimationType::Walk(_) => AnimationConfig {
                frames: 6,
                row_offset: 56 * 2,
                frame_duration: Duration::from_millis(100),
            },
            AnimationType::Sleep => AnimationConfig {
                frames: 6,
                row_offset: 56 * 4,
                frame_duration: Duration::from_millis(1000),
            },
        }
    }

    fn get_direction_offset(&self) -> usize {
        match self {
            AnimationType::Idle(dir) | AnimationType::Walk(dir) => *dir as usize,
            AnimationType::Sleep => 0, // Sleep always faces down
        }
    }
}

struct AnimationConfig {
    frames: usize,
    row_offset: usize,
    frame_duration: Duration,
}

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    animation_type: AnimationType,
    current_direction: Direction,
}

impl PlayerAnimation {
    pub fn new() -> Self {
        Self::with_animation(AnimationType::Idle(Direction::Down))
    }

    pub fn with_animation(animation_type: AnimationType) -> Self {
        let config = animation_type.get_config();
        let direction = match animation_type {
            AnimationType::Idle(dir) | AnimationType::Walk(dir) => dir,
            AnimationType::Sleep => Direction::Down,
        };
        Self {
            timer: Timer::new(config.frame_duration, TimerMode::Repeating),
            frame: 0,
            animation_type,
            current_direction: direction,
        }
    }

    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            let config = self.animation_type.get_config();
            self.frame = (self.frame + 1) % config.frames;
        }
    }

    pub fn update_state(&mut self, new_type: AnimationType) {
        if self.animation_type != new_type {
            let config = new_type.get_config();

            // Keep the current direction if the new animation is not directional
            let direction = match self.animation_type {
                AnimationType::Idle(dir) | AnimationType::Walk(dir) => dir,
                AnimationType::Sleep => Direction::Down,
            };

            *self = Self {
                timer: Timer::new(config.frame_duration, TimerMode::Repeating),
                frame: 0,
                animation_type: new_type,
                current_direction: direction,
            };
        }
    }

    pub fn changed(&self) -> bool {
        self.timer.just_finished()
    }

    pub fn get_atlas_index(&self) -> usize {
        let config = self.animation_type.get_config();
        let direction_offset = self.animation_type.get_direction_offset();
        config.row_offset + (direction_offset * config.frames) + self.frame
    }
}

fn update_animation_movement(mut player_query: Query<(&MovementController, &mut PlayerAnimation)>) {
    for (controller, mut animation) in &mut player_query {
        if controller.intent != Vec2::ZERO {
            // Only update direction if there's movement
            let new_direction = if controller.intent.x.abs() > controller.intent.y.abs() {
                if controller.intent.x > 0.0 {
                    Direction::Right
                } else {
                    Direction::Left
                }
            } else {
                if controller.intent.y > 0.0 {
                    Direction::Up
                } else {
                    Direction::Down
                }
            };

            animation.current_direction = new_direction;
            animation.update_state(AnimationType::Walk(new_direction));
        } else {
            let direction = animation.current_direction.clone();
            animation.update_state(AnimationType::Idle(direction));
        }
    }
}

fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

fn update_animation_atlas(mut query: Query<(&PlayerAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            if animation.changed() {
                atlas.index = animation.get_atlas_index();
            }
        }
    }
}

fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut step_query: Query<&PlayerAnimation>,
) {
    for animation in &mut step_query {
        if let AnimationType::Walk(_) = animation.animation_type {
            if !animation.changed() || !(animation.frame == 2 || animation.frame == 5) {
                continue;
            }

            let rng = &mut rand::thread_rng();
            if let Some(random_step) = player_assets.steps.choose(rng) {
                commands.spawn((
                    AudioPlayer(random_step.clone()),
                    PlaybackSettings::DESPAWN,
                    SoundEffect,
                ));
            }
        }
    }
}
